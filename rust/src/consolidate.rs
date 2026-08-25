#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod otl;

unsafe extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::handle::{
    FdHandle, GlyphHandle, Handle, HandleState, handle_from_index, handle_name_eq_bytes,
    otfcc_handle_copy, otfcc_handle_dispose,
};

use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};

use crate::font::caryll_font::Font;
use crate::support::NULL;
use crate::support::glyph_order::GlyphOrder;
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, Pos, ShapeId, TableId};
use crate::support::fmt::Hex4Upper;

use crate::table::cff::CffTable;
use crate::table::colr::{ColrLayer, ColrMapping, ColrTable, colr_layer_dup};
use crate::table::gdef::GdefTable;

use crate::table::_tsi::{TsiEntry, TsiEntryType, TsiTable};

use crate::table::glyf::{
    ComponentReference, GlyfTable, Glyph, Point, PostscriptHintMask, PostscriptStemDef,
    RefAnchorStatus,
};

use crate::table::otl::classdef::ClassDef;
use crate::table::otl::{
    Feature, FeatureRef, LanguageSystem, Lookup, LookupRef, LookupType, OTL_TYPE_GPOS_CHAINING,
    OTL_TYPE_GPOS_CURSIVE, OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE,
    OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GSUB_ALTERNATE,
    OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE,
    OTL_TYPE_GSUB_SINGLE, OtlTable, Subtable, SubtablePtr,
};

use crate::consolidate::otl::chaining::consolidate_chaining;
use crate::consolidate::otl::common::fontop_consolidate_class_def;
use crate::consolidate::otl::gdef::consolidate_gdef;
use crate::consolidate::otl::gpos_cursive::consolidate_gpos_cursive;
use crate::consolidate::otl::gpos_pair::consolidate_gpos_pair;
use crate::consolidate::otl::gpos_single::consolidate_gpos_single;
use crate::consolidate::otl::gsub_ligature::consolidate_gsub_ligature;
use crate::consolidate::otl::gsub_multi::{consolidate_gsub_alternative, consolidate_gsub_multi};
use crate::consolidate::otl::gsub_reverse::consolidate_gsub_reverse;
use crate::consolidate::otl::gsub_single::consolidate_gsub_single;
use crate::consolidate::otl::mark::{consolidate_mark_to_ligature, consolidate_mark_to_single};
use crate::support::glyph_order::{otfcc_gord_consolidate_handle, otfcc_set_glyph_order_by_name};
use crate::table::_tsi::tsi_entry_dup;
use crate::table::glyf::{glyf_component_reference_empty, otfcc_new_glyf_glyph};
use crate::table::otl::{
    otl_feature_list_filter_env, otl_feature_ref_list_filter_env, otl_lookup_list_filter_env,
    otl_lookup_ref_list_filter_env,
};
use crate::vf::vq::VQ;
use crate::vf::vq::{vq_get_still, vq_neutral, vq_point_linear_tfm, vq_replace};

pub type OtlConsolidationFunction =
    Option<unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool>;
