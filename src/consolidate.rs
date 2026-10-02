pub mod otl;

use crate::logger::ByteStr;
use crate::support::handle::{
    FdHandle, GlyphHandle, Handle, HandleState, handle_from_index, handle_name_eq_bytes,
};


use crate::font::caryll_font::Font;
use crate::support::glyph_order::GlyphOrder;
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, Pos, ShapeId, TableId};

use crate::table::cff::CffTable;
use crate::table::colr::{ColrMapping, ColrTable};

use crate::table::_tsi::{TsiEntry, TsiEntryType, TsiTable};

use crate::table::glyf::{
    ComponentReference, GlyfTable, Glyph, PostscriptHintMask, PostscriptStemDef,
    RefAnchorStatus,
};

use crate::table::otl::{
    Lookup, LookupList, LookupType, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CURSIVE,
    OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK,
    OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING,
    OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE,
    OtlTable, Subtable,
};

use crate::consolidate::otl::chaining::consolidate_chaining;
use crate::consolidate::otl::common::fontop_consolidate_class_def;
use crate::consolidate::otl::gdef::consolidate_gdef;
use crate::consolidate::otl::gpos_cursive::consolidate_gpos_cursive;
use crate::consolidate::otl::gpos_pair::consolidate_gpos_pair;
use crate::consolidate::otl::gpos_single::consolidate_gpos_single;
use crate::consolidate::otl::gsub_ligature::consolidate_gsub_ligature;
use crate::consolidate::otl::gsub_multi::{consolidate_gsub_alternative, consolidate_gsub_multi};
use crate::consolidate::otl::gsub_reverse::consolidate_gsub_reverse;
use crate::consolidate::otl::gsub_single::consolidate_gsub_single;
use crate::consolidate::otl::mark::{consolidate_mark_to_ligature, consolidate_mark_to_single};
use crate::support::glyph_order::{otfcc_gord_consolidate_handle, otfcc_set_glyph_order_by_name};
use crate::table::_tsi::tsi_entry_dup;
use crate::table::glyf::{glyf_component_reference_empty, otfcc_new_glyf_glyph};
use crate::table::otl::{
    otl_feature_list_punch_holes, otl_feature_ref_list_filter_env, otl_lookup_list_punch_holes,
    otl_lookup_ref_list_filter_env,
};
use crate::vf::vq::VQ;
use crate::vf::vq::{vq_get_still, vq_neutral, vq_point_linear_tfm};

