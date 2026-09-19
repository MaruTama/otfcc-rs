use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use crate::support::handle::{GlyphHandle, Handle, HandleState};

use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};

use crate::font::caryll_font::Font;
use crate::support::options::Options;
use crate::support::primitives::{GlyphClass, GlyphId};

use crate::table::otl::{
    Anchor, BaseArray, BaseRecord, LigatureArray, LigatureBaseRecord, MarkArray, MarkRecord,
    Subtable,
};

use crate::support::glyph_order::otfcc_gord_consolidate_handle;
use crate::table::otl::subtables::gpos_common::dispose_mark_array;
use crate::table::otl::subtables::gpos_mark_to_ligature::dispose_lig_array;
use crate::table::otl::subtables::gpos_mark_to_single::dispose_base_array;

#[derive(Debug)]
struct MarkHashValue {
    name: Vec<u8>,
    mark_class: GlyphClass,
    anchor: Anchor,
}
#[derive(Debug)]
struct BaseHashValue {
    name: Vec<u8>,
    anchors: Vec<Anchor>,
}
#[derive(Debug)]
struct LigHashValue {
    name: Vec<u8>,
    component_count: GlyphId,
    anchors: Vec<Vec<Anchor>>,
}
fn consolidate_mark_array(
    font: &Font,
    options: &Options,
    mark_array: &mut MarkArray,
    class_count: GlyphClass,
) {
    let mut h: BTreeMap<GlyphId, MarkHashValue> = BTreeMap::new();
    for rec in mark_array.iter_mut() {
        // Guaranteed `Some`: `consolidate_otl` (and hence this function)
        // only ever runs when `glyf` is present, and `otfcc_consolidate_font`
        // always populates `glyph_order` before that, whenever `glyf` is
        // present.
        if !otfcc_gord_consolidate_handle(font.glyph_order.as_deref().unwrap(), &mut rec.glyph) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"[Consolidate] Ignored unknown glyph name ", &rec.glyph.name, b".",),
            );
        } else {
            let gid: GlyphId = rec.glyph.index;
            let anchor: Anchor = rec.anchor;
            let mark_class: GlyphClass = rec.mark_class;
            match h.entry(gid) {
                Entry::Vacant(v) if anchor.present && mark_class < class_count => {
                    v.insert(MarkHashValue {
                        name: rec.glyph.name.clone(),
                        mark_class,
                        anchor,
                    });
                }
                _ => {
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(b"[Consolidate] Ignored invalid or double-mapping mark definition for /",
                            &rec.glyph.name,
                            b".",
                        ),
                    );
                }
            }
        }
    }
    dispose_mark_array(mark_array);
    // `handle_from_consolidated` (which used to take `entry.name` as an
    // owned `SdsRaw`, dup it internally, and leave the caller to free the
    // original) had no other callers by the time the `sds` sweep reached
    // it and was deleted outright: `entry.name` is already the exact
    // `Vec<u8>` a `Handle` wants, so it moves straight in -- no sds round
    // trip, no `sdsfree` afterward.
    for (gid, entry) in h.into_iter() {
        mark_array.push(MarkRecord {
            glyph: Handle {
                state: HandleState::Consolidated,
                index: gid,
                name: entry.name,
            } as GlyphHandle,
            mark_class: entry.mark_class,
            anchor: entry.anchor,
        });
    }
}
fn consolidate_base_array(
    font: &Font,
    options: &Options,
    base_array: &mut BaseArray,
) {
    let mut h: BTreeMap<GlyphId, BaseHashValue> = BTreeMap::new();
    for rec in base_array.iter_mut() {
        // Guaranteed `Some`: `consolidate_otl` (and hence this function)
        // only ever runs when `glyf` is present, and `otfcc_consolidate_font`
        // always populates `glyph_order` before that, whenever `glyf` is
        // present.
        if !otfcc_gord_consolidate_handle(font.glyph_order.as_deref().unwrap(), &mut rec.glyph) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"[Consolidate] Ignored unknown glyph name ", &rec.glyph.name, b".",),
            );
        } else {
            let gid: GlyphId = rec.glyph.index;
            match h.entry(gid) {
                Entry::Vacant(v) => {
                    let name: Vec<u8> = rec.glyph.name.clone();
                    let anchors: Vec<Anchor> = ::core::mem::take(&mut rec.anchors);
                    v.insert(BaseHashValue { name, anchors });
                }
                Entry::Occupied(_) => {
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(
                            b"[Consolidate] Ignored anchor double-definition for /",
                            &rec.glyph.name,
                            b".",
                        ),
                    );
                }
            }
        }
    }
    dispose_base_array(base_array);
    for (gid, entry) in h.into_iter() {
        base_array.push(BaseRecord {
            glyph: Handle {
                state: HandleState::Consolidated,
                index: gid,
                name: entry.name,
            } as GlyphHandle,
            anchors: entry.anchors,
        });
    }
}
fn consolidate_lig_array(
    font: &Font,
    options: &Options,
    lig_array: &mut LigatureArray,
) {
    let mut h: BTreeMap<GlyphId, LigHashValue> = BTreeMap::new();
    for rec in lig_array.iter_mut() {
        // Guaranteed `Some`: `consolidate_otl` (and hence this function)
        // only ever runs when `glyf` is present, and `otfcc_consolidate_font`
        // always populates `glyph_order` before that, whenever `glyf` is
        // present.
        if !otfcc_gord_consolidate_handle(font.glyph_order.as_deref().unwrap(), &mut rec.glyph) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"[Consolidate] Ignored unknown glyph name ", &rec.glyph.name, b".",),
            );
        } else {
            let gid: GlyphId = rec.glyph.index;
            match h.entry(gid) {
                Entry::Vacant(v) => {
                    let name: Vec<u8> = rec.glyph.name.clone();
                    let component_count: GlyphId = rec.component_count;
                    let anchors: Vec<Vec<Anchor>> = ::core::mem::take(&mut rec.anchors);
                    v.insert(LigHashValue {
                        name,
                        component_count,
                        anchors,
                    });
                }
                Entry::Occupied(_) => {
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(
                            b"[Consolidate] Ignored anchor double-definition for /",
                            &rec.glyph.name,
                            b".",
                        ),
                    );
                }
            }
        }
    }
    dispose_lig_array(lig_array);
    for (gid, entry) in h.into_iter() {
        lig_array.push(LigatureBaseRecord {
            glyph: Handle {
                state: HandleState::Consolidated,
                index: gid,
                name: entry.name,
            } as GlyphHandle,
            component_count: entry.component_count,
            anchors: entry.anchors,
        });
    }
}
pub fn consolidate_mark_to_single(font: &Font, _subtable: &mut Subtable, options: &Options) -> bool {
    let Subtable::GposMarkToSingle(subtable) = _subtable else {
        unreachable!()
    };
    consolidate_mark_array(
        font,
        options,
        &mut subtable.mark_array,
        subtable.class_count,
    );
    consolidate_base_array(font, options, &mut subtable.base_array);
    subtable.mark_array.len() == 0_usize || subtable.base_array.len() == 0_usize
}
pub fn consolidate_mark_to_ligature(font: &Font, _subtable: &mut Subtable, options: &Options) -> bool {
    let Subtable::GposMarkToLigature(subtable) = _subtable else {
        unreachable!()
    };
    consolidate_mark_array(
        font,
        options,
        &mut subtable.mark_array,
        subtable.class_count,
    );
    consolidate_lig_array(font, options, &mut subtable.lig_array);
    subtable.mark_array.len() == 0_usize || subtable.lig_array.len() == 0_usize
}
