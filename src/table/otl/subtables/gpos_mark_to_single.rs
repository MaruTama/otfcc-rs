use crate::logger::ByteStr;
use crate::support::handle::{
    GlyphHandle, Handle, HandleState, handle_from_name,
};
use crate::support::parsed_json::ParsedValue;
use crate::table::otl::budget::OtlReadBudget;
use crate::table::otl::coverage::{Coverage, push_to_coverage, read_coverage};

use crate::bk::block::bk_new_block_from_buffer;
use crate::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::bk::graph::bk_build_block;
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
use crate::support::font_reader::FontReader;
use crate::support::primitives::{GlyphClass, GlyphId, count_u16};
use crate::table::otl::coverage::build_coverage;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::subtables::gpos_common::{
    bk_from_anchor, otl_anchor_absent, otl_parse_anchor, otl_parse_mark_array, otl_read_anchor,
    otl_read_mark_array,
};
use crate::table::otl::{
    Anchor, BaseArray, BaseRecord, GposMarkToSingleSubtable, MarkArray, Subtable,
};
use crate::vendor::json::JsonType;
// `BaseRecord.anchors` is a plain `Vec<Anchor>` now and `glyph: GlyphHandle`
// already has its own `Drop`, so a `BaseArray` (`Vec<BaseRecord>`) fully
// self-drops -- clearing it (still needed: `consolidate/otl/mark.rs`'s dedup
// pass clears an in-place array mid-function, not just at end of scope) is
// exactly `*arr = Vec::new()`.
pub(crate) fn dispose_base_array(arr: &mut BaseArray) {
    *arr = Vec::new();
}
// `2 * bases.len() * class_count` (the BaseArray's byte-length guard) is a
// real, previously-undocumented overflow-defeats-guard bug: `bases.len()`
// can be as large as the glyph count (bounded by `GlyphId`, up to 65535)
// and `class_count` is an independent, unbounded `u16` read straight from
// the file -- their product can exceed `i32::MAX` (65535*65535*2 is
// ~8.6 billion), the same class of bug as `cmap.rs`'s `n_groups` guard,
// just reached by two independently-large factors instead of one. Fixed
// with `checked_mul` before ever calling `require_room`.
pub fn otl_read_gpos_mark_to_single(
    data: &[u8],
    subtable_offset: u32,
    _max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    let mut subtable = GposMarkToSingleSubtable {
        class_count: 0,
        mark_array: Vec::new(),
        base_array: Vec::new(),
    };

    'parse: {
        let mut header = match FontReader::new(data).at(subtable_offset as usize) {
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
        let Ok(base_array_rel) = header.u16() else {
            break 'parse;
        };

        let marks: Coverage = read_coverage(data, subtable_offset.wrapping_add(marks_rel as u32), budget);
        let bases: Coverage = read_coverage(data, subtable_offset.wrapping_add(bases_rel as u32), budget);
        if marks.is_empty() || bases.is_empty() {
            break 'parse;
        }

        subtable.class_count = class_count as GlyphClass;
        let mark_array_offset = subtable_offset.wrapping_add(mark_array_rel as u32);
        otl_read_mark_array(&mut subtable.mark_array, &marks, data, mark_array_offset);

        let base_array_offset = subtable_offset.wrapping_add(base_array_rel as u32);
        let Ok(mut base_reader) = FontReader::new(data).at(base_array_offset as usize) else {
            break 'parse;
        };
        let Ok(base_count) = base_reader.u16() else {
            break 'parse;
        };
        if base_count as usize != bases.len() {
            break 'parse;
        }
        let Some(total_anchors) = bases.len().checked_mul(class_count as usize) else {
            break 'parse;
        };
        if base_reader.require_room(total_anchors, 2).is_err() {
            break 'parse;
        }

        for base in &bases {
            // See `MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE`'s doc comment
            // (`gpos_common.rs`): this loop's single sequential `base_
            // reader` (unlike `gpos_mark_to_ligature.rs`'s per-entry
            // fresh readers) already keeps *this* subtable's own total
            // bounded to the real buffer -- it can't be re-aliased from
            // within one call. What this closes is many separate MarkTo-
            // Single/MarkToLigature subtables each independently pointing
            // their BaseArray at the very same maximal-cost bytes; a
            // table-wide budget is the only thing that can see that.
            if !budget.try_spend_mark_attach_anchors(class_count as usize) {
                break;
            }
            let mut base_anchors: Vec<Anchor> = Vec::with_capacity(class_count as usize);
            for _ in 0..class_count {
                let anchor_rel = base_reader.u16().unwrap();
                if anchor_rel != 0 {
                    base_anchors.push(otl_read_anchor(
                        data,
                        base_array_offset.wrapping_add(anchor_rel as u32),
                    ));
                } else {
                    base_anchors.push(otl_anchor_absent());
                }
            }
            subtable.base_array.push(BaseRecord {
                glyph: base.clone(),
                anchors: base_anchors,
            });
        }
        return Some(Subtable::GposMarkToSingle(subtable));
    }

    None
}
pub fn otl_gpos_dump_mark_to_single(st: &Subtable) -> BuiltValue {
    let Subtable::GposMarkToSingle(subtable) = st else {
        unreachable!()
    };
    let mut _subtable = BuiltValue::new_object(3);
    let mut _marks = BuiltValue::new_object(subtable.mark_array.len());
    let mut _bases = BuiltValue::new_object(subtable.base_array.len());
    for mark in subtable.mark_array.iter() {
        let mut _mark = BuiltValue::new_object(3);
        let mark_class_name: Vec<u8> = crate::bytesbuild!(b"anchor", mark.mark_class as i32,);
        _mark.push_field(b"class", BuiltValue::str_truncated_at_nul(&mark_class_name));
        _mark.push_field(b"x", BuiltValue::Int(mark.anchor.x as i64));
        _mark.push_field(b"y", BuiltValue::Int(mark.anchor.y as i64));
        _marks.push_field_bytes_key(&mark.glyph.name, _mark.preserialize());
    }
    for base in subtable.base_array.iter() {
        let mut _base = BuiltValue::new_object(subtable.class_count as usize);
        // `k`'s own value feeds the output key (`anchor<class id>`), so
        // this needs `.enumerate()`; bounded by `subtable.class_count`,
        // not assumed equal to `base.anchors.len()` (same count-vs-length
        // caution as PR #422/#423/#426/#427).
        for (k, anchor) in base
            .anchors
            .iter()
            .enumerate()
            .take(subtable.class_count as usize)
        {
            if anchor.present {
                let mut _anchor = BuiltValue::new_object(2);
                _anchor.push_field(b"x", BuiltValue::Int(anchor.x as i64));
                _anchor.push_field(b"y", BuiltValue::Int(anchor.y as i64));
                let mark_class_name_0: Vec<u8> = crate::bytesbuild!(b"anchor", k as i32);
                _base.push_field_bytes_key(&mark_class_name_0, _anchor);
            }
        }
        _bases.push_field_bytes_key(&base.glyph.name, _base.preserialize());
    }
    _subtable.push_field(b"marks", _marks);
    _subtable.push_field(b"bases", _bases);
    _subtable
}
fn parse_bases(
    bases: Option<&ParsedValue>,
    base_array: &mut BaseArray,
    h: &std::collections::BTreeMap<Vec<u8>, GlyphClass>,
) {
    let class_count: GlyphClass = count_u16(h.len());
    let Some(fields) = bases.and_then(ParsedValue::as_object) else {
        return;
    };
    for (key, base_record) in fields {
        let gname = &key[..key.len() - 1];
        let mut base: BaseRecord = BaseRecord {
            glyph: Handle::new(HandleState::Empty, 0, Vec::new()),
            anchors: Vec::new(),
        };
        base.glyph = handle_from_name(Some(gname.to_vec())) as GlyphHandle;
        // Indexed by `class_id` below, out of JSON key order -- pre-sized
        // and filled with "absent" rather than built with `.push()`.
        base.anchors = vec![otl_anchor_absent(); class_count as usize];
        match base_record.as_object() {
            None => {
                base_array.push(base);
            }
            Some(inner_fields) => {
                for (name_key, val) in inner_fields {
                    let class_name = &name_key[..name_key.len() - 1];
                    match h.get(class_name) {
                        None => {
                            tracing::warn!("[OTFCC-fea] Invalid anchor class name <{}> for /{}. This base anchor is ignored.\n", ByteStr(class_name), ByteStr(gname));
                        }
                        Some(&class_id) => {
                            base.anchors[class_id as usize] =
                                otl_parse_anchor(Some(val));
                        }
                    }
                }
                base_array.push(base);
            }
        }
    }
}
pub fn otl_gpos_parse_mark_to_single(
    _subtable: Option<&ParsedValue>,
) -> Option<Subtable> {
    let marks = _subtable.and_then(|v| v.get_typed(b"marks", JsonType::Object));
    let bases = _subtable.and_then(|v| v.get_typed(b"bases", JsonType::Object));
    let (Some(marks), Some(bases)) = (marks, bases) else {
        return None;
    };
    let mut mark_array: MarkArray = Vec::new();
    let mut h: std::collections::BTreeMap<Vec<u8>, GlyphClass> = std::collections::BTreeMap::new();
    let class_count = otl_parse_mark_array(Some(marks), &mut mark_array, &mut h);
    let mut base_array: BaseArray = Vec::new();
    parse_bases(Some(bases), &mut base_array, &h);
    Some(Subtable::GposMarkToSingle(GposMarkToSingleSubtable {
        class_count,
        mark_array,
        base_array,
    }))
}
pub fn build_gpos_mark_to_single(
    _subtable: &Subtable,
    mut _heuristics: BuildHeuristics,
) -> Buffer {
    let Subtable::GposMarkToSingle(subtable) = _subtable else {
        unreachable!()
    };
    let mut marks: Coverage = Vec::new();
    for mark in subtable.mark_array.iter() {
        push_to_coverage(&mut marks, mark.glyph.clone());
    }
    let mut bases: Coverage = Vec::new();
    for base in subtable.base_array.iter() {
        push_to_coverage(&mut bases, base.glyph.clone());
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
    let mut base_array: BkBlock = bk_new_block(vec![bk_int(
        BkCellType::B16,
        (subtable.base_array.len()) as u32,
    )]);
    for base in subtable.base_array.iter() {
        // Same count-vs-length caution as the dump side above:
        // `.take()` on the field, not an assumption about `.len()`.
        for anchor in base.anchors.iter().take(subtable.class_count as usize) {
            bk_push(
                &mut base_array,
                vec![bk_ptr(BkCellType::P16, bk_from_anchor(*anchor))],
            );
        }
    }
    bk_push(
        &mut root,
        vec![
            bk_ptr(BkCellType::P16, Some(mark_array)),
            bk_ptr(BkCellType::P16, Some(base_array)),
        ],
    );
    return bk_build_block(root);
}

#[cfg(test)]
mod otl_read_gpos_mark_to_single_tests {
    use super::*;

    // format(2)@0, marksOffset(2)@2 -> 12, basesOffset(2)@4 -> 18,
    // classCount(2)@6, markArrayOffset(2)@8 -> 24, baseArrayOffset(2)@10
    // -> 26; marks coverage @12 (glyph 5); bases coverage @18 (glyph 6);
    // mark array @24 (markCount=0, avoiding any dependency on
    // `otl_read_anchor`/a real Anchor subtable); base array @26
    // (baseCount + baseCount*classCount anchor offsets, all absent).
    fn well_formed_data(class_count: u16) -> Vec<u8> {
        let mut data = vec![0u8; 30];
        data[2..4].copy_from_slice(&12u16.to_be_bytes());
        data[4..6].copy_from_slice(&18u16.to_be_bytes());
        data[6..8].copy_from_slice(&class_count.to_be_bytes());
        data[8..10].copy_from_slice(&24u16.to_be_bytes());
        data[10..12].copy_from_slice(&26u16.to_be_bytes());
        data[12..14].copy_from_slice(&1u16.to_be_bytes());
        data[14..16].copy_from_slice(&1u16.to_be_bytes());
        data[16..18].copy_from_slice(&5u16.to_be_bytes());
        data[18..20].copy_from_slice(&1u16.to_be_bytes());
        data[20..22].copy_from_slice(&1u16.to_be_bytes());
        data[22..24].copy_from_slice(&6u16.to_be_bytes());
        data[24..26].copy_from_slice(&0u16.to_be_bytes()); // markCount = 0
        data[26..28].copy_from_slice(&1u16.to_be_bytes()); // baseCount
        data[28..30].copy_from_slice(&0u16.to_be_bytes()); // anchorOffset[0][0] = absent
        data
    }

    #[test]
    fn well_formed_table_reads_the_base_array() {
        let data = well_formed_data(1);
        let result = otl_read_gpos_mark_to_single(&data, 0, 0, &mut OtlReadBudget::new());
        let Some(Subtable::GposMarkToSingle(ref subtable)) = result else {
            unreachable!()
        };
        assert_eq!(subtable.class_count, 1);
        assert_eq!(subtable.base_array.len(), 1);
        assert_eq!(subtable.base_array[0].glyph.index, 6);
        assert!(!subtable.base_array[0].anchors[0].present);
    }

    #[test]
    fn base_count_mismatch_with_coverage_is_rejected() {
        let mut data = well_formed_data(1);
        data[26..28].copy_from_slice(&2u16.to_be_bytes()); // baseCount claims 2, coverage has only 1
        let result = otl_read_gpos_mark_to_single(&data, 0, 0, &mut OtlReadBudget::new());
        assert!(result.is_none());
    }

    // Builds a table with `base_count` glyphs in the bases Coverage (a
    // plain format-1 list -- `base_count` is kept small in the test below,
    // so this stays cheap) and a base array with exactly
    // `base_count * class_count` sequential (all-absent) anchor slots.
    fn many_bases_data(base_count: u16, class_count: u16) -> Vec<u8> {
        let mut d = vec![0u8; 12];
        let bases_offset: u16 = 18;
        let mark_array_offset = bases_offset + 4 + 2 * base_count;
        let base_array_offset = mark_array_offset + 2;
        d[2..4].copy_from_slice(&12u16.to_be_bytes()); // marks coverage @12
        d[4..6].copy_from_slice(&bases_offset.to_be_bytes());
        d[6..8].copy_from_slice(&class_count.to_be_bytes());
        d[8..10].copy_from_slice(&mark_array_offset.to_be_bytes());
        d[10..12].copy_from_slice(&base_array_offset.to_be_bytes());

        // marks coverage @12: format 1, 1 glyph.
        d.extend_from_slice(&1u16.to_be_bytes());
        d.extend_from_slice(&1u16.to_be_bytes());
        d.extend_from_slice(&0u16.to_be_bytes());
        assert_eq!(d.len(), bases_offset as usize);

        // bases coverage: format 1, base_count distinct glyphs.
        d.extend_from_slice(&1u16.to_be_bytes());
        d.extend_from_slice(&base_count.to_be_bytes());
        for gid in 0..base_count {
            d.extend_from_slice(&(gid + 1).to_be_bytes());
        }
        assert_eq!(d.len(), mark_array_offset as usize);

        // mark array: markCount = 0.
        d.extend_from_slice(&0u16.to_be_bytes());
        assert_eq!(d.len(), base_array_offset as usize);

        // base array: baseCount, then base_count*class_count absent slots.
        d.extend_from_slice(&base_count.to_be_bytes());
        for _ in 0..(base_count as u32 * class_count as u32) {
            d.extend_from_slice(&0u16.to_be_bytes());
        }
        d
    }

    #[test]
    fn mark_attach_anchor_budget_truncates_base_array_once_exhausted() {
        // Regression test for the table-wide `mark_attach_anchors` budget
        // (`OtlReadBudget`): this subtable's own single sequential
        // `base_reader` already bounds ITS OWN total against the real
        // buffer (that's what `anchor_array_shorter_than_class_count_
        // times_base_count_is_rejected` above confirms), so the budget's
        // job here is purely the cross-subtable case -- many MarkToSingle/
        // MarkToLigature subtables in the same table each independently
        // maxing out their own allowance. Simulated directly (draining
        // the shared budget first) rather than by constructing multiple
        // real subtables, which is exactly equivalent from this function's
        // point of view (it only ever sees the budget's current value).
        let mut budget = OtlReadBudget { mark_attach_anchors: 3, ..OtlReadBudget::new() };
        let data = many_bases_data(10, 1);
        let result = otl_read_gpos_mark_to_single(&data, 0, 0, &mut budget);
        let Some(Subtable::GposMarkToSingle(ref subtable)) = result else {
            unreachable!()
        };
        // Only 3 anchor-slot units were left in the budget (class_count=1,
        // so 1 unit per base): the loop must stop there, not read (or
        // panic on) the other 7 bases the table itself declares.
        assert_eq!(subtable.base_array.len(), 3);
    }

    #[test]
    fn anchor_array_shorter_than_class_count_times_base_count_is_rejected() {
        // The guard here (`bases.len() * class_count`, checked via
        // `checked_mul` on `usize`) is what the original computed as
        // unchecked `i32` arithmetic -- for a large enough `class_count`
        // and `bases.len()`, that product can exceed `i32::MAX` and wrap.
        // Demonstrating the exact overflow needs an impractically large
        // base coverage; this instead confirms the guard rejects a
        // shortfall at an ordinary scale (class_count raised from 1 to 5,
        // but the base array still has room for only 1 anchor slot).
        let data = well_formed_data(5); // baseCount=1, but only 1 anchor slot is present, not 5
        let result = otl_read_gpos_mark_to_single(&data, 0, 0, &mut OtlReadBudget::new());
        assert!(result.is_none());
    }
}
