#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use crate::support::handle::{handle_from_consolidated, GlyphHandle};

use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphClass, GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::font::caryll_font::{Font};




use crate::table::otl::{Anchor, BaseArray, BaseRecord, LigatureArray, LigatureBaseRecord, MarkArray, MarkRecord, Subtable, GposMarkToLigatureSubtable, GposMarkToSingleSubtable, OtlTable};




use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::table::otl::subtables::gpos_common::{dispose_mark_array};
use crate::table::otl::subtables::gpos_mark_to_ligature::{dispose_lig_array};
use crate::table::otl::subtables::gpos_mark_to_single::{dispose_base_array};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree};

struct MarkHashValue {
    name: SdsRaw,
    mark_class: GlyphClass,
    anchor: Anchor,
}
struct BaseHashValue {
    name: SdsRaw,
    anchors: *mut Anchor,
}
struct LigHashValue {
    name: SdsRaw,
    component_count: GlyphId,
    anchors: *mut *mut Anchor,
}
unsafe extern "C" fn consolidate_mark_array(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut options: *const Options,
    mut mark_array: *mut MarkArray,
    mut class_count: GlyphClass,
) {
    let mut h: BTreeMap<GlyphId, MarkHashValue> = BTreeMap::new();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*mark_array).len() {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (&mut (*mark_array))[k as usize].glyph,
        ) {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Ignored unknown glyph name ",
                    (&(*mark_array))[k as usize].glyph.name,
                    b".",
                ),
            );
        } else {
            let gid: GlyphId = (&(*mark_array))[k as usize].glyph.index;
            let anchor: Anchor = (&(*mark_array))[k as usize].anchor;
            let mark_class: GlyphClass = (&(*mark_array))[k as usize].mark_class;
            match h.entry(gid) {
                Entry::Vacant(v) if anchor.present && mark_class < class_count => {
                    v.insert(MarkHashValue {
                        name: sdsdup((&(*mark_array))[k as usize].glyph.name),
                        mark_class,
                        anchor,
                    });
                }
                _ => {
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
                            b"[Consolidate] Ignored invalid or double-mapping mark definition for /",
                            (&(*mark_array))[k as usize].glyph.name,
                            b".",
                        ),
                    );
                }
            }
        }
        k = k.wrapping_add(1);
    }
    dispose_mark_array(mark_array);
    for (gid, entry) in h.into_iter() {
        (*mark_array).push(
            MarkRecord {
                glyph: handle_from_consolidated(gid, entry.name) as GlyphHandle,
                mark_class: entry.mark_class,
                anchor: entry.anchor,
            },
        );
        sdsfree(entry.name);
    }
}
unsafe extern "C" fn consolidate_base_array(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut options: *const Options,
    mut base_array: *mut BaseArray,
) {
    let mut h: BTreeMap<GlyphId, BaseHashValue> = BTreeMap::new();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*base_array).len() {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (&mut (*base_array))[k as usize].glyph,
        ) {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Ignored unknown glyph name ",
                    (&(*base_array))[k as usize].glyph.name,
                    b".",
                ),
            );
        } else {
            let gid: GlyphId = (&(*base_array))[k as usize].glyph.index;
            match h.entry(gid) {
                Entry::Vacant(v) => {
                    let name: SdsRaw = sdsdup((&(*base_array))[k as usize].glyph.name);
                    let anchors: *mut Anchor = (&(*base_array))[k as usize].anchors;
                    let ref mut fresh0 = (&mut (*base_array))[k as usize].anchors;
                    *fresh0 = ::core::ptr::null_mut::<Anchor>();
                    v.insert(BaseHashValue { name, anchors });
                }
                Entry::Occupied(_) => {
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(
                            sdsempty(),
                            b"[Consolidate] Ignored anchor double-definition for /",
                            (&(*base_array))[k as usize].glyph.name,
                            b".",
                        ),
                    );
                }
            }
        }
        k = k.wrapping_add(1);
    }
    dispose_base_array(base_array);
    for (gid, entry) in h.into_iter() {
        (*base_array).push(
            BaseRecord {
                glyph: handle_from_consolidated(gid, entry.name) as GlyphHandle,
                anchors: entry.anchors,
            },
        );
        sdsfree(entry.name);
    }
}
unsafe extern "C" fn consolidate_lig_array(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut options: *const Options,
    mut lig_array: *mut LigatureArray,
) {
    let mut h: BTreeMap<GlyphId, LigHashValue> = BTreeMap::new();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*lig_array).len() {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (&mut (*lig_array))[k as usize].glyph,
        ) {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Ignored unknown glyph name ",
                    (&(*lig_array))[k as usize].glyph.name,
                    b".",
                ),
            );
        } else {
            let gid: GlyphId = (&(*lig_array))[k as usize].glyph.index;
            match h.entry(gid) {
                Entry::Vacant(v) => {
                    let name: SdsRaw = sdsdup((&(*lig_array))[k as usize].glyph.name);
                    let component_count: GlyphId = (&(*lig_array))[k as usize].component_count;
                    let anchors: *mut *mut Anchor = (&(*lig_array))[k as usize].anchors;
                    let ref mut fresh0 = (&mut (*lig_array))[k as usize].anchors;
                    *fresh0 = ::core::ptr::null_mut::<*mut Anchor>();
                    v.insert(LigHashValue { name, component_count, anchors });
                }
                Entry::Occupied(_) => {
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(
                            sdsempty(),
                            b"[Consolidate] Ignored anchor double-definition for /",
                            (&(*lig_array))[k as usize].glyph.name,
                            b".",
                        ),
                    );
                }
            }
        }
        k = k.wrapping_add(1);
    }
    dispose_lig_array(lig_array);
    for (gid, entry) in h.into_iter() {
        (*lig_array).push(
            LigatureBaseRecord {
                glyph: handle_from_consolidated(gid, entry.name) as GlyphHandle,
                component_count: entry.component_count,
                anchors: entry.anchors,
            },
        );
        sdsfree(entry.name);
    }
}
pub unsafe extern "C" fn consolidate_mark_to_single(
    mut font: *mut Font,
    mut table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut GposMarkToSingleSubtable = &raw mut (*_subtable).gpos_mark_to_single as *mut GposMarkToSingleSubtable;
    consolidate_mark_array(
        font,
        table,
        options,
        &raw mut (*subtable).mark_array,
        (*subtable).class_count,
    );
    consolidate_base_array(font, table, options, &raw mut (*subtable).base_array);
    return (*subtable).mark_array.len() == 0 as usize
        || (*subtable).base_array.len() == 0 as usize;
}
pub unsafe extern "C" fn consolidate_mark_to_ligature(
    mut font: *mut Font,
    mut table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut GposMarkToLigatureSubtable = &raw mut (*_subtable).gpos_mark_to_ligature as *mut GposMarkToLigatureSubtable;
    consolidate_mark_array(
        font,
        table,
        options,
        &raw mut (*subtable).mark_array,
        (*subtable).class_count,
    );
    consolidate_lig_array(font, table, options, &raw mut (*subtable).lig_array);
    return (*subtable).mark_array.len() == 0 as usize
        || (*subtable).lig_array.len() == 0 as usize;
}
