#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};
unsafe extern "C" {
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn otfcc_to_f2dot14(x: ::core::ffi::c_double) -> i16;
    fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16;
    static iVQ: __caryll_vectorinterface_VQ;
    static table_iVmtx: __caryll_elementinterface_table_vmtx;
    static table_iVORG: __caryll_elementinterface_table_VORG;
    static table_iHmtx: __caryll_elementinterface_table_hmtx;
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    static otl_iSubtableList: __caryll_vectorinterface_otl_SubtableList;
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn buflen(buf: *mut caryll_Buffer) -> usize;
    fn bufwrite8(buf: *mut caryll_Buffer, byte: u8);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn bufwrite_bytes(buf: *mut caryll_Buffer, size: usize, str: *const u8);
    fn aglfn_setupNames(map: *mut otfcc_GlyphOrder);
    fn sha1_init(ctx: *mut SHA1_CTX);
    fn sha1_update(ctx: *mut SHA1_CTX, data: *const BYTE, len: usize);
    fn sha1_final(ctx: *mut SHA1_CTX, hash: *mut BYTE);
}




use crate::support::alloc::{__caryll_allocate_clean};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{f16dot16, glyphid_t, pos_t, tableid_t};
use crate::vendor::sds::{Hex2Upper, Hex4Upper, sds};
use crate::font::caryll_font::{otfcc_Font};
use crate::support::{NULL};
use crate::support::glyph_order::{otfcc_GlyphOrder, otfcc_GlyphOrderEntry, otfcc_GlyphOrderPackage};
use crate::support::sha1::{BYTE, SHA1_CTX};









use crate::table::VORG::{__caryll_elementinterface_table_VORG, table_VORG};

use crate::table::cmap::{cmap_Entry};




use crate::table::glyf::{glyf_ComponentReference, glyf_Contour, glyf_Glyph, table_glyf};



use crate::table::hmtx::{__caryll_elementinterface_table_hmtx, table_hmtx};



use crate::table::otl::{__caryll_vectorinterface_otl_SubtableList, otl_ChainingRule, otl_Lookup, otl_Subtable, otl_SubtableList, otl_SubtablePtr, otl_chaining_canonical, otl_chaining_poly, otl_type_gpos_chaining, otl_type_gsub_chaining, table_OTL};




use crate::table::vmtx::{__caryll_elementinterface_table_vmtx, table_vmtx};