unsafe fn by_stem_pos(
    a: *const PostscriptStemDef,
    b: *const PostscriptStemDef,
) -> ::core::ffi::c_int {
    if (*a).position == (*b).position {
        return (*a).map as ::core::ffi::c_int - (*b).map as ::core::ffi::c_int;
    } else if (*a).position > (*b).position {
        return 1 as ::core::ffi::c_int;
    } else {
        return -(1 as ::core::ffi::c_int);
    };
}
unsafe fn by_mask_pointindex(
    a: *const PostscriptHintMask,
    b: *const PostscriptHintMask,
) -> ::core::ffi::c_int {
    return if (*a).contours_before as ::core::ffi::c_int
        == (*b).contours_before as ::core::ffi::c_int
    {
        (*a).points_before as ::core::ffi::c_int - (*b).points_before as ::core::ffi::c_int
    } else {
        (*a).contours_before as ::core::ffi::c_int - (*b).contours_before as ::core::ffi::c_int
    };
}
unsafe fn consolidate_glyph_contours(g: *mut Glyph, options: &Options) {
    // `Vec::retain` visits every element once, in order, regardless of
    // whether earlier ones were kept -- so `j` here tracks the same
    // "original index" the C-shaped loop counted, and dropped contours are
    // freed automatically (a `Contour`'s only owned resources are its
    // points' `VQ` `Vec`s; no `Handle` involved, unlike `ReferenceList`).
    let mut j: ShapeId = 0 as ShapeId;
    (*g).contours.retain(|contour| {
        let keep = !contour.is_empty();
        if !keep {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Removed empty contour #",
                    j as ::core::ffi::c_int,
                    b" in glyph ",
                    &(*g).name,
                    b".\n",
                ),
            );
        }
        j = j.wrapping_add(1);
        keep
    });
}
unsafe fn consolidate_glyph_references(
    g: *mut Glyph,
    font: *mut Font,
    options: &Options,
) {
    (*g).references.retain_mut(|r| {
        let ok = otfcc_gord_consolidate_handle(
            (*font)
                .glyph_order
                .as_deref_mut()
                .map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder),
            &raw mut r.glyph,
        );
        if !ok {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Ignored absent glyph component reference /",
                    &r.glyph.name,
                    b" within /",
                    &(*g).name,
                    b".\n",
                ),
            );
            // `retain_mut` drops rejected elements itself -- every
            // `ComponentReference` field auto-drops -- so no explicit
            // dispose call is needed here anymore.
        }
        ok
    });
}
unsafe fn consolidate_glyph_hints(g: *mut Glyph) {
    if !(*g).stem_h.is_empty() {
        let stem_h: &mut Vec<PostscriptStemDef> = &mut (*g).stem_h;
        let mut j: ShapeId = 0 as ShapeId;
        while (j as usize) < stem_h.len() {
            stem_h[j as usize].map = j as u16;
            j = j.wrapping_add(1);
        }
        stem_h.sort_by(|a, b| {
            by_stem_pos(a as *const PostscriptStemDef, b as *const PostscriptStemDef).cmp(&0)
        });
    }
    if !(*g).stem_v.is_empty() {
        let stem_v: &mut Vec<PostscriptStemDef> = &mut (*g).stem_v;
        let mut j_0: ShapeId = 0 as ShapeId;
        while (j_0 as usize) < stem_v.len() {
            stem_v[j_0 as usize].map = j_0 as u16;
            j_0 = j_0.wrapping_add(1);
        }
        stem_v.sort_by(|a, b| {
            by_stem_pos(a as *const PostscriptStemDef, b as *const PostscriptStemDef).cmp(&0)
        });
    }
    let mut hmap: Vec<ShapeId> = vec![0; (*g).stem_h.len()];
    let mut vmap: Vec<ShapeId> = vec![0; (*g).stem_v.len()];
    let stem_h: &Vec<PostscriptStemDef> = &(*g).stem_h;
    let mut j_1: ShapeId = 0 as ShapeId;
    while (j_1 as usize) < stem_h.len() {
        hmap[stem_h[j_1 as usize].map as usize] = j_1;
        j_1 = j_1.wrapping_add(1);
    }
    let stem_v: &Vec<PostscriptStemDef> = &(*g).stem_v;
    let mut j_2: ShapeId = 0 as ShapeId;
    while (j_2 as usize) < stem_v.len() {
        vmap[stem_v[j_2 as usize].map as usize] = j_2;
        j_2 = j_2.wrapping_add(1);
    }
    if !(*g).hint_masks.is_empty() {
        let stem_h_len = (*g).stem_h.len();
        let stem_v_len = (*g).stem_v.len();
        let hint_masks: &mut Vec<PostscriptHintMask> = &mut (*g).hint_masks;
        hint_masks.sort_by(|a, b| {
            by_mask_pointindex(
                a as *const PostscriptHintMask,
                b as *const PostscriptHintMask,
            )
            .cmp(&0)
        });
        let mut j_3: ShapeId = 0 as ShapeId;
        while (j_3 as usize) < hint_masks.len() {
            let oldmask: PostscriptHintMask = hint_masks[j_3 as usize];
            let mut k: ShapeId = 0 as ShapeId;
            while (k as usize) < stem_h_len {
                hint_masks[j_3 as usize].mask_h[k as usize] =
                    oldmask.mask_h[hmap[k as usize] as usize];
                k = k.wrapping_add(1);
            }
            let mut k_0: ShapeId = 0 as ShapeId;
            while (k_0 as usize) < stem_v_len {
                hint_masks[j_3 as usize].mask_v[k_0 as usize] =
                    oldmask.mask_v[vmap[k_0 as usize] as usize];
                k_0 = k_0.wrapping_add(1);
            }
            j_3 = j_3.wrapping_add(1);
        }
    }
    if !(*g).contour_masks.is_empty() {
        let stem_h_len = (*g).stem_h.len();
        let stem_v_len = (*g).stem_v.len();
        let contour_masks: &mut Vec<PostscriptHintMask> = &mut (*g).contour_masks;
        contour_masks.sort_by(|a, b| {
            by_mask_pointindex(
                a as *const PostscriptHintMask,
                b as *const PostscriptHintMask,
            )
            .cmp(&0)
        });
        let mut j_4: ShapeId = 0 as ShapeId;
        while (j_4 as usize) < contour_masks.len() {
            let oldmask_0: PostscriptHintMask = contour_masks[j_4 as usize];
            let mut k_1: ShapeId = 0 as ShapeId;
            while (k_1 as usize) < stem_h_len {
                contour_masks[j_4 as usize].mask_h[k_1 as usize] =
                    oldmask_0.mask_h[hmap[k_1 as usize] as usize];
                k_1 = k_1.wrapping_add(1);
            }
            let mut k_2: ShapeId = 0 as ShapeId;
            while (k_2 as usize) < stem_v_len {
                contour_masks[j_4 as usize].mask_v[k_2 as usize] =
                    oldmask_0.mask_v[vmap[k_2 as usize] as usize];
                k_2 = k_2.wrapping_add(1);
            }
            j_4 = j_4.wrapping_add(1);
        }
    }
}
unsafe fn consolidate_fd_select(
    h: *mut FdHandle,
    cff: *mut CffTable,
    options: &Options,
    gname: &Vec<u8>,
) {
    if cff.is_null() || (*cff).fd_array.is_empty() {
        return;
    }
    let fd_array: &Vec<Box<CffTable>> = &(*cff).fd_array;
    if (*h).state == HandleState::Index {
        if (*h).index as usize >= fd_array.len() {
            (*h).index = 0 as GlyphId;
        }
        let idx = (*h).index;
        *h = Handle {
            state: HandleState::Consolidated,
            index: idx,
            name: fd_array[idx as usize].font_name.clone(),
        } as FdHandle;
    } else if !(*h).name.is_empty() {
        let mut found: bool = false;
        let mut j: TableId = 0 as TableId;
        while (j as usize) < fd_array.len() {
            if handle_name_eq_bytes(&(*h).name, &fd_array[j as usize].font_name) {
                found = true;
                *h = Handle {
                    state: HandleState::Consolidated,
                    index: j as GlyphId,
                    name: fd_array[j as usize].font_name.clone(),
                } as FdHandle;
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        if !found {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] CID Subfont ",
                    &(*h).name,
                    b" is not defined. (in glyph /",
                    gname,
                    b").\n",
                ),
            );
            otfcc_handle_dispose(h as *mut Handle);
        }
    } else if !(*h).name.is_empty() {
        otfcc_handle_dispose(h as *mut Handle);
    }
}
pub unsafe fn consolidate_glyph(g: *mut Glyph, font: *mut Font, options: &Options) {
    consolidate_glyph_contours(g, options);
    consolidate_glyph_references(g, font, options);
    consolidate_glyph_hints(g);
    consolidate_fd_select(
        &raw mut (*g).fd_select,
        (*font)
            .cff
            .as_deref_mut()
            .map_or(::core::ptr::null_mut(), |c| c as *mut CffTable),
        options,
        &(*g).name,
    );
}
pub unsafe fn get_point_coordinates(
    table: *mut GlyfTable,
    gr: *mut ComponentReference,
    n: ShapeId,
    stated: *mut ShapeId,
    x: *mut VQ,
    y: *mut VQ,
    options: &Options,
) -> bool {
    let j: GlyphId = (*gr).glyph.index;
    let g: *mut Glyph = &raw mut **(&mut (*table))[j as usize].as_mut().unwrap();
    let mut c: ShapeId = 0 as ShapeId;
    while (c as usize) < (*g).contours.len() {
        let mut pj: ShapeId = 0 as ShapeId;
        while (pj as usize) < (&(*g).contours)[c as usize].len() {
            if *stated as ::core::ffi::c_int == n as ::core::ffi::c_int {
                let p: *mut Point = &raw mut (&mut (*g).contours)[c as usize][pj as usize];
                vq_replace(
                    x,
                    vq_point_linear_tfm(
                        (*gr).x.clone(),
                        (*gr).a as Pos,
                        (*p).x.clone(),
                        (*gr).b as Pos,
                        (*p).y.clone(),
                    ) as VQ,
                );
                vq_replace(
                    y,
                    vq_point_linear_tfm(
                        (*gr).y.clone(),
                        (*gr).c as Pos,
                        (*p).x.clone(),
                        (*gr).d as Pos,
                        (*p).y.clone(),
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
    while (r as usize) < (*g).references.len() {
        let rr: *mut ComponentReference = &raw mut (&mut (*g).references)[r as usize];
        consolidate_anchor_ref(table, gr, rr, options);
        let mut ref_0: ComponentReference = (glyf_component_reference_empty)();
        ref_0.glyph = handle_from_index((&(*g).references)[r as usize].glyph.index) as GlyphHandle;
        ref_0.a = (*gr).a * (*rr).a + (*rr).b * (*gr).c;
        ref_0.b = (*rr).a * (*gr).b + (*rr).b * (*gr).d;
        ref_0.c = (*gr).a * (*rr).c + (*gr).c * (*rr).d;
        ref_0.d = (*gr).b * (*rr).c + (*rr).d * (*gr).d;
        vq_replace(
            &raw mut ref_0.x,
            vq_point_linear_tfm(
                (*rr).x.clone(),
                (*rr).a as Pos,
                (*gr).x.clone(),
                (*rr).b as Pos,
                (*gr).y.clone(),
            ) as VQ,
        );
        vq_replace(
            &raw mut ref_0.y,
            vq_point_linear_tfm(
                (*rr).y.clone(),
                (*rr).c as Pos,
                (*gr).x.clone(),
                (*rr).d as Pos,
                (*gr).y.clone(),
            ) as VQ,
        );
        let success: bool =
            get_point_coordinates(table, &raw mut ref_0, n, stated, x, y, options);
        // `ref_0` is a plain owned local; every field auto-drops when it
        // goes out of scope here (or at the `return true` below), so no
        // explicit dispose call is needed.
        if success {
            return true;
        }
        r = r.wrapping_add(1);
    }
    return false;
}
pub unsafe fn consolidate_anchor_ref(
    table: *mut GlyfTable,
    gr: *mut ComponentReference,
    rr: *mut ComponentReference,
    options: &Options,
) -> bool {
    if (*rr).is_anchored == RefAnchorStatus::AnchorConsolidated
        || (*rr).is_anchored == RefAnchorStatus::Xy
    {
        return true;
    }
    if (*rr).is_anchored == RefAnchorStatus::AnchorConsolidatingAnchor
        || (*rr).is_anchored == RefAnchorStatus::AnchorConsolidatingXy
    {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"Found circular reference of out-of-range point reference in anchored reference.",
            ),
        );
        (*rr).is_anchored = RefAnchorStatus::Xy;
        return false;
    }
    if (*rr).is_anchored == RefAnchorStatus::AnchorAnchor {
        (*rr).is_anchored = RefAnchorStatus::AnchorConsolidatingAnchor;
    } else {
        (*rr).is_anchored = RefAnchorStatus::AnchorConsolidatingXy;
    }
    let mut inner_x: VQ = (vq_neutral)();
    let mut outer_x: VQ = (vq_neutral)();
    let mut inner_y: VQ = (vq_neutral)();
    let mut outer_y: VQ = (vq_neutral)();
    let mut inner_counter: ShapeId = 0 as ShapeId;
    let mut outer_counter: ShapeId = 0 as ShapeId;
    let mut rr1: ComponentReference = (glyf_component_reference_empty)();
    rr1.glyph = handle_from_index((*rr).glyph.index) as GlyphHandle;
    let s1: bool = get_point_coordinates(
        table,
        gr,
        (*rr).outer,
        &raw mut outer_counter,
        &raw mut outer_x,
        &raw mut outer_y,
        options,
    );
    let s2: bool = get_point_coordinates(
        table,
        &raw mut rr1,
        (*rr).inner,
        &raw mut inner_counter,
        &raw mut inner_x,
        &raw mut inner_y,
        options,
    );
    if !s1 {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"Failed to access point ",
                (*rr).outer as ::core::ffi::c_int,
                b" in outer glyph.",
            ),
        );
    }
    if !s2 {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"Failed to access point ",
                (*rr).outer as ::core::ffi::c_int,
                b" in reference to ",
                &(*rr).glyph.name,
                b".",
            ),
        );
    }
    let rrx: VQ = vq_point_linear_tfm(
        outer_x.clone(),
        -((*rr).a as Pos),
        inner_x.clone(),
        -((*rr).b as Pos),
        inner_y.clone(),
    );
    let rry: VQ = vq_point_linear_tfm(
        outer_y.clone(),
        -((*rr).c as Pos),
        inner_x.clone(),
        -((*rr).d as Pos),
        inner_y.clone(),
    );
    if (*rr).is_anchored == RefAnchorStatus::AnchorConsolidatingAnchor {
        vq_replace(&raw mut (*rr).x, rrx);
        vq_replace(&raw mut (*rr).y, rry);
        (*rr).is_anchored = RefAnchorStatus::AnchorConsolidated;
    } else {
        if fabs(
            vq_get_still((*rr).x.clone()) as ::core::ffi::c_double
                - vq_get_still(rrx.clone()) as ::core::ffi::c_double,
        ) > 0.5f64
            && fabs(
                vq_get_still((*rr).y.clone()) as ::core::ffi::c_double
                    - vq_get_still(rry.clone()) as ::core::ffi::c_double,
            ) > 0.5f64
        {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"Anchored reference to ",
                    &(*rr).glyph.name,
                    b" does not match its X/Y offset data.",
                ),
            );
        }
        (*rr).is_anchored = RefAnchorStatus::AnchorConsolidated;
    }
    // `rr1`/`inner_x`/`inner_y`/`outer_x`/`outer_y` (and, in this branch,
    // `rrx`/`rry`) are all plain owned locals that were never moved out --
    // they auto-drop at the `return false` below, so no explicit dispose
    // calls are needed.
    return false;
}
pub unsafe fn consolidate_glyf(font: *mut Font, options: &Options) {
    if (*font).glyph_order.is_none() || (*font).glyf.is_none() {
        return;
    }
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*glyf).len() {
        if (&(*glyf))[j as usize].is_some() {
            consolidate_glyph(
                &raw mut **(&mut (*glyf))[j as usize].as_mut().unwrap(),
                font,
                options,
            );
        } else {
            (&mut (*glyf))[j as usize] = Some(otfcc_new_glyf_glyph());
        }
        j = j.wrapping_add(1);
    }
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*glyf).len() {
        let g: *mut Glyph = &raw mut **(&mut (*glyf))[j_0 as usize].as_mut().unwrap();
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(&(*g).name),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut gr: ComponentReference = (glyf_component_reference_empty)();
            gr.glyph = handle_from_index(j_0) as GlyphHandle;
            let mut r: ShapeId = 0 as ShapeId;
            while (r as usize) < (*g).references.len() {
                let rr: *mut ComponentReference = &raw mut (&mut (*g).references)[r as usize];
                consolidate_anchor_ref(glyf, &raw mut gr, rr, options);
                r = r.wrapping_add(1);
            }
            // `gr` is a plain owned local; every field auto-drops when it
            // goes out of scope at the end of this block, so no explicit
            // dispose call is needed.
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
        j_0 = j_0.wrapping_add(1);
    }
}
pub unsafe fn consolidate_cmap(font: *mut Font, options: &Options) {
    let glyph_order: *mut GlyphOrder = (*font)
        .glyph_order
        .as_deref_mut()
        .map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder);
    if !glyph_order.is_null() && (*font).cmap.is_some() {
        // A failed resolution disposes the entry's `Handle` in place
        // (leaving it in the map with an empty name) rather than
        // removing the entry -- `dump_cmap`'s "skip if name is null"
        // check is what actually hides it later.
        for (&unicode, glyph) in (*font).cmap.as_mut().unwrap().unicodes.iter_mut() {
            if !otfcc_gord_consolidate_handle(glyph_order, glyph as *mut GlyphHandle) {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"[Consolidate] Ignored mapping U+",
                        Hex4Upper(unicode as u32),
                        b" to non-existent glyph /",
                        &glyph.name,
                        b".\n",
                    ),
                );
                otfcc_handle_dispose(glyph as *mut GlyphHandle);
            }
        }
    }
    if !glyph_order.is_null() && (*font).cmap.is_some() {
        for (key, glyph) in (*font).cmap.as_mut().unwrap().uvs.iter_mut() {
            if !otfcc_gord_consolidate_handle(glyph_order, glyph as *mut GlyphHandle) {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"[Consolidate] Ignored UVS mapping [U+",
                        Hex4Upper(key.unicode as u32),
                        b" U+",
                        Hex4Upper(key.selector as u32),
                        b"] to non-existent glyph /",
                        &glyph.name,
                        b".\n",
                    ),
                );
                otfcc_handle_dispose(glyph as *mut GlyphHandle);
            }
        }
    }
}
unsafe fn __declare_otl_consolidation(
    type_0: LookupType,
    fn_0: OtlConsolidationFunction,
    font: *mut Font,
    table: *mut OtlTable,
    lookup: *mut Lookup,
    options: &Options,
) {
    if lookup.is_null() || (*lookup).subtables.is_empty() || (*lookup).type_0 != type_0 {
        return;
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(&(*lookup).name),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*lookup).subtables.len() {
            if (&(*lookup).subtables)[j as usize].is_none() {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"[Consolidate] Ignored empty subtable ",
                        j as ::core::ffi::c_int,
                        b" of lookup ",
                        &(*lookup).name,
                        b".\n",
                    ),
                );
            } else {
                let mut subtable_removed: bool = false;
                let sub_ptr: SubtablePtr = (&mut (*lookup).subtables)[j as usize]
                    .as_deref_mut()
                    .unwrap() as *mut Subtable;
                subtable_removed =
                    fn_0.expect("non-null function pointer")(font, table, sub_ptr, options);
                if subtable_removed {
                    // Was a `fndel: SubtableRemover` parameter, one
                    // `LookupType`-keyed function pointer per call site
                    // below, each `transmute`d from `*mut ConcreteType` to
                    // `*mut Subtable` -- sound only because `Subtable` used
                    // to be a union with no discriminant to misinterpret.
                    // Now that it is an enum, `Subtable`'s own `Drop` does
                    // this dispatch, self-describing off the enum's tag, so
                    // setting the slot to `None` (dropping the `Box` in
                    // place) is all that is needed -- no per-type function
                    // pointer, no separate explicit `Box::from_raw`.
                    (&mut (*lookup).subtables)[j as usize] = None;
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(
                            b"[Consolidate] Ignored empty subtable ",
                            j as ::core::ffi::c_int,
                            b" of lookup ",
                            &(*lookup).name,
                            b".\n",
                        ),
                    );
                }
            }
            j = j.wrapping_add(1);
        }
        let mut k: TableId = 0 as TableId;
        let mut j_0: TableId = 0 as TableId;
        while (j_0 as usize) < (*lookup).subtables.len() {
            if (&(*lookup).subtables)[j_0 as usize].is_some() {
                let fresh4 = k;
                k = k.wrapping_add(1);
                // `.take()` moves the `Box` out of slot `j_0`, leaving `None`
                // behind there -- required now that elements are owned
                // `Box`es rather than freely-aliasable raw pointers: a plain
                // copy-assign would leave two slots owning the same `Box`,
                // and `Vec::truncate` below (unlike the old raw-pointer
                // `Vec`, which had nothing to drop) runs `Drop` on every
                // truncated-away element, which would double-free it.
                (&mut (*lookup).subtables)[fresh4 as usize] =
                    (&mut (*lookup).subtables)[j_0 as usize].take();
            }
            j_0 = j_0.wrapping_add(1);
        }
        (*lookup).subtables.truncate(k as usize);
        if k == 0 {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Lookup ",
                    &(*lookup).name,
                    b" is empty and will be removed.\n",
                ),
            );
        }
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn otfcc_consolidate_lookup(
    font: *mut Font,
    table: *mut OtlTable,
    lookup: *mut Lookup,
    options: &Options,
) {
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_SINGLE,
        Some(
            consolidate_gsub_single
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_MULTIPLE,
        Some(
            consolidate_gsub_multi
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_ALTERNATE,
        Some(
            consolidate_gsub_alternative
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_LIGATURE,
        Some(
            consolidate_gsub_ligature
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_CHAINING,
        Some(
            consolidate_chaining
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GSUB_REVERSE,
        Some(
            consolidate_gsub_reverse
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_SINGLE,
        Some(
            consolidate_gpos_single
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_PAIR,
        Some(
            consolidate_gpos_pair
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_CURSIVE,
        Some(
            consolidate_gpos_cursive
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_CHAINING,
        Some(
            consolidate_chaining
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_MARK_TO_BASE,
        Some(
            consolidate_mark_to_single
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_MARK_TO_MARK,
        Some(
            consolidate_mark_to_single
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        OTL_TYPE_GPOS_MARK_TO_LIGATURE,
        Some(
            consolidate_mark_to_ligature
                as unsafe fn(*mut Font, *mut OtlTable, *mut Subtable, &Options) -> bool,
        ),
        font,
        table,
        lookup,
        options,
    );
}
unsafe extern "C" fn lookup_ref_is_not_empty(
    r_lut: *const LookupRef,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !r_lut.is_null() && !(*r_lut).is_null() && !(**r_lut).subtables.is_empty();
}
unsafe extern "C" fn feature_ref_is_not_empty(
    r_feat: *const FeatureRef,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !r_feat.is_null() && !(*r_feat).is_null() && !(**r_feat).lookups.is_empty();
}
unsafe extern "C" fn lookup_is_not_empty(
    r_lut: *const Lookup,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !r_lut.is_null() && !(*r_lut).subtables.is_empty();
}
unsafe extern "C" fn feature_is_not_empty(
    r_feat: *const Feature,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !r_feat.is_null() && !(*r_feat).lookups.is_empty();
}
unsafe fn consolidate_otl_table(
    font: *mut Font,
    table: *mut OtlTable,
    options: &Options,
) {
    if (*font).glyph_order.is_none() || table.is_null() {
        return;
    }
    loop {
        let feat_n: TableId = (*table).features.len() as TableId;
        let lut_n: TableId = (*table).lookups.len() as TableId;
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*table).lookups.len() {
            otfcc_consolidate_lookup(
                font,
                table,
                &raw mut *(&mut (*table).lookups)[j as usize],
                options,
            );
            j = j.wrapping_add(1);
        }
        let mut j_0: TableId = 0 as TableId;
        while (j_0 as usize) < (*table).features.len() {
            let feature: *mut Feature = &raw mut *(&mut (*table).features)[j_0 as usize];
            otl_lookup_ref_list_filter_env(
                &raw mut (*feature).lookups,
                Some(
                    lookup_ref_is_not_empty
                        as unsafe extern "C" fn(*const LookupRef, *mut ::core::ffi::c_void) -> bool,
                ),
                NULL,
            );
            j_0 = j_0.wrapping_add(1);
        }
        let mut j_1: TableId = 0 as TableId;
        while (j_1 as usize) < (*table).languages.len() {
            let lang: *mut LanguageSystem = &raw mut *(&mut (*table).languages)[j_1 as usize];
            otl_feature_ref_list_filter_env(
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
        otl_lookup_list_filter_env(
            &raw mut (*table).lookups,
            Some(
                lookup_is_not_empty
                    as unsafe extern "C" fn(*const Lookup, *mut ::core::ffi::c_void) -> bool,
            ),
            NULL,
        );
        otl_feature_list_filter_env(
            &raw mut (*table).features,
            Some(
                feature_is_not_empty
                    as unsafe extern "C" fn(*const Feature, *mut ::core::ffi::c_void) -> bool,
            ),
            NULL,
        );
        let feat_n1: TableId = (*table).features.len() as TableId;
        let lut_n1: TableId = (*table).lookups.len() as TableId;
        if feat_n1 as ::core::ffi::c_int >= feat_n as ::core::ffi::c_int
            && lut_n1 as ::core::ffi::c_int >= lut_n as ::core::ffi::c_int
        {
            break;
        }
    }
}
unsafe fn consolidate_otl(font: *mut Font, options: &Options) {
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"GSUB"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        consolidate_otl_table(
            font,
            (*font)
                .gsub
                .as_deref_mut()
                .map_or(::core::ptr::null_mut(), |t| t as *mut OtlTable),
            options,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"GPOS"),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        consolidate_otl_table(
            font,
            (*font)
                .gpos
                .as_deref_mut()
                .map_or(::core::ptr::null_mut(), |t| t as *mut OtlTable),
            options,
        );
        ___loggedstep_v_0 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"GDEF"),
    );
    let mut ___loggedstep_v_1: bool = true;
    while ___loggedstep_v_1 {
        consolidate_gdef(
            font,
            (*font)
                .gdef
                .as_deref_mut()
                .map_or(::core::ptr::null_mut(), |g| g as *mut GdefTable),
            options,
        );
        ___loggedstep_v_1 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
unsafe fn consolidate_colr(font: *mut Font, options: &Options) {
    if font.is_null() || (*font).colr.is_none() || (*font).glyph_order.is_none() {
        return;
    }
    let glyph_order: *mut GlyphOrder = (*font)
        .glyph_order
        .as_deref_mut()
        .map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder);
    let mut consolidated: ColrTable = Vec::new();
    let source: &mut Vec<ColrMapping> = (*font).colr.as_mut().unwrap();
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < source.len() {
        let mapping: &mut ColrMapping = &mut source[__caryll_index];
        while keep != 0 {
            if !otfcc_gord_consolidate_handle(glyph_order, &raw mut mapping.glyph) {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"[Consolidate] Ignored missing glyph of /",
                        &mapping.glyph.name,
                    ),
                );
            } else {
                let mut m: ColrMapping = ColrMapping {
                    glyph: Handle {
                        state: HandleState::Empty,
                        index: 0,
                        name: Vec::new(),
                    },
                    layers: Vec::new(),
                };
                otfcc_handle_copy(&raw mut m.glyph, &raw mut mapping.glyph);
                let mut __caryll_index_0: usize = 0 as usize;
                let mut keep_0: usize = 1 as usize;
                while keep_0 != 0 && __caryll_index_0 < mapping.layers.len() {
                    let layer: &mut ColrLayer = &mut mapping.layers[__caryll_index_0];
                    while keep_0 != 0 {
                        if !otfcc_gord_consolidate_handle(glyph_order, &raw mut layer.glyph) {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[Consolidate] Ignored missing glyph of /",
                                    &layer.glyph.name,
                                ),
                            );
                        } else {
                            m.layers.push(colr_layer_dup(layer));
                        }
                        keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    }
                    keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    __caryll_index_0 = __caryll_index_0.wrapping_add(1);
                }
                if mapping.layers.len() != 0 {
                    consolidated.push(m);
                } else {
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(
                            b"[Consolidate] COLR decomposition for /",
                            &mapping.glyph.name,
                            b" is empth",
                        ),
                    );
                    // `m` is dropped here (its `Handle` and `layers: Vec<ColrLayer>`
                    // freed by their own compiler-generated drop glue) rather than
                    // pushed into `consolidated` -- no manual dispose call needed.
                }
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    (*font).colr = Some(consolidated);
}
unsafe fn consolidate_tsi(
    font: *mut Font,
    mut _tsi: *mut Option<TsiTable>,
    options: &Options,
) {
    if font.is_null()
        || (*font).glyf.is_none()
        || (*_tsi).is_none()
        || (*font).glyph_order.is_none()
    {
        return;
    }
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    let glyph_order: *mut GlyphOrder = (*font)
        .glyph_order
        .as_deref_mut()
        .map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder);
    let mut consolidated: TsiTable = Vec::new();
    // `Option<Vec<u8>>` per slot preserves the old null/non-null
    // distinction (`None` = "no entry yet for this GID", `Some` = has
    // content, even if empty) that the raw `*mut SdsRaw` array's
    // `is_null()` checks relied on -- a plain assignment below correctly
    // drops whatever was there before, so the old explicit
    // free-before-overwrite is now implicit.
    let mut gid_entries: Vec<Option<Vec<u8>>> = vec![None; (*glyf).len()];
    let entries: &mut Vec<TsiEntry> = (*_tsi).as_mut().unwrap();
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < entries.len() {
        let entry: *mut TsiEntry = &mut entries[__caryll_index];
        while keep != 0 {
            if (*entry).type_0 as ::core::ffi::c_uint
                == TsiEntryType::Glyph as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if otfcc_gord_consolidate_handle(glyph_order, &raw mut (*entry).glyph) {
                    gid_entries[(*entry).glyph.index as usize] =
                        Some(::core::mem::take(&mut (*entry).content));
                } else {
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(
                            b"[Consolidate] Ignored missing glyph of /",
                            &(*entry).glyph.name,
                        ),
                    );
                }
            } else {
                let e: TsiEntry = tsi_entry_dup(&*entry);
                consolidated.push(e);
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*glyf).len() {
        let mut e_0: TsiEntry = TsiEntry {
            type_0: TsiEntryType::Glyph,
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: Vec::new(),
            },
            content: Vec::new(),
        };
        e_0.type_0 = TsiEntryType::Glyph;
        e_0.glyph = handle_from_index(j) as GlyphHandle;
        otfcc_gord_consolidate_handle(glyph_order, &raw mut e_0.glyph);
        e_0.content = gid_entries[j as usize].take().unwrap_or_default();
        consolidated.push(e_0);
        j = j.wrapping_add(1);
    }
    consolidated.sort_by(|a, b| {
        (a.type_0 as u32)
            .cmp(&(b.type_0 as u32))
            .then(a.glyph.index.cmp(&b.glyph.index))
    });
    // Old `tsi` (the previous `*_tsi`) drops naturally here, when this
    // assignment overwrites it -- no explicit `table_tsi_free` needed.
    *_tsi = Some(consolidated);
}
pub unsafe fn otfcc_consolidate_font(font: *mut Font, options: &Options) {
    let glyf: *mut GlyfTable = (*font)
        .glyf
        .as_mut()
        .map_or(::core::ptr::null_mut(), |g| g as *mut GlyfTable);
    if !glyf.is_null() && (*font).glyph_order.is_none() {
        // Built directly via `Box::new`, not `OTFCC_PKG_GLYPH_ORDER.create`
        // (`malloc`) + `Box::from_raw` -- `Box::from_raw` requires the
        // pointer to have come from Rust's global allocator, which a bare
        // libc `malloc` is not guaranteed to match. `go` stays a raw-pointer
        // alias into `go_box` for the rest of this block (unchanged from
        // here down), matching the `GaspTable`/`CmapTable` "accumulator is
        // `Option<Box<X>>`/`Box<X>` from the start" idiom.
        let mut go_box: Box<GlyphOrder> = Box::new(GlyphOrder {
            entries: Vec::new(),
            by_gid: ::std::collections::BTreeMap::new(),
            by_name: ::std::collections::HashMap::new(),
        });
        let go: *mut GlyphOrder = go_box.as_mut() as *mut GlyphOrder;
        let mut j: GlyphId = 0 as GlyphId;
        while (j as usize) < (*glyf).len() {
            let name: Vec<u8>;
            let glyf_name_empty: bool = (&(*glyf))[j as usize].as_deref().unwrap().name.is_empty();
            if !glyf_name_empty {
                name = (&(*glyf))[j as usize].as_deref().unwrap().name.clone();
            } else {
                name = crate::bytesbuild!(b"$$gid", j as ::core::ffi::c_int);
                let ref mut fresh0 = (&mut (*glyf))[j as usize].as_mut().unwrap().name;
                *fresh0 = name.clone();
            }
            // `.clone()`, not a move: `otfcc_set_glyph_order_by_name` always
            // consumes its own copy (no ownership contract to track any
            // more -- see its doc comment), but `name` is still needed
            // below regardless of whether this call succeeds or fails, for
            // the log message and/or the retry loop.
            if !otfcc_set_glyph_order_by_name(go, name.clone(), j) {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(b"[Consolidate] Glyph name ", &name, b" is already in use.",),
                );
                let mut suffix: u32 = 2 as u32;
                let mut success: bool = false;
                loop {
                    let newname: Vec<u8> = crate::bytesbuild!(&name, b"_", suffix);
                    success = otfcc_set_glyph_order_by_name(go, newname.clone(), j);
                    if !success {
                        suffix = suffix.wrapping_add(1 as u32);
                    } else {
                        logger_log_sds(
                            &mut *options.logger.borrow_mut(),
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(
                                b"[Consolidate] Glyph ",
                                &name,
                                b" is renamed into ",
                                &newname,
                                b".",
                            ),
                        );
                        let ref mut fresh1 = (&mut (*glyf))[j as usize].as_mut().unwrap().name;
                        *fresh1 = newname;
                    }
                    if success {
                        break;
                    }
                }
            }
            j = j.wrapping_add(1);
        }
        (*font).glyph_order = Some(go_box);
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"glyf"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        consolidate_glyf(font, options);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"cmap"),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        consolidate_cmap(font, options);
        ___loggedstep_v_0 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    if !glyf.is_null() {
        consolidate_otl(font, options);
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"COLR"),
    );
    let mut ___loggedstep_v_1: bool = true;
    while ___loggedstep_v_1 {
        consolidate_colr(font, options);
        ___loggedstep_v_1 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"TSI_01"),
    );
    let mut ___loggedstep_v_2: bool = true;
    while ___loggedstep_v_2 {
        consolidate_tsi(font, &raw mut (*font).tsi_01, options);
        ___loggedstep_v_2 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"TSI_23"),
    );
    let mut ___loggedstep_v_3: bool = true;
    while ___loggedstep_v_3 {
        consolidate_tsi(font, &raw mut (*font).tsi_23, options);
        ___loggedstep_v_3 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"TSI5"),
    );
    let mut ___loggedstep_v_4: bool = true;
    while ___loggedstep_v_4 {
        fontop_consolidate_class_def(
            font,
            (*font)
                .tsi5
                .as_deref_mut()
                .map_or(::core::ptr::null_mut(), |c| c as *mut ClassDef),
            options,
        );
        ___loggedstep_v_4 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
