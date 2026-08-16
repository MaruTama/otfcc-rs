#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{Handle, HandleState, GlyphHandle};

use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};

use crate::font::caryll_font::{Font};




use crate::table::gdef::{CaretValueList, CaretValueRecord, GdefTable, clear_lig_carets};




















use crate::table::otl::classdef::ClassDef;




use crate::consolidate::otl::common::{fontop_consolidate_class_def};
use crate::support::glyph_order::{GlyphOrder, OTFCC_PKG_GLYPH_ORDER};
use crate::table::otl::classdef::{OTL_I_CLASS_DEF};

pub unsafe fn consolidate_gdef(
    mut font: *mut Font,
    mut gdef: *mut GdefTable,
    mut options: *const Options,
) {
    if font.is_null() || (*font).glyph_order.is_none() || gdef.is_null() {
        return;
    }
    let glyph_order: *mut GlyphOrder = (*font)
        .glyph_order
        .as_deref_mut()
        .map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder);
    if !(*gdef).glyph_class_def.is_null() {
        fontop_consolidate_class_def(font, (*gdef).glyph_class_def, options);
        OTL_I_CLASS_DEF.shrink.expect("non-null function pointer")((*gdef).glyph_class_def);
        if (*(*gdef).glyph_class_def).glyphs.is_empty() {
            OTL_I_CLASS_DEF.free.expect("non-null function pointer")((*gdef).glyph_class_def);
            (*gdef).glyph_class_def = ::core::ptr::null_mut::<ClassDef>();
        }
    }
    if !(*gdef).mark_attach_class_def.is_null() {
        fontop_consolidate_class_def(font, (*gdef).mark_attach_class_def, options);
        OTL_I_CLASS_DEF.shrink.expect("non-null function pointer")((*gdef).mark_attach_class_def);
        if (*(*gdef).mark_attach_class_def).glyphs.is_empty() {
            OTL_I_CLASS_DEF.free.expect("non-null function pointer")((*gdef).mark_attach_class_def);
            (*gdef).mark_attach_class_def = ::core::ptr::null_mut::<ClassDef>();
        }
    }
    if !(*gdef).lig_carets.is_empty() {
        let lig_carets: &mut Vec<CaretValueRecord> = &mut (*gdef).lig_carets;
        // Deduplicates by glyph id, first occurrence wins -- a later
        // duplicate is logged as a warning and dropped (its own caret list
        // simply stays behind in `lig_carets` and gets freed when that
        // `Vec` is cleared below, since it was never taken out).
        // `BTreeMap`, not `IndexMap`: the original also did a HASH_SORT by
        // glyph id right before reading entries back out, so the final
        // order is ascending by glyph id, not insertion order. Same shape
        // as `consolidate_gpos_cursive`'s uthash -> `BTreeMap` rewrite
        // (rust/README.md), with a `CaretValueList` (`Vec<CaretValue>`,
        // moved out via `mem::take`) in place of a `Copy` value type.
        //
        // Two behavioral differences from the previous three instances,
        // both confirmed by fully reading this function rather than
        // assumed from the shape: a glyph handle that fails to resolve is
        // silently skipped here, with no "[Consolidate] Ignored missing
        // glyph" warning the other three log; and unlike those, the
        // original here `sdsdup`s the glyph name *unconditionally* before
        // checking for a duplicate, leaking that copy on the duplicate
        // path. That leak is invisible in output bytes (same category as
        // other incidental leaks this migration has let disappear
        // elsewhere) and disappears naturally here too, since this only
        // dups the name when actually inserting -- the warning message
        // reads the name directly off the un-consolidated entry instead.
        let mut seen: std::collections::BTreeMap<i32, (Vec<u8>, CaretValueList)> =
            std::collections::BTreeMap::new();
        let mut j: GlyphId = 0 as GlyphId;
        while (j as usize) < lig_carets.len() {
            if OTFCC_PKG_GLYPH_ORDER
                .consolidate_handle
                .expect("non-null function pointer")(
                glyph_order,
                &raw mut lig_carets[j as usize].glyph,
            ) {
                let gid: i32 = lig_carets[j as usize].glyph.index as i32;
                if seen.contains_key(&gid) {
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(b"[Consolidate] Detected caret value double-mapping about glyph ",
                            &lig_carets[j as usize].glyph.name,
                        ),
                    );
                } else {
                    let gname: Vec<u8> = lig_carets[j as usize].glyph.name.clone();
                    if !gname.is_empty() {
                        let carets: CaretValueList =
                            ::core::mem::take(&mut lig_carets[j as usize].carets);
                        seen.insert(gid, (gname, carets));
                    }
                }
            }
            j = j.wrapping_add(1);
        }
        clear_lig_carets(&raw mut (*gdef).lig_carets);
        for (gid, (gname, carets)) in seen {
            (*gdef).lig_carets.push(CaretValueRecord {
                glyph: Handle {
                    state: HandleState::Consolidated,
                    index: gid as GlyphId,
                    name: gname,
                } as GlyphHandle,
                carets,
            });
        }
    }
}