// Stage L-7: of the 13 dispatch call sites in `otfcc_consolidate_lookup`
// below, only `consolidate_chaining` (2 of the 13) ever reads anything
// about the table beyond the one `&mut Subtable` it's handed -- the other
// 11 calls (10 distinct functions) never touched `table` at all, so it is
// gone from their signatures entirely. `__declare_otl_consolidation` takes
// `fn_0` as `impl Fn(&Font, &mut Subtable, &Options) -> bool` (not a bare
// `fn` pointer) precisely so `otfcc_consolidate_lookup` can pass a
// capturing closure for the two chaining calls (closing over the
// `lookups`/`self_index`/`self_name` it now receives) while every other
// call site just passes the plain function -- no shared function-pointer
// type needs to carry context none of the other 11 ever used.
fn by_stem_pos(a: &PostscriptStemDef, b: &PostscriptStemDef) -> i32 {
    if a.position == b.position {
        a.map as i32 - b.map as i32
    } else if a.position > b.position {
        1_i32
    } else {
        -1_i32
    }
}
fn by_mask_pointindex(a: &PostscriptHintMask, b: &PostscriptHintMask) -> i32 {
    if a.contours_before as i32 == b.contours_before as i32 {
        a.points_before as i32 - b.points_before as i32
    } else {
        a.contours_before as i32 - b.contours_before as i32
    }
}
fn consolidate_glyph_contours(g: &mut Glyph) {
    // `Vec::retain` visits every element once, in order, regardless of
    // whether earlier ones were kept -- so `j` here tracks the same
    // "original index" the C-shaped loop counted, and dropped contours are
    // freed automatically (a `Contour`'s only owned resources are its
    // points' `VQ` `Vec`s; no `Handle` involved, unlike `ReferenceList`).
    let mut j: ShapeId = 0 as ShapeId;
    let name = &g.name;
    g.contours.retain(|contour| {
        let keep = !contour.is_empty();
        if !keep {
            tracing::warn!("[Consolidate] Removed empty contour #{} in glyph {}.\n", j as i32, ByteStr(name));
        }
        j = j.wrapping_add(1);
        keep
    });
}
fn consolidate_glyph_references(g: &mut Glyph, glyph_order: &GlyphOrder) {
    let name = &g.name;
    g.references.retain_mut(|r| {
        let ok = otfcc_gord_consolidate_handle(glyph_order, &mut r.glyph);
        if !ok {
            tracing::warn!("[Consolidate] Ignored absent glyph component reference /{} within /{}.\n", ByteStr(&r.glyph.name), ByteStr(name));
            // `retain_mut` drops rejected elements itself -- every
            // `ComponentReference` field auto-drops -- so no explicit
            // dispose call is needed here anymore.
        }
        ok
    });
}
fn consolidate_glyph_hints(g: &mut Glyph) {
    if !g.stem_h.is_empty() {
        let stem_h: &mut Vec<PostscriptStemDef> = &mut g.stem_h;
        for (j, stem) in stem_h.iter_mut().enumerate() {
            stem.map = j as u16;
        }
        stem_h.sort_by(|a, b| by_stem_pos(a, b).cmp(&0));
    }
    if !g.stem_v.is_empty() {
        let stem_v: &mut Vec<PostscriptStemDef> = &mut g.stem_v;
        for (j, stem) in stem_v.iter_mut().enumerate() {
            stem.map = j as u16;
        }
        stem_v.sort_by(|a, b| by_stem_pos(a, b).cmp(&0));
    }
    let mut hmap: Vec<ShapeId> = vec![0; g.stem_h.len()];
    let mut vmap: Vec<ShapeId> = vec![0; g.stem_v.len()];
    for (j, stem) in g.stem_h.iter().enumerate() {
        hmap[stem.map as usize] = j as ShapeId;
    }
    for (j, stem) in g.stem_v.iter().enumerate() {
        vmap[stem.map as usize] = j as ShapeId;
    }
    if !g.hint_masks.is_empty() {
        let hint_masks: &mut Vec<PostscriptHintMask> = &mut g.hint_masks;
        hint_masks.sort_by(|a, b| by_mask_pointindex(a, b).cmp(&0));
        for mask in hint_masks.iter_mut() {
            let oldmask: PostscriptHintMask = *mask;
            for (k, &hm) in hmap.iter().enumerate() {
                mask.mask_h[k] = oldmask.mask_h[hm as usize];
            }
            for (k, &vm) in vmap.iter().enumerate() {
                mask.mask_v[k] = oldmask.mask_v[vm as usize];
            }
        }
    }
    if !g.contour_masks.is_empty() {
        let contour_masks: &mut Vec<PostscriptHintMask> = &mut g.contour_masks;
        contour_masks.sort_by(|a, b| by_mask_pointindex(a, b).cmp(&0));
        for mask in contour_masks.iter_mut() {
            let oldmask: PostscriptHintMask = *mask;
            for (k, &hm) in hmap.iter().enumerate() {
                mask.mask_h[k] = oldmask.mask_h[hm as usize];
            }
            for (k, &vm) in vmap.iter().enumerate() {
                mask.mask_v[k] = oldmask.mask_v[vm as usize];
            }
        }
    }
}
fn consolidate_fd_select(h: &mut FdHandle, cff: Option<&CffTable>, gname: &[u8]) {
    let Some(cff) = cff else {
        return;
    };
    if cff.fd_array.is_empty() {
        return;
    }
    let fd_array: &Vec<Box<CffTable>> = &cff.fd_array;
    if h.state == HandleState::Index {
        if h.index as usize >= fd_array.len() {
            h.index = 0 as GlyphId;
        }
        let idx = h.index;
        *h = Handle::new(HandleState::Consolidated, idx, fd_array[idx as usize].font_name.clone()) as FdHandle;
    } else if !h.name.is_empty() {
        let found = fd_array.iter().position(|fd| handle_name_eq_bytes(&h.name, &fd.font_name));
        if let Some(j) = found {
            *h = Handle::new(HandleState::Consolidated, j as GlyphId, fd_array[j].font_name.clone());
        } else {
            tracing::warn!("[Consolidate] CID Subfont {} is not defined. (in glyph /{}).\n", ByteStr(&h.name), ByteStr(gname));
            *h = Handle::default();
        }
    } else if !h.name.is_empty() {
        // Unreachable: the preceding `else if !h.name.is_empty()` already
        // covers this same condition with nothing mutating `h.name` in
        // between -- kept verbatim from the original c2rust translation
        // rather than pruned as drive-by cleanup outside this PR's scope.
        *h = Handle::default();
    }
}
pub fn consolidate_glyph(
    g: &mut Glyph,
    glyph_order: &GlyphOrder,
    cff: Option<&CffTable>,
) {
    consolidate_glyph_contours(g);
    consolidate_glyph_references(g, glyph_order);
    consolidate_glyph_hints(g);
    consolidate_fd_select(&mut g.fd_select, cff, &g.name);
}
// `get_point_coordinates` and `consolidate_anchor_ref` (below) are
// mutually recursive over a composite glyph's `references` graph -- a
// fuzz-found font whose composite glyphs formed a reference cycle (glyph A
// includes B as a component, B includes A) sent this pair recursing
// forever, an AddressSanitizer-confirmed stack overflow. The existing
// `RefAnchorStatus::AnchorConsolidating*` state machine already catches
// cycles specifically in *anchor*-type references (see the "Found
// circular reference..." log below), but `get_point_coordinates`'s own
// walk over a glyph's `references` (searching for point index `n`) had no
// such guard. `depth` counts stack frames across both functions (each
// recursive call, whichever function makes it, increments by exactly 1),
// so this bound covers both recursion paths, not just the anchor one.
// 10 matches TYPE2_SUBR_NESTING's precedent elsewhere in this crate --
// real fonts' composite nesting never approaches double digits, and
// running out of budget only means "point not found" (mirrors the
// existing cycle-detection return), never a wrong-but-silent answer.
pub const MAX_COMPONENT_REFERENCE_DEPTH: u32 = 10;
// Stage M-43 (see RUST_MIGRATION.md): both functions took raw pointers
// because `is_anchored`/`x`/`y` -- the only three `ComponentReference`
// fields either one ever mutates -- lived as plain fields, so mutating one
// while the walk holds a *shared* view of the rest of `table` needed
// `unsafe`. Those three fields are now `Cell<RefAnchorStatus>`/
// `RefCell<VQ>`/`RefCell<VQ>` (see the doc comment on `ComponentReference`
// itself in `table/glyf.rs`), which gives interior mutability through a
// plain shared `&ComponentReference` -- so both functions now take
// `table: &GlyfTable` and `gr`/`rr`: `&ComponentReference`, drop
// `unsafe fn`, and are otherwise byte-for-byte the same walk: the read/
// recurse/mutate order below is unchanged from the raw-pointer version
// this replaced, field for field and branch for branch, since that order
// (not just the final values) is what the existing cycle-detection
// guards' own semantics depend on.
/// `get_point_coordinates`'s three in/out parameters: how far the walk has
/// counted so far (`stated`) and the coordinates it writes once `stated`
/// reaches the target `n` (`x`/`y`). All three are always read and written
/// together at every one of this function's own call sites (never one
/// without the other two), which is exactly what bundling into one `&mut`
/// out-parameter is for -- `clippy::too_many_arguments` flagged the
/// unbundled 8-parameter form this replaces.
pub struct PointSearch {
    pub stated: ShapeId,
    pub x: VQ,
    pub y: VQ,
}
pub fn get_point_coordinates(
    table: &GlyfTable,
    gr: &ComponentReference,
    n: ShapeId,
    search: &mut PointSearch,
    options: &Options,
    depth: u32,
) -> bool {
    if depth >= MAX_COMPONENT_REFERENCE_DEPTH {
        return false;
    }
    let j: GlyphId = gr.glyph.index;
    let g: &Glyph = table[j as usize].as_deref().unwrap();
    // Plain iteration, not `u16` index counters: a contour or reference list
    // is not bounded to 16 bits here, and a counter that wraps at 65,536 can
    // never reach a length of 65,536 (the same shape as the `glyf` hang).
    for p in g.contours.iter().flatten() {
        if search.stated as i32 == n as i32 {
            search.x = vq_point_linear_tfm(
                gr.x.borrow().clone(),
                gr.a as Pos,
                p.x.clone(),
                gr.b as Pos,
                p.y.clone(),
            );
            search.y = vq_point_linear_tfm(
                gr.y.borrow().clone(),
                gr.c as Pos,
                p.x.clone(),
                gr.d as Pos,
                p.y.clone(),
            );
            return true;
        }
        search.stated = (search.stated as i32 + 1_i32) as ShapeId;
    }
    for rr in &g.references {
        consolidate_anchor_ref(table, gr, rr, options, depth + 1);
        let mut ref_0: ComponentReference = (glyf_component_reference_empty)();
        ref_0.glyph = handle_from_index(rr.glyph.index) as GlyphHandle;
        ref_0.a = gr.a * rr.a + rr.b * gr.c;
        ref_0.b = rr.a * gr.b + rr.b * gr.d;
        ref_0.c = gr.a * rr.c + gr.c * rr.d;
        ref_0.d = gr.b * rr.c + rr.d * gr.d;
        ref_0.x = std::cell::RefCell::new(vq_point_linear_tfm(
            rr.x.borrow().clone(),
            rr.a as Pos,
            gr.x.borrow().clone(),
            rr.b as Pos,
            gr.y.borrow().clone(),
        ));
        ref_0.y = std::cell::RefCell::new(vq_point_linear_tfm(
            rr.y.borrow().clone(),
            rr.c as Pos,
            gr.x.borrow().clone(),
            rr.d as Pos,
            gr.y.borrow().clone(),
        ));
        let success: bool = get_point_coordinates(table, &ref_0, n, search, options, depth + 1);
        // `ref_0` is a plain owned local; every field auto-drops when it
        // goes out of scope here (or at the `return true` below), so no
        // explicit dispose call is needed.
        if success {
            return true;
        }
    }
    return false;
}
// See the doc comment on `get_point_coordinates` just above for why this
// function no longer needs `unsafe fn` or raw pointers either.
//
// The one subtlety worth spelling out explicitly (see `RUST_MIGRATION.md`'s
// Stage M-43 entry for the full trace): the two branches below re-read
// `rr.is_anchored.get()` *after* both recursive `get_point_coordinates`
// calls (`s1`/`s2`) have returned, exactly as the raw-pointer version
// re-dereferenced `(*rr).is_anchored` fresh at that point rather than
// reusing a value cached before the recursion -- because a re-entrant call
// that reaches this exact `rr` again during `s1`/`s2` (a real, reachable
// cycle) overwrites `is_anchored` to `Xy` before returning, and this
// function's own final branch has to observe that overwrite the same way
// the original single-address raw pointer did. Caching the pre-recursion
// value in a local here would be a real behavior change, not just a
// cosmetic one.
pub fn consolidate_anchor_ref(
    table: &GlyfTable,
    gr: &ComponentReference,
    rr: &ComponentReference,
    options: &Options,
    depth: u32,
) -> bool {
    if depth >= MAX_COMPONENT_REFERENCE_DEPTH {
        rr.is_anchored.set(RefAnchorStatus::Xy);
        return false;
    }
    if rr.is_anchored.get() == RefAnchorStatus::AnchorConsolidated
        || rr.is_anchored.get() == RefAnchorStatus::Xy
    {
        return true;
    }
    if rr.is_anchored.get() == RefAnchorStatus::AnchorConsolidatingAnchor
        || rr.is_anchored.get() == RefAnchorStatus::AnchorConsolidatingXy
    {
        tracing::warn!("Found circular reference of out-of-range point reference in anchored reference.");
        rr.is_anchored.set(RefAnchorStatus::Xy);
        return false;
    }
    if rr.is_anchored.get() == RefAnchorStatus::AnchorAnchor {
        rr.is_anchored.set(RefAnchorStatus::AnchorConsolidatingAnchor);
    } else {
        rr.is_anchored.set(RefAnchorStatus::AnchorConsolidatingXy);
    }
    let mut outer: PointSearch = PointSearch {
        stated: 0 as ShapeId,
        x: (vq_neutral)(),
        y: (vq_neutral)(),
    };
    let mut inner: PointSearch = PointSearch {
        stated: 0 as ShapeId,
        x: (vq_neutral)(),
        y: (vq_neutral)(),
    };
    let mut rr1: ComponentReference = (glyf_component_reference_empty)();
    rr1.glyph = handle_from_index(rr.glyph.index) as GlyphHandle;
    let s1: bool = get_point_coordinates(table, gr, rr.outer, &mut outer, options, depth + 1);
    let s2: bool = get_point_coordinates(table, &rr1, rr.inner, &mut inner, options, depth + 1);
    if !s1 {
        tracing::warn!("Failed to access point {} in outer glyph.", rr.outer as i32);
    }
    if !s2 {
        tracing::warn!("Failed to access point {} in reference to {}.", rr.outer as i32, ByteStr(&rr.glyph.name));
    }
    let rrx: VQ = vq_point_linear_tfm(
        outer.x.clone(),
        -(rr.a as Pos),
        inner.x.clone(),
        -(rr.b as Pos),
        inner.y.clone(),
    );
    let rry: VQ = vq_point_linear_tfm(
        outer.y.clone(),
        -(rr.c as Pos),
        inner.x.clone(),
        -(rr.d as Pos),
        inner.y.clone(),
    );
    if rr.is_anchored.get() == RefAnchorStatus::AnchorConsolidatingAnchor {
        rr.x.replace(rrx);
        rr.y.replace(rry);
        rr.is_anchored.set(RefAnchorStatus::AnchorConsolidated);
    } else {
        // `f64::abs` is IEEE-754 `fabs` bit for bit (see `vf/vq.rs`'s own
        // note); this file's `unsafe extern "C" { fn fabs(...) }` import
        // (removed in Stage M-45; see RUST_MIGRATION.md) is gone.
        if (vq_get_still(rr.x.borrow().clone()) as f64
            - vq_get_still(rrx.clone()) as f64)
            .abs()
            > 0.5f64
            && (vq_get_still(rr.y.borrow().clone()) as f64
                - vq_get_still(rry.clone()) as f64)
                .abs()
                > 0.5f64
        {
            tracing::warn!("Anchored reference to {} does not match its X/Y offset data.", ByteStr(&rr.glyph.name));
        }
        rr.is_anchored.set(RefAnchorStatus::AnchorConsolidated);
    }
    // `rr1`/`inner`/`outer` (and, in this branch, `rrx`/`rry`) are all plain
    // owned locals that were never moved out -- they auto-drop at the
    // `return false` below, so no explicit dispose calls are needed.
    return false;
}
pub fn consolidate_glyf(font: &mut Font, options: &Options) {
    if font.glyph_order.is_none() || font.glyf.is_none() {
        return;
    }
    let glyph_order: &GlyphOrder = font.glyph_order.as_deref().unwrap();
    let cff: Option<&CffTable> = font.cff.as_deref();
    let glyf: &mut GlyfTable = font.glyf.as_mut().unwrap();
    for slot in glyf.iter_mut() {
        if let Some(glyph) = slot {
            consolidate_glyph(glyph, glyph_order, cff);
        } else {
            *slot = Some(otfcc_new_glyf_glyph());
        }
    }
    // `consolidate_anchor_ref` recurses over the reference graph and can
    // revisit *any* glyph in the table (not just the one being processed)
    // while resolving anchor points -- but it mutates only
    // `ComponentReference.is_anchored`/`x`/`y`, now `Cell`/`RefCell` (see
    // that struct's own doc comment and `get_point_coordinates`'s/
    // `consolidate_anchor_ref`'s in this file), so a single shared `&
    // GlyfTable` reference for the whole walk below is all either function
    // needs: no raw pointer, and no `unsafe`.
    let table: &GlyfTable = glyf;
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < table.len() {
        let g: &Glyph = table[j_0 as usize].as_deref().unwrap();
        let stage = crate::logger::stage(ByteStr(&g.name));
        let mut gr: ComponentReference = (glyf_component_reference_empty)();
        gr.glyph = handle_from_index(j_0) as GlyphHandle;
        for rr in &g.references {
            consolidate_anchor_ref(table, &gr, rr, options, 0);
        }
        // `gr` is a plain owned local; every field auto-drops when it
        // goes out of scope at the end of this iteration, so no
        // explicit dispose call is needed.
        stage.finish();
        j_0 = j_0.wrapping_add(1);
    }
}
pub fn consolidate_cmap(font: &mut Font) {
    let glyph_order: Option<&GlyphOrder> = font.glyph_order.as_deref();
    if let Some(glyph_order) = glyph_order.filter(|_| font.cmap.is_some()) {
        // A failed resolution disposes the entry's `Handle` in place
        // (leaving it in the map with an empty name) rather than
        // removing the entry -- `dump_cmap`'s "skip if name is null"
        // check is what actually hides it later.
        for (&unicode, glyph) in font.cmap.as_mut().unwrap().unicodes.iter_mut() {
            if !otfcc_gord_consolidate_handle(glyph_order, glyph) {
                tracing::warn!("[Consolidate] Ignored mapping U+{:04X} to non-existent glyph /{}.\n", unicode as u32, ByteStr(&glyph.name));
                *glyph = Handle::default();
            }
        }
    }
    if let Some(glyph_order) = glyph_order.filter(|_| font.cmap.is_some()) {
        for (key, glyph) in font.cmap.as_mut().unwrap().uvs.iter_mut() {
            if !otfcc_gord_consolidate_handle(glyph_order, glyph) {
                tracing::warn!("[Consolidate] Ignored UVS mapping [U+{:04X} U+{:04X}] to non-existent glyph /{}.\n", key.unicode, key.selector, ByteStr(&glyph.name));
                *glyph = Handle::default();
            }
        }
    }
}
fn __declare_otl_consolidation(
    type_0: LookupType,
    fn_0: impl Fn(&GlyphOrder, &mut Subtable) -> bool,
    glyph_order: &GlyphOrder,
    lookup: &mut Lookup,
) {
    if lookup.subtables.is_empty() || lookup.type_0 != type_0 {
        return;
    }
    let stage = crate::logger::stage(ByteStr(&lookup.name));
    // The "Ignored empty subtable" warning below can fire up to 300,000
    // times for a font whose subtables mostly fail to parse (a lookup can
    // hold up to `MAX_TOTAL_SUBTABLES_PER_LOOKUP` (1,000) subtables and a
    // table up to `MAX_TOTAL_LOOKUPS_PER_TABLE` (300) lookups); CI fuzz
    // found exactly this shape. `tracing::warn!` formats nothing when
    // warnings are not being printed, so that volume costs nothing then.
    for (j, slot) in lookup.subtables.iter_mut().enumerate() {
        if slot.is_none() {
            tracing::warn!("[Consolidate] Ignored empty subtable {} of lookup {}.\n", j as i32, ByteStr(&lookup.name));
        } else {
            let sub = slot.as_deref_mut().unwrap();
            let subtable_removed = fn_0(glyph_order, sub);
            if subtable_removed {
                // Was a `fndel: SubtableRemover` parameter, one
                // `LookupType`-keyed function pointer per call site
                // below, each `transmute`d from `*mut ConcreteType` to
                // `*mut Subtable` -- sound only because `Subtable` used
                // to be a union with no discriminant to misinterpret.
                // Now that it is an enum, `Subtable`'s own `Drop` does
                // this dispatch, self-describing off the enum's tag, so
                // setting the slot to `None` (dropping the `Box` in
                // place) is all that is needed -- no per-type function
                // pointer, no separate explicit `Box::from_raw`.
                *slot = None;
                tracing::warn!("[Consolidate] Ignored empty subtable {} of lookup {}.\n", j as i32, ByteStr(&lookup.name));
            }
        }
    }
    // `Vec::retain` drops every discarded `Box<Subtable>` in place (its
    // `Drop` runs as part of the retain-internal shift), the same thing
    // the old manual `.take()`-then-`truncate()` two-pass compaction did
    // by hand -- no risk of the double-owned-`Box` hazard that reasoning
    // used to warn about, since `retain` never leaves two slots pointing
    // at the same allocation to begin with.
    lookup.subtables.retain(|s| s.is_some());
    if lookup.subtables.is_empty() {
        tracing::warn!("[Consolidate] Lookup {} is empty and will be removed.\n", ByteStr(&lookup.name));
    }
    stage.finish();
}
pub fn otfcc_consolidate_lookup(
    glyph_order: &GlyphOrder,
    lookups: &LookupList,
    self_index: TableId,
    self_name: &[u8],
    lookup: &mut Lookup,
    options: &Options,
) {
    __declare_otl_consolidation(OTL_TYPE_GSUB_SINGLE, consolidate_gsub_single, glyph_order, lookup);
    __declare_otl_consolidation(OTL_TYPE_GSUB_MULTIPLE, consolidate_gsub_multi, glyph_order, lookup);
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_ALTERNATE,
        consolidate_gsub_alternative,
        glyph_order,
        lookup,
    );
    __declare_otl_consolidation(OTL_TYPE_GSUB_LIGATURE, consolidate_gsub_ligature, glyph_order, lookup);
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_CHAINING,
        |f, sub| consolidate_chaining(f, lookups, self_index, self_name, sub, options),
        glyph_order,
        lookup,
    );
    __declare_otl_consolidation(OTL_TYPE_GSUB_REVERSE, consolidate_gsub_reverse, glyph_order, lookup);
    __declare_otl_consolidation(OTL_TYPE_GPOS_SINGLE, consolidate_gpos_single, glyph_order, lookup);
    __declare_otl_consolidation(OTL_TYPE_GPOS_PAIR, consolidate_gpos_pair, glyph_order, lookup);
    __declare_otl_consolidation(OTL_TYPE_GPOS_CURSIVE, consolidate_gpos_cursive, glyph_order, lookup);
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_CHAINING,
        |f, sub| consolidate_chaining(f, lookups, self_index, self_name, sub, options),
        glyph_order,
        lookup,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_MARK_TO_BASE,
        consolidate_mark_to_single,
        glyph_order,
        lookup,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_MARK_TO_MARK,
        consolidate_mark_to_single,
        glyph_order,
        lookup,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_MARK_TO_LIGATURE,
        consolidate_mark_to_ligature,
        glyph_order,
        lookup,
    );
}
// Stage L-7: `table` is a real `&mut OtlTable` now, not a raw pointer --
// closing the aliasing hazard the plan doc flagged this stage for. The one
// wrinkle: `otfcc_consolidate_lookup`'s call into `consolidate_chaining`
// still needs read access to *every* lookup, including the very one whose
// `&mut Lookup` this loop is holding at the time (a chaining rule can name
// its own containing lookup -- `k == self_index` below). A blanket
// `&table.lookups` alongside a live `&mut Lookup` borrowed from inside
// that same `Vec` is a real, ordinary (not just Stacked-Borrows-flavored)
// borrow-checker conflict -- there is no way around it by index alone.
// `Option::take()` resolves it: physically remove the lookup being
// processed from its slot (leaving a plain `None` there, not a dangling
// borrow) before handing out `&table.lookups`, then put it back
// afterwards. A naive version of this (deliberately *not* what this does)
// would silently break self-reference -- with the lookup missing from the
// list, a name/index scan that includes itself would come up empty, and
// `consolidate_chaining` would treat a real self-reference as an invalid
// lookup and discard it, a genuine output regression. `self_index`/
// `self_name` (the latter cloned *before* the `take`, since it borrows
// from the very value about to be reborrowed mutably) are threaded down
// so `consolidate_chaining` can special-case exactly that slot instead of
// reading it (as `None`) from `lookups`.
fn consolidate_otl_table(glyph_order: Option<&GlyphOrder>, table: Option<&mut OtlTable>, options: &Options) {
    // Every lookup consolidator below reads exactly one thing from the font:
    // its glyph order (checked across `consolidate/otl/` -- nothing else).
    // So this takes `glyph_order`, not the `Font`, which is what lets the
    // caller borrow `font.glyph_order` and `font.gsub`/`.gpos` (disjoint
    // fields) at the same time without a raw pointer.
    let Some(glyph_order) = glyph_order else {
        return;
    };
    let Some(table) = table else {
        return;
    };
    loop {
        for j in 0..table.lookups.len() {
            // A hole here (`None`) means a previous iteration of this
            // same fixed-point loop already punched it -- nothing left
            // to consolidate at this slot.
            let mut current = table.lookups[j].take();
            if let Some(lookup) = current.as_deref_mut() {
                let self_name = lookup.name.clone();
                otfcc_consolidate_lookup(
                    glyph_order,
                    &table.lookups,
                    j as TableId,
                    &self_name,
                    lookup,
                    options,
                );
            }
            table.lookups[j] = current;
        }
        for feature in table.features.iter_mut().flatten() {
            otl_lookup_ref_list_filter_env(&mut feature.lookups, &table.lookups, |lut| {
                lut.is_some_and(|l| !l.subtables.is_empty())
            });
        }
        for lang in table.languages.iter_mut() {
            // `required_feature` is a single borrowed `Option<FeatureIdx>`,
            // not a list element `otl_feature_ref_list_filter_env` (below)
            // ever touches -- it was set once at parse time and otherwise
            // never revisited. Below, this same pass drops every `Feature`
            // whose `.lookups` is empty from `table.features` (punching a
            // hole where its `Box` used to live); a `required_feature`
            // still pointing at one of those becomes a dangling read the
            // very next time this language is dumped or built.
            // `feature_ref_is_not_empty`'s check (`.lookups.is_empty()`) is
            // applied here too, so a `required_feature` is cleared in the
            // same pass, by the same rule, as every other reference to
            // that feature -- closing a real fuzzer-found use-after-free
            // (heap-use-after-free reading a freed `Feature`'s `name` from
            // `otfcc_dump_otl`, ASan-confirmed). `feature_at` resolving to
            // `None` (an out-of-range index, never expected here, or a
            // hole punched by an *earlier* iteration of this same loop)
            // is treated the same as "empty": either way, nothing valid to
            // require.
            if let Some(rf) = lang.required_feature {
                let target_empty = crate::table::otl::feature_at(&table.features, rf)
                    .is_none_or(|f| f.lookups.is_empty());
                if target_empty {
                    lang.required_feature = None;
                }
            }
            otl_feature_ref_list_filter_env(&mut lang.features, &table.features, |feat| {
                feat.is_some_and(|f| !f.lookups.is_empty())
            });
        }
        // A hole-preserving `Vec` never shrinks, unlike the old
        // `Vec::retain`-based compaction this replaces -- `punched_lookups`/
        // `punched_features` are the explicit "did this pass change
        // anything" signal the fixed-point loop below now watches instead
        // of `.len()`.
        let punched_lookups =
            otl_lookup_list_punch_holes(&mut table.lookups, |lut| !lut.subtables.is_empty());
        let punched_features =
            otl_feature_list_punch_holes(&mut table.features, |feat| !feat.lookups.is_empty());
        if !punched_lookups && !punched_features {
            break;
        }
    }
}
fn consolidate_otl(font: &mut Font, options: &Options) {
    let glyph_order = font.glyph_order.as_deref();
    let stage = crate::logger::stage("GSUB");
    {
        consolidate_otl_table(glyph_order, font.gsub.as_deref_mut(), options);
        stage.finish();
    }
    let stage = crate::logger::stage("GPOS");
    {
        consolidate_otl_table(glyph_order, font.gpos.as_deref_mut(), options);
        stage.finish();
    }
    let stage = crate::logger::stage("GDEF");
    {
        consolidate_gdef(glyph_order, font.gdef.as_deref_mut());
        stage.finish();
    }
}
fn consolidate_colr(font: &mut Font) {
    if font.colr.is_none() || font.glyph_order.is_none() {
        return;
    }
    // Guaranteed `Some` by the early return above.
    let glyph_order: &GlyphOrder = font.glyph_order.as_deref().unwrap();
    let mut consolidated: ColrTable = Vec::new();
    let source: &mut Vec<ColrMapping> = font.colr.as_mut().unwrap();
    for mapping in source.iter_mut() {
        if !otfcc_gord_consolidate_handle(glyph_order, &mut mapping.glyph) {
            tracing::warn!("[Consolidate] Ignored missing glyph of /{}", ByteStr(&mapping.glyph.name));
        } else {
            let mut m: ColrMapping = ColrMapping {
                glyph: mapping.glyph.clone(),
                layers: Vec::new(),
            };
            for layer in mapping.layers.iter_mut() {
                if !otfcc_gord_consolidate_handle(glyph_order, &mut layer.glyph) {
                    tracing::warn!("[Consolidate] Ignored missing glyph of /{}", ByteStr(&layer.glyph.name));
                } else {
                    m.layers.push(layer.clone());
                }
            }
            if !mapping.layers.is_empty() {
                consolidated.push(m);
            } else {
                tracing::warn!("[Consolidate] COLR decomposition for /{} is empth", ByteStr(&mapping.glyph.name));
                // `m` is dropped here (its `Handle` and `layers: Vec<ColrLayer>`
                // freed by their own compiler-generated drop glue) rather than
                // pushed into `consolidated` -- no manual dispose call needed.
            }
        }
    }
    font.colr = Some(consolidated);
}
// Takes `glyf`/`glyph_order` as separate borrowed pieces (rather than
// `font: &mut Font` alongside `tsi: &mut Option<TsiTable>`, which would
// be two simultaneous borrows of the same `Font` whenever a caller
// passes `&mut font.tsi_01`/`&mut font.tsi_23`) -- the caller
// (`otfcc_consolidate_font`) borrows `font.glyf`/`font.glyph_order`
// (shared) and `font.tsi_01`/`font.tsi_23` (mutable, one at a time) as
// disjoint fields directly off `font`, which Rust allows even though a
// single `&Font`/`&mut Font` funneled through this function's own
// parameter list would not.
fn consolidate_tsi(glyf: &GlyfTable, glyph_order: &GlyphOrder, tsi: &mut Option<TsiTable>) {
    if tsi.is_none() {
        return;
    }
    let mut consolidated: TsiTable = Vec::new();
    // `Option<Vec<u8>>` per slot preserves the old null/non-null
    // distinction (`None` = "no entry yet for this GID", `Some` = has
    // content, even if empty) that the raw `*mut SdsRaw` array's
    // `is_null()` checks relied on -- a plain assignment below correctly
    // drops whatever was there before, so the old explicit
    // free-before-overwrite is now implicit.
    let mut gid_entries: Vec<Option<Vec<u8>>> = vec![None; glyf.len()];
    let entries: &mut Vec<TsiEntry> = tsi.as_mut().unwrap();
    for entry in entries.iter_mut() {
        if entry.type_0 == TsiEntryType::Glyph {
            if otfcc_gord_consolidate_handle(glyph_order, &mut entry.glyph) {
                gid_entries[entry.glyph.index as usize] =
                    Some(::core::mem::take(&mut entry.content));
            } else {
                tracing::warn!("[Consolidate] Ignored missing glyph of /{}", ByteStr(&entry.glyph.name));
            }
        } else {
            // `tsi_entry_dup` is a safe fn now that this stack includes
            // #364's `_tsi.rs` conversion.
            consolidated.push(tsi_entry_dup(entry));
        }
    }
    for (j, entry) in gid_entries.iter_mut().enumerate() {
        let mut e_0: TsiEntry = TsiEntry {
            type_0: TsiEntryType::Glyph,
            glyph: Handle::new(HandleState::Empty, 0, Vec::new()),
            content: Vec::new(),
        };
        e_0.type_0 = TsiEntryType::Glyph;
        e_0.glyph = handle_from_index(j as GlyphId) as GlyphHandle;
        otfcc_gord_consolidate_handle(glyph_order, &mut e_0.glyph);
        e_0.content = entry.take().unwrap_or_default();
        consolidated.push(e_0);
    }
    consolidated.sort_by(|a, b| {
        (a.type_0 as u32)
            .cmp(&(b.type_0 as u32))
            .then(a.glyph.index.cmp(&b.glyph.index))
    });
    // Old `tsi` (the previous value) drops naturally here, when this
    // assignment overwrites it -- no explicit `table_tsi_free` needed.
    *tsi = Some(consolidated);
}
pub fn otfcc_consolidate_font(font: &mut Font, options: &Options) {
    // See `Options::consolidate_warning_budget`'s own doc comment: reset
    // once per font here, not just once at `Options` creation, since one
    // `Options` can drive many conversions over its life.
    options
        .consolidate_warning_budget
        .set(crate::consolidate::otl::chaining::CONSOLIDATE_WARNING_BUDGET);
    // `font.glyf` itself is never reassigned anywhere below (individual
    // glyph slots inside it may be, but the `Option` wrapping the whole
    // table is not), so capturing "is a glyf table present at all" once,
    // up front, is equivalent to re-checking `font.glyf.is_some()` at
    // each point the original raw-pointer version did.
    let has_glyf = font.glyf.is_some();
    if has_glyf && font.glyph_order.is_none() {
        // Built directly via `Box::new`, not `OTFCC_PKG_GLYPH_ORDER.create`
        // (`malloc`) + `Box::from_raw` -- `Box::from_raw` requires the
        // pointer to have come from Rust's global allocator, which a bare
        // libc `malloc` is not guaranteed to match. `go` borrows `go_box`
        // for the rest of this block (unchanged from here down), matching
        // the `GaspTable`/`CmapTable` "accumulator is `Option<Box<X>>`/
        // `Box<X>` from the start" idiom.
        let mut go_box: Box<GlyphOrder> = Box::new(GlyphOrder {
            entries: Vec::new(),
            by_gid: ::std::collections::BTreeMap::new(),
            by_name: ::std::collections::HashMap::new(),
        });
        let go: &mut GlyphOrder = go_box.as_mut();
        let glyf: &mut GlyfTable = font.glyf.as_mut().unwrap();
        for (gid, slot) in glyf.iter_mut().enumerate() {
            let g = slot.as_mut().unwrap();
            let gid = gid as GlyphId;
            let name: Vec<u8> = if g.name.is_empty() {
                let name = crate::bytesbuild!(b"$$gid", gid as i32);
                g.name = name.clone();
                name
            } else {
                g.name.clone()
            };
            // `.clone()`, not a move: `otfcc_set_glyph_order_by_name` always
            // consumes its own copy (no ownership contract to track any
            // more -- see its doc comment), but `name` is still needed
            // below regardless of whether this call succeeds or fails, for
            // the log message and/or the retry loop.
            if !otfcc_set_glyph_order_by_name(go, name.clone(), gid) {
                tracing::warn!("[Consolidate] Glyph name {} is already in use.", ByteStr(&name));
                let mut suffix: u32 = 2_u32;
                let mut success: bool;
                loop {
                    let newname: Vec<u8> = crate::bytesbuild!(&name, b"_", suffix);
                    success = otfcc_set_glyph_order_by_name(go, newname.clone(), gid);
                    if !success {
                        suffix = suffix.wrapping_add(1_u32);
                    } else {
                        tracing::warn!("[Consolidate] Glyph {} is renamed into {}.", ByteStr(&name), ByteStr(&newname));
                        g.name = newname;
                    }
                    if success {
                        break;
                    }
                }
            }
        }
        font.glyph_order = Some(go_box);
    }
    let stage = crate::logger::stage("glyf");
    consolidate_glyf(font, options);
    stage.finish();
    let stage = crate::logger::stage("cmap");
    consolidate_cmap(font);
    stage.finish();
    if has_glyf {
        // The lookup consolidators read exactly one thing from the font --
        // its glyph order -- so `consolidate_otl` splits `font.glyph_order`
        // off from `font.gsub`/`.gpos`/`.gdef` (disjoint fields) and hands
        // each piece to the safe dispatch. Was `unsafe fn` over a
        // `font: *mut Font`, on the belief (Stage L-7's note) that this
        // needed the `Font` borrows split apart in a way that was out of
        // scope; it turned out to need only that one field.
        consolidate_otl(font, options);
    }
    let stage = crate::logger::stage("COLR");
    consolidate_colr(font);
    stage.finish();
    let stage = crate::logger::stage("TSI_01");
    if let (Some(glyf), Some(glyph_order)) = (font.glyf.as_ref(), font.glyph_order.as_deref()) {
        consolidate_tsi(glyf, glyph_order, &mut font.tsi_01);
    }
    stage.finish();
    let stage = crate::logger::stage("TSI_23");
    if let (Some(glyf), Some(glyph_order)) = (font.glyf.as_ref(), font.glyph_order.as_deref()) {
        consolidate_tsi(glyf, glyph_order, &mut font.tsi_23);
    }
    stage.finish();
    let stage = crate::logger::stage("TSI5");
    fontop_consolidate_class_def(font.glyph_order.as_deref(), font.tsi5.as_deref_mut());
    stage.finish();
}

