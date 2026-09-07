use crate::font::caryll_font::Font;
use crate::support::buffer::Buffer;
use crate::support::glyph_order::GlyphOrder;
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, Pos};
use crate::support::sha1::Sha1Ctx;
use crate::support::fmt::{Hex2Upper, Hex4Upper, SdsPart};

use crate::table::glyf::{GlyfTable, Glyph};

use crate::table::otl::{
    ChainingRule, ChainingSubtable, Lookup, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GSUB_CHAINING,
    Subtable, SubtableList,
};

use crate::support::aglfn::aglfn_setup_names;
use crate::support::glyph_order::{
    gord_lookup_name, otfcc_gord_name_a_field_shared, otfcc_set_glyph_order_by_gid,
};
use crate::support::primitives::{otfcc_to_f2dot14, otfcc_to_fixed};
use crate::support::sha1::{sha1_final, sha1_init, sha1_update};
use crate::vf::vq::{VQ, VqSegment};
use crate::vf::vq::{vq_create_still, vq_inplace_plus};

#[derive(Copy, Clone)]
pub struct GlyphHash {
    pub hash: [u8; 20],
}
fn hash_vqs(buf: &mut Buffer, s: VqSegment) {
    buf.write_u8(s.discriminant_byte());
    match s {
        VqSegment::Still(still) => {
            buf.write_u32be(otfcc_to_fixed(still as ::core::ffi::c_double) as u32);
        }
        VqSegment::Delta(delta) => {
            // `delta.region: *const VqRegion` is a deliberately-raw
            // borrowed pointer (Stage 7-2-f: a longer-lived, individually
            // `Box`-owned `VqRegion` inside `FvarTable.masters`, never a
            // c2rust residue), so only this deref needs `unsafe`.
            let region = unsafe { &*delta.region };
            buf.write_u32be(otfcc_to_fixed(delta.quantity as ::core::ffi::c_double) as u32);
            buf.write_u32be(region.dimensions as u32);
            for span in &region.spans {
                buf.write_u32be(otfcc_to_f2dot14(span.start as ::core::ffi::c_double) as u32);
                buf.write_u32be(otfcc_to_f2dot14(span.peak as ::core::ffi::c_double) as u32);
                buf.write_u32be(otfcc_to_f2dot14(span.end as ::core::ffi::c_double) as u32);
            }
        }
    }
}
fn hash_vq(buf: &mut Buffer, x: VQ) {
    buf.write_u32be(otfcc_to_fixed(x.kernel as ::core::ffi::c_double) as u32);
    buf.write_u32be(x.shift.len() as u32);
    for j in 0..x.shift.len() {
        hash_vqs(buf, x.shift[j]);
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
        hash_vq(buf, r.x.clone());
        hash_vq(buf, r.y.clone());
        buf.write_u32be(otfcc_to_f2dot14(r.a as ::core::ffi::c_double) as u32);
        buf.write_u32be(otfcc_to_f2dot14(r.b as ::core::ffi::c_double) as u32);
        buf.write_u32be(otfcc_to_f2dot14(r.c as ::core::ffi::c_double) as u32);
        buf.write_u32be(otfcc_to_f2dot14(r.d as ::core::ffi::c_double) as u32);
    }
    buf.write_u8(')' as i32 as u8);
    buf.write_u8('s' as i32 as u8);
    buf.write_u8('H' as i32 as u8);
    buf.write_u8('(' as i32 as u8);
    for stem in g.stem_h.iter() {
        buf.write_u32be(otfcc_to_fixed(stem.position as ::core::ffi::c_double) as u32);
        buf.write_u32be(otfcc_to_fixed(stem.width as ::core::ffi::c_double) as u32);
    }
    buf.write_u8(')' as i32 as u8);
    buf.write_u8('s' as i32 as u8);
    buf.write_u8('V' as i32 as u8);
    buf.write_u8('(' as i32 as u8);
    for stem in g.stem_v.iter() {
        buf.write_u32be(otfcc_to_fixed(stem.position as ::core::ffi::c_double) as u32);
        buf.write_u32be(otfcc_to_fixed(stem.width as ::core::ffi::c_double) as u32);
    }
    buf.write_u8(')' as i32 as u8);
    buf.write_u8('m' as i32 as u8);
    buf.write_u8('H' as i32 as u8);
    buf.write_u8('(' as i32 as u8);
    for mask in g.hint_masks.iter() {
        buf.write_u16be(mask.contours_before);
        buf.write_u16be(mask.points_before);
        for k in 0..g.stem_h.len() {
            buf.write_u8(mask.mask_h[k] as u8);
        }
        for k in 0..g.stem_v.len() {
            buf.write_u8(mask.mask_v[k] as u8);
        }
    }
    buf.write_u8(')' as i32 as u8);
    buf.write_u8('m' as i32 as u8);
    buf.write_u8('C' as i32 as u8);
    buf.write_u8('(' as i32 as u8);
    for mask in g.contour_masks.iter() {
        buf.write_u16be(mask.contours_before);
        buf.write_u16be(mask.points_before);
        for k in 0..g.stem_h.len() {
            buf.write_u8(mask.mask_h[k] as u8);
        }
        for k in 0..g.stem_v.len() {
            buf.write_u8(mask.mask_v[k] as u8);
        }
    }
    buf.write_u8(')' as i32 as u8);
    buf.write_u8('I' as i32 as u8);
    buf.write_u32be(g.instructions.len() as u32);
    buf.write_bytes(&g.instructions);
    let mut ctx: Sha1Ctx = Sha1Ctx {
        data: [0; 64],
        datalen: 0,
        bitlen: 0,
        state: [0; 5],
        k: [0; 4],
    };
    let mut hash: [u8; 20] = [0; 20];
    sha1_init(&mut ctx);
    sha1_update(&mut ctx, &buf.data);
    sha1_final(&mut ctx, &mut hash);
    let mut h_0: GlyphHash = GlyphHash { hash: [0; 20] };
    for j in 0..SHA1_BLOCK_SIZE as usize {
        h_0.hash[j] = hash[j];
    }
    return h_0;
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
    // Only ever called (from `otfcc_unconsolidate_font`) under a
    // `.glyf.is_some()` guard.
    let num_glyphs: GlyphId = font.glyf.as_ref().unwrap().len() as GlyphId;
    let prefix: Vec<u8> = if !options.glyph_name_prefix.is_null() {
        // `options.glyph_name_prefix: *const c_char` is a genuine external
        // C-string boundary (CLI-supplied), not c2rust residue.
        crate::bytesbuild!(unsafe {
            crate::support::fmt::CCharRef::from_ptr(options.glyph_name_prefix)
        })
    } else {
        Vec::new()
    };
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
                otfcc_set_glyph_order_by_gid(&mut glyph_order, j, newname_0)
            } else {
                otfcc_set_glyph_order_by_gid(&mut glyph_order, j, gname)
            };
            font.glyf.as_mut().unwrap()[j as usize]
                .as_mut()
                .unwrap()
                .name = shared_name;
        } else if !(options.ignore_glyph_order || options.name_glyphs_by_gid) {
            let existing_name = glyf[j as usize].as_deref().unwrap().name.clone();
            if !existing_name.is_empty() {
                let gname_0: Vec<u8> = crate::bytesbuild!(&prefix, &existing_name);
                let shared_name_1 = otfcc_set_glyph_order_by_gid(&mut glyph_order, j, gname_0);
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
    if let Some(post_name_map) = post_name_map {
        if !options.ignore_glyph_order && !options.name_glyphs_by_gid {
            for (_, &idx) in post_name_map.by_gid.iter() {
                let entry = &post_name_map.entries[idx];
                let gname_1: Vec<u8> = crate::bytesbuild!(&prefix, &entry.name);
                otfcc_set_glyph_order_by_gid(&mut glyph_order, entry.gid, gname_1);
            }
        }
    }
    if let Some(cmap) = font.cmap.as_ref().filter(|_| !options.name_glyphs_by_gid) {
        let mut aglfn = GlyphOrder {
            entries: Vec::new(),
            by_gid: std::collections::BTreeMap::new(),
            by_name: std::collections::HashMap::new(),
        };
        aglfn_setup_names(&mut aglfn);
        for (&unicode, glyph) in cmap.unicodes.iter() {
            if glyph.index as i32 > 0_i32 {
                let mut name_bytes: Vec<u8> = Vec::new();
                if unicode > 0_i32 && unicode < 0xffff_i32 {
                    otfcc_gord_name_a_field_shared(&aglfn, unicode as GlyphId, &mut name_bytes);
                }
                let name: Vec<u8>;
                if name_bytes.is_empty() {
                    name = crate::bytesbuild!(&prefix, b"uni", Hex4Upper(unicode as u32));
                } else {
                    name = crate::bytesbuild!(&prefix, &name_bytes);
                }
                otfcc_set_glyph_order_by_gid(&mut glyph_order, glyph.index, name);
            }
        }
        // `aglfn` is a plain local value now -- it drops here on its own,
        // no `otfcc_glyph_order_free` call needed.
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
        otfcc_set_glyph_order_by_gid(&mut glyph_order, j_1, name_0);
    }
    glyph_order
}
fn name_glyphs(font: &mut Font, gord: &GlyphOrder) {
    // Only ever called (from `otfcc_unconsolidate_font`) under a
    // `.glyf.is_some()` guard.
    let glyf = font.glyf.as_mut().unwrap();
    for j in 0..glyf.len() as GlyphId {
        let g = glyf[j as usize].as_mut().unwrap();
        let mut glyph_name: Vec<u8> = Vec::new();
        otfcc_gord_name_a_field_shared(gord, j, &mut glyph_name);
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
fn unconsolidate_chaining(lookup: &mut Lookup) {
    // The original C (c/lib/otf-reader/unconsolidate.c) computes a
    // `total_rules` count in a first pass over the subtables and never uses
    // it afterward (no capacity-reservation call, no other reference) --
    // genuinely dead code upstream, not a c2rust artifact. Confirmed by
    // inspection: the loop body only reads subtable fields into a local
    // accumulator with no other side effects. Omitted here.
    let mut newsts: SubtableList = Vec::new();
    'subtables: for j in 0..lookup.subtables.len() {
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
        let Some(mut sub_box) = lookup.subtables[j].take() else {
            continue;
        };
        let Subtable::Chaining(sub_chaining) = &mut *sub_box else {
            unreachable!()
        };
        match sub_chaining {
            ChainingSubtable::Poly(ruleset) => {
                // `None` would only appear here if the original binary read
                // failed partway through this same lookup and pushed a
                // placeholder; provably never the case for any payload this
                // crate builds successfully, so `.expect` turns that into a
                // clean panic instead of reproducing the old
                // null-pointer-deref UB. (Fuzzing did find a `None` here
                // before `chaining/read.rs`'s per-rule read functions were
                // changed to never push a failed individual rule into
                // `ruleset.rules` in the first place -- see that file's own
                // comment at each push site.)
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
                // `ChainingRule` has no custom `Drop`, so swapping its value
                // out through the `&mut` borrow (leaving a cheap empty
                // default behind for `sub_box` to drop normally) is a plain
                // safe move -- no raw-pointer surgery needed, unlike the
                // pre-enum version.
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
    // Was `otl_subtable_list_dispose_dependent(..); (*lookup).subtables =
    // newsts;` -- the plain assignment already drops the old
    // `Vec<Option<Box<Subtable>>>` in place (correctly disposing anything
    // left as `Some`: entries this loop didn't touch -- e.g. a `Classified`
    // subtable, or every remaining slot once the budget above cuts the loop
    // short -- before replacing it, so there is nothing left to do eagerly.
    lookup.subtables = newsts;
}
fn expand_chain(lookup: &mut Lookup) {
    match lookup.type_0 {
        OTL_TYPE_GSUB_CHAINING | OTL_TYPE_GPOS_CHAINING => {
            unconsolidate_chaining(lookup);
        }
        _ => {}
    };
}
fn expand_chaining_lookups(font: &mut Font) {
    if let Some(gsub) = font.gsub.as_deref_mut() {
        for lookup in gsub.lookups.iter_mut() {
            expand_chain(lookup);
        }
    }
    if let Some(gpos) = font.gpos.as_deref_mut() {
        for lookup in gpos.lookups.iter_mut() {
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
    for j in 0..glyf.len() as GlyphId {
        let g = glyf[j as usize].as_mut().unwrap();
        let adw: Pos = hmtx.metrics[(if (j as u32) < count_a {
            j as u32
        } else {
            count_a.wrapping_sub(1_u32)
        }) as usize]
            .advance_width as Pos;
        let lsb: Pos = if (j as u32) < count_a {
            hmtx.metrics[j as usize].lsb
        } else {
            hmtx.left_side_bearing[(j as u32).wrapping_sub(count_a) as usize]
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
        for j_0 in 0..vorg.num_vert_origin_y_metrics as GlyphId {
            let entry = vorg.entries[j_0 as usize];
            if (entry.gid as usize) < glyf_len {
                v[entry.gid as usize] = entry.vertical_origin as Pos;
            }
        }
        vorgs = Some(v);
    }
    let glyf = font.glyf.as_mut().unwrap();
    for j_1 in 0..glyf.len() as GlyphId {
        let g = glyf[j_1 as usize].as_mut().unwrap();
        let adh: Pos = vmtx.metrics[(if (j_1 as u32) < count_a {
            j_1 as u32
        } else {
            count_a.wrapping_sub(1_u32)
        }) as usize]
            .advance_height as Pos;
        let tsb: Pos = if (j_1 as u32) < count_a {
            vmtx.metrics[j_1 as usize].tsb
        } else {
            vmtx.top_side_bearing[(j_1 as u32).wrapping_sub(count_a) as usize]
        };
        vq_inplace_plus(&mut g.advance_height, vq_create_still(adh));
        vq_inplace_plus(
            &mut g.vertical_origin,
            vq_create_still(if let Some(v) = &vorgs {
                v[j_1 as usize]
            } else {
                tsb + g.stat.y_max
            }),
        );
    }
}
fn merge_ltsh(font: &mut Font) {
    if let Some(glyf) = font.glyf.as_mut() {
        if let Some(ltsh) = &font.ltsh {
            let n = (glyf.len() as GlyphId).min(ltsh.num_glyphs);
            for j in 0..n {
                glyf[j as usize].as_mut().unwrap().y_pel = ltsh.y_pels[j as usize];
            }
        }
    }
}
pub fn otfcc_unconsolidate_font(font: &mut Font, options: &Options) {
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
