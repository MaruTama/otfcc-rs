#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};




use crate::support::alloc::{__caryll_allocate_clean};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, Pos, TableId};
use crate::vendor::sds::{Hex2Upper, Hex4Upper, SdsRaw};
use crate::font::caryll_font::{Font};
use crate::support::{NULL};
use crate::support::glyph_order::{GlyphOrder, GlyphOrderEntry};
use crate::support::sha1::{BYTE, Sha1Ctx};









use crate::table::VORG::VorgTable;

use crate::table::cmap::{CmapEntry};




use crate::table::glyf::{ComponentReference, Contour, Glyph, GlyfTable};



use crate::table::hmtx::HmtxTable;



use crate::table::otl::{ChainingRule, Lookup, Subtable, SubtableList, SubtablePtr, ChainingType, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GSUB_CHAINING, OtlTable};




use crate::table::vmtx::VmtxTable;


use crate::vf::region::{VqAxisSpan};
use crate::vf::vq::{VQ, VQSegType, VqSegment};
use crate::support::aglfn::{aglfn_setup_names};
use crate::support::buffer::{buffree, buflen, bufnew, bufwrite16b, bufwrite32b, bufwrite8, bufwrite_bytes};
use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::support::primitives::{otfcc_to_f2dot14, otfcc_to_fixed};
use crate::support::sha1::{sha1_final, sha1_init, sha1_update};
use crate::table::VORG::{TABLE_I_VORG};
use crate::table::hmtx::{TABLE_I_HMTX};
use crate::table::otl::{OTL_I_SUBTABLE_LIST};
use crate::table::vmtx::{TABLE_I_VMTX};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree, sdsnew};
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
    bufwrite32b(buf, x.shift.length as u32);
    for j in 0..x.shift.length {
        hash_vqs(buf, *x.shift.items.offset(j as isize));
    }
}
pub unsafe extern "C" fn name_glyph_by_hash(
    mut g: *mut Glyph,
    mut glyf: *mut GlyfTable,
) -> GlyphHash {
    let buf: *mut Buffer = bufnew();
    bufwrite8(buf, 'H' as i32 as u8);
    hash_vq(buf, (*g).advanceWidth);
    bufwrite8(buf, 'h' as i32 as u8);
    hash_vq(buf, (*g).horizontalOrigin);
    bufwrite8(buf, 'V' as i32 as u8);
    hash_vq(buf, (*g).advanceHeight);
    bufwrite8(buf, 'v' as i32 as u8);
    hash_vq(buf, (*g).verticalOrigin);
    bufwrite8(buf, 'C' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).contours.length {
        bufwrite8(buf, '(' as i32 as u8);
        let c: *mut Contour = (*g).contours.items.offset(j as isize) as *mut Contour;
        for k in 0..(*c).length {
            let point = (*c).items.offset(k as isize);
            hash_vq(buf, (*point).x);
            hash_vq(buf, (*point).y);
            bufwrite8(buf, ((*point).onCurve != 0) as u8);
        }
        bufwrite8(buf, ')' as i32 as u8);
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'R' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).references.length {
        let r: *mut ComponentReference =
            (*g).references.items.offset(j as isize) as *mut ComponentReference;
        let mut h: GlyphHash = name_glyph_by_hash(
            *(*glyf).items.offset((*r).glyph.index as isize) as *mut Glyph,
            glyf,
        );
        bufwrite_bytes(
            buf,
            SHA1_BLOCK_SIZE as usize,
            &raw mut h.hash as *mut u8,
        );
        hash_vq(buf, (*r).x);
        hash_vq(buf, (*r).y);
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
    let mut numGlyphs: GlyphId = (*(*font).glyf).length as GlyphId;
    let mut prefix: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*options).glyph_name_prefix.is_null() {
        prefix = sdsnew((*options).glyph_name_prefix);
    } else {
        prefix = sdsempty();
    }
    for j in 0..numGlyphs {
        let mut g: *mut Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut Glyph;
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
                .lookupName
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
                        .lookupName
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
                    .setByGID
                    .expect("non-null function pointer")(
                    glyph_order, j, newname_0
                );
                if !(*g).name.is_null() {
                    sdsfree((*g).name);
                }
                (*g).name = sdsdup(shared_name);
                sdsfree(gname);
            } else {
                let mut shared_name_0: SdsRaw = OTFCC_PKG_GLYPH_ORDER
                    .setByGID
                    .expect("non-null function pointer")(
                    glyph_order, j, gname
                );
                if !(*g).name.is_null() {
                    sdsfree((*g).name);
                }
                (*g).name = sdsdup(shared_name_0);
            }
        } else if !((*options).ignore_glyph_order || (*options).name_glyphs_by_gid) {
            if !(*g).name.is_null() {
                let mut gname_0: SdsRaw = crate::sdsbuild!(sdsempty(), prefix, (*g).name);
                let shared_name_1: SdsRaw = OTFCC_PKG_GLYPH_ORDER
                    .setByGID
                    .expect("non-null function pointer")(
                    glyph_order, j, gname_0
                );
                if !(*g).name.is_null() {
                    sdsfree((*g).name);
                }
                (*g).name = sdsdup(shared_name_1);
            }
        }
    }
    if !(*font).post.is_null()
        && !(*(*font).post).post_name_map.is_null()
        && !(*options).ignore_glyph_order
        && !(*options).name_glyphs_by_gid
    {
        let mut s: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
        let mut tmp: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
        s = (*(*(*font).post).post_name_map).byGID;
        tmp = (if !(*(*(*font).post).post_name_map).byGID.is_null() {
            (*(*(*(*font).post).post_name_map).byGID).hhID.next
        } else {
            NULL
        }) as *mut GlyphOrderEntry as *mut GlyphOrderEntry;
        while !s.is_null() {
            let mut gname_1: SdsRaw = crate::sdsbuild!(sdsempty(), prefix, (*s).name);
            OTFCC_PKG_GLYPH_ORDER
                .setByGID
                .expect("non-null function pointer")(glyph_order, (*s).gid, gname_1);
            s = tmp;
            tmp = (if !tmp.is_null() {
                (*tmp).hhID.next
            } else {
                NULL
            }) as *mut GlyphOrderEntry as *mut GlyphOrderEntry;
        }
    }
    if !(*font).cmap.is_null() && !(*options).name_glyphs_by_gid {
        let mut aglfn: *mut GlyphOrder =
            (
                OTFCC_PKG_GLYPH_ORDER
                    .create
                    .expect("non-null function pointer"))();
        aglfn_setup_names(aglfn);
        let mut s_0: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
        s_0 = (*(*font).cmap).unicodes;
        while !s_0.is_null() {
            if (*s_0).glyph.index as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                let mut name: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if (*s_0).unicode > 0 as ::core::ffi::c_int
                    && (*s_0).unicode < 0xffff as ::core::ffi::c_int
                {
                    OTFCC_PKG_GLYPH_ORDER
                        .nameAField_Shared
                        .expect("non-null function pointer")(
                        aglfn,
                        (*s_0).unicode as GlyphId,
                        &raw mut name,
                    );
                }
                if name.is_null() {
                    name = crate::sdsbuild!(sdsempty(), prefix, b"uni", Hex4Upper(((*s_0).unicode) as u32));
                } else {
                    name = crate::sdsbuild!(sdsempty(), prefix, name);
                }
                OTFCC_PKG_GLYPH_ORDER
                    .setByGID
                    .expect("non-null function pointer")(
                    glyph_order, (*s_0).glyph.index, name
                );
            }
            s_0 = (*s_0).hh.next as *mut CmapEntry;
        }
        OTFCC_PKG_GLYPH_ORDER.free.expect("non-null function pointer")(aglfn);
    }
    for j_1 in 0..numGlyphs {
        let mut name_0: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
        OTFCC_PKG_GLYPH_ORDER
            .setByGID
            .expect("non-null function pointer")(glyph_order, j_1, name_0);
    }
    sdsfree(prefix);
    return glyph_order;
}
unsafe extern "C" fn name_glyphs(mut font: *mut Font, mut gord: *mut GlyphOrder) {
    if gord.is_null() {
        return;
    }
    for j in 0..(*(*font).glyf).length as GlyphId {
        let g: *mut Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut Glyph;
        let mut glyph_name: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
        OTFCC_PKG_GLYPH_ORDER
            .nameAField_Shared
            .expect("non-null function pointer")(gord, j, &raw mut glyph_name);
        if !(*g).name.is_null() {
            sdsfree((*g).name);
        }
        (*g).name = sdsdup(glyph_name);
    }
}
unsafe extern "C" fn unconsolidate_chaining(
    _font: *mut Font,
    lookup: *mut Lookup,
    _table: *mut OtlTable,
) {
    // The original C (c/lib/otf-reader/unconsolidate.c) computes a
    // `totalRules` count in a first pass over the subtables and never uses
    // it afterward (no capacity-reservation call, no other reference) --
    // genuinely dead code upstream, not a c2rust artifact. Confirmed by
    // inspection: the loop body only reads subtable fields into a local
    // accumulator with no other side effects. Omitted here.
    let mut newsts: SubtableList = SubtableList {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<SubtablePtr>(),
    };
    OTL_I_SUBTABLE_LIST.init.expect("non-null function pointer")(&raw mut newsts);
    for j in 0..(*lookup).subtables.length as TableId {
        let slot = (*lookup).subtables.items.offset(j as isize);
        if (*slot).is_null() {
            continue;
        }
        let sub: SubtablePtr = *slot;
        if (*sub).chaining.type_0 == ChainingType::Poly {
            let rules_count = (*sub).chaining.c2rust_unnamed.c2rust_unnamed.rulesCount;
            for k in 0..rules_count as ::core::ffi::c_int {
                let rule_slot = (*sub)
                    .chaining
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(k as isize);
                let st: *mut Subtable = __caryll_allocate_clean(
                    ::core::mem::size_of::<Subtable>() as usize,
                    278 as ::core::ffi::c_ulong,
                ) as *mut Subtable;
                (*st).chaining.type_0 = ChainingType::Canonical;
                // Transfer ownership of the rule out of *rule_slot.
                (*st).chaining.c2rust_unnamed.rule = **rule_slot;
                free(*rule_slot as *mut ::core::ffi::c_void);
                *rule_slot = ::core::ptr::null_mut::<ChainingRule>();
                OTL_I_SUBTABLE_LIST.push.expect("non-null function pointer")(
                    &raw mut newsts,
                    st as SubtablePtr,
                );
            }
            free((*sub).chaining.c2rust_unnamed.c2rust_unnamed.rules as *mut ::core::ffi::c_void);
            (*sub).chaining.c2rust_unnamed.c2rust_unnamed.rules =
                ::core::ptr::null_mut::<*mut ChainingRule>();
            free(sub as *mut ::core::ffi::c_void);
            *slot = ::core::ptr::null_mut::<Subtable>();
        } else if (*sub).chaining.type_0 == ChainingType::Canonical {
            let st_0: *mut Subtable = __caryll_allocate_clean(
                ::core::mem::size_of::<Subtable>() as usize,
                289 as ::core::ffi::c_ulong,
            ) as *mut Subtable;
            (*st_0).chaining.type_0 = ChainingType::Canonical;
            (*st_0).chaining.c2rust_unnamed.rule = (*sub).chaining.c2rust_unnamed.rule;
            OTL_I_SUBTABLE_LIST.push.expect("non-null function pointer")(
                &raw mut newsts,
                st_0 as SubtablePtr,
            );
            *slot = ::core::ptr::null_mut::<Subtable>();
        }
    }
    OTL_I_SUBTABLE_LIST
        .disposeDependent
        .expect("non-null function pointer")(&raw mut (*lookup).subtables, lookup);
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
    if !(*font).GSUB.is_null() {
        for j in 0..(*(*font).GSUB).lookups.length {
            let lookup: *mut Lookup = *(*(*font).GSUB).lookups.items.offset(j as isize) as *mut Lookup;
            expand_chain(font, lookup, (*font).GSUB);
        }
    }
    if !(*font).GPOS.is_null() {
        for j in 0..(*(*font).GPOS).lookups.length {
            let lookup: *mut Lookup = *(*(*font).GPOS).lookups.items.offset(j as isize) as *mut Lookup;
            expand_chain(font, lookup, (*font).GPOS);
        }
    }
}
unsafe extern "C" fn merge_hmtx(font: *mut Font) {
    if !(!(*font).hhea.is_null() && !(*font).hmtx.is_null() && !(*font).glyf.is_null()) {
        return;
    }
    let count_a: u32 = (*(*font).hhea).numberOfMetrics as u32;
    for j in 0..(*(*font).glyf).length as GlyphId {
        let g: *mut Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut Glyph;
        let adw: Pos = (*(*(*font).hmtx).metrics.offset(
            (if (j as u32) < count_a {
                j as u32
            } else {
                count_a.wrapping_sub(1 as u32)
            }) as isize,
        ))
        .advanceWidth as Pos;
        let lsb: Pos = if (j as u32) < count_a {
            (*(*(*font).hmtx).metrics.offset(j as isize)).lsb
        } else {
            *(*(*font).hmtx)
                .leftSideBearing
                .offset((j as u32).wrapping_sub(count_a) as isize)
        };
        I_VQ.inplacePlus.expect("non-null function pointer")(
            &raw mut (*g).advanceWidth,
            I_VQ.createStill.expect("non-null function pointer")(adw) as VQ,
        );
        I_VQ.inplacePlus.expect("non-null function pointer")(
            &raw mut (*g).horizontalOrigin,
            I_VQ.createStill.expect("non-null function pointer")(-lsb + (*g).stat.xMin) as VQ,
        );
    }
    TABLE_I_HMTX.free.expect("non-null function pointer")((*font).hmtx);
    (*font).hmtx = ::core::ptr::null_mut::<HmtxTable>();
}
unsafe extern "C" fn merge_vmtx(font: *mut Font) {
    if !(!(*font).vhea.is_null() && !(*font).vmtx.is_null() && !(*font).glyf.is_null()) {
        return;
    }
    let count_a: u32 = (*(*font).vhea).numOfLongVerMetrics as u32;
    let mut vorgs: *mut Pos = ::core::ptr::null_mut::<Pos>();
    if !(*font).VORG.is_null() {
        vorgs = __caryll_allocate_clean(
            (::core::mem::size_of::<Pos>() as usize).wrapping_mul((*(*font).glyf).length),
            351 as ::core::ffi::c_ulong,
        ) as *mut Pos;
        for j in 0..(*(*font).glyf).length as GlyphId {
            *vorgs.offset(j as isize) = (*(*font).VORG).defaultVerticalOrigin;
        }
        for j_0 in 0..(*(*font).VORG).numVertOriginYMetrics as GlyphId {
            if ((*(*(*font).VORG).entries.offset(j_0 as isize)).gid as usize)
                < (*(*font).glyf).length
            {
                *vorgs.offset((*(*(*font).VORG).entries.offset(j_0 as isize)).gid as isize) =
                    (*(*(*font).VORG).entries.offset(j_0 as isize)).verticalOrigin as Pos;
            }
        }
        TABLE_I_VORG.free.expect("non-null function pointer")((*font).VORG);
        (*font).VORG = ::core::ptr::null_mut::<VorgTable>();
    }
    for j_1 in 0..(*(*font).glyf).length as GlyphId {
        let g: *mut Glyph = *(*(*font).glyf).items.offset(j_1 as isize) as *mut Glyph;
        let adh: Pos = (*(*(*font).vmtx).metrics.offset(
            (if (j_1 as u32) < count_a {
                j_1 as u32
            } else {
                count_a.wrapping_sub(1 as u32)
            }) as isize,
        ))
        .advanceHeight as Pos;
        let tsb: Pos = if (j_1 as u32) < count_a {
            (*(*(*font).vmtx).metrics.offset(j_1 as isize)).tsb
        } else {
            *(*(*font).vmtx)
                .topSideBearing
                .offset((j_1 as u32).wrapping_sub(count_a) as isize)
        };
        I_VQ.inplacePlus.expect("non-null function pointer")(
            &raw mut (*g).advanceHeight,
            I_VQ.createStill.expect("non-null function pointer")(adh) as VQ,
        );
        I_VQ.inplacePlus.expect("non-null function pointer")(
            &raw mut (*g).verticalOrigin,
            I_VQ.createStill.expect("non-null function pointer")(if !vorgs.is_null() {
                *vorgs.offset(j_1 as isize)
            } else {
                tsb + (*g).stat.yMax
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
    if !(*font).glyf.is_null() && !(*font).LTSH.is_null() {
        let n = ((*(*font).glyf).length as GlyphId).min((*(*font).LTSH).numGlyphs);
        for j in 0..n {
            (**(*(*font).glyf).items.offset(j as isize)).yPel =
                *(*(*font).LTSH).yPels.offset(j as isize);
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