#[cfg(test)]
mod consolidate_otl_table_tests {
    use super::*;
    use crate::support::handle::{Handle, HandleState, LookupHandle};
    use crate::table::otl::{
        ChainLookupApplication, ChainingRule, ChainingSubtable, FeatureIdx, LookupIdx, new_feature,
        new_language, new_lookup,
    };

    fn empty_font_with_glyph_order() -> Box<Font> {
        Box::new(Font {
            subtype: crate::font::caryll_font::FontSubtype::Ttf,
            fvar: None,
            head: None,
            hhea: None,
            maxp: None,
            os_2: None,
            hmtx: None,
            post: None,
            hdmx: None,
            vhea: None,
            vmtx: None,
            vorg: None,
            cff: None,
            glyf: None,
            cmap: None,
            name: None,
            meta: None,
            fpgm: None,
            prep: None,
            cvt_: None,
            gasp: None,
            vdmx: None,
            ltsh: None,
            gsub: None,
            gpos: None,
            gdef: None,
            base: None,
            cpal: None,
            colr: None,
            svg: None,
            tsi_01: None,
            tsi_23: None,
            tsi5: None,
            glyph_order: Some(Box::new(GlyphOrder {
                entries: Vec::new(),
                by_gid: Default::default(),
                by_name: Default::default(),
            })),
        })
    }