use crate::vf::region::{vq_AxisSpan};
use crate::vf::vq::{VQ, VQ_DELTA, VQ_STILL, __caryll_vectorinterface_VQ, vq_Segment};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphHash {
    pub hash: [u8; 20],
}
unsafe extern "C" fn hashVQS(buf: *mut caryll_Buffer, s: vq_Segment) {
    bufwrite8(buf, s.type_0 as u8);
    match s.type_0 {
        VQ_STILL => {
            bufwrite32b(
                buf,
                otfcc_to_fixed(s.val.still as ::core::ffi::c_double) as u32,
            );
        }
        VQ_DELTA => {
            bufwrite32b(
                buf,
                otfcc_to_fixed(s.val.delta.quantity as ::core::ffi::c_double) as u32,
            );
            bufwrite32b(buf, (*s.val.delta.region).dimensions as u32);
            for j in 0..(*s.val.delta.region).dimensions as usize {
                let span: *const vq_AxisSpan =
                    (&raw const (*s.val.delta.region).spans as *const vq_AxisSpan)
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
unsafe extern "C" fn hashVQ(buf: *mut caryll_Buffer, x: VQ) {
    bufwrite32b(
        buf,
        otfcc_to_fixed(x.kernel as ::core::ffi::c_double) as u32,
    );
    bufwrite32b(buf, x.shift.length as u32);
    for j in 0..x.shift.length {
        hashVQS(buf, *x.shift.items.offset(j as isize));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nameGlyphByHash(
    mut g: *mut glyf_Glyph,
    mut glyf: *mut table_glyf,
) -> GlyphHash {
    let buf: *mut caryll_Buffer = bufnew();
    bufwrite8(buf, 'H' as i32 as u8);
    hashVQ(buf, (*g).advanceWidth);
    bufwrite8(buf, 'h' as i32 as u8);
    hashVQ(buf, (*g).horizontalOrigin);
    bufwrite8(buf, 'V' as i32 as u8);
    hashVQ(buf, (*g).advanceHeight);
    bufwrite8(buf, 'v' as i32 as u8);
    hashVQ(buf, (*g).verticalOrigin);
    bufwrite8(buf, 'C' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).contours.length {
        bufwrite8(buf, '(' as i32 as u8);
        let c: *mut glyf_Contour = (*g).contours.items.offset(j as isize) as *mut glyf_Contour;
        for k in 0..(*c).length {
            let point = (*c).items.offset(k as isize);
            hashVQ(buf, (*point).x);
            hashVQ(buf, (*point).y);
            bufwrite8(buf, ((*point).onCurve != 0) as u8);
        }
        bufwrite8(buf, ')' as i32 as u8);
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'R' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).references.length {
        let r: *mut glyf_ComponentReference =
            (*g).references.items.offset(j as isize) as *mut glyf_ComponentReference;
        let mut h: GlyphHash = nameGlyphByHash(
            *(*glyf).items.offset((*r).glyph.index as isize) as *mut glyf_Glyph,
            glyf,
        );
        bufwrite_bytes(
            buf,
            SHA1_BLOCK_SIZE as usize,
            &raw mut h.hash as *mut u8,
        );
        hashVQ(buf, (*r).x);
        hashVQ(buf, (*r).y);
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
    for j in 0..(*g).stemH.length {
        let stem = (*g).stemH.items.offset(j as isize);
        bufwrite32b(buf, otfcc_to_fixed((*stem).position as ::core::ffi::c_double) as u32);
        bufwrite32b(buf, otfcc_to_fixed((*stem).width as ::core::ffi::c_double) as u32);
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 's' as i32 as u8);
    bufwrite8(buf, 'V' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).stemV.length {
        let stem = (*g).stemV.items.offset(j as isize);
        bufwrite32b(buf, otfcc_to_fixed((*stem).position as ::core::ffi::c_double) as u32);
        bufwrite32b(buf, otfcc_to_fixed((*stem).width as ::core::ffi::c_double) as u32);
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'm' as i32 as u8);
    bufwrite8(buf, 'H' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).hintMasks.length {
        let mask = (*g).hintMasks.items.offset(j as isize);
        bufwrite16b(buf, (*mask).contoursBefore);
        bufwrite16b(buf, (*mask).pointsBefore);
        for k in 0..(*g).stemH.length {
            bufwrite8(buf, (*mask).maskH[k] as u8);
        }
        for k in 0..(*g).stemV.length {
            bufwrite8(buf, (*mask).maskV[k] as u8);
        }
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'm' as i32 as u8);
    bufwrite8(buf, 'C' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).contourMasks.length {
        let mask = (*g).contourMasks.items.offset(j as isize);
        bufwrite16b(buf, (*mask).contoursBefore);
        bufwrite16b(buf, (*mask).pointsBefore);
        for k in 0..(*g).stemH.length {
            bufwrite8(buf, (*mask).maskH[k] as u8);
        }
        for k in 0..(*g).stemV.length {
            bufwrite8(buf, (*mask).maskV[k] as u8);
        }
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'I' as i32 as u8);
    bufwrite32b(buf, (*g).instructionsLength as u32);
    bufwrite_bytes(buf, (*g).instructionsLength as usize, (*g).instructions);
    let mut ctx: SHA1_CTX = SHA1_CTX {
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
unsafe extern "C" fn createGlyphOrder(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) -> *mut otfcc_GlyphOrder {
    let mut glyph_order: *mut otfcc_GlyphOrder =
        (
            otfcc_pkgGlyphOrder
                .create
                .expect("non-null function pointer"))();
    let mut numGlyphs: glyphid_t = (*(*font).glyf).length as glyphid_t;
    let mut prefix: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*options).glyph_name_prefix.is_null() {
        prefix = sdsnew((*options).glyph_name_prefix);
    } else {
        prefix = sdsempty();
    }
    for j in 0..numGlyphs {
        let mut g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut glyf_Glyph;
        if (*options).name_glyphs_by_hash {
            let h: GlyphHash = nameGlyphByHash(g, (*font).glyf);
            let mut gname: sds = sdsempty();
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
            if otfcc_pkgGlyphOrder
                .lookupName
                .expect("non-null function pointer")(glyph_order, gname)
            {
                let mut n: glyphid_t = 2 as glyphid_t;
                let mut stillIn: bool = false;
                loop {
                    if stillIn {
                        n = (n as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
                    }
                    let mut newname: sds = crate::sdsbuild!(sdsempty(), gname, b"-", prefix, n as ::core::ffi::c_int);
                    stillIn = otfcc_pkgGlyphOrder
                        .lookupName
                        .expect("non-null function pointer")(
                        glyph_order, newname
                    );
                    sdsfree(newname);
                    if !stillIn {
                        break;
                    }
                }
                let mut newname_0: sds = crate::sdsbuild!(sdsempty(), gname, b"-", prefix, n as ::core::ffi::c_int);
                let mut sharedName: sds = otfcc_pkgGlyphOrder
                    .setByGID
                    .expect("non-null function pointer")(
                    glyph_order, j, newname_0
                );
                if !(*g).name.is_null() {
                    sdsfree((*g).name);
                }
                (*g).name = sdsdup(sharedName);
                sdsfree(gname);
            } else {
                let mut sharedName_0: sds = otfcc_pkgGlyphOrder
                    .setByGID
                    .expect("non-null function pointer")(
                    glyph_order, j, gname
                );
                if !(*g).name.is_null() {
                    sdsfree((*g).name);
                }
                (*g).name = sdsdup(sharedName_0);
            }
        } else if !((*options).ignore_glyph_order || (*options).name_glyphs_by_gid) {
            if !(*g).name.is_null() {
                let mut gname_0: sds = crate::sdsbuild!(sdsempty(), prefix, (*g).name);
                let sharedName_1: sds = otfcc_pkgGlyphOrder
                    .setByGID
                    .expect("non-null function pointer")(
                    glyph_order, j, gname_0
                );
                if !(*g).name.is_null() {
                    sdsfree((*g).name);
                }
                (*g).name = sdsdup(sharedName_1);
            }
        }
    }
    if !(*font).post.is_null()
        && !(*(*font).post).post_name_map.is_null()
        && !(*options).ignore_glyph_order
        && !(*options).name_glyphs_by_gid
    {
        let mut s: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
        let mut tmp: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
        s = (*(*(*font).post).post_name_map).byGID;
        tmp = (if !(*(*(*font).post).post_name_map).byGID.is_null() {
            (*(*(*(*font).post).post_name_map).byGID).hhID.next
        } else {
            NULL
        }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
        while !s.is_null() {
            let mut gname_1: sds = crate::sdsbuild!(sdsempty(), prefix, (*s).name);
            otfcc_pkgGlyphOrder
                .setByGID
                .expect("non-null function pointer")(glyph_order, (*s).gid, gname_1);
            s = tmp;
            tmp = (if !tmp.is_null() {
                (*tmp).hhID.next
            } else {
                NULL
            }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
        }
    }
    if !(*font).cmap.is_null() && !(*options).name_glyphs_by_gid {
        let mut aglfn: *mut otfcc_GlyphOrder =
            (
                otfcc_pkgGlyphOrder
                    .create
                    .expect("non-null function pointer"))();
        aglfn_setupNames(aglfn);
        let mut s_0: *mut cmap_Entry = ::core::ptr::null_mut::<cmap_Entry>();
        s_0 = (*(*font).cmap).unicodes;
        while !s_0.is_null() {
            if (*s_0).glyph.index as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                let mut name: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if (*s_0).unicode > 0 as ::core::ffi::c_int
                    && (*s_0).unicode < 0xffff as ::core::ffi::c_int
                {
                    otfcc_pkgGlyphOrder
                        .nameAField_Shared
                        .expect("non-null function pointer")(
                        aglfn,
                        (*s_0).unicode as glyphid_t,
                        &raw mut name,
                    );
                }
                if name.is_null() {
                    name = crate::sdsbuild!(sdsempty(), prefix, b"uni", Hex4Upper(((*s_0).unicode) as u32));
                } else {
                    name = crate::sdsbuild!(sdsempty(), prefix, name);
                }
                otfcc_pkgGlyphOrder
                    .setByGID
                    .expect("non-null function pointer")(
                    glyph_order, (*s_0).glyph.index, name
                );
            }
            s_0 = (*s_0).hh.next as *mut cmap_Entry;
        }
        otfcc_pkgGlyphOrder.free.expect("non-null function pointer")(aglfn);
    }
    for j_1 in 0..numGlyphs {
        let mut name_0: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if j_1 > 1 {
            name_0 = crate::sdsbuild!(sdsempty(), prefix, b"glyph", j_1 as ::core::ffi::c_int);
        } else if j_1 == 1 {
            if !(*(*(*font).glyf)
                .items
                .offset(1 as ::core::ffi::c_int as isize))
            .is_null()
                && (**(*(*font).glyf)
                    .items
                    .offset(1 as ::core::ffi::c_int as isize))
                .contours
                .length
                    == 0
                && (**(*(*font).glyf)
                    .items
                    .offset(1 as ::core::ffi::c_int as isize))
                .references
                .length
                    == 0
            {
                name_0 = crate::sdsbuild!(sdsempty(), prefix, b".null");
            } else {
                name_0 = crate::sdsbuild!(sdsempty(), prefix, b"glyph", j_1 as ::core::ffi::c_int);
            }
        } else {
            name_0 = crate::sdsbuild!(sdsempty(), prefix, b".notdef");
        }
        otfcc_pkgGlyphOrder
            .setByGID
            .expect("non-null function pointer")(glyph_order, j_1, name_0);
    }
    sdsfree(prefix);
    return glyph_order;
}
unsafe extern "C" fn nameGlyphs(mut font: *mut otfcc_Font, mut gord: *mut otfcc_GlyphOrder) {
    if gord.is_null() {
        return;
    }
    for j in 0..(*(*font).glyf).length as glyphid_t {
        let g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut glyf_Glyph;
        let mut glyphName: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
        otfcc_pkgGlyphOrder
            .nameAField_Shared
            .expect("non-null function pointer")(gord, j, &raw mut glyphName);
        if !(*g).name.is_null() {
            sdsfree((*g).name);
        }
        (*g).name = sdsdup(glyphName);
    }
}
unsafe extern "C" fn unconsolidate_chaining(
    _font: *mut otfcc_Font,
    lookup: *mut otl_Lookup,
    _table: *mut table_OTL,
) {
    // The original C (c/lib/otf-reader/unconsolidate.c) computes a
    // `totalRules` count in a first pass over the subtables and never uses
    // it afterward (no capacity-reservation call, no other reference) --
    // genuinely dead code upstream, not a c2rust artifact. Confirmed by
    // inspection: the loop body only reads subtable fields into a local
    // accumulator with no other side effects. Omitted here.
    let mut newsts: otl_SubtableList = otl_SubtableList {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<otl_SubtablePtr>(),
    };
    otl_iSubtableList.init.expect("non-null function pointer")(&raw mut newsts);
    for j in 0..(*lookup).subtables.length as tableid_t {
        let slot = (*lookup).subtables.items.offset(j as isize);
        if (*slot).is_null() {
            continue;
        }
        let sub: otl_SubtablePtr = *slot;
        if (*sub).chaining.type_0 == otl_chaining_poly {
            let rules_count = (*sub).chaining.c2rust_unnamed.c2rust_unnamed.rulesCount;
            for k in 0..rules_count as ::core::ffi::c_int {
                let rule_slot = (*sub)
                    .chaining
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(k as isize);
                let st: *mut otl_Subtable = __caryll_allocate_clean(
                    ::core::mem::size_of::<otl_Subtable>() as usize,
                    278 as ::core::ffi::c_ulong,
                ) as *mut otl_Subtable;
                (*st).chaining.type_0 = otl_chaining_canonical;
                // Transfer ownership of the rule out of *rule_slot.
                (*st).chaining.c2rust_unnamed.rule = **rule_slot;
                free(*rule_slot as *mut ::core::ffi::c_void);
                *rule_slot = ::core::ptr::null_mut::<otl_ChainingRule>();
                otl_iSubtableList.push.expect("non-null function pointer")(
                    &raw mut newsts,
                    st as otl_SubtablePtr,
                );
            }
            free((*sub).chaining.c2rust_unnamed.c2rust_unnamed.rules as *mut ::core::ffi::c_void);
            (*sub).chaining.c2rust_unnamed.c2rust_unnamed.rules =
                ::core::ptr::null_mut::<*mut otl_ChainingRule>();
            free(sub as *mut ::core::ffi::c_void);
            *slot = ::core::ptr::null_mut::<otl_Subtable>();
        } else if (*sub).chaining.type_0 == otl_chaining_canonical {
            let st_0: *mut otl_Subtable = __caryll_allocate_clean(
                ::core::mem::size_of::<otl_Subtable>() as usize,
                289 as ::core::ffi::c_ulong,
            ) as *mut otl_Subtable;
            (*st_0).chaining.type_0 = otl_chaining_canonical;
            (*st_0).chaining.c2rust_unnamed.rule = (*sub).chaining.c2rust_unnamed.rule;
            otl_iSubtableList.push.expect("non-null function pointer")(
                &raw mut newsts,
                st_0 as otl_SubtablePtr,
            );
            *slot = ::core::ptr::null_mut::<otl_Subtable>();
        }
    }
    otl_iSubtableList
        .disposeDependent
        .expect("non-null function pointer")(&raw mut (*lookup).subtables, lookup);
    (*lookup).subtables = newsts;
}
unsafe extern "C" fn expandChain(font: *mut otfcc_Font, lookup: *mut otl_Lookup, table: *mut table_OTL) {
    match (*lookup).type_0 {
        otl_type_gsub_chaining | otl_type_gpos_chaining => {
            unconsolidate_chaining(font, lookup, table);
        }
        _ => {}
    };
}
unsafe extern "C" fn expandChainingLookups(font: *mut otfcc_Font) {
    if !(*font).GSUB.is_null() {
        for j in 0..(*(*font).GSUB).lookups.length {
            let lookup: *mut otl_Lookup = *(*(*font).GSUB).lookups.items.offset(j as isize) as *mut otl_Lookup;
            expandChain(font, lookup, (*font).GSUB);
        }
    }
    if !(*font).GPOS.is_null() {
        for j in 0..(*(*font).GPOS).lookups.length {
            let lookup: *mut otl_Lookup = *(*(*font).GPOS).lookups.items.offset(j as isize) as *mut otl_Lookup;
            expandChain(font, lookup, (*font).GPOS);
        }
    }
}
unsafe extern "C" fn mergeHmtx(font: *mut otfcc_Font) {
    if !(!(*font).hhea.is_null() && !(*font).hmtx.is_null() && !(*font).glyf.is_null()) {
        return;
    }
    let count_a: u32 = (*(*font).hhea).numberOfMetrics as u32;
    for j in 0..(*(*font).glyf).length as glyphid_t {
        let g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut glyf_Glyph;
        let adw: pos_t = (*(*(*font).hmtx).metrics.offset(
            (if (j as u32) < count_a {
                j as u32
            } else {
                count_a.wrapping_sub(1 as u32)
            }) as isize,
        ))
        .advanceWidth as pos_t;
        let lsb: pos_t = if (j as u32) < count_a {
            (*(*(*font).hmtx).metrics.offset(j as isize)).lsb
        } else {
            *(*(*font).hmtx)
                .leftSideBearing
                .offset((j as u32).wrapping_sub(count_a) as isize)
        };
        iVQ.inplacePlus.expect("non-null function pointer")(
            &raw mut (*g).advanceWidth,
            iVQ.createStill.expect("non-null function pointer")(adw) as VQ,
        );
        iVQ.inplacePlus.expect("non-null function pointer")(
            &raw mut (*g).horizontalOrigin,
            iVQ.createStill.expect("non-null function pointer")(-lsb + (*g).stat.xMin) as VQ,
        );
    }
    table_iHmtx.free.expect("non-null function pointer")((*font).hmtx);
    (*font).hmtx = ::core::ptr::null_mut::<table_hmtx>();
}
unsafe extern "C" fn mergeVmtx(font: *mut otfcc_Font) {
    if !(!(*font).vhea.is_null() && !(*font).vmtx.is_null() && !(*font).glyf.is_null()) {
        return;
    }
    let count_a: u32 = (*(*font).vhea).numOfLongVerMetrics as u32;
    let mut vorgs: *mut pos_t = ::core::ptr::null_mut::<pos_t>();
    if !(*font).VORG.is_null() {
        vorgs = __caryll_allocate_clean(
            (::core::mem::size_of::<pos_t>() as usize).wrapping_mul((*(*font).glyf).length),
            351 as ::core::ffi::c_ulong,
        ) as *mut pos_t;
        for j in 0..(*(*font).glyf).length as glyphid_t {
            *vorgs.offset(j as isize) = (*(*font).VORG).defaultVerticalOrigin;
        }
        for j_0 in 0..(*(*font).VORG).numVertOriginYMetrics as glyphid_t {
            if ((*(*(*font).VORG).entries.offset(j_0 as isize)).gid as usize)
                < (*(*font).glyf).length
            {
                *vorgs.offset((*(*(*font).VORG).entries.offset(j_0 as isize)).gid as isize) =
                    (*(*(*font).VORG).entries.offset(j_0 as isize)).verticalOrigin as pos_t;
            }
        }
        table_iVORG.free.expect("non-null function pointer")((*font).VORG);
        (*font).VORG = ::core::ptr::null_mut::<table_VORG>();
    }
    for j_1 in 0..(*(*font).glyf).length as glyphid_t {
        let g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j_1 as isize) as *mut glyf_Glyph;
        let adh: pos_t = (*(*(*font).vmtx).metrics.offset(
            (if (j_1 as u32) < count_a {
                j_1 as u32
            } else {
                count_a.wrapping_sub(1 as u32)
            }) as isize,
        ))
        .advanceHeight as pos_t;
        let tsb: pos_t = if (j_1 as u32) < count_a {
            (*(*(*font).vmtx).metrics.offset(j_1 as isize)).tsb
        } else {
            *(*(*font).vmtx)
                .topSideBearing
                .offset((j_1 as u32).wrapping_sub(count_a) as isize)
        };
        iVQ.inplacePlus.expect("non-null function pointer")(
            &raw mut (*g).advanceHeight,
            iVQ.createStill.expect("non-null function pointer")(adh) as VQ,
        );
        iVQ.inplacePlus.expect("non-null function pointer")(
            &raw mut (*g).verticalOrigin,
            iVQ.createStill.expect("non-null function pointer")(if !vorgs.is_null() {
                *vorgs.offset(j_1 as isize)
            } else {
                tsb + (*g).stat.yMax
            }) as VQ,
        );
    }
    if !vorgs.is_null() {
        free(vorgs as *mut ::core::ffi::c_void);
        vorgs = ::core::ptr::null_mut::<pos_t>();
    }
    table_iVmtx.free.expect("non-null function pointer")((*font).vmtx);
    (*font).vmtx = ::core::ptr::null_mut::<table_vmtx>();
}
unsafe extern "C" fn mergeLTSH(font: *mut otfcc_Font) {
    if !(*font).glyf.is_null() && !(*font).LTSH.is_null() {
        let n = ((*(*font).glyf).length as glyphid_t).min((*(*font).LTSH).numGlyphs);
        for j in 0..n {
            (**(*(*font).glyf).items.offset(j as isize)).yPel =
                *(*(*font).LTSH).yPels.offset(j as isize);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_unconsolidateFont(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    mergeHmtx(font);
    mergeVmtx(font);
    mergeLTSH(font);
    expandChainingLookups(font);
    if !(*font).glyf.is_null() {
        let mut gord: *mut otfcc_GlyphOrder = createGlyphOrder(font, options);
        nameGlyphs(font, gord);
        otfcc_pkgGlyphOrder.free.expect("non-null function pointer")(gord);
    }
}
pub const SHA1_BLOCK_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
