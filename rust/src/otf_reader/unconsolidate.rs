#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
#![allow(improper_ctypes_definitions)] // VQ now owns a Vec; these extern "C" fns are internal-only (vtable dispatch, no real FFI boundary) -- goes away with the vtable/extern "C" cleanup, see rust/README.md
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, Pos};
use crate::vendor::sds::{Hex2Upper, Hex4Upper, SdsPart};
use crate::font::caryll_font::{Font};
use crate::support::glyph_order::{GlyphOrder};
use crate::support::sha1::{BYTE, Sha1Ctx};














use crate::table::glyf::{ComponentReference, Contour, Glyph, GlyfTable};






use crate::table::otl::{ChainingRule, ChainingSubtable, Lookup, Subtable, SubtableList, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GSUB_CHAINING, OtlTable};






use crate::vf::region::{VqAxisSpan};
use crate::vf::vq::{VQ, VqSegment};
use crate::support::aglfn::{aglfn_setup_names};
use crate::support::buffer::{buffree, buflen, bufnew, bufwrite16b, bufwrite32b, bufwrite8, bufwrite_bytes};
use crate::support::glyph_order::{
    gord_lookup_name, otfcc_glyph_order_create, otfcc_glyph_order_free,
    otfcc_gord_name_a_field_shared, otfcc_set_glyph_order_by_gid,
};
use crate::support::primitives::{otfcc_to_f2dot14, otfcc_to_fixed};
use crate::support::sha1::{sha1_final, sha1_init, sha1_update};
use crate::vf::vq::{I_VQ};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphHash {
    pub hash: [u8; 20],
}
unsafe fn hash_vqs(buf: *mut Buffer, s: VqSegment) {
    bufwrite8(buf, s.discriminant_byte());
    match s {
        VqSegment::Still(still) => {
            bufwrite32b(
                buf,
                otfcc_to_fixed(still as ::core::ffi::c_double) as u32,
            );
        }
        VqSegment::Delta(delta) => {
            bufwrite32b(
                buf,
                otfcc_to_fixed(delta.quantity as ::core::ffi::c_double) as u32,
            );
            bufwrite32b(buf, (*delta.region).dimensions as u32);
            for j in 0..(*delta.region).dimensions as usize {
                let span: *const VqAxisSpan =
                    (&raw const (*delta.region).spans as *const VqAxisSpan)
                        .offset(j as isize);
                bufwrite32b(
                    buf,
                    otfcc_to_f2dot14((*span).start as ::core::ffi::c_double) as u32,
                );
                bufwrite32b(
                    buf,
                    otfcc_to_f2dot14((*span).peak as ::core::ffi::c_double) as u32,
                );
                bufwrite32b(
                    buf,
                    otfcc_to_f2dot14((*span).end as ::core::ffi::c_double) as u32,
                );
            }
        }
    }
}
unsafe fn hash_vq(buf: *mut Buffer, x: VQ) {
    bufwrite32b(
        buf,
        otfcc_to_fixed(x.kernel as ::core::ffi::c_double) as u32,
    );
    bufwrite32b(buf, x.shift.len() as u32);
    for j in 0..x.shift.len() {
        hash_vqs(buf, x.shift[j]);
    }
}
pub unsafe fn name_glyph_by_hash(
    mut g: *const Glyph,
    mut glyf: *const GlyfTable,
) -> GlyphHash {
    let buf: *mut Buffer = bufnew();
    bufwrite8(buf, 'H' as i32 as u8);
    hash_vq(buf, (*g).advance_width.clone());
    bufwrite8(buf, 'h' as i32 as u8);
    hash_vq(buf, (*g).horizontal_origin.clone());
    bufwrite8(buf, 'V' as i32 as u8);
    hash_vq(buf, (*g).advance_height.clone());
    bufwrite8(buf, 'v' as i32 as u8);
    hash_vq(buf, (*g).vertical_origin.clone());
    bufwrite8(buf, 'C' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).contours.len() {
        bufwrite8(buf, '(' as i32 as u8);
        let c: *const Contour = &(&(*g).contours)[j];
        for k in 0..(*c).len() {
            let point = &(&(*c))[k];
            hash_vq(buf, point.x.clone());
            hash_vq(buf, point.y.clone());
            bufwrite8(buf, (point.on_curve != 0) as u8);
        }
        bufwrite8(buf, ')' as i32 as u8);
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'R' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).references.len() {
        let r: *const ComponentReference = &(&(*g).references)[j];
        let mut h: GlyphHash = name_glyph_by_hash(
            (&(*glyf))[(*r).glyph.index as usize].as_deref().unwrap() as *const Glyph,
            glyf,
        );
        bufwrite_bytes(
            buf,
            SHA1_BLOCK_SIZE as usize,
            &raw mut h.hash as *mut u8,
        );
        hash_vq(buf, (*r).x.clone());
        hash_vq(buf, (*r).y.clone());
        bufwrite32b(
            buf,
            otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as u32,
        );
        bufwrite32b(
            buf,
            otfcc_to_f2dot14((*r).b as ::core::ffi::c_double) as u32,
        );
        bufwrite32b(
            buf,
            otfcc_to_f2dot14((*r).c as ::core::ffi::c_double) as u32,
        );
        bufwrite32b(
            buf,
            otfcc_to_f2dot14((*r).d as ::core::ffi::c_double) as u32,
        );
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 's' as i32 as u8);
    bufwrite8(buf, 'H' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for stem in (*g).stem_h.iter() {
        bufwrite32b(buf, otfcc_to_fixed(stem.position as ::core::ffi::c_double) as u32);
        bufwrite32b(buf, otfcc_to_fixed(stem.width as ::core::ffi::c_double) as u32);
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 's' as i32 as u8);
    bufwrite8(buf, 'V' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for stem in (*g).stem_v.iter() {
        bufwrite32b(buf, otfcc_to_fixed(stem.position as ::core::ffi::c_double) as u32);
        bufwrite32b(buf, otfcc_to_fixed(stem.width as ::core::ffi::c_double) as u32);
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'm' as i32 as u8);
    bufwrite8(buf, 'H' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for mask in (*g).hint_masks.iter() {
        bufwrite16b(buf, mask.contours_before);
        bufwrite16b(buf, mask.points_before);
        for k in 0..(*g).stem_h.len() {
            bufwrite8(buf, mask.mask_h[k] as u8);
        }
        for k in 0..(*g).stem_v.len() {
            bufwrite8(buf, mask.mask_v[k] as u8);
        }
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'm' as i32 as u8);
    bufwrite8(buf, 'C' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for mask in (*g).contour_masks.iter() {
        bufwrite16b(buf, mask.contours_before);
        bufwrite16b(buf, mask.points_before);
        for k in 0..(*g).stem_h.len() {
            bufwrite8(buf, mask.mask_h[k] as u8);
        }
        for k in 0..(*g).stem_v.len() {
            bufwrite8(buf, mask.mask_v[k] as u8);
        }
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'I' as i32 as u8);
    bufwrite32b(buf, (*g).instructions_length as u32);
    bufwrite_bytes(buf, (*g).instructions_length as usize, (*g).instructions);
    let mut ctx: Sha1Ctx = Sha1Ctx {
        data: [0; 64],
        datalen: 0,
        bitlen: 0,
        state: [0; 5],
        k: [0; 4],
    };
    let mut hash: [u8; 20] = [0; 20];
    sha1_init(&raw mut ctx);
    sha1_update(&raw mut ctx, (*buf).data as *const BYTE, buflen(buf));
    sha1_final(&raw mut ctx, &raw mut hash as *mut BYTE);
    let mut h_0: GlyphHash = GlyphHash { hash: [0; 20] };
    for j in 0..SHA1_BLOCK_SIZE as usize {
        h_0.hash[j] = hash[j];
    }
    buffree(buf);
    return h_0;
}
unsafe fn create_glyph_order(
    mut font: *mut Font,
    mut options: *const Options,
) -> *mut GlyphOrder {
    let mut glyph_order: *mut GlyphOrder =
        (
            otfcc_glyph_order_create)();
    // Only ever called (from `otfcc_unconsolidate_font`) under a
    // `.glyf.is_some()` guard.
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    let mut num_glyphs: GlyphId = (*glyf).len() as GlyphId;
    let prefix: Vec<u8> = if !(*options).glyph_name_prefix.is_null() {
        crate::bytesbuild!((*options).glyph_name_prefix)
    } else {
        Vec::new()
    };
    for j in 0..num_glyphs {
        let mut g: *mut Glyph = &raw mut **(&mut (*glyf))[j as usize].as_mut().unwrap();
        if (*options).name_glyphs_by_hash {
            let h: GlyphHash = name_glyph_by_hash(g, glyf);
            let mut gname: Vec<u8> = Vec::new();
            for j_0 in 0..SHA1_BLOCK_SIZE as u16 {
                if j_0 % 4 == 0 && j_0 / 4 != 0 {
                    gname.extend_from_slice(b"-");
                }
                Hex2Upper((h.hash[j_0 as usize] as ::core::ffi::c_int) as u32)
                    .append_to_vec(&mut gname);
            }
            if gord_lookup_name(glyph_order, gname.clone())
            {
                let mut n: GlyphId = 2 as GlyphId;
                let mut still_in: bool = false;
                loop {
                    if still_in {
                        n = (n as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
                    }
                    let newname: Vec<u8> =
                        crate::bytesbuild!(&gname, b"-", &prefix, n as ::core::ffi::c_int);
                    still_in = gord_lookup_name(
                        glyph_order, newname
                    );
                    if !still_in {
                        break;
                    }
                }
                let newname_0: Vec<u8> =
                    crate::bytesbuild!(&gname, b"-", &prefix, n as ::core::ffi::c_int);
                let shared_name: Vec<u8> = otfcc_set_glyph_order_by_gid(
                    glyph_order, j, newname_0
                );
                (*g).name = shared_name;
            } else {
                let shared_name_0: Vec<u8> = otfcc_set_glyph_order_by_gid(
                    glyph_order, j, gname
                );
                (*g).name = shared_name_0;
            }
        } else if !((*options).ignore_glyph_order || (*options).name_glyphs_by_gid) {
            if !(*g).name.is_empty() {
                let gname_0: Vec<u8> = crate::bytesbuild!(&prefix, &(*g).name);
                let shared_name_1: Vec<u8> = otfcc_set_glyph_order_by_gid(
                    glyph_order, j, gname_0
                );
                (*g).name = shared_name_1;
            }
        }
    }
    let post_name_map: *mut GlyphOrder = (*font)
        .post
        .as_deref()
        .map_or(::core::ptr::null_mut(), |p| p.post_name_map);
    if !post_name_map.is_null()
        && !(*options).ignore_glyph_order
        && !(*options).name_glyphs_by_gid
    {
        for (_, &idx) in (*post_name_map).by_gid.iter() {
            let entry = &(&(*post_name_map).entries)[idx];
            let gname_1: Vec<u8> = crate::bytesbuild!(&prefix, &entry.name);
            otfcc_set_glyph_order_by_gid(glyph_order, entry.gid, gname_1);
        }
    }
    if (*font).cmap.is_some() && !(*options).name_glyphs_by_gid {
        let mut aglfn: *mut GlyphOrder =
            (
                otfcc_glyph_order_create)();
        aglfn_setup_names(aglfn);
        for (&unicode, glyph) in (*font).cmap.as_ref().unwrap().unicodes.iter() {
            if glyph.index as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                let mut name_bytes: Vec<u8> = Vec::new();
                if unicode > 0 as ::core::ffi::c_int
                    && unicode < 0xffff as ::core::ffi::c_int
                {
                    otfcc_gord_name_a_field_shared(
                        aglfn,
                        unicode as GlyphId,
                        &raw mut name_bytes,
                    );
                }
                let name: Vec<u8>;
                if name_bytes.is_empty() {
                    name = crate::bytesbuild!(&prefix, b"uni", Hex4Upper(unicode as u32));
                } else {
                    name = crate::bytesbuild!(&prefix, &name_bytes);
                }
                otfcc_set_glyph_order_by_gid(
                    glyph_order, glyph.index, name
                );
            }
        }
        otfcc_glyph_order_free(aglfn);
    }
    for j_1 in 0..num_glyphs {
        let name_0: Vec<u8>;
        if j_1 > 1 {
            name_0 = crate::bytesbuild!(&prefix, b"glyph", j_1 as ::core::ffi::c_int);
        } else if j_1 == 1 {
            if (&(*glyf))[1 as usize].is_some()
                && (&(*glyf))[1 as usize].as_deref().unwrap()
                .contours
                .is_empty()
                && (&(*glyf))[1 as usize].as_deref().unwrap()
                .references
                .is_empty()
            {
                name_0 = crate::bytesbuild!(&prefix, b".null");
            } else {
                name_0 = crate::bytesbuild!(&prefix, b"glyph", j_1 as ::core::ffi::c_int);
            }
        } else {
            name_0 = crate::bytesbuild!(&prefix, b".notdef");
        }
        otfcc_set_glyph_order_by_gid(glyph_order, j_1, name_0);
    }
    return glyph_order;
}
unsafe fn name_glyphs(mut font: *mut Font, mut gord: *mut GlyphOrder) {
    if gord.is_null() {
        return;
    }
    // Only ever called (from `otfcc_unconsolidate_font`) under a
    // `.glyf.is_some()` guard.
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    for j in 0..(*glyf).len() as GlyphId {
        let g: *mut Glyph = &raw mut **(&mut (*glyf))[j as usize].as_mut().unwrap();
        let mut glyph_name: Vec<u8> = Vec::new();
        otfcc_gord_name_a_field_shared(gord, j, &raw mut glyph_name);
        (*g).name = glyph_name;
    }
}
unsafe fn unconsolidate_chaining(
    _font: *mut Font,
    lookup: *mut Lookup,
    _table: *mut OtlTable,
) {
    // The original C (c/lib/otf-reader/unconsolidate.c) computes a
    // `total_rules` count in a first pass over the subtables and never uses
    // it afterward (no capacity-reservation call, no other reference) --
    // genuinely dead code upstream, not a c2rust artifact. Confirmed by
    // inspection: the loop body only reads subtable fields into a local
    // accumulator with no other side effects. Omitted here.
    let mut newsts: SubtableList = Vec::new();
    for j in 0..(*lookup).subtables.len() {
        // `.take()` moves the `Box` out of the slot, leaving `None` behind.
        // `Subtable` implements `Drop`, so its payload can't be moved out
        // by value through a pattern match (even via `*sub_box`) -- only
        // mutated through a `&mut` borrow, which is all the branches below
        // need. `sub_box` itself drops normally at the end of each
        // iteration, cleaning up whatever's left behind (empty after the
        // `mem::take`s below).
        let Some(mut sub_box) = (&mut (*lookup).subtables)[j].take() else {
            continue;
        };
        let Subtable::Chaining(sub_chaining) = &mut *sub_box else { unreachable!() };
        match sub_chaining {
            ChainingSubtable::Poly(ruleset) => {
                // `None` would only appear here if the original binary read
                // failed partway through this same lookup and pushed a
                // placeholder; provably never the case for any payload this
                // crate builds successfully, so `.expect` turns that into a
                // clean panic instead of reproducing the old
                // null-pointer-deref UB.
                for rule_slot in ::core::mem::take(&mut ruleset.rules) {
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
    // left as `Some`: entries this loop didn't touch, e.g. a `Classified`
    // subtable) before replacing it, so there is nothing left to do eagerly.
    (*lookup).subtables = newsts;
}
unsafe fn expand_chain(font: *mut Font, lookup: *mut Lookup, table: *mut OtlTable) {
    match (*lookup).type_0 {
        OTL_TYPE_GSUB_CHAINING | OTL_TYPE_GPOS_CHAINING => {
            unconsolidate_chaining(font, lookup, table);
        }
        _ => {}
    };
}
unsafe fn expand_chaining_lookups(font: *mut Font) {
    if let Some(gsub) = (*font).gsub.as_mut() {
        let gsub: *mut OtlTable = gsub.as_mut() as *mut OtlTable;
        for j in 0..(*gsub).lookups.len() {
            let lookup: *mut Lookup = &raw mut *(&mut (*gsub).lookups)[j];
            expand_chain(font, lookup, gsub);
        }
    }
    if let Some(gpos) = (*font).gpos.as_mut() {
        let gpos: *mut OtlTable = gpos.as_mut() as *mut OtlTable;
        for j in 0..(*gpos).lookups.len() {
            let lookup: *mut Lookup = &raw mut *(&mut (*gpos).lookups)[j];
            expand_chain(font, lookup, gpos);
        }
    }
}
unsafe fn merge_hmtx(font: *mut Font) {
    if !((*font).hhea.is_some() && (*font).hmtx.is_some() && (*font).glyf.is_some()) {
        return;
    }
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    let count_a: u32 = (*font).hhea.as_deref().unwrap().number_of_metrics as u32;
    let hmtx = (*font).hmtx.take().unwrap();
    for j in 0..(*glyf).len() as GlyphId {
        let g: *mut Glyph = &raw mut **(&mut (*glyf))[j as usize].as_mut().unwrap();
        let adw: Pos = hmtx.metrics[(if (j as u32) < count_a {
            j as u32
        } else {
            count_a.wrapping_sub(1 as u32)
        }) as usize]
            .advance_width as Pos;
        let lsb: Pos = if (j as u32) < count_a {
            hmtx.metrics[j as usize].lsb
        } else {
            hmtx.left_side_bearing[(j as u32).wrapping_sub(count_a) as usize]
        };
        I_VQ.inplace_plus.expect("non-null function pointer")(
            &raw mut (*g).advance_width,
            I_VQ.create_still.expect("non-null function pointer")(adw) as VQ,
        );
        I_VQ.inplace_plus.expect("non-null function pointer")(
            &raw mut (*g).horizontal_origin,
            I_VQ.create_still.expect("non-null function pointer")(-lsb + (*g).stat.x_min) as VQ,
        );
    }
}
unsafe fn merge_vmtx(font: *mut Font) {
    if !((*font).vhea.is_some() && (*font).vmtx.is_some() && (*font).glyf.is_some()) {
        return;
    }
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    let count_a: u32 = (*font).vhea.as_deref().unwrap().num_of_long_ver_metrics as u32;
    let vmtx = (*font).vmtx.take().unwrap();
    let mut vorgs: Option<Vec<Pos>> = None;
    if let Some(vorg) = (*font).vorg.take() {
        let mut v: Vec<Pos> = vec![vorg.default_vertical_origin; (*glyf).len()];
        for j_0 in 0..vorg.num_vert_origin_y_metrics as GlyphId {
            if ((*vorg.entries.offset(j_0 as isize)).gid as usize) < (*glyf).len() {
                v[(*vorg.entries.offset(j_0 as isize)).gid as usize] =
                    (*vorg.entries.offset(j_0 as isize)).vertical_origin as Pos;
            }
        }
        vorgs = Some(v);
    }
    for j_1 in 0..(*glyf).len() as GlyphId {
        let g: *mut Glyph = &raw mut **(&mut (*glyf))[j_1 as usize].as_mut().unwrap();
        let adh: Pos = vmtx.metrics[(if (j_1 as u32) < count_a {
            j_1 as u32
        } else {
            count_a.wrapping_sub(1 as u32)
        }) as usize]
            .advance_height as Pos;
        let tsb: Pos = if (j_1 as u32) < count_a {
            vmtx.metrics[j_1 as usize].tsb
        } else {
            vmtx.top_side_bearing[(j_1 as u32).wrapping_sub(count_a) as usize]
        };
        I_VQ.inplace_plus.expect("non-null function pointer")(
            &raw mut (*g).advance_height,
            I_VQ.create_still.expect("non-null function pointer")(adh) as VQ,
        );
        I_VQ.inplace_plus.expect("non-null function pointer")(
            &raw mut (*g).vertical_origin,
            I_VQ.create_still.expect("non-null function pointer")(if let Some(v) = &vorgs {
                v[j_1 as usize]
            } else {
                tsb + (*g).stat.y_max
            }) as VQ,
        );
    }
}
unsafe fn merge_ltsh(font: *mut Font) {
    if let Some(glyf) = (*font).glyf.as_mut() {
        let glyf: *mut GlyfTable = glyf as *mut GlyfTable;
        if let Some(ltsh) = &(*font).ltsh {
            let n = ((*glyf).len() as GlyphId).min(ltsh.num_glyphs);
            for j in 0..n {
                (&mut (*glyf))[j as usize].as_mut().unwrap().y_pel =
                    *ltsh.y_pels.offset(j as isize);
            }
        }
    }
}
pub unsafe fn otfcc_unconsolidate_font(
    mut font: *mut Font,
    mut options: *const Options,
) {
    merge_hmtx(font);
    merge_vmtx(font);
    merge_ltsh(font);
    expand_chaining_lookups(font);
    if (*font).glyf.is_some() {
        let mut gord: *mut GlyphOrder = create_glyph_order(font, options);
        name_glyphs(font, gord);
        otfcc_glyph_order_free(gord);
    }
}
pub const SHA1_BLOCK_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
