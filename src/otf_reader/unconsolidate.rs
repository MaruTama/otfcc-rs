use crate::font::model::{Font, FontSubtype};
use crate::otf_writer::stat::cubic_outline_bounds;
use crate::support::fmt::{BytePart, Hex2Upper, Hex4Upper};
use crate::support::glyph_order::GlyphOrder;
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, Pos, count_u16};
use otfcc_binary::Buffer;

use crate::table::glyf::{GlyfTable, Glyph, PostscriptHintMask};

use crate::table::otl::kind::lookup_kind;
use crate::table::otl::{ChainingRule, ChainingSubtable, Lookup, Subtable, SubtableList};

use crate::support::glyph_order::{
    gord_lookup_name, gord_name_a_field_shared, set_glyph_order_by_gid,
};
use crate::support::primitives::{to_f2dot14, to_fixed};
use crate::support::unicode::aglfn::aglfn_name;
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
            buf.write_u32be(u32::from(region.dimensions));
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
        buf.write_u8(u8::from(mask.mask_h.get(i)));
    }
    for i in 0..n_stem_v {
        buf.write_u8(u8::from(mask.mask_v.get(i)));
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
            buf.write_u8(u8::from(point.on_curve != 0));
        }
        buf.write_u8(')' as i32 as u8);
    }
    buf.write_u8(')' as i32 as u8);
    buf.write_u8('R' as i32 as u8);
    buf.write_u8('(' as i32 as u8);
    for r in &g.references {
        let h: GlyphHash = name_glyph_by_hash(
            glyf[r.glyph.index as usize]
                .as_deref()
                .expect("glyph slot is filled"),
            glyf,
        );
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
    GlyphHash {
        hash: digest.into(),
    }
}
/// The glyph's SHA-1 name: the digest in upper-case hex, in dash-separated
/// groups of four bytes.
fn hash_glyph_name(glyf: &GlyfTable, j: GlyphId) -> Vec<u8> {
    let h: GlyphHash = name_glyph_by_hash(
        glyf[j as usize].as_deref().expect("glyph slot is filled"),
        glyf,
    );
    let mut gname: Vec<u8> = Vec::new();
    for (byte_idx, &byte) in h.hash.iter().enumerate() {
        if byte_idx % 4 == 0 && byte_idx != 0 {
            gname.extend_from_slice(b"-");
        }
        Hex2Upper(u32::from(byte)).append_to_vec(&mut gname);
    }
    gname
}
/// Gives glyph `j` the name `gname`, or `gname-<prefix><n>` for the first
/// `n` from 2 that is not taken yet when glyphs have identical hashes.
/// Returns the name the glyph order stored.
fn name_glyph_uniquely(
    glyph_order: &mut GlyphOrder,
    j: GlyphId,
    gname: Vec<u8>,
    prefix: &[u8],
) -> Vec<u8> {
    if !gord_lookup_name(glyph_order, gname.clone()) {
        return set_glyph_order_by_gid(glyph_order, j, gname);
    }
    let mut n: GlyphId = 2 as GlyphId;
    let mut still_in: bool = false;
    loop {
        if still_in {
            n = (i32::from(n) + 1) as GlyphId;
        }
        let newname: Vec<u8> = crate::bytesbuild!(&gname, b"-", prefix, i32::from(n));
        still_in = gord_lookup_name(glyph_order, newname);
        if !still_in {
            break;
        }
    }
    let unique_name: Vec<u8> = crate::bytesbuild!(&gname, b"-", prefix, i32::from(n));
    set_glyph_order_by_gid(glyph_order, j, unique_name)
}
/// Names glyphs from what the font itself says about them: their hash with
/// `--name-by-hash`, otherwise the name they already carry (unless the glyph
/// order is being ignored or named by GID). The glyph's own `name` field
/// is updated to the name the glyph order stored.
fn name_glyphs_from_font(
    font: &mut Font,
    options: &Options,
    prefix: &[u8],
    glyph_order: &mut GlyphOrder,
    num_glyphs: GlyphId,
) {
    for j in 0..num_glyphs {
        let glyf = font.glyf.as_ref().expect("glyf table is present");
        let stored_name = if options.name_glyphs_by_hash {
            let gname = hash_glyph_name(glyf, j);
            Some(name_glyph_uniquely(glyph_order, j, gname, prefix))
        } else if !(options.ignore_glyph_order || options.name_glyphs_by_gid) {
            let existing_name = glyf[j as usize]
                .as_deref()
                .expect("glyph slot is filled")
                .name
                .clone();
            if existing_name.is_empty() {
                None
            } else {
                let prefixed_name: Vec<u8> = crate::bytesbuild!(prefix, &existing_name);
                Some(set_glyph_order_by_gid(glyph_order, j, prefixed_name))
            }
        } else {
            None
        };
        if let Some(name) = stored_name {
            font.glyf.as_mut().expect("glyf table is present")[j as usize]
                .as_mut()
                .expect("glyph slot is filled")
                .name = name;
        }
    }
}
/// Names glyphs from the `post` table's glyph names, unless names are being
/// ignored or made from GIDs.
fn name_glyphs_from_post(
    font: &Font,
    options: &Options,
    prefix: &[u8],
    glyph_order: &mut GlyphOrder,
) {
    let post_name_map: Option<&GlyphOrder> = font
        .post
        .as_deref()
        .and_then(|p| p.post_name_map.as_deref());
    if let Some(post_name_map) = post_name_map
        && !options.ignore_glyph_order
        && !options.name_glyphs_by_gid
    {
        for &idx in post_name_map.by_gid.values() {
            let entry = &post_name_map.entries[idx];
            let gname: Vec<u8> = crate::bytesbuild!(prefix, &entry.name);
            set_glyph_order_by_gid(glyph_order, entry.gid, gname);
        }
    }
}
/// Names glyphs after the first Unicode value in `cmap` that reaches them: the
/// AGLFN name when there is one for a BMP value, else `uniXXXX`.
fn name_glyphs_from_cmap(
    font: &Font,
    options: &Options,
    prefix: &[u8],
    glyph_order: &mut GlyphOrder,
) {
    let Some(cmap) = font.cmap.as_ref().filter(|_| !options.name_glyphs_by_gid) else {
        return;
    };
    for (&unicode, glyph) in cmap.unicodes.iter() {
        if i32::from(glyph.index) > 0 {
            let aglfn_name = if unicode > 0 && unicode < 0xffff {
                aglfn_name(unicode as u32)
            } else {
                None
            };
            let name: Vec<u8> = match aglfn_name {
                Some(n) => crate::bytesbuild!(prefix, n),
                None => crate::bytesbuild!(prefix, b"uni", Hex4Upper(unicode as u32)),
            };
            set_glyph_order_by_gid(glyph_order, glyph.index, name);
        }
    }
}
/// Names every glyph still unnamed: `.notdef` for GID 0, `.null` for an empty
/// GID 1, and `glyph<N>` for the rest.
fn name_remaining_glyphs(
    font: &Font,
    prefix: &[u8],
    glyph_order: &mut GlyphOrder,
    num_glyphs: GlyphId,
) {
    let glyf = font.glyf.as_ref().expect("glyf table is present");
    for gid in 0..num_glyphs {
        let glyph_name: Vec<u8> = if gid > 1 {
            crate::bytesbuild!(prefix, b"glyph", i32::from(gid))
        } else if gid == 1 {
            if glyf[1]
                .as_deref()
                .is_some_and(|g| g.contours.is_empty() && g.references.is_empty())
            {
                crate::bytesbuild!(prefix, b".null")
            } else {
                crate::bytesbuild!(prefix, b"glyph", i32::from(gid))
            }
        } else {
            crate::bytesbuild!(prefix, b".notdef")
        };
        set_glyph_order_by_gid(glyph_order, gid, glyph_name);
    }
}
/// Builds the glyph order of a font read from binary, trying name sources in
/// order: the font's own names or hashes, then `post`, then `cmap`, then the
/// generated fallbacks. `set_glyph_order_by_gid` keeps the first name a GID
/// gets, so an earlier source wins over a later one.
fn create_glyph_order(font: &mut Font, options: &Options) -> GlyphOrder {
    let mut glyph_order = GlyphOrder {
        entries: Vec::new(),
        by_gid: std::collections::BTreeMap::new(),
        by_name: std::collections::HashMap::new(),
    };
    // Only ever called (from `unconsolidate_font`) under a
    // `.glyf.is_some()` guard.
    let num_glyphs: GlyphId = count_u16(font.glyf.as_ref().expect("glyf table is present").len());
    let prefix: Vec<u8> = options.glyph_name_prefix.clone().unwrap_or_default();
    name_glyphs_from_font(font, options, &prefix, &mut glyph_order, num_glyphs);
    name_glyphs_from_post(font, options, &prefix, &mut glyph_order);
    name_glyphs_from_cmap(font, options, &prefix, &mut glyph_order);
    name_remaining_glyphs(font, &prefix, &mut glyph_order, num_glyphs);
    glyph_order
}
fn name_glyphs(font: &mut Font, gord: &GlyphOrder) {
    // Only ever called (from `unconsolidate_font`) under a
    // `.glyf.is_some()` guard.
    let glyf = font.glyf.as_mut().expect("glyf table is present");
    for (gid, slot) in glyf.iter_mut().enumerate() {
        let g = slot.as_mut().expect("glyph slot is filled");
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
// across a whole table; `otl/read.rs`'s `MAX_TOTAL_SUBTABLES_PER_TABLE`
// bounds the subtables read), but those two caps multiply
// here: fuzzing found a lookup with ~700 subtables that, combined,
// expanded into 80,000+ standalone subtables -- each one then walked
// again by `consolidate_chaining` and serialized to JSON, tens of seconds
// of work from what was a ~500KB file. This is the backstop that bounds
// the product, not just each factor.
const MAX_TOTAL_UNCONSOLIDATED_SUBTABLES_PER_LOOKUP: usize = 20_000;
pub(crate) fn unconsolidate_chaining(lookup: &mut Lookup) {
    let mut newsts: SubtableList = Vec::new();
    let mut truncated = false;
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
                        truncated = true;
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
    // The old list still holds whatever this loop left as `Some` (e.g. a
    // `Classified` subtable, or every remaining slot once the budget above
    // cuts the loop short); assigning over it drops those.
    // Whatever is still in the old list was not moved out: once the loop is
    // cut short, those are the subtables that are dropped.
    if truncated || lookup.subtables.iter().any(Option::is_some) {
        tracing::warn!(
            "[otl] Lookup {} has more contextual rules than otfcc writes out ({}); the rest are ignored.\n",
            crate::logger::ByteStr(&lookup.name),
            MAX_TOTAL_UNCONSOLIDATED_SUBTABLES_PER_LOOKUP
        );
    }
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
    let count_a: u32 = u32::from(
        font.hhea
            .as_deref()
            .expect("hhea table is present")
            .number_of_metrics,
    );
    let hmtx = font.hmtx.take().expect("hmtx table is present");
    // A CFF glyph already has the width its charstring gives; `hmtx` is the
    // one OpenType layout uses, so it replaces that. Its left side bearing
    // is left to the curves, which the build takes it from again.
    let cff = font.subtype == FontSubtype::Cff;
    let glyf = font.glyf.as_mut().expect("glyf table is present");
    let count_a = count_a as usize;
    // The CFF table, not `maxp`, sets a CFF font's glyph count, so `hmtx`
    // can cover fewer glyphs; the rest keep their charstring widths.
    let covered = count_a + hmtx.left_side_bearing.len();
    for (j, slot) in glyf.iter_mut().enumerate() {
        let g = slot.as_mut().expect("glyph slot is filled");
        if cff {
            if j >= covered {
                break;
            }
            let adw = hmtx.metrics[j.min(count_a - 1)].advance_width;
            g.advance_width = vq_create_still(adw as Pos);
            continue;
        }
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
    let count_a: u32 = u32::from(
        font.vhea
            .as_deref()
            .expect("vhea table is present")
            .num_of_long_ver_metrics,
    );
    let vmtx = font.vmtx.take().expect("vmtx table is present");
    let mut vorgs: Option<Vec<Pos>> = None;
    if let Some(vorg) = font.vorg.take() {
        let glyf_len = font.glyf.as_ref().expect("glyf table is present").len();
        let mut v: Vec<Pos> = vec![vorg.default_vertical_origin; glyf_len];
        for entry in &vorg.entries[..vorg.num_vert_origin_y_metrics as usize] {
            if (entry.gid as usize) < glyf_len {
                v[entry.gid as usize] = Pos::from(entry.vertical_origin);
            }
        }
        vorgs = Some(v);
    }
    // The glyph's top: a TrueType glyph's from its `glyf` header, a CFF
    // glyph's from its curves, which have no header to read it from.
    let cubic = font.subtype == FontSubtype::Cff;
    let glyf = font.glyf.as_mut().expect("glyf table is present");
    let count_a = count_a as usize;
    // `vmtx` is sized by `maxp`, but a CFF font's glyph count comes from its
    // CFF table and can be larger: the glyphs past what `vmtx` covers keep
    // no vertical metrics, as `merge_hmtx` does for widths.
    let covered = count_a + vmtx.top_side_bearing.len();
    for (j, slot) in glyf.iter_mut().enumerate().take(covered) {
        let g = slot.as_mut().expect("glyph slot is filled");
        let y_max = if cubic {
            cubic_outline_bounds(&g.contours).map_or(0.0, |b| b[3])
        } else {
            g.stat.y_max
        };
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
                tsb + y_max
            }),
        );
    }
}
fn merge_ltsh(font: &mut Font) {
    if let Some(glyf) = font.glyf.as_mut()
        && let Some(ltsh) = &font.ltsh
    {
        let n = (count_u16(glyf.len())).min(ltsh.num_glyphs) as usize;
        for (slot, &y_pel) in glyf.iter_mut().zip(&ltsh.y_pels[..n]) {
            slot.as_mut().expect("glyph slot is filled").y_pel = y_pel;
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