    // The fuzzer-found, ASan-confirmed bug this pins down: a
    // `LanguageSystem.required_feature` is a lone borrowed `*const Feature`
    // into `table.features` that nothing here used to revisit once set at
    // parse time. When the `Feature` it points at ends up with no valid
    // lookups (every lookup referencing it turned out to have zero usable
    // subtables) and gets dropped from `table.features` by this same
    // consolidation pass, `required_feature` was left dangling -- read
    // later by `otfcc_dump_otl`/the build path, an actual heap-use-after-
    // free (confirmed via a debug-std ASan build: `AddressSanitizer:
    // heap-use-after-free ... freed by ... otl_feature_list_filter_env ...
    // READ of size 8 ... in otfcc_dump_otl`). `lang.features` (the *list* of
    // borrowed feature refs) was already correctly pruned in this same
    // pass; `required_feature` (the lone one) was not.
    #[test]
    fn required_feature_pointing_at_a_lookup_with_no_valid_subtables_is_cleared_not_left_dangling() {
        // `LookupIdx(0)`/`FeatureIdx(0)` reference the one lookup/feature
        // slot below directly -- no raw pointers or `unsafe` needed to
        // build this fixture anymore, now that the cross-references are
        // plain indices rather than borrows into a `Box` this test would
        // otherwise have to keep pinned in place.
        let mut table = Box::new(OtlTable {
            lookups: vec![Some(new_lookup())], // subtables empty -- "no valid subtables"
            features: Vec::new(),
            languages: Vec::new(),
        });

        let mut feature = new_feature();
        feature.lookups.push(LookupIdx(0));
        table.features.push(Some(feature));

        let mut lang = new_language();
        lang.required_feature = Some(FeatureIdx(0));
        lang.features.push(FeatureIdx(0));
        table.languages.push(lang);

        let font = empty_font_with_glyph_order();
        let options = Options::default();

        consolidate_otl_table(font.glyph_order.as_deref(), Some(table.as_mut()), &options);

        assert!(table.languages[0].required_feature.is_none());
        // Consolidation now punches holes instead of compacting -- an
        // emptied-out `table.features`/`.lookups` still has one slot each,
        // just `None` rather than removed outright.
        assert!(table.features.iter().all(Option::is_none));
        assert!(table.lookups.iter().all(Option::is_none));
    }

