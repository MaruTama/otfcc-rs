#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod otl;

use libc::{free, strcmp};
unsafe extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}


use crate::support::handle::{HandleState, handle_consolidate_to, handle_from_index, FdHandle, GlyphHandle, Handle, otfcc_handle_copy, otfcc_handle_dispose};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, Pos, ShapeId, TableId};
use crate::vendor::sds::{Hex4Upper, SdsRaw};
use crate::font::caryll_font::{Font};
use crate::support::{NULL};
use crate::support::glyph_order::GlyphOrder;

use crate::table::cff::{CffTable};
use crate::table::colr::{ColrLayer, ColrLayerList, ColrMapping, ColrTable};






use crate::table::_tsi::{TsiEntryType, TsiTable, TsiEntry};
use crate::table::cmap::{CmapEntry, CmapUvsEntry};




use crate::table::glyf::{RefAnchorStatus, ComponentReference, Glyph, GlyphPtr, Point, PostscriptHintMask, PostscriptStemDef, GlyfTable};







use crate::table::otl::{Feature, FeaturePtr, FeatureRef, LanguageSystem, Lookup, LookupPtr, LookupRef, LookupType, Subtable, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CURSIVE, OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE, ChainingSubtable, GposCursiveSubtable, GposMarkToLigatureSubtable, GposMarkToSingleSubtable, GposPairSubtable, GposSingleSubtable, GsubLigatureSubtable, GsubMultiSubtable, GsubReverseSubtable, GsubSingleSubtable, OtlTable};
use crate::table::otl::classdef::{ClassDef};







use crate::vf::vq::VQ;
use crate::consolidate::otl::gdef::{consolidate_gdef};
use crate::consolidate::otl::chaining::{consolidate_chaining};
use crate::consolidate::otl::common::{fontop_consolidate_class_def};
use crate::consolidate::otl::gpos_cursive::{consolidate_gpos_cursive};
use crate::consolidate::otl::gpos_pair::{consolidate_gpos_pair};
use crate::consolidate::otl::gpos_single::{consolidate_gpos_single};
use crate::consolidate::otl::gsub_ligature::{consolidate_gsub_ligature};
use crate::consolidate::otl::gsub_multi::{consolidate_gsub_alternative, consolidate_gsub_multi};
use crate::consolidate::otl::gsub_reverse::{consolidate_gsub_reverse};
use crate::consolidate::otl::gsub_single::{consolidate_gsub_single};
use crate::consolidate::otl::mark::{consolidate_mark_to_ligature, consolidate_mark_to_single};
use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::table::colr::{COLR_I_LAYER, COLR_I_LAYER_LIST, COLR_I_MAPPING, TABLE_I_COLR};
use crate::table::_tsi::{TABLE_I_TSI, TSI_I_ENTRY};
use crate::table::glyf::{GLYF_I_COMPONENT_REFERENCE, GLYF_I_CONTOUR_LIST, GLYF_I_MASK_LIST, GLYF_I_REFERENCE_LIST, GLYF_I_STEM_DEF_LIST, otfcc_new_glyf_glyph};
use crate::table::otl::{OTL_I_FEATURE_LIST, OTL_I_FEATURE_REF_LIST, OTL_I_LOOKUP_LIST, OTL_I_LOOKUP_REF_LIST};
use crate::table::otl::subtables::chaining::common::{I_SUBTABLE_CHAINING};
use crate::table::otl::subtables::gpos_cursive::{I_SUBTABLE_GPOS_CURSIVE};
use crate::table::otl::subtables::gpos_mark_to_ligature::{I_SUBTABLE_GPOS_MARK_TO_LIGATURE};
use crate::table::otl::subtables::gpos_mark_to_single::{I_SUBTABLE_GPOS_MARK_TO_SINGLE};
use crate::table::otl::subtables::gpos_pair::{I_SUBTABLE_GPOS_PAIR};
use crate::table::otl::subtables::gpos_single::{I_SUBTABLE_GPOS_SINGLE};
use crate::table::otl::subtables::gsub_ligature::{I_SUBTABLE_GSUB_LIGATURE};
use crate::table::otl::subtables::gsub_multi::{I_SUBTABLE_GSUB_MULTI};
use crate::table::otl::subtables::gsub_reverse::{I_SUBTABLE_GSUB_REVERSE};
use crate::table::otl::subtables::gsub_single::{I_SUBTABLE_GSUB_SINGLE};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree};
use crate::vf::vq::{I_VQ};

pub type SubtableRemover = Option<unsafe extern "C" fn(*mut Subtable) -> ()>;
pub type OtlConsolidationFunction = Option<
    unsafe extern "C" fn(
        *mut Font,
        *mut OtlTable,
        *mut Subtable,
        *const Options,
    ) -> bool,
