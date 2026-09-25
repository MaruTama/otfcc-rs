use crate::support::handle::{
    GlyphHandle, Handle, HandleState, handle_from_name,
};
use crate::support::parsed_json::ParsedValue;
use crate::table::otl::coverage::{Coverage, push_to_coverage, read_coverage};

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::bk::bkgraph::bk_build_block;
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
use crate::support::font_reader::FontReader;
use crate::support::options::Options;
use crate::support::primitives::{GlyphClass, GlyphId};
use crate::table::otl::coverage::build_coverage;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::subtables::gpos_common::{
    bk_from_anchor, otl_anchor_absent, otl_parse_anchor, otl_parse_mark_array, otl_read_anchor,
    otl_read_mark_array,
};
use crate::table::otl::{
    Anchor, GposMarkToLigatureSubtable, LigatureArray, LigatureBaseRecord, MarkArray, Subtable,
};
use crate::vendor::json::JsonType;
// `LigatureBaseRecord.anchors` is a plain `Vec<Vec<Anchor>>` now and
// `glyph: GlyphHandle` already has its own `Drop`, so a `LigatureArray`
// (`Vec<LigatureBaseRecord>`) fully self-drops -- clearing it (still needed:
// `consolidate/otl/mark.rs`'s dedup pass clears an in-place array
// mid-function, not just at end of scope) is exactly `*arr = Vec::new()`.
pub(crate) fn dispose_lig_array(arr: &mut LigatureArray) {
    *arr = Vec::new();
}
// `2 * component_count * class_count` (the LigatureAttach's byte-length
// guard) is the same overflow-defeats-guard shape as
// `gpos_mark_to_single.rs`'s `bases.len() * class_count`, but sharper
// here: *both* factors are independently unbounded `u16` fields read
// straight from the file (unlike `bases.len()`, which is at least bounded
// by the actual glyph count), so the product can reach 65535*65535*2 --
// ~8.6 billion -- from a much smaller, more plausible crafted input.
// Fixed the same way: `checked_mul` on `usize` before `require_room`.
pub fn otl_read_gpos_mark_to_ligature(
    data: &[u8],
    offset: u32,
    _max_glyphs: GlyphId,
) -> Option<Subtable> {
    let mut subtable = GposMarkToLigatureSubtable {
        class_count: 0,
        mark_array: Vec::new(),
        lig_array: Vec::new(),
    };

    'parse: {
        let mut header = match FontReader::new(data).at(offset as usize) {
            Ok(r) => r,
            Err(_) => break 'parse,
        };
        if header.skip(2).is_err() {
            break 'parse; // format, unused
        }
        let Ok(marks_rel) = header.u16() else {
            break 'parse;
        };
        let Ok(bases_rel) = header.u16() else {
            break 'parse;
        };
        let Ok(class_count) = header.u16() else {
            break 'parse;
        };
        let Ok(mark_array_rel) = header.u16() else {
            break 'parse;
        };
        let Ok(lig_array_rel) = header.u16() else {
            break 'parse;
        };

        let marks: Coverage = read_coverage(data, offset.wrapping_add(marks_rel as u32));
        let bases: Coverage = read_coverage(data, offset.wrapping_add(bases_rel as u32));
        if marks.is_empty() || bases.is_empty() {
            break 'parse;
        }

        subtable.class_count = class_count as GlyphClass;
        let mark_array_offset = offset.wrapping_add(mark_array_rel as u32);
        otl_read_mark_array(&mut subtable.mark_array, &marks, data, mark_array_offset);

        let lig_array_offset = offset.wrapping_add(lig_array_rel as u32);
        let Ok(mut lr) = FontReader::new(data).at(lig_array_offset as usize) else {
            break 'parse;
        };
        let Ok(lig_count) = lr.u16() else {
            break 'parse;
        };
        if lig_count as usize != bases.len() {
            break 'parse;
        }
        if lr.require_room(lig_count as usize, 2).is_err() {
            break 'parse;
        }
        let mut lig_attach_offsets = Vec::with_capacity(lig_count as usize);
        for _ in 0..lig_count {
            lig_attach_offsets.push(lig_array_offset.wrapping_add(lr.u16().unwrap() as u32));
        }

        for (j, &lig_attach_offset) in lig_attach_offsets.iter().enumerate() {
            let Ok(mut ar) = FontReader::new(data).at(lig_attach_offset as usize) else {
                break 'parse;
            };
            let Ok(component_count) = ar.u16() else {
                break 'parse;
            };
            let Some(total_anchors) = (component_count as usize).checked_mul(class_count as usize)
            else {
                break 'parse;
            };
            if ar.require_room(total_anchors, 2).is_err() {
                break 'parse;
            }
            let mut lig = LigatureBaseRecord {
                glyph: bases[j].clone(),
                component_count,
                anchors: Vec::with_capacity(component_count as usize),
            };
            for _ in 0..component_count {
                let mut component: Vec<Anchor> = Vec::with_capacity(class_count as usize);
                for _ in 0..class_count {
                    let anchor_rel = ar.u16().unwrap();
                    if anchor_rel != 0 {
                        component.push(otl_read_anchor(
                            data,
                            lig_attach_offset.wrapping_add(anchor_rel as u32),
                        ));
                    } else {
                        component.push(otl_anchor_absent());
                    }
                }
                lig.anchors.push(component);
            }
            subtable.lig_array.push(lig);
        }

        return Some(Subtable::GposMarkToLigature(subtable));
    }
    None
}
pub fn otl_gpos_dump_mark_to_ligature(st: &Subtable) -> BuiltValue {
    let Subtable::GposMarkToLigature(subtable) = st else {
        unreachable!()
    };
    let mut _subtable = BuiltValue::new_object(3);
    let mut _marks = BuiltValue::new_object(subtable.mark_array.len());
    let mut _bases = BuiltValue::new_object(subtable.lig_array.len());
    for mark in subtable.mark_array.iter() {
        let mut _mark = BuiltValue::new_object(3);
        let mark_class_name: Vec<u8> = crate::bytesbuild!(b"ac_", mark.mark_class as i32,);
        _mark.push_field(b"class", BuiltValue::str_truncated_at_nul(&mark_class_name));
        _mark.push_field(b"x", BuiltValue::Int(mark.anchor.x as i64));
        _mark.push_field(b"y", BuiltValue::Int(mark.anchor.y as i64));
        _marks.push_field_bytes_key(&mark.glyph.name, _mark.preserialize());
    }
    for base in subtable.lig_array.iter() {
        let base_anchors: &Vec<Vec<Anchor>> = &base.anchors;
        let mut _base = BuiltValue::new_array(base.component_count as usize);
        // Bounded by `base.component_count`, not assumed equal to
        // `base_anchors.len()` (both the read and JSON-parse paths always
        // build exactly `component_count` entries, but this function has
        // no reason to rely on that instead of the field itself).
        for component_anchors in base_anchors.iter().take(base.component_count as usize) {
            let mut _bk = BuiltValue::new_object(subtable.class_count as usize);
            // `m`'s own value feeds the output key (`ac_<class id>`), so
            // this needs `.enumerate()`, not a bare `.iter()`.
            for (m, anchor) in component_anchors
                .iter()
                .enumerate()
                .take(subtable.class_count as usize)
            {
                if anchor.present {
                    let mut _anchor = BuiltValue::new_object(2);
                    _anchor.push_field(b"x", BuiltValue::Int(anchor.x as i64));
                    _anchor.push_field(b"y", BuiltValue::Int(anchor.y as i64));
                    let mark_class_name_0: Vec<u8> = crate::bytesbuild!(b"ac_", m as i32);
                    _bk.push_field_bytes_key(&mark_class_name_0, _anchor);
                }
            }
            _base.push_item(_bk);
        }
        _bases.push_field_bytes_key(&base.glyph.name, _base.preserialize());
    }
    _subtable.push_field(b"classCount", BuiltValue::Int(subtable.class_count as i64));
    _subtable.push_field(b"marks", _marks);
    _subtable.push_field(b"bases", _bases);
    _subtable
}
fn parse_bases(
    bases: Option<&ParsedValue>,
    lig_array: &mut LigatureArray,
    h: &std::collections::BTreeMap<Vec<u8>, GlyphClass>,
    options: &Options,
) {
    let class_count: GlyphClass = h.len() as GlyphClass;
    let Some(fields) = bases.and_then(ParsedValue::as_object) else {
        return;
    };
    for (key, base_record) in fields {
        let gname = &key[..key.len() - 1];
        let mut lig: LigatureBaseRecord = LigatureBaseRecord {
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: Vec::new(),
            },
            component_count: 0,
            anchors: Vec::new(),
        };
        lig.component_count = 0 as GlyphId;
        lig.anchors = Vec::new();
        lig.glyph = handle_from_name(Some(gname.to_vec())) as GlyphHandle;
        match base_record.as_array() {
            None => {
                lig_array.push(lig);
            }
            Some(components) => {
                lig.component_count = components.len() as GlyphId;
                lig.anchors = Vec::with_capacity(lig.component_count as usize);
                for (k, component_record) in components.iter().enumerate() {
                    // Indexed by `class_id` below, out of JSON key order --
                    // pre-sized and filled with "absent" rather than built with
                    // `.push()`.
                    lig.anchors
                        .push(vec![otl_anchor_absent(); class_count as usize]);
                    if let Some(inner_fields) = component_record.as_object() {
                        for (name_key, val) in inner_fields {
                            let class_name = &name_key[..name_key.len() - 1];
                            // `strlen`-bounded, matching
                            // `otl_parse_mark_array`'s registration key
                            // exactly.
                            match h.get(class_name) {
                                None => {
                                    logger_log_sds(
                                        &mut *options.logger.borrow_mut(),
                                        LOG_VL_IMPORTANT,
                                        LoggerType::Warning,
                                        crate::bytesbuild!(
                                            b"[OTFCC-fea] Invalid anchor class name <",
                                            class_name,
                                            b"> for /",
                                            gname,
                                            b". This base anchor is ignored.\n",
                                        ),
                                    );
                                }
                                Some(&class_id) => {
                                    lig.anchors[k][class_id as usize] =
                                        otl_parse_anchor(Some(val));
                                }
                            }
                        }
                    }
                }
                lig_array.push(lig);
            }
        }
    }
}
pub fn otl_gpos_parse_mark_to_ligature(
    _subtable: Option<&ParsedValue>,
    options: &Options,
) -> Option<Subtable> {
    let marks = _subtable.and_then(|v| v.get_typed(b"marks", JsonType::Object));
    let bases = _subtable.and_then(|v| v.get_typed(b"bases", JsonType::Object));
    let (Some(marks), Some(bases)) = (marks, bases) else {
        return None;
    };
    let mut mark_array: MarkArray = Vec::new();
    let mut h: std::collections::BTreeMap<Vec<u8>, GlyphClass> = std::collections::BTreeMap::new();
    otl_parse_mark_array(Some(marks), &mut mark_array, &mut h);
    let class_count = h.len() as GlyphClass;
    let mut lig_array: LigatureArray = Vec::new();
    parse_bases(Some(bases), &mut lig_array, &h, options);
    Some(Subtable::GposMarkToLigature(GposMarkToLigatureSubtable {
        class_count,
        mark_array,
        lig_array,
    }))
}
pub fn otfcc_build_gpos_mark_to_ligature(
    _subtable: &Subtable,
    mut _heuristics: BuildHeuristics,
) -> Buffer {
    let Subtable::GposMarkToLigature(subtable) = _subtable else {
        unreachable!()
    };
    let mut marks: Coverage = Vec::new();
    for mark in subtable.mark_array.iter() {
        push_to_coverage(&mut marks, mark.glyph.clone());
    }
    let mut bases: Coverage = Vec::new();
    for lig in subtable.lig_array.iter() {
        push_to_coverage(&mut bases, lig.glyph.clone());
    }
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, 1_u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(&marks))),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(&bases))),
        ),
        bk_int(
            BkCellType::B16,
            (subtable.class_count as i32) as u32,
        ),
    ]);
    let mut mark_array: BkBlock = bk_new_block(vec![bk_int(
        BkCellType::B16,
        (subtable.mark_array.len()) as u32,
    )]);
    for mark in subtable.mark_array.iter() {
        bk_push(
            &mut mark_array,
            vec![
                bk_int(BkCellType::B16, (mark.mark_class as i32) as u32),
                bk_ptr(BkCellType::P16, bk_from_anchor(mark.anchor)),
            ],
        );
    }
    let mut ligature_array: BkBlock = bk_new_block(vec![bk_int(
        BkCellType::B16,
        (subtable.lig_array.len()) as u32,
    )]);
    for lig in subtable.lig_array.iter() {
        let mut attach: BkBlock = bk_new_block(vec![bk_int(
            BkCellType::B16,
            (lig.component_count as i32) as u32,
        )]);
        // Same count-vs-length caution as the dump side above:
        // `.take()` on the field, not an assumption about `.len()`.
        for component_anchors in lig.anchors.iter().take(lig.component_count as usize) {
            for anchor in component_anchors.iter().take(subtable.class_count as usize) {
                bk_push(
                    &mut attach,
                    vec![bk_ptr(BkCellType::P16, bk_from_anchor(*anchor))],
                );
            }
        }
        bk_push(&mut ligature_array, vec![bk_ptr(BkCellType::P16, Some(attach))]);
    }
    bk_push(
        &mut root,
        vec![
            bk_ptr(BkCellType::P16, Some(mark_array)),
            bk_ptr(BkCellType::P16, Some(ligature_array)),
        ],
    );
    bk_build_block(root)
}

