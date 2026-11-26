use crate::font::model::Font;
use crate::logger::ByteStr;
use crate::support::glyph_order::GlyphOrder;
use crate::support::glyph_order::gord_consolidate_handle;
use crate::support::handle::{
    FdHandle, GlyphHandle, Handle, HandleState, handle_from_index, handle_name_eq_bytes,
};
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, Pos, ShapeId};
use crate::table::cff::CffTable;
use crate::table::glyf::{
    ComponentReference, GlyfTable, Glyph, PostscriptHintMask, PostscriptStemDef, RefAnchorStatus,
};
use crate::table::glyf::{glyf_component_reference_empty, new_glyf_glyph};
use crate::vf::vq::VQ;
use crate::vf::vq::{vq_get_still, vq_neutral, vq_point_linear_tfm};

fn by_stem_pos(a: &PostscriptStemDef, b: &PostscriptStemDef) -> i32 {
    if a.position == b.position {
        i32::from(a.map) - i32::from(b.map)
    } else if a.position > b.position {
        1
    } else {
        -1
    }
}
fn by_mask_pointindex(a: &PostscriptHintMask, b: &PostscriptHintMask) -> i32 {
    if i32::from(a.contours_before) == i32::from(b.contours_before) {
        i32::from(a.points_before) - i32::from(b.points_before)
    } else {
        i32::from(a.contours_before) - i32::from(b.contours_before)
    }
}
fn consolidate_glyph_contours(g: &mut Glyph) {
    // `Vec::retain` visits every element once, in order, regardless of
    // whether earlier ones were kept -- so `j` here is each contour's index
    // before any is dropped, and dropped contours are
    // freed automatically (a `Contour`'s only owned resources are its
    // points' `VQ` `Vec`s; no `Handle` involved, unlike `ReferenceList`).
    let mut j: ShapeId = 0 as ShapeId;
    let name = &g.name;
    g.contours.retain(|contour| {
        let keep = !contour.is_empty();
        if !keep {
            tracing::warn!(
                "[Consolidate] Removed empty contour #{} in glyph {}.\n",
                i32::from(j),
                ByteStr(name)
            );
        }
        j = j.wrapping_add(1);
        keep
    });
}
fn consolidate_glyph_references(g: &mut Glyph, glyph_order: &GlyphOrder) {
    let name = &g.name;
    g.references.retain_mut(|r| {
        let ok = gord_consolidate_handle(glyph_order, &mut r.glyph);
        if !ok {
            tracing::warn!(
                "[Consolidate] Ignored absent glyph component reference /{} within /{}.\n",
                ByteStr(&r.glyph.name),
                ByteStr(name)
            );
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
                mask.mask_h.set(k, oldmask.mask_h.get(hm as usize));
            }
            for (k, &vm) in vmap.iter().enumerate() {
                mask.mask_v.set(k, oldmask.mask_v.get(vm as usize));
            }
        }
    }
    if !g.contour_masks.is_empty() {
        let contour_masks: &mut Vec<PostscriptHintMask> = &mut g.contour_masks;
        contour_masks.sort_by(|a, b| by_mask_pointindex(a, b).cmp(&0));
        for mask in contour_masks.iter_mut() {
            let oldmask: PostscriptHintMask = *mask;
            for (k, &hm) in hmap.iter().enumerate() {
                mask.mask_h.set(k, oldmask.mask_h.get(hm as usize));
            }
            for (k, &vm) in vmap.iter().enumerate() {
                mask.mask_v.set(k, oldmask.mask_v.get(vm as usize));
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
        *h = Handle::new(
            HandleState::Consolidated,
            idx,
            fd_array[idx as usize].font_name.clone(),
        ) as FdHandle;
    } else if !h.name.is_empty() {
        let found = fd_array
            .iter()
            .position(|fd| handle_name_eq_bytes(&h.name, &fd.font_name));
        if let Some(j) = found {
            *h = Handle::new(
                HandleState::Consolidated,
                j as GlyphId,
                fd_array[j].font_name.clone(),
            );
        } else {
            tracing::warn!(
                "[Consolidate] CID Subfont {} is not defined. (in glyph /{}).\n",
                ByteStr(&h.name),
                ByteStr(gname)
            );
            *h = Handle::default();
        }
    } else if !h.name.is_empty() {
        // Unreachable: the preceding `else if !h.name.is_empty()` already
        // covers this same condition with nothing mutating `h.name` in
        // between.
        *h = Handle::default();
    }
}
pub fn consolidate_glyph(g: &mut Glyph, glyph_order: &GlyphOrder, cff: Option<&CffTable>) {
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
// The walk mutates only `is_anchored`/`x`/`y` of a `ComponentReference`,
// which are `Cell`/`RefCell`s (see `ComponentReference` in `table/glyf.rs`)
// so it can run over a shared `&GlyfTable`. The read/recurse/mutate order
// below matters: the cycle-detection guards depend on it, not just on the
// final values.
/// `get_point_coordinates`'s three in/out parameters: how far the walk has
/// counted so far (`stated`) and the coordinates it writes once `stated`
/// reaches the target `n` (`x`/`y`). All three are always read and written
/// together, so they travel as one `&mut`.
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
    let g: &Glyph = table[j as usize]
        .as_deref()
        .expect(FILLED_BY_CONSOLIDATE_GLYF);
    // Plain iteration, not `u16` index counters: a contour or reference list
    // is not bounded to 16 bits here, and a counter that wraps at 65,536 can
    // never reach a length of 65,536 (the same shape as the `glyf` hang).
    for p in g.contours.iter().flatten() {
        if i32::from(search.stated) == i32::from(n) {
            // (a·x + c·y, b·x + d·y), as in `otf_writer/stat.rs`.
            search.x = vq_point_linear_tfm(
                gr.x.borrow().clone(),
                gr.a as Pos,
                p.x.clone(),
                gr.c as Pos,
                p.y.clone(),
            );
            search.y = vq_point_linear_tfm(
                gr.y.borrow().clone(),
                gr.b as Pos,
                p.x.clone(),
                gr.d as Pos,
                p.y.clone(),
            );
            return true;
        }
        search.stated = (i32::from(search.stated) + 1) as ShapeId;
    }
    // `g`'s own anchored components are resolved in `g`'s own coordinates,
    // the outer point being one of `g` where it stands, not where `gr`
    // places it: their offsets are `g`'s, as stored, and only then moved
    // by `gr` below.
    let mut g_itself: ComponentReference = (glyf_component_reference_empty)();
    g_itself.glyph = handle_from_index(j) as GlyphHandle;
    for rr in &g.references {
        consolidate_anchor_ref(table, &g_itself, rr, options, depth + 1);
        let mut glyph_ref: ComponentReference = (glyf_component_reference_empty)();
        glyph_ref.glyph = handle_from_index(rr.glyph.index) as GlyphHandle;
        // `rr` placed where `gr` places its glyph, as in `otf_writer/stat.rs`.
        glyph_ref.a = gr.a * rr.a + gr.c * rr.b;
        glyph_ref.b = gr.b * rr.a + gr.d * rr.b;
        glyph_ref.c = gr.a * rr.c + gr.c * rr.d;
        glyph_ref.d = gr.b * rr.c + gr.d * rr.d;
        let (ox, oy) = (rr.x.borrow().clone(), rr.y.borrow().clone());
        let (ox, oy) = if rr.scaled_offset {
            let zero = vq_neutral;
            (
                vq_point_linear_tfm(zero(), rr.a as Pos, ox.clone(), rr.c as Pos, oy.clone()),
                vq_point_linear_tfm(zero(), rr.b as Pos, ox, rr.d as Pos, oy),
            )
        } else {
            (ox, oy)
        };
        glyph_ref.x = std::cell::RefCell::new(vq_point_linear_tfm(
            gr.x.borrow().clone(),
            gr.a as Pos,
            ox.clone(),
            gr.c as Pos,
            oy.clone(),
        ));
        glyph_ref.y = std::cell::RefCell::new(vq_point_linear_tfm(
            gr.y.borrow().clone(),
            gr.b as Pos,
            ox,
            gr.d as Pos,
            oy,
        ));
        let success: bool = get_point_coordinates(table, &glyph_ref, n, search, options, depth + 1);
        // `ref_0` is a plain owned local; every field auto-drops when it
        // goes out of scope here (or at the `return true` below), so no
        // explicit dispose call is needed.
        if success {
            return true;
        }
    }
    false
}
// The two branches below re-read `rr.is_anchored.get()` *after* both
// recursive `get_point_coordinates` calls (`s1`/`s2`) have returned, rather
// than reusing a value cached before the recursion: a re-entrant call that
// reaches this exact `rr` again during `s1`/`s2` (a real, reachable cycle)
// overwrites `is_anchored` to `Xy` before returning, and the final branch
// has to observe that. Caching it in a local would change behavior.
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
        tracing::warn!(
            "Found circular reference of out-of-range point reference in anchored reference."
        );
        rr.is_anchored.set(RefAnchorStatus::Xy);
        return false;
    }
    if rr.is_anchored.get() == RefAnchorStatus::AnchorAnchor {
        rr.is_anchored
            .set(RefAnchorStatus::AnchorConsolidatingAnchor);
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
        tracing::warn!(
            "Failed to access point {} in outer glyph.",
            i32::from(rr.outer)
        );
    }
    if !s2 {
        tracing::warn!(
            "Failed to access point {} in reference to {}.",
            i32::from(rr.outer),
            ByteStr(&rr.glyph.name)
        );
    }
    // The offset that brings the component's point onto the outer one:
    // outer − M·inner, with M as in `get_point_coordinates`.
    let rrx: VQ = vq_point_linear_tfm(
        outer.x.clone(),
        -(rr.a as Pos),
        inner.x.clone(),
        -(rr.c as Pos),
        inner.y.clone(),
    );
    let rry: VQ = vq_point_linear_tfm(
        outer.y.clone(),
        -(rr.b as Pos),
        inner.x.clone(),
        -(rr.d as Pos),
        inner.y.clone(),
    );
    // The component is written as an anchor, so the offset the anchor
    // gives is where it is drawn; x/y given beside it (in JSON) that say
    // otherwise are warned about and replaced, so the bounding box is that
    // of the glyph as drawn.
    let from_font = rr.is_anchored.get() == RefAnchorStatus::AnchorConsolidatingAnchor;
    let differs = (vq_get_still(rr.x.borrow().clone()) - vq_get_still(rrx.clone())).abs() > 0.5
        || (vq_get_still(rr.y.borrow().clone()) - vq_get_still(rry.clone())).abs() > 0.5;
    if !from_font && differs {
        tracing::warn!(
            "Anchored reference to {} does not match its X/Y offset data.",
            ByteStr(&rr.glyph.name)
        );
    }
    if from_font || (s1 && s2) {
        rr.x.replace(rrx);
        rr.y.replace(rry);
    }
    rr.is_anchored.set(RefAnchorStatus::AnchorConsolidated);
    // `rr1`/`inner`/`outer` (and, in this branch, `rrx`/`rry`) are all plain
    // owned locals that were never moved out -- they auto-drop at the
    // `return false` below, so no explicit dispose calls are needed.
    false
}
/// `consolidate_glyf` fills every empty `glyf` slot before it resolves any
/// reference, so the reference walkers below never meet a hole.
const FILLED_BY_CONSOLIDATE_GLYF: &str = "consolidate_glyf filled every empty glyf slot first";
pub fn consolidate_glyf(font: &mut Font, options: &Options) {
    if font.glyph_order.is_none() || font.glyf.is_none() {
        return;
    }
    let glyph_order: &GlyphOrder = font
        .glyph_order
        .as_deref()
        .expect("the early return above checked glyph_order");
    let cff: Option<&CffTable> = font.cff.as_deref();
    let glyf: &mut GlyfTable = font
        .glyf
        .as_mut()
        .expect("the early return above checked glyf");
    for slot in glyf.iter_mut() {
        if let Some(glyph) = slot {
            consolidate_glyph(glyph, glyph_order, cff);
        } else {
            *slot = Some(new_glyf_glyph());
        }
    }
    // `consolidate_anchor_ref` recurses over the reference graph and can
    // revisit *any* glyph in the table (not just the one being processed)
    // while resolving anchor points, but it mutates only
    // `ComponentReference.is_anchored`/`x`/`y` (`Cell`/`RefCell`), so a
    // single shared `&GlyfTable` serves the whole walk below.
    let table: &GlyfTable = glyf;
    // At most 65,535 glyphs (the binary count is 16-bit, and
    // `support::json_limits` bounds the JSON one), so every index fits a
    // `GlyphId`.
    for (j, slot) in table.iter().enumerate() {
        let j_0 = j as GlyphId;
        let g: &Glyph = slot.as_deref().expect(FILLED_BY_CONSOLIDATE_GLYF);
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
        let mut g0 = new_glyf_glyph();
        g0.references.push(reference_to(1));
        let mut g1 = new_glyf_glyph();
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
    use otfcc_json::parse_json;

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