    // `consolidate_otl_table` takes the font's glyph order alone (the only
    // thing any lookup consolidator reads), and returns without touching the
    // table when there is none. `otfcc_consolidate_font` cannot actually
    // reach it that way today -- it errors out earlier for `glyf` without a
    // glyph order -- so no fixture exercises the guard, which is exactly why
    // it is pinned here: an emptied-out lookup would be punched away below
    // if the early return were ever lost.
    #[test]
    fn without_a_glyph_order_the_otl_table_is_left_untouched() {
        let mut table = Box::new(OtlTable {
            lookups: vec![Some(new_lookup())], // no subtables: would be punched if visited
            features: Vec::new(),
            languages: Vec::new(),
        });
        let options = Options::default();

        consolidate_otl_table(None, Some(table.as_mut()), &options);

        assert!(table.lookups[0].is_some());
    }

    fn self_referencing_chaining_lookup(lookup_type: LookupType, app_lookup: LookupHandle) -> Box<Lookup> {
        let mut lookup = new_lookup();
        lookup.name = b"self_ref_lookup".to_vec();
        lookup.type_0 = lookup_type;
        lookup.subtables.push(Some(Box::new(Subtable::Chaining(
            ChainingSubtable::Canonical(ChainingRule {
                match_count: 0,
                input_begins: 0,
                input_ends: 0,
                match_0: Vec::new(),
                apply: vec![ChainLookupApplication {
                    index: 0,
                    lookup: app_lookup,
                }],
            }),
        ))));
        lookup
    }