#[cfg(test)]
mod otl_read_gpos_mark_to_ligature_tests {
    use super::*;

    // format(2)@0, marksOffset(2)@2 -> 12, ligatureOffset(2)@4 -> 18,
    // classCount(2)@6, markArrayOffset(2)@8 -> 24, ligatureArrayOffset(2)
    // @10 -> 26; marks coverage @12 (glyph 5); ligature coverage @18
    // (glyph 6); mark array @24 (markCount=0, avoiding any dependency on
    // a real Anchor subtable); ligature array @26 (ligatureCount=1,
    // ligAttachOffsets[0]=4 -> 30); LigatureAttach @30
    // (componentCount=1, one absent anchor).
    fn well_formed_data() -> Vec<u8> {
        let mut data = vec![0u8; 34];
        data[2..4].copy_from_slice(&12u16.to_be_bytes());
        data[4..6].copy_from_slice(&18u16.to_be_bytes());
        data[6..8].copy_from_slice(&1u16.to_be_bytes()); // classCount
        data[8..10].copy_from_slice(&24u16.to_be_bytes());
        data[10..12].copy_from_slice(&26u16.to_be_bytes());
        data[12..14].copy_from_slice(&1u16.to_be_bytes());
        data[14..16].copy_from_slice(&1u16.to_be_bytes());
        data[16..18].copy_from_slice(&5u16.to_be_bytes());
        data[18..20].copy_from_slice(&1u16.to_be_bytes());
        data[20..22].copy_from_slice(&1u16.to_be_bytes());
        data[22..24].copy_from_slice(&6u16.to_be_bytes());
        data[24..26].copy_from_slice(&0u16.to_be_bytes()); // markCount = 0
        data[26..28].copy_from_slice(&1u16.to_be_bytes()); // ligatureCount
        data[28..30].copy_from_slice(&4u16.to_be_bytes()); // ligAttachOffsets[0] -> 30
        data[30..32].copy_from_slice(&1u16.to_be_bytes()); // componentCount
        data[32..34].copy_from_slice(&0u16.to_be_bytes()); // anchorOffset = absent
        data
    }

