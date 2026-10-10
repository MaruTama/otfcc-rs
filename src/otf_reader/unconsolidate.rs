use crate::font::model::Font;
use otfcc_binary::Buffer;
use crate::support::glyph_order::GlyphOrder;
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, Pos, count_u16};
use crate::support::fmt::{Hex2Upper, Hex4Upper, BytePart};

use crate::table::glyf::{GlyfTable, Glyph, PostscriptHintMask};

use crate::table::otl::kind::lookup_kind;
use crate::table::otl::{ChainingRule, ChainingSubtable, Lookup, Subtable, SubtableList};

use crate::support::unicode::aglfn::aglfn_name;
use crate::support::glyph_order::{
    gord_lookup_name, gord_name_a_field_shared, set_glyph_order_by_gid,
};
use crate::support::primitives::{to_f2dot14, to_fixed};
use crate::vf::vq::{VQ, VqSegment};
use crate::vf::vq::{vq_create_still, vq_inplace_plus};
use sha1::{Digest, Sha1};

#[derive(Copy, Clone, Debug)]
pub struct GlyphHash {
    pub hash: [u8; 20],
}
fn hash_vqs(buf: &mut Buffer, s: &VqSegment) {
    buf.write_u8(s.discriminant_byte());
    match s {
        VqSegment::Still(still) => {
            buf.write_u32be(to_fixed(*still) as u32);
        }
        VqSegment::Delta(delta) => {
            // `delta.region` shares the `Rc<VqRegion>` `FvarTable.masters`
            // holds.
            let region = &delta.region;
            buf.write_u32be(to_fixed(delta.quantity) as u32);
            buf.write_u32be(region.dimensions as u32);
            for span in &region.spans {
                buf.write_u32be(to_f2dot14(span.start) as u32);
                buf.write_u32be(to_f2dot14(span.peak) as u32);
                buf.write_u32be(to_f2dot14(span.end) as u32);
            }
        }
    }
}
fn hash_vq(buf: &mut Buffer, x: VQ) {
    buf.write_u32be(to_fixed(x.kernel) as u32);
    buf.write_u32be(x.shift.len() as u32);
    for s in &x.shift {
        hash_vqs(buf, s);
    }
}
/// Hash one hint/contour mask: its position, then the first `n_stem_h`/
/// `n_stem_v` bits of each axis (the glyph's own stem counts, not the
/// mask's full 256). A stem count past 256 panics, as it always has.
fn hash_mask(buf: &mut Buffer, mask: &PostscriptHintMask, n_stem_h: usize, n_stem_v: usize) {
    buf.write_u16be(mask.contours_before);
    buf.write_u16be(mask.points_before);
    for i in 0..n_stem_h {
        buf.write_u8(mask.mask_h.get(i) as u8);
    }
    for i in 0..n_stem_v {
        buf.write_u8(mask.mask_v.get(i) as u8);
    }
}
pub fn name_glyph_by_hash(g: &Glyph, glyf: &GlyfTable) -> GlyphHash {
    let mut buf = Buffer::new();
    let buf = &mut buf;
    buf.write_u8('H' as i32 as u8);
    hash_vq(buf, g.advance_width.clone());
    buf.write_u8('h' as i32 as u8);
    hash_vq(buf, g.horizontal_origin.clone());
    buf.write_u8('V' as i32 as u8);
    hash_vq(buf, g.advance_height.clone());
    buf.write_u8('v' as i32 as u8);
    hash_vq(buf, g.vertical_origin.clone());
    buf.write_u8('C' as i32 as u8);
    buf.write_u8('(' as i32 as u8);
    for c in &g.contours {
        buf.write_u8('(' as i32 as u8);
        for point in c {
            hash_vq(buf, point.x.clone());
            hash_vq(buf, point.y.clone());
            buf.write_u8((point.on_curve != 0) as u8);
        }
        buf.write_u8(')' as i32 as u8);
    }
    buf.write_u8(')' as i32 as u8);
    buf.write_u8('R' as i32 as u8);
    buf.write_u8('(' as i32 as u8);
    for r in &g.references {
        let h: GlyphHash = name_glyph_by_hash(glyf[r.glyph.index as usize].as_deref().unwrap(), glyf);
        buf.write_bytes(&h.hash);
        hash_vq(buf, r.x.borrow().clone());
        hash_vq(buf, r.y.borrow().clone());
        buf.write_u32be(to_f2dot14(r.a) as u32);
        buf.write_u32be(to_f2dot14(r.b) as u32);
        buf.write_u32be(to_f2dot14(r.c) as u32);
        buf.write_u32be(to_f2dot14(r.d) as u32);
    }
    buf.write_u8(')' as i32 as u8);
    buf.write_u8('s' as i32 as u8);
    buf.write_u8('H' as i32 as u8);
    buf.write_u8('(' as i32 as u8);
    for stem in g.stem_h.iter() {
        buf.write_u32be(to_fixed(stem.position) as u32);
        buf.write_u32be(to_fixed(stem.width) as u32);
    }
    buf.write_u8(')' as i32 as u8);
    buf.write_u8('s' as i32 as u8);
    buf.write_u8('V' as i32 as u8);
    buf.write_u8('(' as i32 as u8);
    for stem in g.stem_v.iter() {
        buf.write_u32be(to_fixed(stem.position) as u32);
        buf.write_u32be(to_fixed(stem.width) as u32);
    }
    buf.write_u8(')' as i32 as u8);
    buf.write_u8('m' as i32 as u8);
    buf.write_u8('H' as i32 as u8);
    buf.write_u8('(' as i32 as u8);
    for mask in g.hint_masks.iter() {
        hash_mask(buf, mask, g.stem_h.len(), g.stem_v.len());
    }
    buf.write_u8(')' as i32 as u8);
    buf.write_u8('m' as i32 as u8);
    buf.write_u8('C' as i32 as u8);
    buf.write_u8('(' as i32 as u8);
    for mask in g.contour_masks.iter() {
        hash_mask(buf, mask, g.stem_h.len(), g.stem_v.len());
    }
    buf.write_u8(')' as i32 as u8);
    buf.write_u8('I' as i32 as u8);
    buf.write_u32be(g.instructions.len() as u32);
    buf.write_bytes(&g.instructions);
    let digest = Sha1::digest(&buf.data);
    GlyphHash { hash: digest.into() }
}
fn create_glyph_order(font: &mut Font, options: &Options) -> GlyphOrder {
    // Built as a plain local value rather than via `otfcc_glyph_order_create`
    // (which stays `unsafe fn`, a genuine `Box::into_raw` ownership
    // boundary) -- boxing only happens if/when a caller needs a raw
    // pointer, which none of this function's callers do any more.
    let mut glyph_order = GlyphOrder {
        entries: Vec::new(),
        by_gid: std::collections::BTreeMap::new(),
        by_name: std::collections::HashMap::new(),
    };
    // Only ever called (from `unconsolidate_font`) under a
    // `.glyf.is_some()` guard.
    let num_glyphs: GlyphId = count_u16(font.glyf.as_ref().unwrap().len());
    let prefix: Vec<u8> = options.glyph_name_prefix.clone().unwrap_or_default();
    for j in 0..num_glyphs {
        // Each iteration reads glyph `j` fully (`name_glyph_by_hash`/the
        // `.name.is_empty()` check below, both immutable) before writing
        // its `name` field -- never both at once -- so a fresh mutable
        // reborrow of the same slot for the write never overlaps the
        // read.
        let glyf = font.glyf.as_ref().unwrap();
        if options.name_glyphs_by_hash {
            let h: GlyphHash = name_glyph_by_hash(glyf[j as usize].as_deref().unwrap(), glyf);
            let mut gname: Vec<u8> = Vec::new();
            for j_0 in 0..SHA1_BLOCK_SIZE as u16 {
                if j_0 % 4 == 0 && j_0 / 4 != 0 {
                    gname.extend_from_slice(b"-");
                }
                Hex2Upper((h.hash[j_0 as usize] as i32) as u32)
                    .append_to_vec(&mut gname);
            }
            let shared_name = if gord_lookup_name(&glyph_order, gname.clone()) {
                let mut n: GlyphId = 2 as GlyphId;
                let mut still_in: bool = false;
                loop {
                    if still_in {
                        n = (n as i32 + 1_i32) as GlyphId;
                    }
                    let newname: Vec<u8> =
                        crate::bytesbuild!(&gname, b"-", &prefix, n as i32);
                    still_in = gord_lookup_name(&glyph_order, newname);
                    if !still_in {
                        break;
                    }
                }
                let newname_0: Vec<u8> =
                    crate::bytesbuild!(&gname, b"-", &prefix, n as i32);
                set_glyph_order_by_gid(&mut glyph_order, j, newname_0)
            } else {
                set_glyph_order_by_gid(&mut glyph_order, j, gname)
            };
            font.glyf.as_mut().unwrap()[j as usize]
                .as_mut()
                .unwrap()
                .name = shared_name;
        } else if !(options.ignore_glyph_order || options.name_glyphs_by_gid) {
            let existing_name = glyf[j as usize].as_deref().unwrap().name.clone();
            if !existing_name.is_empty() {
                let gname_0: Vec<u8> = crate::bytesbuild!(&prefix, &existing_name);
                let shared_name_1 = set_glyph_order_by_gid(&mut glyph_order, j, gname_0);
                font.glyf.as_mut().unwrap()[j as usize]
                    .as_mut()
                    .unwrap()
                    .name = shared_name_1;
            }
        }
    }
    let post_name_map: Option<&GlyphOrder> = font
        .post
        .as_deref()
        .and_then(|p| p.post_name_map.as_deref());
    if let Some(post_name_map) = post_name_map
        && !options.ignore_glyph_order && !options.name_glyphs_by_gid {
            for &idx in post_name_map.by_gid.values() {
                let entry = &post_name_map.entries[idx];
                let gname_1: Vec<u8> = crate::bytesbuild!(&prefix, &entry.name);
                set_glyph_order_by_gid(&mut glyph_order, entry.gid, gname_1);
            }
        }
    if let Some(cmap) = font.cmap.as_ref().filter(|_| !options.name_glyphs_by_gid) {
        for (&unicode, glyph) in cmap.unicodes.iter() {
            if glyph.index as i32 > 0_i32 {
                let aglfn_name = if unicode > 0_i32 && unicode < 0xffff_i32 {
                    aglfn_name(unicode as u32)
                } else {
                    None
                };
                let name: Vec<u8> = match aglfn_name {
                    Some(n) => crate::bytesbuild!(&prefix, n),
                    None => crate::bytesbuild!(&prefix, b"uni", Hex4Upper(unicode as u32)),
                };
                set_glyph_order_by_gid(&mut glyph_order, glyph.index, name);
            }
        }
    }
    let glyf = font.glyf.as_ref().unwrap();
    for j_1 in 0..num_glyphs {
        let name_0: Vec<u8>;
        if j_1 > 1 {
            name_0 = crate::bytesbuild!(&prefix, b"glyph", j_1 as i32);
        } else if j_1 == 1 {
            if glyf[1_usize].is_some()
                && glyf[1_usize].as_deref().unwrap().contours.is_empty()
                && glyf[1_usize].as_deref().unwrap().references.is_empty()
            {
                name_0 = crate::bytesbuild!(&prefix, b".null");
            } else {
                name_0 = crate::bytesbuild!(&prefix, b"glyph", j_1 as i32);
            }
        } else {
            name_0 = crate::bytesbuild!(&prefix, b".notdef");
        }
        set_glyph_order_by_gid(&mut glyph_order, j_1, name_0);
    }
    glyph_order
}
fn name_glyphs(font: &mut Font, gord: &GlyphOrder) {
    // Only ever called (from `unconsolidate_font`) under a
    // `.glyf.is_some()` guard.
    let glyf = font.glyf.as_mut().unwrap();
    for (gid, slot) in glyf.iter_mut().enumerate() {
        let g = slot.as_mut().unwrap();
        let mut glyph_name: Vec<u8> = Vec::new();
        gord_name_a_field_shared(gord, gid as GlyphId, &mut glyph_name);
        g.name = glyph_name;
    }
}
// This function expands every rule of every `Poly` (binary-read) chaining
// subtable into its own standalone `Canonical` subtable -- the shape the
// rest of the dump pipeline (and the JSON output) expects. Each factor
// feeding that expansion is individually capped upstream at parse time
// (`chaining/read.rs`'s `MAX_TOTAL_RULES_PER_TABLE` bounds rules built
// across a whole table; `otl/read.rs`'s `MAX_TOTAL_SUBTABLES_PER_LOOKUP`
// bounds one lookup's own subtable count), but those two caps multiply
// here: fuzzing found a lookup with ~700 subtables that, combined,
// expanded into 80,000+ standalone subtables -- each one then walked
// again by `consolidate_chaining` and serialized to JSON, tens of seconds
// of work from what was a ~500KB file. This is the backstop that bounds
// the product, not just each factor.
const MAX_TOTAL_UNCONSOLIDATED_SUBTABLES_PER_LOOKUP: usize = 20_000;
pub(crate) fn unconsolidate_chaining(lookup: &mut Lookup) {
    let mut newsts: SubtableList = Vec::new();
    'subtables: for slot in lookup.subtables.iter_mut() {
        if newsts.len() >= MAX_TOTAL_UNCONSOLIDATED_SUBTABLES_PER_LOOKUP {
            break 'subtables;
        }
        // `.take()` moves the `Box` out of the slot, leaving `None` behind.
        // `Subtable` implements `Drop`, so its payload can't be moved out
        // by value through a pattern match (even via `*sub_box`) -- only
        // mutated through a `&mut` borrow, which is all the branches below
        // need. `sub_box` itself drops normally at the end of each
        // iteration, cleaning up whatever's left behind (empty after the
        // `mem::take`s below).
        let Some(mut sub_box) = slot.take() else {
            continue;
        };
        let Subtable::Chaining(sub_chaining) = &mut *sub_box else {
            unreachable!()
        };
        match sub_chaining {
            ChainingSubtable::Poly(ruleset) => {
                // `chaining/read.rs` never pushes a failed rule into
                // `ruleset.rules`, so every slot of a successfully read
                // lookup is `Some`.
                for rule_slot in ::core::mem::take(&mut ruleset.rules) {
                    if newsts.len() >= MAX_TOTAL_UNCONSOLIDATED_SUBTABLES_PER_LOOKUP {
                        break 'subtables;
                    }
                    let boxed_rule: Box<ChainingRule> =
                        rule_slot.expect("chaining rule slot should never be None here");
                    newsts.push(Some(Box::new(Subtable::Chaining(
                        ChainingSubtable::Canonical(*boxed_rule),
                    ))));
                }
                // `sub_box` drops at the end of this iteration -- its
                // `ruleset.rules` is already empty (taken above) and
                // `bc`/`ic`/`fc` (never populated by the binary-read path)
                // are `None`, so this is a cheap no-op, not a leak.
            }
            ChainingSubtable::Canonical(rule) => {
                // Swap the rule out through the `&mut` borrow, leaving a
                // cheap empty default behind for `sub_box` to drop.
                let taken_rule = ::core::mem::take(rule);
                newsts.push(Some(Box::new(Subtable::Chaining(
                    ChainingSubtable::Canonical(taken_rule),
                ))));
            }
            // Never actually produced by the binary-read path this function
            // consumes from (only `classifier.rs`'s build-time pass creates
            // `Classified` subtables, and those are consumed and freed
            // within that same build call, never stored back into a
            // `SubtableList`) -- kept as a safe no-op rather than
            // `unreachable!()` since nothing upstream enforces that
            // invariant structurally.
            ChainingSubtable::Classified(_) => {}
        }
    }
    // Dropping the old list here disposes anything this loop left as
    // `Some` (e.g. a `Classified` subtable, or every remaining slot once
    // the budget above cuts the loop short).
    lookup.subtables = newsts;
}
fn expand_chain(lookup: &mut Lookup) {
    if let Some(kind) = lookup_kind(lookup.lookup_type) {
        kind.unconsolidate(lookup);
    }
}
fn expand_chaining_lookups(font: &mut Font) {
    // Every slot is still `Some` here -- `unconsolidate_*` only ever runs
    // on the binary-read/dump path, on a table that has never gone
    // through `consolidate_otl_table`'s hole-punching.
    if let Some(gsub) = font.gsub.as_deref_mut() {
        for lookup in gsub.lookups.iter_mut().flatten() {
            expand_chain(lookup);
        }
    }
    if let Some(gpos) = font.gpos.as_deref_mut() {
        for lookup in gpos.lookups.iter_mut().flatten() {
            expand_chain(lookup);
        }
    }
}
fn merge_hmtx(font: &mut Font) {
    if !(font.hhea.is_some() && font.hmtx.is_some() && font.glyf.is_some()) {
        return;
    }
    let count_a: u32 = font.hhea.as_deref().unwrap().number_of_metrics as u32;
    let hmtx = font.hmtx.take().unwrap();
    let glyf = font.glyf.as_mut().unwrap();
    let count_a = count_a as usize;
    for (j, slot) in glyf.iter_mut().enumerate() {
        let g = slot.as_mut().unwrap();
        // The first `count_a` glyphs have a full metric each; every glyph
        // past them shares the last metric's advance width and takes its
        // left side bearing from the trailing `left_side_bearing` array.
        let (adw, lsb): (Pos, Pos) = if j < count_a {
            (hmtx.metrics[j].advance_width as Pos, hmtx.metrics[j].lsb)
        } else {
            (
                hmtx.metrics[count_a.wrapping_sub(1)].advance_width as Pos,
                hmtx.left_side_bearing[j - count_a],
            )
        };
        vq_inplace_plus(&mut g.advance_width, vq_create_still(adw));
        vq_inplace_plus(
            &mut g.horizontal_origin,
            vq_create_still(-lsb + g.stat.x_min),
        );
    }
}
fn merge_vmtx(font: &mut Font) {
    if !(font.vhea.is_some() && font.vmtx.is_some() && font.glyf.is_some()) {
        return;
    }
    let count_a: u32 = font.vhea.as_deref().unwrap().num_of_long_ver_metrics as u32;
    let vmtx = font.vmtx.take().unwrap();
    let mut vorgs: Option<Vec<Pos>> = None;
    if let Some(vorg) = font.vorg.take() {
        let glyf_len = font.glyf.as_ref().unwrap().len();
        let mut v: Vec<Pos> = vec![vorg.default_vertical_origin; glyf_len];
        for entry in &vorg.entries[..vorg.num_vert_origin_y_metrics as usize] {
            if (entry.gid as usize) < glyf_len {
                v[entry.gid as usize] = entry.vertical_origin as Pos;
            }
        }
        vorgs = Some(v);
    }
    let glyf = font.glyf.as_mut().unwrap();
    let count_a = count_a as usize;
    for (j, slot) in glyf.iter_mut().enumerate() {
        let g = slot.as_mut().unwrap();
        // Same long-metric / shared-last-metric split as `merge_hmtx`.
        let (adh, tsb): (Pos, Pos) = if j < count_a {
            (vmtx.metrics[j].advance_height as Pos, vmtx.metrics[j].tsb)
        } else {
            (
                vmtx.metrics[count_a.wrapping_sub(1)].advance_height as Pos,
                vmtx.top_side_bearing[j - count_a],
            )
        };
        vq_inplace_plus(&mut g.advance_height, vq_create_still(adh));
        vq_inplace_plus(
            &mut g.vertical_origin,
            vq_create_still(if let Some(v) = &vorgs {
                v[j]
            } else {
                tsb + g.stat.y_max
            }),
        );
    }
}
fn merge_ltsh(font: &mut Font) {
    if let Some(glyf) = font.glyf.as_mut()
        && let Some(ltsh) = &font.ltsh {
            let n = (count_u16(glyf.len())).min(ltsh.num_glyphs) as usize;
            for (slot, &y_pel) in glyf.iter_mut().zip(&ltsh.y_pels[..n]) {
                slot.as_mut().unwrap().y_pel = y_pel;
            }
        }
}
pub fn unconsolidate_font(font: &mut Font, options: &Options) {
    merge_hmtx(font);
    merge_vmtx(font);
    merge_ltsh(font);
    expand_chaining_lookups(font);
    if font.glyf.is_some() {
        let gord = create_glyph_order(font, options);
        name_glyphs(font, &gord);
    }
}
pub const SHA1_BLOCK_SIZE: i32 = 20_i32;

#[cfg(test)]
mod tests {
    use super::*;

    /// NIST FIPS 180-1's own test vector (`SHA1("abc")`), pinning that the
    /// `sha1` crate this module depends on for glyph-hash naming
    /// (`--name-by-hash`) still computes real SHA-1 -- nothing else in this
    /// crate's test suite exercises `sha1::Sha1` directly (no golden fixture
    /// currently uses `--name-by-hash`; `tests/golden.rs`'s byte-exact
    /// comparisons are what actually prove any *specific font's* hash-named
    /// output is correct, once one does).
    #[test]
    fn sha1_matches_known_test_vector() {
        let digest = Sha1::digest(b"abc");
        assert_eq!(
            digest.as_slice(),
            [
                0xa9, 0x99, 0x3e, 0x36, 0x47, 0x06, 0x81, 0x6a, 0xba, 0x3e, 0x25, 0x71, 0x78, 0x50,
                0xc2, 0x6c, 0x9c, 0xd0, 0xd8, 0x9d,
            ]
        );
    }
}