    // Stage L-7's own reason for existing: `consolidate_otl_table` now
    // `take()`s the lookup being processed out of `table.lookups` before
    // handing `consolidate_chaining` a shared `&LookupList` (so that
    // shared borrow can't alias the `&mut Subtable` also being threaded
    // through), which means a naive scan for "does lookup k exist" would
    // see the current lookup's own slot as an empty hole. A chaining rule
    // whose one lookup application names its own containing lookup (a
    // real OpenType idiom, e.g. an iterative contextual substitution) is
    // exactly the case that would silently break: without the `self_index`/
    // `self_name` special-casing this test pins down, the self-reference
    // would be misdiagnosed as an invalid lookup and discarded.
    #[test]
    fn chaining_rule_naming_its_own_lookup_by_name_resolves_instead_of_being_invalidated() {
        let lookup = self_referencing_chaining_lookup(
            OTL_TYPE_GSUB_CHAINING,
            Handle::new(HandleState::Name, 0, b"self_ref_lookup".to_vec()),
        );
        let mut table = Box::new(OtlTable {
            lookups: vec![Some(lookup)],
            features: Vec::new(),
            languages: Vec::new(),
        });
        let font = empty_font_with_glyph_order();
        let options = Options::default();

        consolidate_otl_table(font.glyph_order.as_deref(), Some(table.as_mut()), &options);

        let resolved = table.lookups[0]
            .as_deref()
            .expect("the self-referencing lookup itself must survive consolidation");
        let Subtable::Chaining(ChainingSubtable::Canonical(rule)) =
            resolved.subtables[0].as_deref().unwrap()
        else {
            unreachable!()
        };
        assert_eq!(
            rule.apply.len(),
            1,
            "the self-referencing apply entry must not have been dropped as invalid"
        );
        assert_eq!(rule.apply[0].lookup.state, HandleState::Consolidated);
        assert_eq!(rule.apply[0].lookup.index, 0);
        assert_eq!(rule.apply[0].lookup.name, b"self_ref_lookup");
    }

