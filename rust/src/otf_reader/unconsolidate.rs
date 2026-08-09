#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
#![allow(improper_ctypes_definitions)] // VQ now owns a Vec; these extern "C" fns are internal-only (vtable dispatch, no real FFI boundary) -- goes away with the vtable/extern "C" cleanup, see rust/README.md
use libc::{free};




use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::handle::{sds_to_vec};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, Pos};
use crate::vendor::sds::{Hex2Upper, Hex4Upper, SdsRaw};
use crate::font::caryll_font::{Font};
use crate::support::glyph_order::{GlyphOrder};
use crate::support::sha1::{BYTE, Sha1Ctx};









use crate::table::vorg::VorgTable;





use crate::table::glyf::{ComponentReference, Contour, Glyph, GlyfTable};



use crate::table::hmtx::HmtxTable;



use crate::table::otl::{ChainingRule, ChainingRuleSet, ChainingSubtable, Lookup, Subtable, SubtableList, SubtablePtr, ChainingType, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GSUB_CHAINING, OtlTable};




use crate::table::vmtx::VmtxTable;


use crate::vf::region::{VqAxisSpan};
use crate::vf::vq::{VQ, VQSegType, VqSegment};
use crate::support::aglfn::{aglfn_setup_names};
use crate::support::buffer::{buffree, buflen, bufnew, bufwrite16b, bufwrite32b, bufwrite8, bufwrite_bytes};
use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::support::primitives::{otfcc_to_f2dot14, otfcc_to_fixed};
use crate::support::sha1::{sha1_final, sha1_init, sha1_update};
use crate::table::vorg::{TABLE_I_VORG};
use crate::table::hmtx::{TABLE_I_HMTX};
use crate::table::otl::{otl_subtable_list_dispose_dependent};
use crate::table::vmtx::{TABLE_I_VMTX};
use crate::vendor::sds::{sdsempty, sdsfree, sdsnew};
use crate::vf::vq::{I_VQ};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphHash {
    pub hash: [u8; 20],
}
unsafe extern "C" fn hash_vqs(buf: *mut Buffer, s: VqSegment) {
    bufwrite8(buf, s.type_0 as u8);
    match s.type_0 {
        VQSegType::Still => {
            bufwrite32b(
                buf,
                otfcc_to_fixed(s.val.still as ::core::ffi::c_double) as u32,
            );
        }
        VQSegType::Delta => {
            bufwrite32b(
                buf,
                otfcc_to_fixed(s.val.delta.quantity as ::core::ffi::c_double) as u32,
            );
            bufwrite32b(buf, (*s.val.delta.region).dimensions as u32);
            for j in 0..(*s.val.delta.region).dimensions as usize {
                let span: *const VqAxisSpan =
                    (&raw const (*s.val.delta.region).spans as *const VqAxisSpan)
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
unsafe extern "C" fn hash_vq(buf: *mut Buffer, x: VQ) {
    bufwrite32b(
        buf,
        otfcc_to_fixed(x.kernel as ::core::ffi::c_double) as u32,
    );
    bufwrite32b(buf, x.shift.len() as u32);
    for j in 0..x.shift.len() {
        hash_vqs(buf, x.shift[j]);
    }
}
pub unsafe extern "C" fn name_glyph_by_hash(
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
unsafe extern "C" fn create_glyph_order(
    mut font: *mut Font,
    mut options: *const Options,
) -> *mut GlyphOrder {
    let mut glyph_order: *mut GlyphOrder =
        (
            OTFCC_PKG_GLYPH_ORDER
                .create
                .expect("non-null function pointer"))();
    let mut num_glyphs: GlyphId = (*(*font).glyf).len() as GlyphId;
    let mut prefix: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*options).glyph_name_prefix.is_null() {
        prefix = sdsnew((*options).glyph_name_prefix);
    } else {
        prefix = sdsempty();
    }
    for j in 0..num_glyphs {
        let mut g: *mut Glyph = &raw mut **(&mut (*(*font).glyf))[j as usize].as_mut().unwrap();
        if (*options).name_glyphs_by_hash {
            let h: GlyphHash = name_glyph_by_hash(g, (*font).glyf);
            let mut gname: SdsRaw = sdsempty();
            for j_0 in 0..SHA1_BLOCK_SIZE as u16 {
                if j_0 % 4 == 0 && j_0 / 4 != 0 {
                    gname = crate::sdsbuild!(
                        gname,
                        b"-",
                        Hex2Upper((h.hash[j_0 as usize] as ::core::ffi::c_int) as u32),
                    );
                } else {
                    gname = crate::sdsbuild!(
                        gname,
                        Hex2Upper((h.hash[j_0 as usize] as ::core::ffi::c_int) as u32),
                    );
                }
            }
            if OTFCC_PKG_GLYPH_ORDER
                .lookup_name
                .expect("non-null function pointer")(glyph_order, gname)
            {
                let mut n: GlyphId = 2 as GlyphId;
                let mut still_in: bool = false;
                loop {
                    if still_in {
                        n = (n as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
                    }
                    let mut newname: SdsRaw = crate::sdsbuild!(sdsempty(), gname, b"-", prefix, n as ::core::ffi::c_int);
                    still_in = OTFCC_PKG_GLYPH_ORDER
                        .lookup_name
                        .expect("non-null function pointer")(
                        glyph_order, newname
                    );
                    sdsfree(newname);
                    if !still_in {
                        break;
                    }
                }
                let mut newname_0: SdsRaw = crate::sdsbuild!(sdsempty(), gname, b"-", prefix, n as ::core::ffi::c_int);
                let mut shared_name: SdsRaw = OTFCC_PKG_GLYPH_ORDER
                    .set_by_gid
                    .expect("non-null function pointer")(
                    glyph_order, j, newname_0
                );
                (*g).name = sds_to_vec(shared_name);
                sdsfree(gname);
            } else {
                let mut shared_name_0: SdsRaw = OTFCC_PKG_GLYPH_ORDER
                    .set_by_gid
                    .expect("non-null function pointer")(
                    glyph_order, j, gname
                );
                (*g).name = sds_to_vec(shared_name_0);
            }
        } else if !((*options).ignore_glyph_order || (*options).name_glyphs_by_gid) {
            if !(*g).name.is_empty() {
                let mut gname_0: SdsRaw = crate::sdsbuild!(sdsempty(), prefix, &(*g).name);
                let shared_name_1: SdsRaw = OTFCC_PKG_GLYPH_ORDER
                    .set_by_gid
                    .expect("non-null function pointer")(
                    glyph_order, j, gname_0
                );
                (*g).name = sds_to_vec(shared_name_1);
            }
        }
    }
    if !(*font).post.is_null()
        && !(*(*font).post).post_name_map.is_null()
        && !(*options).ignore_glyph_order
        && !(*options).name_glyphs_by_gid
    {
        for (_, &s) in (*(*(*font).post).post_name_map).by_gid.iter() {
            let mut gname_1: SdsRaw = crate::sdsbuild!(sdsempty(), prefix, (*s).name);
            OTFCC_PKG_GLYPH_ORDER
                .set_by_gid
                .expect("non-null function pointer")(glyph_order, (*s).gid, gname_1);
        }
    }
    if !(*font).cmap.is_null() && !(*options).name_glyphs_by_gid {
        let mut aglfn: *mut GlyphOrder =
            (
                OTFCC_PKG_GLYPH_ORDER
                    .create
                    .expect("non-null function pointer"))();
        aglfn_setup_names(aglfn);
        for (&unicode, glyph) in (*(*font).cmap).unicodes.iter() {
            if glyph.index as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                let mut name: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if unicode > 0 as ::core::ffi::c_int
                    && unicode < 0xffff as ::core::ffi::c_int
                {
                    OTFCC_PKG_GLYPH_ORDER
                        .name_a_field_shared
                        .expect("non-null function pointer")(
                        aglfn,
                        unicode as GlyphId,
                        &raw mut name,
                    );
                }
                if name.is_null() {
                    name = crate::sdsbuild!(sdsempty(), prefix, b"uni", Hex4Upper(unicode as u32));
                } else {
                    name = crate::sdsbuild!(sdsempty(), prefix, name);
                }
                OTFCC_PKG_GLYPH_ORDER
                    .set_by_gid
                    .expect("non-null function pointer")(
                    glyph_order, glyph.index, name
                );
            }
        }
        OTFCC_PKG_GLYPH_ORDER.free.expect("non-null function pointer")(aglfn);
    }
    for j_1 in 0..num_glyphs {
        let mut name_0: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if j_1 > 1 {
            name_0 = crate::sdsbuild!(sdsempty(), prefix, b"glyph", j_1 as ::core::ffi::c_int);
        } else if j_1 == 1 {
            if (&(*(*font).glyf))[1 as usize].is_some()
                && (&(*(*font).glyf))[1 as usize].as_deref().unwrap()
                .contours
                .is_empty()
                && (&(*(*font).glyf))[1 as usize].as_deref().unwrap()
                .references
                .is_empty()
            {
                name_0 = crate::sdsbuild!(sdsempty(), prefix, b".null");
            } else {
                name_0 = crate::sdsbuild!(sdsempty(), prefix, b"glyph", j_1 as ::core::ffi::c_int);
            }
        } else {
            name_0 = crate::sdsbuild!(sdsempty(), prefix, b".notdef");
        }
        OTFCC_PKG_GLYPH_ORDER
            .set_by_gid
            .expect("non-null function pointer")(glyph_order, j_1, name_0);
    }
    sdsfree(prefix);
    return glyph_order;
}
unsafe extern "C" fn name_glyphs(mut font: *mut Font, mut gord: *mut GlyphOrder) {
    if gord.is_null() {
        return;
    }
    for j in 0..(*(*font).glyf).len() as GlyphId {
        let g: *mut Glyph = &raw mut **(&mut (*(*font).glyf))[j as usize].as_mut().unwrap();
        let mut glyph_name: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
        OTFCC_PKG_GLYPH_ORDER
            .name_a_field_shared
            .expect("non-null function pointer")(gord, j, &raw mut glyph_name);
        (*g).name = sds_to_vec(glyph_name);
    }
}
unsafe extern "C" fn unconsolidate_chaining(
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
        let sub: SubtablePtr = (&(*lookup).subtables)[j];
        if sub.is_null() {
            continue;
        }
        let sub_chaining: *mut ChainingSubtable = &raw mut (*sub).chaining as *mut ChainingSubtable;
        if (*sub_chaining).type_0 == ChainingType::Poly {
            let ruleset: *mut ChainingRuleSet =
                &raw mut (*sub_chaining).c2rust_unnamed.c2rust_unnamed as *mut ChainingRuleSet;
            // `mem::take` both hands us the rule list by value (an owned
            // `Vec<Option<Box<ChainingRule>>>` to iterate) and leaves an
            // empty one behind in `*ruleset` -- so there's nothing left to
            // double-free when `sub`'s raw block is freed below. `None`
            // would only appear here if the original binary read failed
            // partway through this same lookup and pushed a placeholder;
            // provably never the case for any payload this crate builds
            // successfully, so `.expect` turns that into a clean panic
            // instead of reproducing the old null-pointer-deref UB.
            for rule_slot in ::core::mem::take(&mut (*ruleset).rules) {
                let boxed_rule: Box<ChainingRule> =
                    rule_slot.expect("chaining rule slot should never be None here");
                let st: *mut Subtable = __caryll_allocate_clean(
                    ::core::mem::size_of::<Subtable>() as usize,
                    278 as ::core::ffi::c_ulong,
                ) as *mut Subtable;
                let st_chaining: *mut ChainingSubtable = &raw mut (*st).chaining as *mut ChainingSubtable;
                (*st_chaining).type_0 = ChainingType::Canonical;
                // Move the rule's contents out of the `Box` (deallocating
                // just the box's own heap slot through the same allocator
                // that made it) into the `Canonical` variant's
                // `ManuallyDrop` slot -- simpler than the old raw-pointer
                // `ptr::read`, since `Box` supports moving its pointee out
                // directly.
                ::core::ptr::write(
                    &raw mut (*st_chaining).c2rust_unnamed.rule,
                    ::core::mem::ManuallyDrop::new(*boxed_rule),
                );
                newsts.push(st as SubtablePtr);
            }
            free(sub as *mut ::core::ffi::c_void);
            (&mut (*lookup).subtables)[j] = ::core::ptr::null_mut::<Subtable>();
        } else if (*sub_chaining).type_0 == ChainingType::Canonical {
            let st_0: *mut Subtable = __caryll_allocate_clean(
                ::core::mem::size_of::<Subtable>() as usize,
                289 as ::core::ffi::c_ulong,
            ) as *mut Subtable;
            let st_0_chaining: *mut ChainingSubtable = &raw mut (*st_0).chaining as *mut ChainingSubtable;
            (*st_0_chaining).type_0 = ChainingType::Canonical;
            // Same disguised move as the `Poly` branch above, but note: `sub`
            // (the whole outer `Subtable` block) is never `free()`'d on this
            // branch in the original C, either -- an existing, intentional-looking
            // leak that predates this conversion. `ptr::read` here preserves it
            // exactly: the source bytes (including the moved-from `Vec` header)
            // are left untouched in `sub`'s leaked allocation, never dropped.
            ::core::ptr::write(
                &raw mut (*st_0_chaining).c2rust_unnamed.rule,
                ::core::mem::ManuallyDrop::new(::core::ptr::read(
                    &raw const (*sub_chaining).c2rust_unnamed.rule as *const ChainingRule,
                )),
            );
            newsts.push(st_0 as SubtablePtr);
            (&mut (*lookup).subtables)[j] = ::core::ptr::null_mut::<Subtable>();
        }
    }
    otl_subtable_list_dispose_dependent(&raw mut (*lookup).subtables, lookup);
    (*lookup).subtables = newsts;
}
unsafe extern "C" fn expand_chain(font: *mut Font, lookup: *mut Lookup, table: *mut OtlTable) {
    match (*lookup).type_0 {
        OTL_TYPE_GSUB_CHAINING | OTL_TYPE_GPOS_CHAINING => {
            unconsolidate_chaining(font, lookup, table);
        }
        _ => {}
    };
}
unsafe extern "C" fn expand_chaining_lookups(font: *mut Font) {
    if !(*font).gsub.is_null() {
        for j in 0..(*(*font).gsub).lookups.len() {
            let lookup: *mut Lookup = &raw mut *(&mut (*(*font).gsub).lookups)[j];
            expand_chain(font, lookup, (*font).gsub);
        }
    }
    if !(*font).gpos.is_null() {
        for j in 0..(*(*font).gpos).lookups.len() {
            let lookup: *mut Lookup = &raw mut *(&mut (*(*font).gpos).lookups)[j];
            expand_chain(font, lookup, (*font).gpos);
        }
    }
}
unsafe extern "C" fn merge_hmtx(font: *mut Font) {
    if !(!(*font).hhea.is_null() && !(*font).hmtx.is_null() && !(*font).glyf.is_null()) {
        return;
    }
    let count_a: u32 = (*(*font).hhea).number_of_metrics as u32;
    for j in 0..(*(*font).glyf).len() as GlyphId {
        let g: *mut Glyph = &raw mut **(&mut (*(*font).glyf))[j as usize].as_mut().unwrap();
        let adw: Pos = (*(*(*font).hmtx).metrics.offset(
            (if (j as u32) < count_a {
                j as u32
            } else {
                count_a.wrapping_sub(1 as u32)
            }) as isize,
        ))
        .advance_width as Pos;
        let lsb: Pos = if (j as u32) < count_a {
            (*(*(*font).hmtx).metrics.offset(j as isize)).lsb
        } else {
            *(*(*font).hmtx)
                .left_side_bearing
                .offset((j as u32).wrapping_sub(count_a) as isize)
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
    TABLE_I_HMTX.free.expect("non-null function pointer")((*font).hmtx);
    (*font).hmtx = ::core::ptr::null_mut::<HmtxTable>();
}
unsafe extern "C" fn merge_vmtx(font: *mut Font) {
    if !(!(*font).vhea.is_null() && !(*font).vmtx.is_null() && !(*font).glyf.is_null()) {
        return;
    }
    let count_a: u32 = (*(*font).vhea).num_of_long_ver_metrics as u32;
    let mut vorgs: *mut Pos = ::core::ptr::null_mut::<Pos>();
    if !(*font).vorg.is_null() {
        vorgs = __caryll_allocate_clean(
            (::core::mem::size_of::<Pos>() as usize).wrapping_mul((*(*font).glyf).len()),
            351 as ::core::ffi::c_ulong,
        ) as *mut Pos;
        for j in 0..(*(*font).glyf).len() as GlyphId {
            *vorgs.offset(j as isize) = (*(*font).vorg).default_vertical_origin;
        }
        for j_0 in 0..(*(*font).vorg).num_vert_origin_y_metrics as GlyphId {
            if ((*(*(*font).vorg).entries.offset(j_0 as isize)).gid as usize)
                < (*(*font).glyf).len()
            {
                *vorgs.offset((*(*(*font).vorg).entries.offset(j_0 as isize)).gid as isize) =
                    (*(*(*font).vorg).entries.offset(j_0 as isize)).vertical_origin as Pos;
            }
        }
        TABLE_I_VORG.free.expect("non-null function pointer")((*font).vorg);
        (*font).vorg = ::core::ptr::null_mut::<VorgTable>();
    }
    for j_1 in 0..(*(*font).glyf).len() as GlyphId {
        let g: *mut Glyph = &raw mut **(&mut (*(*font).glyf))[j_1 as usize].as_mut().unwrap();
        let adh: Pos = (*(*(*font).vmtx).metrics.offset(
            (if (j_1 as u32) < count_a {
                j_1 as u32
            } else {
                count_a.wrapping_sub(1 as u32)
            }) as isize,
        ))
        .advance_height as Pos;
        let tsb: Pos = if (j_1 as u32) < count_a {
            (*(*(*font).vmtx).metrics.offset(j_1 as isize)).tsb
        } else {
            *(*(*font).vmtx)
                .top_side_bearing
                .offset((j_1 as u32).wrapping_sub(count_a) as isize)
        };
        I_VQ.inplace_plus.expect("non-null function pointer")(
            &raw mut (*g).advance_height,
            I_VQ.create_still.expect("non-null function pointer")(adh) as VQ,
        );
        I_VQ.inplace_plus.expect("non-null function pointer")(
            &raw mut (*g).vertical_origin,
            I_VQ.create_still.expect("non-null function pointer")(if !vorgs.is_null() {
                *vorgs.offset(j_1 as isize)
            } else {
                tsb + (*g).stat.y_max
            }) as VQ,
        );
    }
    if !vorgs.is_null() {
        free(vorgs as *mut ::core::ffi::c_void);
        vorgs = ::core::ptr::null_mut::<Pos>();
    }
    TABLE_I_VMTX.free.expect("non-null function pointer")((*font).vmtx);
    (*font).vmtx = ::core::ptr::null_mut::<VmtxTable>();
}
unsafe extern "C" fn merge_ltsh(font: *mut Font) {
    if !(*font).glyf.is_null() && !(*font).ltsh.is_null() {
        let n = ((*(*font).glyf).len() as GlyphId).min((*(*font).ltsh).num_glyphs);
        for j in 0..n {
            (&mut (*(*font).glyf))[j as usize].as_mut().unwrap().y_pel =
                *(*(*font).ltsh).y_pels.offset(j as isize);
        }
    }
}
pub unsafe extern "C" fn otfcc_unconsolidate_font(
    mut font: *mut Font,
    mut options: *const Options,
) {
    merge_hmtx(font);
    merge_vmtx(font);
    merge_ltsh(font);
    expand_chaining_lookups(font);
    if !(*font).glyf.is_null() {
        let mut gord: *mut GlyphOrder = create_glyph_order(font, options);
        name_glyphs(font, gord);
        OTFCC_PKG_GLYPH_ORDER.free.expect("non-null function pointer")(gord);
    }
}
pub const SHA1_BLOCK_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
