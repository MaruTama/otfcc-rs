#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod otl;

use libc::{free, strcmp};
unsafe extern "C" {
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    static tsi_iEntry: __caryll_elementinterface_tsi_Entry;
    static table_iTSI: __caryll_vectorinterface_table_TSI;
    static glyf_iContourList: __caryll_vectorinterface_glyf_ContourList;
    static glyf_iStemDefList: __caryll_vectorinterface_glyf_StemDefList;
    static glyf_iMaskList: __caryll_vectorinterface_glyf_MaskList;
    static glyf_iComponentReference: __caryll_elementinterface_glyf_ComponentReference;
    static glyf_iReferenceList: __caryll_vectorinterface_glyf_ReferenceList;
    static iVQ: __caryll_vectorinterface_VQ;
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    static iSubtable_gsub_single: __caryll_vectorinterface_subtable_gsub_single;
    static iSubtable_gsub_multi: __caryll_vectorinterface_subtable_gsub_multi;
    static iSubtable_gsub_ligature: __caryll_vectorinterface_subtable_gsub_ligature;
    static iSubtable_chaining: __caryll_elementinterface_subtable_chaining;
    static iSubtable_gsub_reverse: __caryll_elementinterface_subtable_gsub_reverse;
    static iSubtable_gpos_single: __caryll_vectorinterface_subtable_gpos_single;
    static iSubtable_gpos_pair: __caryll_elementinterface_subtable_gpos_pair;
    static iSubtable_gpos_cursive: __caryll_vectorinterface_subtable_gpos_cursive;
    static iSubtable_gpos_markToSingle: __caryll_elementinterface_subtable_gpos_markToSingle;
    static iSubtable_gpos_markToLigature: __caryll_elementinterface_subtable_gpos_markToLigature;
    static otl_iLookupList: __caryll_vectorinterface_otl_LookupList;
    static otl_iLookupRefList: __caryll_vectorinterface_otl_LookupRefList;
    static otl_iFeatureList: __caryll_vectorinterface_otl_FeatureList;
    static otl_iFeatureRefList: __caryll_vectorinterface_otl_FeatureRefList;
    static colr_iLayer: __caryll_elementinterface_colr_Layer;
    static colr_iLayerList: __caryll_vectorinterface_colr_LayerList;
    static colr_iMapping: __caryll_elementinterface_colr_Mapping;
    static table_iCOLR: __caryll_vectorinterface_table_COLR;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn otfcc_newGlyf_glyph() -> *mut glyf_Glyph;
    fn fontop_consolidateClassDef(
        font: *mut otfcc_Font,
        cd: *mut otl_ClassDef,
        options: *const otfcc_Options,
    );
    fn consolidate_gsub_single(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gsub_multi(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gsub_alternative(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gsub_ligature(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gsub_reverse(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gpos_single(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gpos_pair(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gpos_cursive(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_chaining(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_mark_to_single(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_mark_to_ligature(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_GDEF(
        font: *mut otfcc_Font,
        gdef: *mut table_GDEF,
        options: *const otfcc_Options,
    );
}


use crate::support::handle::{HANDLE_STATE_EMPTY, HANDLE_STATE_INDEX, handle_consolidateTo, handle_fromIndex, otfcc_FDHandle, otfcc_GlyphHandle, otfcc_Handle, otfcc_Handle_copy, otfcc_Handle_dispose};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};

use crate::support::options::{otfcc_Options};
use crate::support::primitives::{glyphid_t, pos_t, shapeid_t, tableid_t};
use crate::vendor::sds::{Hex4Upper, sds};
use crate::font::caryll_font::{otfcc_Font};
use crate::support::{NULL};
use crate::support::glyph_order::{otfcc_GlyphOrder, otfcc_GlyphOrderPackage};

use crate::table::CFF::{table_CFF};
use crate::table::COLR::{__caryll_elementinterface_colr_Layer, __caryll_elementinterface_colr_Mapping, __caryll_vectorinterface_colr_LayerList, __caryll_vectorinterface_table_COLR, colr_Layer, colr_LayerList, colr_Mapping, table_COLR};

use crate::table::GDEF::{table_GDEF};





use crate::table::_TSI::{TSI_GLYPH, __caryll_elementinterface_tsi_Entry, __caryll_vectorinterface_table_TSI, table_TSI, tsi_Entry};
use crate::table::cmap::{cmap_Entry, cmap_UVS_Entry};




use crate::table::glyf::{REF_ANCHOR_ANCHOR, REF_ANCHOR_CONSOLIDATED, REF_ANCHOR_CONSOLIDATING_ANCHOR, REF_ANCHOR_CONSOLIDATING_XY, REF_XY, __caryll_elementinterface_glyf_ComponentReference, __caryll_vectorinterface_glyf_ContourList, __caryll_vectorinterface_glyf_MaskList, __caryll_vectorinterface_glyf_ReferenceList, __caryll_vectorinterface_glyf_StemDefList, glyf_ComponentReference, glyf_Glyph, glyf_GlyphPtr, glyf_Point, glyf_PostscriptHintMask, glyf_PostscriptStemDef, table_glyf};







use crate::table::otl::{__caryll_elementinterface_subtable_chaining, __caryll_elementinterface_subtable_gpos_markToLigature, __caryll_elementinterface_subtable_gpos_markToSingle, __caryll_elementinterface_subtable_gpos_pair, __caryll_elementinterface_subtable_gsub_reverse, __caryll_vectorinterface_otl_FeatureList, __caryll_vectorinterface_otl_FeatureRefList, __caryll_vectorinterface_otl_LookupList, __caryll_vectorinterface_otl_LookupRefList, __caryll_vectorinterface_subtable_gpos_cursive, __caryll_vectorinterface_subtable_gpos_single, __caryll_vectorinterface_subtable_gsub_ligature, __caryll_vectorinterface_subtable_gsub_multi, __caryll_vectorinterface_subtable_gsub_single, otl_Feature, otl_FeaturePtr, otl_FeatureRef, otl_LanguageSystem, otl_Lookup, otl_LookupPtr, otl_LookupRef, otl_LookupType, otl_Subtable, otl_type_gpos_chaining, otl_type_gpos_cursive, otl_type_gpos_markToBase, otl_type_gpos_markToLigature, otl_type_gpos_markToMark, otl_type_gpos_pair, otl_type_gpos_single, otl_type_gsub_alternate, otl_type_gsub_chaining, otl_type_gsub_ligature, otl_type_gsub_multiple, otl_type_gsub_reverse, otl_type_gsub_single, subtable_chaining, subtable_gpos_cursive, subtable_gpos_markToLigature, subtable_gpos_markToSingle, subtable_gpos_pair, subtable_gpos_single, subtable_gsub_ligature, subtable_gsub_multi, subtable_gsub_reverse, subtable_gsub_single, table_OTL};
use crate::table::otl::classdef::{otl_ClassDef};







use crate::vf::vq::{VQ, __caryll_vectorinterface_VQ};

pub type subtable_remover = Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>;
pub type otl_consolidation_function = Option<
    unsafe extern "C" fn(
        *mut otfcc_Font,
        *mut table_OTL,
        *mut otl_Subtable,
        *const otfcc_Options,
    ) -> bool,
>;
pub type fd_handle = otfcc_FDHandle;
unsafe extern "C" fn by_stem_pos(
    mut a: *const glyf_PostscriptStemDef,
    mut b: *const glyf_PostscriptStemDef,
) -> ::core::ffi::c_int {
    if (*a).position == (*b).position {
        return (*a).map as ::core::ffi::c_int - (*b).map as ::core::ffi::c_int;
    } else if (*a).position > (*b).position {
        return 1 as ::core::ffi::c_int;
    } else {
        return -(1 as ::core::ffi::c_int);
    };
}
unsafe extern "C" fn by_mask_pointindex(
    mut a: *const glyf_PostscriptHintMask,
    mut b: *const glyf_PostscriptHintMask,
) -> ::core::ffi::c_int {
    return if (*a).contoursBefore as ::core::ffi::c_int == (*b).contoursBefore as ::core::ffi::c_int
    {
        (*a).pointsBefore as ::core::ffi::c_int - (*b).pointsBefore as ::core::ffi::c_int
    } else {
        (*a).contoursBefore as ::core::ffi::c_int - (*b).contoursBefore as ::core::ffi::c_int
    };
}
unsafe extern "C" fn consolidateGlyphContours(
    mut g: *mut glyf_Glyph,
    mut options: *const otfcc_Options,
) {
    let mut nContoursConsolidated: shapeid_t = 0 as shapeid_t;
    let mut skip: shapeid_t = 0 as shapeid_t;
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as usize) < (*g).contours.length {
        if (*(*g).contours.items.offset(j as isize)).length != 0 {
            *(*g)
                .contours
                .items
                .offset((j as ::core::ffi::c_int - skip as ::core::ffi::c_int) as isize) =
                *(*g).contours.items.offset(j as isize);
            nContoursConsolidated = (nContoursConsolidated as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as shapeid_t;
        } else {
            glyf_iContourList
                .disposeItem
                .expect("non-null function pointer")(
                &raw mut (*g).contours, j as usize
            );
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Removed empty contour #",
                    j as ::core::ffi::c_int,
                    b" in glyph ",
                    (*g).name,
                    b".\n",
                ),
            );
            skip = (skip as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
        }
        j = j.wrapping_add(1);
    }
    (*g).contours.length = nContoursConsolidated as usize;
}
unsafe extern "C" fn consolidateGlyphReferences(
    mut g: *mut glyf_Glyph,
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    let mut nReferencesConsolidated: shapeid_t = 0 as shapeid_t;
    let mut skip: shapeid_t = 0 as shapeid_t;
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as usize) < (*g).references.length {
        if !otfcc_pkgGlyphOrder
            .consolidateHandle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (*(*g).references.items.offset(j as isize)).glyph,
        ) {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Ignored absent glyph component reference /",
                    (*(*g).references.items.offset(j as isize)).glyph.name,
                    b" within /",
                    (*g).name,
                    b".\n",
                ),
            );
            glyf_iReferenceList
                .disposeItem
                .expect("non-null function pointer")(
                &raw mut (*g).references, j as usize
            );
            skip = (skip as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
        } else {
            *(*g)
                .references
                .items
                .offset((j as ::core::ffi::c_int - skip as ::core::ffi::c_int) as isize) =
                *(*g).references.items.offset(j as isize);
            nReferencesConsolidated = (nReferencesConsolidated as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as shapeid_t;
        }
        j = j.wrapping_add(1);
    }
    (*g).references.length = nReferencesConsolidated as usize;
}
unsafe extern "C" fn consolidateGlyphHints(
    mut g: *mut glyf_Glyph,
    mut _options: *const otfcc_Options,
) {
    if (*g).stemH.length != 0 {
        let mut j: shapeid_t = 0 as shapeid_t;
        while (j as usize) < (*g).stemH.length {
            (*(*g).stemH.items.offset(j as isize)).map = j as u16;
            j = j.wrapping_add(1);
        }
        glyf_iStemDefList.sort.expect("non-null function pointer")(
            &raw mut (*g).stemH,
            Some(
                by_stem_pos
                    as unsafe extern "C" fn(
                        *const glyf_PostscriptStemDef,
                        *const glyf_PostscriptStemDef,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    if (*g).stemV.length != 0 {
        let mut j_0: shapeid_t = 0 as shapeid_t;
        while (j_0 as usize) < (*g).stemV.length {
            (*(*g).stemV.items.offset(j_0 as isize)).map = j_0 as u16;
            j_0 = j_0.wrapping_add(1);
        }
        glyf_iStemDefList.sort.expect("non-null function pointer")(
            &raw mut (*g).stemV,
            Some(
                by_stem_pos
                    as unsafe extern "C" fn(
                        *const glyf_PostscriptStemDef,
                        *const glyf_PostscriptStemDef,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    let mut hmap: *mut shapeid_t = ::core::ptr::null_mut::<shapeid_t>();
    hmap = __caryll_allocate_clean(
        (::core::mem::size_of::<shapeid_t>() as usize).wrapping_mul((*g).stemH.length),
        80 as ::core::ffi::c_ulong,
    ) as *mut shapeid_t;
    let mut vmap: *mut shapeid_t = ::core::ptr::null_mut::<shapeid_t>();
    vmap = __caryll_allocate_clean(
        (::core::mem::size_of::<shapeid_t>() as usize).wrapping_mul((*g).stemV.length),
        82 as ::core::ffi::c_ulong,
    ) as *mut shapeid_t;
    let mut j_1: shapeid_t = 0 as shapeid_t;
    while (j_1 as usize) < (*g).stemH.length {
        *hmap.offset((*(*g).stemH.items.offset(j_1 as isize)).map as isize) = j_1;
        j_1 = j_1.wrapping_add(1);
    }
    let mut j_2: shapeid_t = 0 as shapeid_t;
    while (j_2 as usize) < (*g).stemV.length {
        *vmap.offset((*(*g).stemV.items.offset(j_2 as isize)).map as isize) = j_2;
        j_2 = j_2.wrapping_add(1);
    }
    if (*g).hintMasks.length != 0 {
        glyf_iMaskList.sort.expect("non-null function pointer")(
            &raw mut (*g).hintMasks,
            Some(
                by_mask_pointindex
                    as unsafe extern "C" fn(
                        *const glyf_PostscriptHintMask,
                        *const glyf_PostscriptHintMask,
                    ) -> ::core::ffi::c_int,
            ),
        );
        let mut j_3: shapeid_t = 0 as shapeid_t;
        while (j_3 as usize) < (*g).hintMasks.length {
            let mut oldmask: glyf_PostscriptHintMask = *(*g).hintMasks.items.offset(j_3 as isize);
            let mut k: shapeid_t = 0 as shapeid_t;
            while (k as usize) < (*g).stemH.length {
                (*(*g).hintMasks.items.offset(j_3 as isize)).maskH[k as usize] =
                    oldmask.maskH[*hmap.offset(k as isize) as usize];
                k = k.wrapping_add(1);
            }
            let mut k_0: shapeid_t = 0 as shapeid_t;
            while (k_0 as usize) < (*g).stemV.length {
                (*(*g).hintMasks.items.offset(j_3 as isize)).maskV[k_0 as usize] =
                    oldmask.maskV[*vmap.offset(k_0 as isize) as usize];
                k_0 = k_0.wrapping_add(1);
            }
            j_3 = j_3.wrapping_add(1);
        }
    }
    if (*g).contourMasks.length != 0 {
        glyf_iMaskList.sort.expect("non-null function pointer")(
            &raw mut (*g).contourMasks,
            Some(
                by_mask_pointindex
                    as unsafe extern "C" fn(
                        *const glyf_PostscriptHintMask,
                        *const glyf_PostscriptHintMask,
                    ) -> ::core::ffi::c_int,
            ),
        );
        let mut j_4: shapeid_t = 0 as shapeid_t;
        while (j_4 as usize) < (*g).contourMasks.length {
            let mut oldmask_0: glyf_PostscriptHintMask =
                *(*g).contourMasks.items.offset(j_4 as isize);
            let mut k_1: shapeid_t = 0 as shapeid_t;
            while (k_1 as usize) < (*g).stemH.length {
                (*(*g).contourMasks.items.offset(j_4 as isize)).maskH[k_1 as usize] =
                    oldmask_0.maskH[*hmap.offset(k_1 as isize) as usize];
                k_1 = k_1.wrapping_add(1);
            }
            let mut k_2: shapeid_t = 0 as shapeid_t;
            while (k_2 as usize) < (*g).stemV.length {
                (*(*g).contourMasks.items.offset(j_4 as isize)).maskV[k_2 as usize] =
                    oldmask_0.maskV[*vmap.offset(k_2 as isize) as usize];
                k_2 = k_2.wrapping_add(1);
            }
            j_4 = j_4.wrapping_add(1);
        }
    }
    free(hmap as *mut ::core::ffi::c_void);
    hmap = ::core::ptr::null_mut::<shapeid_t>();
    free(vmap as *mut ::core::ffi::c_void);
    vmap = ::core::ptr::null_mut::<shapeid_t>();
}
unsafe extern "C" fn consolidateFDSelect(
    mut h: *mut fd_handle,
    mut cff: *mut table_CFF,
    mut options: *const otfcc_Options,
    gname: sds,
) {
    if cff.is_null() || (*cff).fdArray.is_null() || (*cff).fdArrayCount == 0 {
        return;
    }
    if (*h).state == HANDLE_STATE_INDEX
    {
        if (*h).index as ::core::ffi::c_int >= (*cff).fdArrayCount as ::core::ffi::c_int {
            (*h).index = 0 as glyphid_t;
        }
        handle_consolidateTo(
            h as *mut otfcc_Handle,
            (*h).index,
            (**(*cff).fdArray.offset((*h).index as isize)).fontName,
        );
    } else if !(*h).name.is_null() {
        let mut found: bool = false;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*cff).fdArrayCount as ::core::ffi::c_int {
            if strcmp(
                (*h).name as *const ::core::ffi::c_char,
                (**(*cff).fdArray.offset(j as isize)).fontName as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                found = true;
                handle_consolidateTo(
                    h as *mut otfcc_Handle,
                    j as glyphid_t,
                    (**(*cff).fdArray.offset(j as isize)).fontName,
                );
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        if !found {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] CID Subfont ",
                    (*h).name,
                    b" is not defined. (in glyph /",
                    gname,
                    b").\n",
                ),
            );
            otfcc_Handle_dispose(h as *mut otfcc_Handle);
        }
    } else if !(*h).name.is_null() {
        otfcc_Handle_dispose(h as *mut otfcc_Handle);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn consolidateGlyph(
    mut g: *mut glyf_Glyph,
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    consolidateGlyphContours(g, options);
    consolidateGlyphReferences(g, font, options);
    consolidateGlyphHints(g, options);
    consolidateFDSelect(&raw mut (*g).fdSelect, (*font).CFF_, options, (*g).name);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getPointCoordinates(
    mut table: *mut table_glyf,
    mut gr: *mut glyf_ComponentReference,
    mut n: shapeid_t,
    mut stated: *mut shapeid_t,
    mut x: *mut VQ,
    mut y: *mut VQ,
    mut options: *const otfcc_Options,
) -> bool {
    let mut j: glyphid_t = (*gr).glyph.index;
    let mut g: *mut glyf_Glyph = *(*table).items.offset(j as isize) as *mut glyf_Glyph;
    let mut c: shapeid_t = 0 as shapeid_t;
    while (c as usize) < (*g).contours.length {
        let mut pj: shapeid_t = 0 as shapeid_t;
        while (pj as usize) < (*(*g).contours.items.offset(c as isize)).length {
            if *stated as ::core::ffi::c_int == n as ::core::ffi::c_int {
                let mut p: *mut glyf_Point = (*(*g).contours.items.offset(c as isize))
                    .items
                    .offset(pj as isize)
                    as *mut glyf_Point;
                iVQ.replace.expect("non-null function pointer")(
                    x,
                    iVQ.pointLinearTfm.expect("non-null function pointer")(
                        (*gr).x,
                        (*gr).a as pos_t,
                        (*p).x,
                        (*gr).b as pos_t,
                        (*p).y,
                    ) as VQ,
                );
                iVQ.replace.expect("non-null function pointer")(
                    y,
                    iVQ.pointLinearTfm.expect("non-null function pointer")(
                        (*gr).y,
                        (*gr).c as pos_t,
                        (*p).x,
                        (*gr).d as pos_t,
                        (*p).y,
                    ) as VQ,
                );
                return true;
            }
            *stated = (*stated as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
            pj = pj.wrapping_add(1);
        }
        c = c.wrapping_add(1);
    }
    let mut r: shapeid_t = 0 as shapeid_t;
    while (r as usize) < (*g).references.length {
        let mut rr: *mut glyf_ComponentReference =
            (*g).references.items.offset(r as isize) as *mut glyf_ComponentReference;
        consolidateAnchorRef(table, gr, rr, options);
        let mut ref_0: glyf_ComponentReference =
            (
                glyf_iComponentReference
                    .empty
                    .expect("non-null function pointer"))();
        ref_0.glyph = handle_fromIndex(
            (*(*g).references.items.offset(r as isize)).glyph.index,
        ) as otfcc_GlyphHandle;
        ref_0.a = (*gr).a * (*rr).a + (*rr).b * (*gr).c;
        ref_0.b = (*rr).a * (*gr).b + (*rr).b * (*gr).d;
        ref_0.c = (*gr).a * (*rr).c + (*gr).c * (*rr).d;
        ref_0.d = (*gr).b * (*rr).c + (*rr).d * (*gr).d;
        iVQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.x,
            iVQ.pointLinearTfm.expect("non-null function pointer")(
                (*rr).x,
                (*rr).a as pos_t,
                (*gr).x,
                (*rr).b as pos_t,
                (*gr).y,
            ) as VQ,
        );
        iVQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.y,
            iVQ.pointLinearTfm.expect("non-null function pointer")(
                (*rr).y,
                (*rr).c as pos_t,
                (*gr).x,
                (*rr).d as pos_t,
                (*gr).y,
            ) as VQ,
        );
        let mut success: bool =
            getPointCoordinates(table, &raw mut ref_0, n, stated, x, y, options);
        glyf_iComponentReference
            .dispose
            .expect("non-null function pointer")(&raw mut ref_0);
        if success {
            return true;
        }
        r = r.wrapping_add(1);
    }
    return false;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn consolidateAnchorRef(
    mut table: *mut table_glyf,
    mut gr: *mut glyf_ComponentReference,
    mut rr: *mut glyf_ComponentReference,
    mut options: *const otfcc_Options,
) -> bool {
    if (*rr).isAnchored == REF_ANCHOR_CONSOLIDATED
        || (*rr).isAnchored == REF_XY
    {
        return true;
    }
    if (*rr).isAnchored == REF_ANCHOR_CONSOLIDATING_ANCHOR
        || (*rr).isAnchored == REF_ANCHOR_CONSOLIDATING_XY
    {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as u8,
            log_type_warning,
            crate::sdsbuild!(
                sdsempty(),
                b"Found circular reference of out-of-range point reference in anchored reference.",
            ),
        );
        (*rr).isAnchored = REF_XY;
        return false;
    }
    if (*rr).isAnchored == REF_ANCHOR_ANCHOR
    {
        (*rr).isAnchored = REF_ANCHOR_CONSOLIDATING_ANCHOR;
    } else {
        (*rr).isAnchored = REF_ANCHOR_CONSOLIDATING_XY;
    }
    let mut innerX: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut outerX: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut innerY: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut outerY: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut innerCounter: shapeid_t = 0 as shapeid_t;
    let mut outerCounter: shapeid_t = 0 as shapeid_t;
    let mut rr1: glyf_ComponentReference =
        (
            glyf_iComponentReference
                .empty
                .expect("non-null function pointer"))();
    rr1.glyph = handle_fromIndex((*rr).glyph.index)
        as otfcc_GlyphHandle;
    let mut s1: bool = getPointCoordinates(
        table,
        gr,
        (*rr).outer,
        &raw mut outerCounter,
        &raw mut outerX,
        &raw mut outerY,
        options,
    );
    let mut s2: bool = getPointCoordinates(
        table,
        &raw mut rr1,
        (*rr).inner,
        &raw mut innerCounter,
        &raw mut innerX,
        &raw mut innerY,
        options,
    );
    if !s1 {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as u8,
            log_type_warning,
            crate::sdsbuild!(
                sdsempty(),
                b"Failed to access point ",
                (*rr).outer as ::core::ffi::c_int,
                b" in outer glyph.",
            ),
        );
    }
    if !s2 {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as u8,
            log_type_warning,
            crate::sdsbuild!(
                sdsempty(),
                b"Failed to access point ",
                (*rr).outer as ::core::ffi::c_int,
                b" in reference to ",
                (*rr).glyph.name,
                b".",
            ),
        );
    }
    let mut rrx: VQ = iVQ.pointLinearTfm.expect("non-null function pointer")(
        outerX,
        -((*rr).a as pos_t),
        innerX,
        -((*rr).b as pos_t),
        innerY,
    );
    let mut rry: VQ = iVQ.pointLinearTfm.expect("non-null function pointer")(
        outerY,
        -((*rr).c as pos_t),
        innerX,
        -((*rr).d as pos_t),
        innerY,
    );
    if (*rr).isAnchored == REF_ANCHOR_CONSOLIDATING_ANCHOR
    {
        iVQ.replace.expect("non-null function pointer")(&raw mut (*rr).x, rrx);
        iVQ.replace.expect("non-null function pointer")(&raw mut (*rr).y, rry);
        (*rr).isAnchored = REF_ANCHOR_CONSOLIDATED;
    } else {
        if fabs(
            iVQ.getStill.expect("non-null function pointer")((*rr).x) as ::core::ffi::c_double
                - iVQ.getStill.expect("non-null function pointer")(rrx) as ::core::ffi::c_double,
        ) > 0.5f64
            && fabs(
                iVQ.getStill.expect("non-null function pointer")((*rr).y) as ::core::ffi::c_double
                    - iVQ.getStill.expect("non-null function pointer")(rry)
                        as ::core::ffi::c_double,
            ) > 0.5f64
        {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"Anchored reference to ",
                    (*rr).glyph.name,
                    b" does not match its X/Y offset data.",
                ),
            );
        }
        (*rr).isAnchored = REF_ANCHOR_CONSOLIDATED;
        iVQ.dispose.expect("non-null function pointer")(&raw mut rrx);
        iVQ.dispose.expect("non-null function pointer")(&raw mut rry);
    }
    glyf_iComponentReference
        .dispose
        .expect("non-null function pointer")(&raw mut rr1);
    iVQ.dispose.expect("non-null function pointer")(&raw mut innerX);
    iVQ.dispose.expect("non-null function pointer")(&raw mut innerY);
    iVQ.dispose.expect("non-null function pointer")(&raw mut outerX);
    iVQ.dispose.expect("non-null function pointer")(&raw mut outerY);
    return false;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn consolidateGlyf(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    if (*font).glyph_order.is_null() || (*font).glyf.is_null() {
        return;
    }
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*(*font).glyf).length {
        if !(*(*(*font).glyf).items.offset(j as isize)).is_null() {
            consolidateGlyph(
                *(*(*font).glyf).items.offset(j as isize) as *mut glyf_Glyph,
                font,
                options,
            );
        } else {
            let ref mut fresh6 = *(*(*font).glyf).items.offset(j as isize);
            *fresh6 = otfcc_newGlyf_glyph() as glyf_GlyphPtr;
        }
        j = j.wrapping_add(1);
    }
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as usize) < (*(*font).glyf).length {
        let mut g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j_0 as isize) as *mut glyf_Glyph;
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), (*g).name),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut gr: glyf_ComponentReference =
                (
                    glyf_iComponentReference
                        .empty
                        .expect("non-null function pointer"))();
            gr.glyph = handle_fromIndex(j_0)
                as otfcc_GlyphHandle;
            let mut r: shapeid_t = 0 as shapeid_t;
            while (r as usize) < (*g).references.length {
                let mut rr: *mut glyf_ComponentReference =
                    (*g).references.items.offset(r as isize) as *mut glyf_ComponentReference;
                consolidateAnchorRef((*font).glyf, &raw mut gr, rr, options);
                r = r.wrapping_add(1);
            }
            glyf_iComponentReference
                .dispose
                .expect("non-null function pointer")(&raw mut gr);
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
        j_0 = j_0.wrapping_add(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn consolidateCmap(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    if !(*font).glyph_order.is_null() && !(*font).cmap.is_null() {
        let mut item: *mut cmap_Entry = ::core::ptr::null_mut::<cmap_Entry>();
        item = (*(*font).cmap).unicodes;
        while !item.is_null() {
            if !otfcc_pkgGlyphOrder
                .consolidateHandle
                .expect("non-null function pointer")(
                (*font).glyph_order, &raw mut (*item).glyph
            ) {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignored mapping U+",
                        Hex4Upper(((*item).unicode) as u32),
                        b" to non-existent glyph /",
                        (*item).glyph.name,
                        b".\n",
                    ),
                );
                otfcc_Handle_dispose(&raw mut (*item).glyph);
            }
            item = (*item).hh.next as *mut cmap_Entry;
        }
    }
    if !(*font).glyph_order.is_null() && !(*font).cmap.is_null() {
        let mut item_0: *mut cmap_UVS_Entry = ::core::ptr::null_mut::<cmap_UVS_Entry>();
        item_0 = (*(*font).cmap).uvs;
        while !item_0.is_null() {
            if !otfcc_pkgGlyphOrder
                .consolidateHandle
                .expect("non-null function pointer")(
                (*font).glyph_order, &raw mut (*item_0).glyph
            ) {
                (*(*options).logger)
                    .logSDS
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignored UVS mapping [U+",
                        Hex4Upper(((*item_0).key.unicode) as u32),
                        b" U+",
                        Hex4Upper(((*item_0).key.selector) as u32),
                        b"] to non-existent glyph /",
                        (*item_0).glyph.name,
                        b".\n",
                    ),
                );
                otfcc_Handle_dispose(&raw mut (*item_0).glyph);
            }
            item_0 = (*item_0).hh.next as *mut cmap_UVS_Entry;
        }
    }
}
unsafe extern "C" fn __declare_otl_consolidation(
    mut type_0: otl_LookupType,
    mut fn_0: otl_consolidation_function,
    mut fndel: subtable_remover,
    mut font: *mut otfcc_Font,
    mut table: *mut table_OTL,
    mut lookup: *mut otl_Lookup,
    mut options: *const otfcc_Options,
) {
    if lookup.is_null()
        || (*lookup).subtables.length == 0
        || (*lookup).type_0 != type_0
    {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), (*lookup).name),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: tableid_t = 0 as tableid_t;
        while (j as usize) < (*lookup).subtables.length {
            if (*(*lookup).subtables.items.offset(j as isize)).is_null() {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignored empty subtable ",
                        j as ::core::ffi::c_int,
                        b" of lookup ",
                        (*lookup).name,
                        b".\n",
                    ),
                );
            } else {
                let mut subtableRemoved: bool = false;
                subtableRemoved = fn_0.expect("non-null function pointer")(
                    font,
                    table,
                    *(*lookup).subtables.items.offset(j as isize) as *mut otl_Subtable,
                    options,
                );
                if subtableRemoved {
                    fndel.expect("non-null function pointer")(
                        *(*lookup).subtables.items.offset(j as isize) as *mut otl_Subtable,
                    );
                    let ref mut fresh3 = *(*lookup).subtables.items.offset(j as isize);
                    *fresh3 = ::core::ptr::null_mut::<otl_Subtable>();
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as u8,
                        log_type_warning,
                        crate::sdsbuild!(
                            sdsempty(),
                            b"[Consolidate] Ignored empty subtable ",
                            j as ::core::ffi::c_int,
                            b" of lookup ",
                            (*lookup).name,
                            b".\n",
                        ),
                    );
                }
            }
            j = j.wrapping_add(1);
        }
        let mut k: tableid_t = 0 as tableid_t;
        let mut j_0: tableid_t = 0 as tableid_t;
        while (j_0 as usize) < (*lookup).subtables.length {
            if !(*(*lookup).subtables.items.offset(j_0 as isize)).is_null() {
                let fresh4 = k;
                k = k.wrapping_add(1);
                let ref mut fresh5 = *(*lookup).subtables.items.offset(fresh4 as isize);
                *fresh5 = *(*lookup).subtables.items.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
        }
        (*lookup).subtables.length = k as usize;
        if k == 0 {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Lookup ",
                    (*lookup).name,
                    b" is empty and will be removed.\n",
                ),
            );
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_consolidate_lookup(
    mut font: *mut otfcc_Font,
    mut table: *mut table_OTL,
    mut lookup: *mut otl_Lookup,
    mut options: *const otfcc_Options,
) {
    __declare_otl_consolidation(
        otl_type_gsub_single,
        Some(
            consolidate_gsub_single
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
            subtable_remover,
        >(iSubtable_gsub_single.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gsub_multiple,
        Some(
            consolidate_gsub_multi
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
            subtable_remover,
        >(iSubtable_gsub_multi.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gsub_alternate,
        Some(
            consolidate_gsub_alternative
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
            subtable_remover,
        >(iSubtable_gsub_multi.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gsub_ligature,
        Some(
            consolidate_gsub_ligature
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
            subtable_remover,
        >(iSubtable_gsub_ligature.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gsub_chaining,
        Some(
            consolidate_chaining
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
            subtable_remover,
        >(iSubtable_chaining.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gsub_reverse,
        Some(
            consolidate_gsub_reverse
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gsub_reverse) -> ()>,
            subtable_remover,
        >(iSubtable_gsub_reverse.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_single,
        Some(
            consolidate_gpos_single
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
            subtable_remover,
        >(iSubtable_gpos_single.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_pair,
        Some(
            consolidate_gpos_pair
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gpos_pair) -> ()>,
            subtable_remover,
        >(iSubtable_gpos_pair.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_cursive,
        Some(
            consolidate_gpos_cursive
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
            subtable_remover,
        >(iSubtable_gpos_cursive.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_chaining,
        Some(
            consolidate_chaining
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
            subtable_remover,
        >(iSubtable_chaining.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_markToBase,
        Some(
            consolidate_mark_to_single
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> ()>,
            subtable_remover,
        >(iSubtable_gpos_markToSingle.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_markToMark,
        Some(
            consolidate_mark_to_single
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> ()>,
            subtable_remover,
        >(iSubtable_gpos_markToSingle.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_markToLigature,
        Some(
            consolidate_mark_to_ligature
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gpos_markToLigature) -> ()>,
            subtable_remover,
        >(iSubtable_gpos_markToLigature.free),
        font,
        table,
        lookup,
        options,
    );
}
unsafe extern "C" fn lookupRefIsNotEmpty(
    mut rLut: *const otl_LookupRef,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !rLut.is_null() && !(*rLut).is_null() && (**rLut).subtables.length > 0 as usize;
}
unsafe extern "C" fn featureRefIsNotEmpty(
    mut rFeat: *const otl_FeatureRef,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !rFeat.is_null() && !(*rFeat).is_null() && (**rFeat).lookups.length > 0 as usize;
}
unsafe extern "C" fn lookupIsNotEmpty(
    mut rLut: *const otl_LookupPtr,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !rLut.is_null() && !(*rLut).is_null() && (**rLut).subtables.length > 0 as usize;
}
unsafe extern "C" fn featureIsNotEmpty(
    mut rFeat: *const otl_FeaturePtr,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !rFeat.is_null() && !(*rFeat).is_null() && (**rFeat).lookups.length > 0 as usize;
}
unsafe extern "C" fn consolidateOTLTable(
    mut font: *mut otfcc_Font,
    mut table: *mut table_OTL,
    mut options: *const otfcc_Options,
) {
    if (*font).glyph_order.is_null() || table.is_null() {
        return;
    }
    loop {
        let mut featN: tableid_t = (*table).features.length as tableid_t;
        let mut lutN: tableid_t = (*table).lookups.length as tableid_t;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as usize) < (*table).lookups.length {
            otfcc_consolidate_lookup(
                font,
                table,
                *(*table).lookups.items.offset(j as isize) as *mut otl_Lookup,
                options,
            );
            j = j.wrapping_add(1);
        }
        let mut j_0: tableid_t = 0 as tableid_t;
        while (j_0 as usize) < (*table).features.length {
            let mut feature: *mut otl_Feature =
                *(*table).features.items.offset(j_0 as isize) as *mut otl_Feature;
            otl_iLookupRefList
                .filterEnv
                .expect("non-null function pointer")(
                &raw mut (*feature).lookups,
                Some(
                    lookupRefIsNotEmpty
                        as unsafe extern "C" fn(
                            *const otl_LookupRef,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                ),
                NULL,
            );
            j_0 = j_0.wrapping_add(1);
        }
        let mut j_1: tableid_t = 0 as tableid_t;
        while (j_1 as usize) < (*table).languages.length {
            let mut lang: *mut otl_LanguageSystem =
                *(*table).languages.items.offset(j_1 as isize) as *mut otl_LanguageSystem;
            otl_iFeatureRefList
                .filterEnv
                .expect("non-null function pointer")(
                &raw mut (*lang).features,
                Some(
                    featureRefIsNotEmpty
                        as unsafe extern "C" fn(
                            *const otl_FeatureRef,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                ),
                NULL,
            );
            j_1 = j_1.wrapping_add(1);
        }
        otl_iLookupList
            .filterEnv
            .expect("non-null function pointer")(
            &raw mut (*table).lookups,
            Some(
                lookupIsNotEmpty
                    as unsafe extern "C" fn(*const otl_LookupPtr, *mut ::core::ffi::c_void) -> bool,
            ),
            NULL,
        );
        otl_iFeatureList
            .filterEnv
            .expect("non-null function pointer")(
            &raw mut (*table).features,
            Some(
                featureIsNotEmpty
                    as unsafe extern "C" fn(
                        *const otl_FeaturePtr,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
            ),
            NULL,
        );
        let mut featN1: tableid_t = (*table).features.length as tableid_t;
        let mut lutN1: tableid_t = (*table).lookups.length as tableid_t;
        if featN1 as ::core::ffi::c_int >= featN as ::core::ffi::c_int
            && lutN1 as ::core::ffi::c_int >= lutN as ::core::ffi::c_int
        {
            break;
        }
    }
}
unsafe extern "C" fn consolidateOTL(mut font: *mut otfcc_Font, mut options: *const otfcc_Options) {
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"GSUB"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        consolidateOTLTable(font, (*font).GSUB, options);
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"GPOS"),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        consolidateOTLTable(font, (*font).GPOS, options);
        ___loggedstep_v_0 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"GDEF"),
    );
    let mut ___loggedstep_v_1: bool = true;
    while ___loggedstep_v_1 {
        consolidate_GDEF(font, (*font).GDEF, options);
        ___loggedstep_v_1 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
unsafe extern "C" fn consolidateCOLR(mut font: *mut otfcc_Font, mut options: *const otfcc_Options) {
    if font.is_null() || (*font).COLR.is_null() || (*font).glyph_order.is_null() {
        return;
    }
    let mut consolidated: *mut table_COLR = (
        table_iCOLR.create.expect("non-null function pointer"))();
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*(*font).COLR).length {
        let mut mapping: *mut colr_Mapping = (*(*font).COLR).items.offset(__caryll_index as isize);
        while keep != 0 {
            if !otfcc_pkgGlyphOrder
                .consolidateHandle
                .expect("non-null function pointer")(
                (*font).glyph_order, &raw mut (*mapping).glyph
            ) {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignored missing glyph of /",
                        (*mapping).glyph.name,
                    ),
                );
            } else {
                let mut m: colr_Mapping = colr_Mapping {
                    glyph: otfcc_Handle {
                        state: HANDLE_STATE_EMPTY,
                        index: 0,
                        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    },
                    layers: colr_LayerList {
                        length: 0,
                        capacity: 0,
                        items: ::core::ptr::null_mut::<colr_Layer>(),
                    },
                };
                otfcc_Handle_copy(
                    &raw mut m.glyph,
                    &raw mut (*mapping).glyph,
                );
                colr_iLayerList.init.expect("non-null function pointer")(&raw mut m.layers);
                let mut __caryll_index_0: usize = 0 as usize;
                let mut keep_0: usize = 1 as usize;
                while keep_0 != 0 && __caryll_index_0 < (*mapping).layers.length {
                    let mut layer: *mut colr_Layer =
                        (*mapping).layers.items.offset(__caryll_index_0 as isize);
                    while keep_0 != 0 {
                        if !otfcc_pkgGlyphOrder
                            .consolidateHandle
                            .expect("non-null function pointer")(
                            (*font).glyph_order,
                            &raw mut (*layer).glyph,
                        ) {
                            (*(*options).logger)
                                .logSDS
                                .expect("non-null function pointer")(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[Consolidate] Ignored missing glyph of /",
                                    (*layer).glyph.name,
                                ),
                            );
                        } else {
                            let mut layer1: colr_Layer = colr_Layer {
                                glyph: otfcc_Handle {
                                    state: HANDLE_STATE_EMPTY,
                                    index: 0,
                                    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                },
                                paletteIndex: 0,
                            };
                            colr_iLayer.copy.expect("non-null function pointer")(
                                &raw mut layer1,
                                layer,
                            );
                            colr_iLayerList.push.expect("non-null function pointer")(
                                &raw mut m.layers,
                                layer1,
                            );
                        }
                        keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    }
                    keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    __caryll_index_0 = __caryll_index_0.wrapping_add(1);
                }
                if (*mapping).layers.length != 0 {
                    table_iCOLR.push.expect("non-null function pointer")(consolidated, m);
                } else {
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as u8,
                        log_type_warning,
                        crate::sdsbuild!(
                            sdsempty(),
                            b"[Consolidate] COLR decomposition for /",
                            (*mapping).glyph.name,
                            b" is empth",
                        ),
                    );
                    colr_iMapping.dispose.expect("non-null function pointer")(&raw mut m);
                }
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    table_iCOLR.free.expect("non-null function pointer")((*font).COLR);
    (*font).COLR = consolidated;
}
unsafe extern "C" fn compareTSIEntry(
    mut a: *const tsi_Entry,
    mut b: *const tsi_Entry,
) -> ::core::ffi::c_int {
    if (*a).type_0 as ::core::ffi::c_uint != (*b).type_0 as ::core::ffi::c_uint {
        return ((*a).type_0 as ::core::ffi::c_uint).wrapping_sub((*b).type_0 as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
    return (*a).glyph.index as ::core::ffi::c_int - (*b).glyph.index as ::core::ffi::c_int;
}
unsafe extern "C" fn consolidateTSI(
    mut font: *mut otfcc_Font,
    mut _tsi: *mut *mut table_TSI,
    mut options: *const otfcc_Options,
) {
    let mut tsi: *mut table_TSI = *_tsi;
    if font.is_null() || (*font).glyf.is_null() || tsi.is_null() || (*font).glyph_order.is_null() {
        return;
    }
    let mut consolidated: *mut table_TSI = (
        table_iTSI.create.expect("non-null function pointer"))();
    let mut gidEntries: *mut sds = ::core::ptr::null_mut::<sds>();
    gidEntries = __caryll_allocate_clean(
        (::core::mem::size_of::<sds>() as usize).wrapping_mul((*(*font).glyf).length),
        448 as ::core::ffi::c_ulong,
    ) as *mut sds;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*tsi).length {
        let mut entry: *mut tsi_Entry = (*tsi).items.offset(__caryll_index as isize);
        while keep != 0 {
            if (*entry).type_0 as ::core::ffi::c_uint
                == TSI_GLYPH as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if otfcc_pkgGlyphOrder
                    .consolidateHandle
                    .expect("non-null function pointer")(
                    (*font).glyph_order,
                    &raw mut (*entry).glyph,
                ) {
                    if !(*gidEntries.offset((*entry).glyph.index as isize)).is_null() {
                        sdsfree(*gidEntries.offset((*entry).glyph.index as isize));
                    }
                    let ref mut fresh2 = *gidEntries.offset((*entry).glyph.index as isize);
                    *fresh2 = (*entry).content;
                    (*entry).content = ::core::ptr::null_mut::<::core::ffi::c_char>();
                } else {
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as u8,
                        log_type_warning,
                        crate::sdsbuild!(
                            sdsempty(),
                            b"[Consolidate] Ignored missing glyph of /",
                            (*entry).glyph.name,
                        ),
                    );
                }
            } else {
                let mut e: tsi_Entry = tsi_Entry {
                    type_0: TSI_GLYPH,
                    glyph: otfcc_Handle {
                        state: HANDLE_STATE_EMPTY,
                        index: 0,
                        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    },
                    content: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                };
                tsi_iEntry.copy.expect("non-null function pointer")(&raw mut e, entry);
                table_iTSI.push.expect("non-null function pointer")(consolidated, e);
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*(*font).glyf).length {
        let mut e_0: tsi_Entry = tsi_Entry {
            type_0: TSI_GLYPH,
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            content: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        e_0.type_0 = TSI_GLYPH;
        e_0.glyph =
            handle_fromIndex(j) as otfcc_GlyphHandle;
        otfcc_pkgGlyphOrder
            .consolidateHandle
            .expect("non-null function pointer")((*font).glyph_order, &raw mut e_0.glyph);
        e_0.content = if !(*gidEntries.offset(j as isize)).is_null() {
            *gidEntries.offset(j as isize)
        } else {
            sdsempty()
        };
        table_iTSI.push.expect("non-null function pointer")(consolidated, e_0);
        j = j.wrapping_add(1);
    }
    table_iTSI.free.expect("non-null function pointer")(tsi);
    free(gidEntries as *mut ::core::ffi::c_void);
    gidEntries = ::core::ptr::null_mut::<sds>();
    table_iTSI.sort.expect("non-null function pointer")(
        consolidated,
        Some(
            compareTSIEntry
                as unsafe extern "C" fn(*const tsi_Entry, *const tsi_Entry) -> ::core::ffi::c_int,
        ),
    );
    *_tsi = consolidated;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_consolidateFont(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    if !(*font).glyf.is_null() && (*font).glyph_order.is_null() {
        let mut go: *mut otfcc_GlyphOrder =
            (
                otfcc_pkgGlyphOrder
                    .create
                    .expect("non-null function pointer"))();
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as usize) < (*(*font).glyf).length {
            let mut name: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut glyfName: sds = (**(*(*font).glyf).items.offset(j as isize)).name;
            if !glyfName.is_null() {
                name = sdsdup(glyfName);
            } else {
                name = crate::sdsbuild!(sdsempty(), b"$$gid", j as ::core::ffi::c_int);
                let ref mut fresh0 = (**(*(*font).glyf).items.offset(j as isize)).name;
                *fresh0 = sdsdup(name);
            }
            if !otfcc_pkgGlyphOrder
                .setByName
                .expect("non-null function pointer")(go, name, j)
            {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Glyph name ",
                        name,
                        b" is already in use.",
                    ),
                );
                let mut suffix: u32 = 2 as u32;
                let mut success: bool = false;
                loop {
                    let mut newname: sds = crate::sdsbuild!(sdsempty(), name, b"_", suffix);
                    success = otfcc_pkgGlyphOrder
                        .setByName
                        .expect("non-null function pointer")(
                        go, newname, j
                    );
                    if !success {
                        sdsfree(newname);
                        suffix = suffix.wrapping_add(1 as u32);
                    } else {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            crate::sdsbuild!(
                                sdsempty(),
                                b"[Consolidate] Glyph ",
                                name,
                                b" is renamed into ",
                                newname,
                                b".",
                            ),
                        );
                        sdsfree((**(*(*font).glyf).items.offset(j as isize)).name);
                        let ref mut fresh1 = (**(*(*font).glyf).items.offset(j as isize)).name;
                        *fresh1 = sdsdup(newname);
                    }
                    if success {
                        break;
                    }
                }
                sdsfree(name);
            }
            j = j.wrapping_add(1);
        }
        (*font).glyph_order = go;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"glyf"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        consolidateGlyf(font, options);
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"cmap"),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        consolidateCmap(font, options);
        ___loggedstep_v_0 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    if !(*font).glyf.is_null() {
        consolidateOTL(font, options);
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"COLR"),
    );
    let mut ___loggedstep_v_1: bool = true;
    while ___loggedstep_v_1 {
        consolidateCOLR(font, options);
        ___loggedstep_v_1 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"TSI_01"),
    );
    let mut ___loggedstep_v_2: bool = true;
    while ___loggedstep_v_2 {
        consolidateTSI(font, &raw mut (*font).TSI_01, options);
        ___loggedstep_v_2 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"TSI_23"),
    );
    let mut ___loggedstep_v_3: bool = true;
    while ___loggedstep_v_3 {
        consolidateTSI(font, &raw mut (*font).TSI_23, options);
        ___loggedstep_v_3 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"TSI5"),
    );
    let mut ___loggedstep_v_4: bool = true;
    while ___loggedstep_v_4 {
        fontop_consolidateClassDef(font, (*font).TSI5 as *mut otl_ClassDef, options);
        ___loggedstep_v_4 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