    #[test]
    fn well_formed_table_reads_the_ligature_array() {
        let data = well_formed_data();
        let result = otl_read_gpos_mark_to_ligature(&data, 0, 0);
        let Some(Subtable::GposMarkToLigature(ref subtable)) = result else {
            unreachable!()
        };
        assert_eq!(subtable.class_count, 1);
        assert_eq!(subtable.lig_array.len(), 1);
        assert_eq!(subtable.lig_array[0].glyph.index, 6);
        assert_eq!(subtable.lig_array[0].component_count, 1);
        assert!(!subtable.lig_array[0].anchors[0][0].present);
    }

    #[test]
    fn ligature_count_mismatch_with_coverage_is_rejected() {
        let mut data = well_formed_data();
        data[26..28].copy_from_slice(&2u16.to_be_bytes()); // ligatureCount claims 2, coverage has only 1
        let result = otl_read_gpos_mark_to_ligature(&data, 0, 0);
        assert!(result.is_none());
    }

    #[test]
    fn max_component_count_times_class_count_is_rejected_not_read_oob() {
        // `component_count * class_count` can reach ~4.3 billion with
        // both factors at `u16::MAX` -- comfortably inside a 64-bit
        // `usize` (so `checked_mul` itself never overflows for any
        // u16-bounded pair; that's the point of using `usize` here
        // instead of the original's `i32`), but `require_room` still
        // correctly rejects it against this 34-byte buffer rather than
        // reading anywhere close to that many bytes.
        let mut data = well_formed_data();
        data[6..8].copy_from_slice(&u16::MAX.to_be_bytes()); // classCount
        data[30..32].copy_from_slice(&u16::MAX.to_be_bytes()); // componentCount
        let result = otl_read_gpos_mark_to_ligature(&data, 0, 0);
        assert!(result.is_none());
    }
}