>;
unsafe extern "C" fn by_stem_pos(
    mut a: *const PostscriptStemDef,
    mut b: *const PostscriptStemDef,
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
    mut a: *const PostscriptHintMask,
    mut b: *const PostscriptHintMask,
) -> ::core::ffi::c_int {
    return if (*a).contours_before as ::core::ffi::c_int == (*b).contours_before as ::core::ffi::c_int
    {
        (*a).points_before as ::core::ffi::c_int - (*b).points_before as ::core::ffi::c_int
    } else {
        (*a).contours_before as ::core::ffi::c_int - (*b).contours_before as ::core::ffi::c_int
    };
}
unsafe extern "C" fn consolidate_glyph_contours(
    mut g: *mut Glyph,
    mut options: *const Options,
) {
    let mut n_contours_consolidated: ShapeId = 0 as ShapeId;
    let mut skip: ShapeId = 0 as ShapeId;
    let mut j: ShapeId = 0 as ShapeId;
    while (j as usize) < (*g).contours.length {
        if (*(*g).contours.items.offset(j as isize)).length != 0 {
            *(*g)
                .contours
                .items
                .offset((j as ::core::ffi::c_int - skip as ::core::ffi::c_int) as isize) =
                *(*g).contours.items.offset(j as isize);
            n_contours_consolidated = (n_contours_consolidated as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as ShapeId;
        } else {
            GLYF_I_CONTOUR_LIST
                .dispose_item
                .expect("non-null function pointer")(
                &raw mut (*g).contours, j as usize
            );
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Removed empty contour #",
                    j as ::core::ffi::c_int,
                    b" in glyph ",
                    (*g).name,
                    b".\n",
                ),
            );
            skip = (skip as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
        }
        j = j.wrapping_add(1);
    }
    (*g).contours.length = n_contours_consolidated as usize;
}
unsafe extern "C" fn consolidate_glyph_references(
    mut g: *mut Glyph,
    mut font: *mut Font,
    mut options: *const Options,
) {
    let mut n_references_consolidated: ShapeId = 0 as ShapeId;
    let mut skip: ShapeId = 0 as ShapeId;
    let mut j: ShapeId = 0 as ShapeId;
    while (j as usize) < (*g).references.length {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (*(*g).references.items.offset(j as isize)).glyph,
        ) {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Ignored absent glyph component reference /",
                    (*(*g).references.items.offset(j as isize)).glyph.name,
                    b" within /",
                    (*g).name,
                    b".\n",
                ),
            );
            GLYF_I_REFERENCE_LIST
                .dispose_item
                .expect("non-null function pointer")(
                &raw mut (*g).references, j as usize
            );
            skip = (skip as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
        } else {
            *(*g)
                .references
                .items
                .offset((j as ::core::ffi::c_int - skip as ::core::ffi::c_int) as isize) =
                *(*g).references.items.offset(j as isize);
            n_references_consolidated = (n_references_consolidated as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as ShapeId;
        }
        j = j.wrapping_add(1);
    }
    (*g).references.length = n_references_consolidated as usize;
}
unsafe extern "C" fn consolidate_glyph_hints(
    mut g: *mut Glyph,
    mut _options: *const Options,
) {
    if (*g).stem_h.length != 0 {
        let mut j: ShapeId = 0 as ShapeId;
        while (j as usize) < (*g).stem_h.length {
            (*(*g).stem_h.items.offset(j as isize)).map = j as u16;
            j = j.wrapping_add(1);
        }
        GLYF_I_STEM_DEF_LIST.sort.expect("non-null function pointer")(
            &raw mut (*g).stem_h,
            Some(
                by_stem_pos
                    as unsafe extern "C" fn(
                        *const PostscriptStemDef,
                        *const PostscriptStemDef,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    if (*g).stem_v.length != 0 {
        let mut j_0: ShapeId = 0 as ShapeId;
        while (j_0 as usize) < (*g).stem_v.length {
            (*(*g).stem_v.items.offset(j_0 as isize)).map = j_0 as u16;
            j_0 = j_0.wrapping_add(1);
        }
        GLYF_I_STEM_DEF_LIST.sort.expect("non-null function pointer")(
            &raw mut (*g).stem_v,
            Some(
                by_stem_pos
                    as unsafe extern "C" fn(
                        *const PostscriptStemDef,
                        *const PostscriptStemDef,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    let mut hmap: *mut ShapeId = ::core::ptr::null_mut::<ShapeId>();
    hmap = __caryll_allocate_clean(
        (::core::mem::size_of::<ShapeId>() as usize).wrapping_mul((*g).stem_h.length),
        80 as ::core::ffi::c_ulong,
    ) as *mut ShapeId;
    let mut vmap: *mut ShapeId = ::core::ptr::null_mut::<ShapeId>();
    vmap = __caryll_allocate_clean(
        (::core::mem::size_of::<ShapeId>() as usize).wrapping_mul((*g).stem_v.length),
        82 as ::core::ffi::c_ulong,
    ) as *mut ShapeId;
    let mut j_1: ShapeId = 0 as ShapeId;
    while (j_1 as usize) < (*g).stem_h.length {
        *hmap.offset((*(*g).stem_h.items.offset(j_1 as isize)).map as isize) = j_1;
        j_1 = j_1.wrapping_add(1);
    }
    let mut j_2: ShapeId = 0 as ShapeId;
    while (j_2 as usize) < (*g).stem_v.length {
        *vmap.offset((*(*g).stem_v.items.offset(j_2 as isize)).map as isize) = j_2;
        j_2 = j_2.wrapping_add(1);
    }
    if (*g).hint_masks.length != 0 {
        GLYF_I_MASK_LIST.sort.expect("non-null function pointer")(
            &raw mut (*g).hint_masks,
            Some(
                by_mask_pointindex
                    as unsafe extern "C" fn(
                        *const PostscriptHintMask,
                        *const PostscriptHintMask,
                    ) -> ::core::ffi::c_int,
            ),
        );
        let mut j_3: ShapeId = 0 as ShapeId;
        while (j_3 as usize) < (*g).hint_masks.length {
            let mut oldmask: PostscriptHintMask = *(*g).hint_masks.items.offset(j_3 as isize);
            let mut k: ShapeId = 0 as ShapeId;
            while (k as usize) < (*g).stem_h.length {
                (*(*g).hint_masks.items.offset(j_3 as isize)).mask_h[k as usize] =
                    oldmask.mask_h[*hmap.offset(k as isize) as usize];
                k = k.wrapping_add(1);
            }
            let mut k_0: ShapeId = 0 as ShapeId;
            while (k_0 as usize) < (*g).stem_v.length {
                (*(*g).hint_masks.items.offset(j_3 as isize)).mask_v[k_0 as usize] =
                    oldmask.mask_v[*vmap.offset(k_0 as isize) as usize];
                k_0 = k_0.wrapping_add(1);
            }
            j_3 = j_3.wrapping_add(1);
        }
    }
    if (*g).contour_masks.length != 0 {
        GLYF_I_MASK_LIST.sort.expect("non-null function pointer")(
            &raw mut (*g).contour_masks,
            Some(
                by_mask_pointindex
                    as unsafe extern "C" fn(
                        *const PostscriptHintMask,
                        *const PostscriptHintMask,
                    ) -> ::core::ffi::c_int,
            ),
        );
        let mut j_4: ShapeId = 0 as ShapeId;
        while (j_4 as usize) < (*g).contour_masks.length {
            let mut oldmask_0: PostscriptHintMask =
                *(*g).contour_masks.items.offset(j_4 as isize);
            let mut k_1: ShapeId = 0 as ShapeId;
            while (k_1 as usize) < (*g).stem_h.length {
                (*(*g).contour_masks.items.offset(j_4 as isize)).mask_h[k_1 as usize] =
                    oldmask_0.mask_h[*hmap.offset(k_1 as isize) as usize];
                k_1 = k_1.wrapping_add(1);
            }
            let mut k_2: ShapeId = 0 as ShapeId;
            while (k_2 as usize) < (*g).stem_v.length {
                (*(*g).contour_masks.items.offset(j_4 as isize)).mask_v[k_2 as usize] =
                    oldmask_0.mask_v[*vmap.offset(k_2 as isize) as usize];
                k_2 = k_2.wrapping_add(1);
            }
            j_4 = j_4.wrapping_add(1);
        }
    }
    free(hmap as *mut ::core::ffi::c_void);
    hmap = ::core::ptr::null_mut::<ShapeId>();
    free(vmap as *mut ::core::ffi::c_void);
    vmap = ::core::ptr::null_mut::<ShapeId>();
}
unsafe extern "C" fn consolidate_fd_select(
    mut h: *mut FdHandle,
    mut cff: *mut CffTable,
    mut options: *const Options,
    gname: SdsRaw,
) {
    if cff.is_null() || (*cff).fd_array.is_null() || (*cff).fd_array_count == 0 {
        return;
    }
    if (*h).state == HandleState::Index
    {
        if (*h).index as ::core::ffi::c_int >= (*cff).fd_array_count as ::core::ffi::c_int {
            (*h).index = 0 as GlyphId;
        }
        handle_consolidate_to(
            h as *mut Handle,
            (*h).index,
            (**(*cff).fd_array.offset((*h).index as isize)).font_name,
        );
    } else if !(*h).name.is_null() {
        let mut found: bool = false;
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_int) < (*cff).fd_array_count as ::core::ffi::c_int {
            if strcmp(
                (*h).name as *const ::core::ffi::c_char,
                (**(*cff).fd_array.offset(j as isize)).font_name as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                found = true;
                handle_consolidate_to(
                    h as *mut Handle,
                    j as GlyphId,
                    (**(*cff).fd_array.offset(j as isize)).font_name,
                );
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        if !found {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] CID Subfont ",
                    (*h).name,
                    b" is not defined. (in glyph /",
                    gname,
                    b").\n",
                ),
            );
            otfcc_handle_dispose(h as *mut Handle);
        }
    } else if !(*h).name.is_null() {
        otfcc_handle_dispose(h as *mut Handle);
    }
}
pub unsafe extern "C" fn consolidate_glyph(
    mut g: *mut Glyph,
    mut font: *mut Font,
    mut options: *const Options,
) {
    consolidate_glyph_contours(g, options);
    consolidate_glyph_references(g, font, options);
    consolidate_glyph_hints(g, options);
    consolidate_fd_select(&raw mut (*g).fd_select, (*font).cff, options, (*g).name);
}
pub unsafe extern "C" fn get_point_coordinates(
    mut table: *mut GlyfTable,
    mut gr: *mut ComponentReference,
    mut n: ShapeId,
    mut stated: *mut ShapeId,
    mut x: *mut VQ,
    mut y: *mut VQ,
    mut options: *const Options,
) -> bool {
    let mut j: GlyphId = (*gr).glyph.index;
    let mut g: *mut Glyph = *(*table).items.offset(j as isize) as *mut Glyph;
    let mut c: ShapeId = 0 as ShapeId;
    while (c as usize) < (*g).contours.length {
        let mut pj: ShapeId = 0 as ShapeId;
        while (pj as usize) < (*(*g).contours.items.offset(c as isize)).length {
            if *stated as ::core::ffi::c_int == n as ::core::ffi::c_int {
                let mut p: *mut Point = (*(*g).contours.items.offset(c as isize))
                    .items
                    .offset(pj as isize)
                    as *mut Point;
                I_VQ.replace.expect("non-null function pointer")(
                    x,
                    I_VQ.point_linear_tfm.expect("non-null function pointer")(
                        (*gr).x,
                        (*gr).a as Pos,
                        (*p).x,
                        (*gr).b as Pos,
                        (*p).y,
                    ) as VQ,
                );
                I_VQ.replace.expect("non-null function pointer")(
                    y,
                    I_VQ.point_linear_tfm.expect("non-null function pointer")(
                        (*gr).y,
                        (*gr).c as Pos,
                        (*p).x,
                        (*gr).d as Pos,
                        (*p).y,
                    ) as VQ,
                );
                return true;
            }
            *stated = (*stated as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
            pj = pj.wrapping_add(1);
        }
        c = c.wrapping_add(1);
    }
    let mut r: ShapeId = 0 as ShapeId;
    while (r as usize) < (*g).references.length {
        let mut rr: *mut ComponentReference =
            (*g).references.items.offset(r as isize) as *mut ComponentReference;
        consolidate_anchor_ref(table, gr, rr, options);
        let mut ref_0: ComponentReference =
            (
                GLYF_I_COMPONENT_REFERENCE
                    .empty
                    .expect("non-null function pointer"))();
        ref_0.glyph = handle_from_index(
            (*(*g).references.items.offset(r as isize)).glyph.index,
        ) as GlyphHandle;
        ref_0.a = (*gr).a * (*rr).a + (*rr).b * (*gr).c;
        ref_0.b = (*rr).a * (*gr).b + (*rr).b * (*gr).d;
        ref_0.c = (*gr).a * (*rr).c + (*gr).c * (*rr).d;
        ref_0.d = (*gr).b * (*rr).c + (*rr).d * (*gr).d;
        I_VQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.x,
            I_VQ.point_linear_tfm.expect("non-null function pointer")(
                (*rr).x,
                (*rr).a as Pos,
                (*gr).x,
                (*rr).b as Pos,
                (*gr).y,
            ) as VQ,
        );
        I_VQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.y,
            I_VQ.point_linear_tfm.expect("non-null function pointer")(
                (*rr).y,
                (*rr).c as Pos,
                (*gr).x,
                (*rr).d as Pos,
                (*gr).y,
            ) as VQ,
        );
        let mut success: bool =
            get_point_coordinates(table, &raw mut ref_0, n, stated, x, y, options);
        GLYF_I_COMPONENT_REFERENCE
            .dispose
            .expect("non-null function pointer")(&raw mut ref_0);
        if success {
            return true;
        }
        r = r.wrapping_add(1);
    }
    return false;
}
pub unsafe extern "C" fn consolidate_anchor_ref(
    mut table: *mut GlyfTable,
    mut gr: *mut ComponentReference,
    mut rr: *mut ComponentReference,
    mut options: *const Options,
) -> bool {
    if (*rr).is_anchored == RefAnchorStatus::AnchorConsolidated
        || (*rr).is_anchored == RefAnchorStatus::Xy
    {
        return true;
    }
    if (*rr).is_anchored == RefAnchorStatus::AnchorConsolidatingAnchor
        || (*rr).is_anchored == RefAnchorStatus::AnchorConsolidatingXy
    {
        (*(*options).logger)
            .log_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::sdsbuild!(
                sdsempty(),
                b"Found circular reference of out-of-range point reference in anchored reference.",
            ),
        );
        (*rr).is_anchored = RefAnchorStatus::Xy;
        return false;
    }
    if (*rr).is_anchored == RefAnchorStatus::AnchorAnchor
    {
        (*rr).is_anchored = RefAnchorStatus::AnchorConsolidatingAnchor;
    } else {
        (*rr).is_anchored = RefAnchorStatus::AnchorConsolidatingXy;
    }
    let mut inner_x: VQ =
        (I_VQ.neutral.expect("non-null function pointer"))();
    let mut outer_x: VQ =
        (I_VQ.neutral.expect("non-null function pointer"))();
    let mut inner_y: VQ =
        (I_VQ.neutral.expect("non-null function pointer"))();
    let mut outer_y: VQ =
        (I_VQ.neutral.expect("non-null function pointer"))();
    let mut inner_counter: ShapeId = 0 as ShapeId;
    let mut outer_counter: ShapeId = 0 as ShapeId;
    let mut rr1: ComponentReference =
        (
            GLYF_I_COMPONENT_REFERENCE
                .empty
                .expect("non-null function pointer"))();
    rr1.glyph = handle_from_index((*rr).glyph.index)
        as GlyphHandle;
    let mut s1: bool = get_point_coordinates(
        table,
        gr,
        (*rr).outer,
        &raw mut outer_counter,
        &raw mut outer_x,
        &raw mut outer_y,
        options,
    );
    let mut s2: bool = get_point_coordinates(
        table,
        &raw mut rr1,
        (*rr).inner,
        &raw mut inner_counter,
        &raw mut inner_x,
        &raw mut inner_y,
        options,
    );
    if !s1 {
        (*(*options).logger)
            .log_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
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
            .log_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
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
    let mut rrx: VQ = I_VQ.point_linear_tfm.expect("non-null function pointer")(
        outer_x,
        -((*rr).a as Pos),
        inner_x,
        -((*rr).b as Pos),
        inner_y,
    );
    let mut rry: VQ = I_VQ.point_linear_tfm.expect("non-null function pointer")(
        outer_y,
        -((*rr).c as Pos),
        inner_x,
        -((*rr).d as Pos),
        inner_y,
    );
    if (*rr).is_anchored == RefAnchorStatus::AnchorConsolidatingAnchor
    {
        I_VQ.replace.expect("non-null function pointer")(&raw mut (*rr).x, rrx);
        I_VQ.replace.expect("non-null function pointer")(&raw mut (*rr).y, rry);
        (*rr).is_anchored = RefAnchorStatus::AnchorConsolidated;
    } else {
        if fabs(
            I_VQ.get_still.expect("non-null function pointer")((*rr).x) as ::core::ffi::c_double
                - I_VQ.get_still.expect("non-null function pointer")(rrx) as ::core::ffi::c_double,
        ) > 0.5f64
            && fabs(
                I_VQ.get_still.expect("non-null function pointer")((*rr).y) as ::core::ffi::c_double
                    - I_VQ.get_still.expect("non-null function pointer")(rry)
                        as ::core::ffi::c_double,
            ) > 0.5f64
        {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"Anchored reference to ",
                    (*rr).glyph.name,
                    b" does not match its X/Y offset data.",
                ),
            );
        }
        (*rr).is_anchored = RefAnchorStatus::AnchorConsolidated;
        I_VQ.dispose.expect("non-null function pointer")(&raw mut rrx);
        I_VQ.dispose.expect("non-null function pointer")(&raw mut rry);
    }
    GLYF_I_COMPONENT_REFERENCE
        .dispose
        .expect("non-null function pointer")(&raw mut rr1);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut inner_x);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut inner_y);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut outer_x);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut outer_y);
    return false;
}
pub unsafe extern "C" fn consolidate_glyf(
    mut font: *mut Font,
    mut options: *const Options,
) {
    if (*font).glyph_order.is_null() || (*font).glyf.is_null() {
        return;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*(*font).glyf).length {
        if !(*(*(*font).glyf).items.offset(j as isize)).is_null() {
            consolidate_glyph(
                *(*(*font).glyf).items.offset(j as isize) as *mut Glyph,
                font,
                options,
            );
        } else {
            let ref mut fresh6 = *(*(*font).glyf).items.offset(j as isize);
            *fresh6 = otfcc_new_glyf_glyph() as GlyphPtr;
        }
        j = j.wrapping_add(1);
    }
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*(*font).glyf).length {
        let mut g: *mut Glyph = *(*(*font).glyf).items.offset(j_0 as isize) as *mut Glyph;
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), (*g).name),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut gr: ComponentReference =
                (
                    GLYF_I_COMPONENT_REFERENCE
                        .empty
                        .expect("non-null function pointer"))();
            gr.glyph = handle_from_index(j_0)
                as GlyphHandle;
            let mut r: ShapeId = 0 as ShapeId;
            while (r as usize) < (*g).references.length {
                let mut rr: *mut ComponentReference =
                    (*g).references.items.offset(r as isize) as *mut ComponentReference;
                consolidate_anchor_ref((*font).glyf, &raw mut gr, rr, options);
                r = r.wrapping_add(1);
            }
            GLYF_I_COMPONENT_REFERENCE
                .dispose
                .expect("non-null function pointer")(&raw mut gr);
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
        j_0 = j_0.wrapping_add(1);
    }
}
pub unsafe extern "C" fn consolidate_cmap(
    mut font: *mut Font,
    mut options: *const Options,
) {
    if !(*font).glyph_order.is_null() && !(*font).cmap.is_null() {
        let mut item: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
        item = (*(*font).cmap).unicodes;
        while !item.is_null() {
            if !OTFCC_PKG_GLYPH_ORDER
                .consolidate_handle
                .expect("non-null function pointer")(
                (*font).glyph_order, &raw mut (*item).glyph
            ) {
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignored mapping U+",
                        Hex4Upper(((*item).unicode) as u32),
                        b" to non-existent glyph /",
                        (*item).glyph.name,
                        b".\n",
                    ),
                );
                otfcc_handle_dispose(&raw mut (*item).glyph);
            }
            item = (*item).hh.next as *mut CmapEntry;
        }
    }
    if !(*font).glyph_order.is_null() && !(*font).cmap.is_null() {
        let mut item_0: *mut CmapUvsEntry = ::core::ptr::null_mut::<CmapUvsEntry>();
        item_0 = (*(*font).cmap).uvs;
        while !item_0.is_null() {
            if !OTFCC_PKG_GLYPH_ORDER
                .consolidate_handle
                .expect("non-null function pointer")(
                (*font).glyph_order, &raw mut (*item_0).glyph
            ) {
                (*(*options).logger)
                    .log_sds
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
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
                otfcc_handle_dispose(&raw mut (*item_0).glyph);
            }
            item_0 = (*item_0).hh.next as *mut CmapUvsEntry;
        }
    }
}
unsafe extern "C" fn __declare_otl_consolidation(
    mut type_0: LookupType,
    mut fn_0: OtlConsolidationFunction,
    mut fndel: SubtableRemover,
    mut font: *mut Font,
    mut table: *mut OtlTable,
    mut lookup: *mut Lookup,
    mut options: *const Options,
) {
    if lookup.is_null()
        || (*lookup).subtables.length == 0
        || (*lookup).type_0 != type_0
    {
        return;
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), (*lookup).name),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*lookup).subtables.length {
            if (*(*lookup).subtables.items.offset(j as isize)).is_null() {
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
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
                let mut subtable_removed: bool = false;
                subtable_removed = fn_0.expect("non-null function pointer")(
                    font,
                    table,
                    *(*lookup).subtables.items.offset(j as isize) as *mut Subtable,
                    options,
                );
                if subtable_removed {
                    fndel.expect("non-null function pointer")(
                        *(*lookup).subtables.items.offset(j as isize) as *mut Subtable,
                    );
                    let ref mut fresh3 = *(*lookup).subtables.items.offset(j as isize);
                    *fresh3 = ::core::ptr::null_mut::<Subtable>();
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
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
        let mut k: TableId = 0 as TableId;
        let mut j_0: TableId = 0 as TableId;
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
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
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
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_consolidate_lookup(
    mut font: *mut Font,
    mut table: *mut OtlTable,
    mut lookup: *mut Lookup,
    mut options: *const Options,
) {
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_SINGLE,
        Some(
            consolidate_gsub_single
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_GSUB_SINGLE.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_MULTIPLE,
        Some(
            consolidate_gsub_multi
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_GSUB_MULTI.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_ALTERNATE,
        Some(
            consolidate_gsub_alternative
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_GSUB_MULTI.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_LIGATURE,
        Some(
            consolidate_gsub_ligature
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_GSUB_LIGATURE.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_CHAINING,
        Some(
            consolidate_chaining
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_CHAINING.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_REVERSE,
        Some(
            consolidate_gsub_reverse
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GsubReverseSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_GSUB_REVERSE.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_SINGLE,
        Some(
            consolidate_gpos_single
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_GPOS_SINGLE.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_PAIR,
        Some(
            consolidate_gpos_pair
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GposPairSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_GPOS_PAIR.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_CURSIVE,
        Some(
            consolidate_gpos_cursive
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_GPOS_CURSIVE.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_CHAINING,
        Some(
            consolidate_chaining
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_CHAINING.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_MARK_TO_BASE,
        Some(
            consolidate_mark_to_single
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GposMarkToSingleSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_GPOS_MARK_TO_SINGLE.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_MARK_TO_MARK,
        Some(
            consolidate_mark_to_single
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GposMarkToSingleSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_GPOS_MARK_TO_SINGLE.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_MARK_TO_LIGATURE,
        Some(
            consolidate_mark_to_ligature
                as unsafe extern "C" fn(
                    *mut Font,
                    *mut OtlTable,
                    *mut Subtable,
                    *const Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GposMarkToLigatureSubtable) -> ()>,
            SubtableRemover,
        >(I_SUBTABLE_GPOS_MARK_TO_LIGATURE.free),
        font,
        table,
        lookup,
        options,
    );
}
unsafe extern "C" fn lookup_ref_is_not_empty(
    mut r_lut: *const LookupRef,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !r_lut.is_null() && !(*r_lut).is_null() && (**r_lut).subtables.length > 0 as usize;
}
unsafe extern "C" fn feature_ref_is_not_empty(
    mut r_feat: *const FeatureRef,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !r_feat.is_null() && !(*r_feat).is_null() && (**r_feat).lookups.length > 0 as usize;
}
unsafe extern "C" fn lookup_is_not_empty(
    mut r_lut: *const LookupPtr,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !r_lut.is_null() && !(*r_lut).is_null() && (**r_lut).subtables.length > 0 as usize;
}
unsafe extern "C" fn feature_is_not_empty(
    mut r_feat: *const FeaturePtr,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !r_feat.is_null() && !(*r_feat).is_null() && (**r_feat).lookups.length > 0 as usize;
}
unsafe extern "C" fn consolidate_otl_table(
    mut font: *mut Font,
    mut table: *mut OtlTable,
    mut options: *const Options,
) {
    if (*font).glyph_order.is_null() || table.is_null() {
        return;
    }
    loop {
        let mut feat_n: TableId = (*table).features.length as TableId;
        let mut lut_n: TableId = (*table).lookups.length as TableId;
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*table).lookups.length {
            otfcc_consolidate_lookup(
                font,
                table,
                *(*table).lookups.items.offset(j as isize) as *mut Lookup,
                options,
            );
            j = j.wrapping_add(1);
        }
        let mut j_0: TableId = 0 as TableId;
        while (j_0 as usize) < (*table).features.length {
            let mut feature: *mut Feature =
                *(*table).features.items.offset(j_0 as isize) as *mut Feature;
            OTL_I_LOOKUP_REF_LIST
                .filter_env
                .expect("non-null function pointer")(
                &raw mut (*feature).lookups,
                Some(
                    lookup_ref_is_not_empty
                        as unsafe extern "C" fn(
                            *const LookupRef,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                ),
                NULL,
            );
            j_0 = j_0.wrapping_add(1);
        }
        let mut j_1: TableId = 0 as TableId;
        while (j_1 as usize) < (*table).languages.length {
            let mut lang: *mut LanguageSystem =
                *(*table).languages.items.offset(j_1 as isize) as *mut LanguageSystem;
            OTL_I_FEATURE_REF_LIST
                .filter_env
                .expect("non-null function pointer")(
                &raw mut (*lang).features,
                Some(
                    feature_ref_is_not_empty
                        as unsafe extern "C" fn(
                            *const FeatureRef,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                ),
                NULL,
            );
            j_1 = j_1.wrapping_add(1);
        }
        OTL_I_LOOKUP_LIST
            .filter_env
            .expect("non-null function pointer")(
            &raw mut (*table).lookups,
            Some(
                lookup_is_not_empty
                    as unsafe extern "C" fn(*const LookupPtr, *mut ::core::ffi::c_void) -> bool,
            ),
            NULL,
        );
        OTL_I_FEATURE_LIST
            .filter_env
            .expect("non-null function pointer")(
            &raw mut (*table).features,
            Some(
                feature_is_not_empty
                    as unsafe extern "C" fn(
                        *const FeaturePtr,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
            ),
            NULL,
        );
        let mut feat_n1: TableId = (*table).features.length as TableId;
        let mut lut_n1: TableId = (*table).lookups.length as TableId;
        if feat_n1 as ::core::ffi::c_int >= feat_n as ::core::ffi::c_int
            && lut_n1 as ::core::ffi::c_int >= lut_n as ::core::ffi::c_int
        {
            break;
        }
    }
}
unsafe extern "C" fn consolidate_otl(mut font: *mut Font, mut options: *const Options) {
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"GSUB"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        consolidate_otl_table(font, (*font).gsub, options);
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"GPOS"),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        consolidate_otl_table(font, (*font).gpos, options);
        ___loggedstep_v_0 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"GDEF"),
    );
    let mut ___loggedstep_v_1: bool = true;
    while ___loggedstep_v_1 {
        consolidate_gdef(font, (*font).gdef, options);
        ___loggedstep_v_1 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
unsafe extern "C" fn consolidate_colr(mut font: *mut Font, mut options: *const Options) {
    if font.is_null() || (*font).colr.is_null() || (*font).glyph_order.is_null() {
        return;
    }
    let mut consolidated: *mut ColrTable = (
        TABLE_I_COLR.create.expect("non-null function pointer"))();
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*(*font).colr).length {
        let mut mapping: *mut ColrMapping = (*(*font).colr).items.offset(__caryll_index as isize);
        while keep != 0 {
            if !OTFCC_PKG_GLYPH_ORDER
                .consolidate_handle
                .expect("non-null function pointer")(
                (*font).glyph_order, &raw mut (*mapping).glyph
            ) {
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignored missing glyph of /",
                        (*mapping).glyph.name,
                    ),
                );
            } else {
                let mut m: ColrMapping = ColrMapping {
                    glyph: Handle {
                        state: HandleState::Empty,
                        index: 0,
                        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    },
                    layers: ColrLayerList {
                        length: 0,
                        capacity: 0,
                        items: ::core::ptr::null_mut::<ColrLayer>(),
                    },
                };
                otfcc_handle_copy(
                    &raw mut m.glyph,
                    &raw mut (*mapping).glyph,
                );
                COLR_I_LAYER_LIST.init.expect("non-null function pointer")(&raw mut m.layers);
                let mut __caryll_index_0: usize = 0 as usize;
                let mut keep_0: usize = 1 as usize;
                while keep_0 != 0 && __caryll_index_0 < (*mapping).layers.length {
                    let mut layer: *mut ColrLayer =
                        (*mapping).layers.items.offset(__caryll_index_0 as isize);
                    while keep_0 != 0 {
                        if !OTFCC_PKG_GLYPH_ORDER
                            .consolidate_handle
                            .expect("non-null function pointer")(
                            (*font).glyph_order,
                            &raw mut (*layer).glyph,
                        ) {
                            (*(*options).logger)
                                .log_sds
                                .expect("non-null function pointer")(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[Consolidate] Ignored missing glyph of /",
                                    (*layer).glyph.name,
                                ),
                            );
                        } else {
                            let mut layer1: ColrLayer = ColrLayer {
                                glyph: Handle {
                                    state: HandleState::Empty,
                                    index: 0,
                                    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                },
                                palette_index: 0,
                            };
                            COLR_I_LAYER.copy.expect("non-null function pointer")(
                                &raw mut layer1,
                                layer,
                            );
                            COLR_I_LAYER_LIST.push.expect("non-null function pointer")(
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
                    TABLE_I_COLR.push.expect("non-null function pointer")(consolidated, m);
                } else {
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(
                            sdsempty(),
                            b"[Consolidate] COLR decomposition for /",
                            (*mapping).glyph.name,
                            b" is empth",
                        ),
                    );
                    COLR_I_MAPPING.dispose.expect("non-null function pointer")(&raw mut m);
                }
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    TABLE_I_COLR.free.expect("non-null function pointer")((*font).colr);
    (*font).colr = consolidated;
}
unsafe extern "C" fn compare_tsi_entry(
    mut a: *const TsiEntry,
    mut b: *const TsiEntry,
) -> ::core::ffi::c_int {
    if (*a).type_0 as ::core::ffi::c_uint != (*b).type_0 as ::core::ffi::c_uint {
        return ((*a).type_0 as ::core::ffi::c_uint).wrapping_sub((*b).type_0 as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
    return (*a).glyph.index as ::core::ffi::c_int - (*b).glyph.index as ::core::ffi::c_int;
}
unsafe extern "C" fn consolidate_tsi(
    mut font: *mut Font,
    mut _tsi: *mut *mut TsiTable,
    mut options: *const Options,
) {
    let mut tsi: *mut TsiTable = *_tsi;
    if font.is_null() || (*font).glyf.is_null() || tsi.is_null() || (*font).glyph_order.is_null() {
        return;
    }
    let mut consolidated: *mut TsiTable = (
        TABLE_I_TSI.create.expect("non-null function pointer"))();
    let mut gid_entries: *mut SdsRaw = ::core::ptr::null_mut::<SdsRaw>();
    gid_entries = __caryll_allocate_clean(
        (::core::mem::size_of::<SdsRaw>() as usize).wrapping_mul((*(*font).glyf).length),
        448 as ::core::ffi::c_ulong,
    ) as *mut SdsRaw;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*tsi).length {
        let mut entry: *mut TsiEntry = (*tsi).items.offset(__caryll_index as isize);
        while keep != 0 {
            if (*entry).type_0 as ::core::ffi::c_uint
                == TsiEntryType::Glyph as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if OTFCC_PKG_GLYPH_ORDER
                    .consolidate_handle
                    .expect("non-null function pointer")(
                    (*font).glyph_order,
                    &raw mut (*entry).glyph,
                ) {
                    if !(*gid_entries.offset((*entry).glyph.index as isize)).is_null() {
                        sdsfree(*gid_entries.offset((*entry).glyph.index as isize));
                    }
                    let ref mut fresh2 = *gid_entries.offset((*entry).glyph.index as isize);
                    *fresh2 = (*entry).content;
                    (*entry).content = ::core::ptr::null_mut::<::core::ffi::c_char>();
                } else {
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(
                            sdsempty(),
                            b"[Consolidate] Ignored missing glyph of /",
                            (*entry).glyph.name,
                        ),
                    );
                }
            } else {
                let mut e: TsiEntry = TsiEntry {
                    type_0: TsiEntryType::Glyph,
                    glyph: Handle {
                        state: HandleState::Empty,
                        index: 0,
                        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    },
                    content: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                };
                TSI_I_ENTRY.copy.expect("non-null function pointer")(&raw mut e, entry);
                TABLE_I_TSI.push.expect("non-null function pointer")(consolidated, e);
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*(*font).glyf).length {
        let mut e_0: TsiEntry = TsiEntry {
            type_0: TsiEntryType::Glyph,
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            content: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        e_0.type_0 = TsiEntryType::Glyph;
        e_0.glyph =
            handle_from_index(j) as GlyphHandle;
        OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")((*font).glyph_order, &raw mut e_0.glyph);
        e_0.content = if !(*gid_entries.offset(j as isize)).is_null() {
            *gid_entries.offset(j as isize)
        } else {
            sdsempty()
        };
        TABLE_I_TSI.push.expect("non-null function pointer")(consolidated, e_0);
        j = j.wrapping_add(1);
    }
    TABLE_I_TSI.free.expect("non-null function pointer")(tsi);
    free(gid_entries as *mut ::core::ffi::c_void);
    gid_entries = ::core::ptr::null_mut::<SdsRaw>();
    TABLE_I_TSI.sort.expect("non-null function pointer")(
        consolidated,
        Some(
            compare_tsi_entry
                as unsafe extern "C" fn(*const TsiEntry, *const TsiEntry) -> ::core::ffi::c_int,
        ),
    );
    *_tsi = consolidated;
}
pub unsafe extern "C" fn otfcc_consolidate_font(
    mut font: *mut Font,
    mut options: *const Options,
) {
    if !(*font).glyf.is_null() && (*font).glyph_order.is_null() {
        let mut go: *mut GlyphOrder =
            (
                OTFCC_PKG_GLYPH_ORDER
                    .create
                    .expect("non-null function pointer"))();
        let mut j: GlyphId = 0 as GlyphId;
        while (j as usize) < (*(*font).glyf).length {
            let mut name: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut glyf_name: SdsRaw = (**(*(*font).glyf).items.offset(j as isize)).name;
            if !glyf_name.is_null() {
                name = sdsdup(glyf_name);
            } else {
                name = crate::sdsbuild!(sdsempty(), b"$$gid", j as ::core::ffi::c_int);
                let ref mut fresh0 = (**(*(*font).glyf).items.offset(j as isize)).name;
                *fresh0 = sdsdup(name);
            }
            if !OTFCC_PKG_GLYPH_ORDER
                .set_by_name
                .expect("non-null function pointer")(go, name, j)
            {
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
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
                    let mut newname: SdsRaw = crate::sdsbuild!(sdsempty(), name, b"_", suffix);
                    success = OTFCC_PKG_GLYPH_ORDER
                        .set_by_name
                        .expect("non-null function pointer")(
                        go, newname, j
                    );
                    if !success {
                        sdsfree(newname);
                        suffix = suffix.wrapping_add(1 as u32);
                    } else {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
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
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"glyf"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        consolidate_glyf(font, options);
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"cmap"),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        consolidate_cmap(font, options);
        ___loggedstep_v_0 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    if !(*font).glyf.is_null() {
        consolidate_otl(font, options);
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"COLR"),
    );
    let mut ___loggedstep_v_1: bool = true;
    while ___loggedstep_v_1 {
        consolidate_colr(font, options);
        ___loggedstep_v_1 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"TSI_01"),
    );
    let mut ___loggedstep_v_2: bool = true;
    while ___loggedstep_v_2 {
        consolidate_tsi(font, &raw mut (*font).tsi_01, options);
        ___loggedstep_v_2 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"TSI_23"),
    );
    let mut ___loggedstep_v_3: bool = true;
    while ___loggedstep_v_3 {
        consolidate_tsi(font, &raw mut (*font).tsi_23, options);
        ___loggedstep_v_3 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"TSI5"),
    );
    let mut ___loggedstep_v_4: bool = true;
    while ___loggedstep_v_4 {
        fontop_consolidate_class_def(font, (*font).tsi5 as *mut ClassDef, options);
        ___loggedstep_v_4 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