    // Same self-reference case as above, but through the index-based
    // resolution branch (`HandleState::Index`) instead of the name-based
    // one -- both branches independently special-case `self_index`. The
    // self-referencing lookup is deliberately at index 1, not 0: the
    // "unresolvable index" fallback also resets to index 0, so a
    // self-index of 0 would make a broken self-index special case
    // indistinguishable from a correctly-handled one (both end up
    // pointing at index 0) -- this placement is what actually exercises
    // the bug this test exists to catch.
    #[test]
    fn chaining_rule_naming_its_own_lookup_by_index_resolves_instead_of_being_invalidated() {
        let mut lookup = self_referencing_chaining_lookup(
            OTL_TYPE_GPOS_CHAINING,
            Handle::new(HandleState::Index, 1, Vec::new()),
        );
        lookup.subtables[0]
            .as_deref_mut()
            .map(|s| {
                let Subtable::Chaining(ChainingSubtable::Canonical(rule)) = s else {
                    unreachable!()
                };
                rule.apply[0].index = 1;
            })
            .unwrap();
        let mut table = Box::new(OtlTable {
            lookups: vec![Some(new_lookup()), Some(lookup)],
            features: Vec::new(),
            languages: Vec::new(),
        });
        let font = empty_font_with_glyph_order();
        let options = Options::default();

        consolidate_otl_table(font.glyph_order.as_deref(), Some(table.as_mut()), &options);

        let resolved = table.lookups[1]
            .as_deref()
            .expect("the self-referencing lookup itself must survive consolidation");
        let Subtable::Chaining(ChainingSubtable::Canonical(rule)) =
            resolved.subtables[0].as_deref().unwrap()
        else {
            unreachable!()
        };
        assert_eq!(rule.apply.len(), 1);
        assert_eq!(rule.apply[0].lookup.state, HandleState::Consolidated);
        assert_eq!(rule.apply[0].lookup.index, 1);
        assert_eq!(rule.apply[0].lookup.name, b"self_ref_lookup");
    }
}

#[cfg(test)]
mod composite_reference_cycle_tests {
    use super::*;

    // A fuzz-found font whose composite glyphs formed a reference cycle
    // (glyph 0 includes glyph 1 as a component, glyph 1 includes glyph 0)
    // sent get_point_coordinates/consolidate_anchor_ref recursing forever
    // -- an AddressSanitizer-confirmed stack overflow (found by CI's fuzz
    // job on an unrelated PR). Two glyphs, each with one plain (non-
    // anchor) reference to the other -- no anchor points needed to
    // reproduce get_point_coordinates's own unbounded self-recursion.
    fn cyclic_glyf_table() -> GlyfTable {
        let mut g0 = otfcc_new_glyf_glyph();
        g0.references.push(reference_to(1));
        let mut g1 = otfcc_new_glyf_glyph();
        g1.references.push(reference_to(0));
        vec![Some(g0), Some(g1)]
    }

    fn reference_to(gid: GlyphId) -> ComponentReference {
        let mut r = glyf_component_reference_empty();
        r.glyph = handle_from_index(gid);
        r.a = 1.;
        r.d = 1.;
        r
    }

    #[test]
    fn get_point_coordinates_stops_at_a_reference_cycle_instead_of_overflowing_the_stack() {
        {
            let table = cyclic_glyf_table();
            let options = Options::default();
            let gr = reference_to(0);
            let mut search = PointSearch {
                stated: 0,
                x: vq_neutral(),
                y: vq_neutral(),
            };
            // Point index 999 doesn't exist anywhere in this table, so a
            // correctly-terminating search must walk every reachable
            // reference (following the cycle up to the depth budget) and
            // then report "not found" -- reaching this assertion at all,
            // rather than the test process crashing, is the regression
            // signal.
            let found = get_point_coordinates(&table, &gr, 999, &mut search, &options, 0);
            assert!(!found);
        }
    }

    #[test]
    fn consolidate_anchor_ref_stops_at_a_reference_cycle_instead_of_overflowing_the_stack() {
        {
            let table = cyclic_glyf_table();
            let options = Options::default();
            let gr = reference_to(0);
            let mut rr = reference_to(1);
            rr.is_anchored.set(RefAnchorStatus::AnchorAnchor);
            rr.outer = 999;
            rr.inner = 999;
            // `consolidate_anchor_ref` always returns `false` at its own
            // end regardless of whether the anchor points it looked for
            // were found -- reaching this assertion at all (rather than
            // the test process crashing) is the regression signal. The
            // point search inside it (via get_point_coordinates) still
            // has to walk the cycle up to the depth budget before giving
            // up, which is exactly the path that used to overflow.
            let resolved = consolidate_anchor_ref(&table, &gr, &rr, &options, 0);
            assert!(!resolved);
            assert_eq!(rr.is_anchored.get(), RefAnchorStatus::AnchorConsolidated);
        }
    }
}

#[cfg(test)]
mod reference_count_tests {
    use super::*;
    use crate::json_reader::read_json;
    use crate::support::parsed_json::parse_json;

    /// A font whose glyph `a` is a composite of `count` references to `b`.
    ///
    /// `read_json` rejects a collection past 65,535 members, so the extra
    /// references are added after reading: the consolidation loops must
    /// terminate on their own, not only because the JSON reader keeps such a
    /// glyph out.
    fn font_with_reference_count(count: usize) -> Box<Font> {
        let json = r#"{"glyf":{"a":{"advanceWidth":1,"references":[{"glyph":"b","x":0,"y":0}]},"b":{"advanceWidth":1}}}"#;
        let mut root = parse_json(json.as_bytes()).expect("test JSON parses");
        let mut font = read_json(&mut root, &Options::default()).expect("font reads");
        let a = font
            .glyf
            .as_mut()
            .expect("glyf")
            .iter_mut()
            .flatten()
            .find(|g| g.name == b"a")
            .expect("glyph a");
        let first = a.references[0].clone();
        a.references.resize(count, first);
        font
    }

    // `consolidate_glyf` used to walk a glyph's references with a `u16`
    // counter, which can never reach a length of 65,536: 65,536 references
    // made it spin forever (and 65,535 did not). If this regresses it hangs
    // instead of failing, like the glyph-count limit test.
    #[test]
    #[cfg_attr(miri, ignore = "needs a genuine 65,536-element list")]
    fn a_glyph_with_65536_references_consolidates_instead_of_hanging() {
        for count in [65_535, 65_536] {
            let mut font = font_with_reference_count(count);
            consolidate_glyf(&mut font, &Options::default());
            let glyf = font.glyf.as_ref().expect("glyf survives");
            let a = glyf
                .iter()
                .flatten()
                .find(|g| g.name == b"a")
                .expect("glyph a survives");
            assert_eq!(a.references.len(), count);
        }
    }
}

