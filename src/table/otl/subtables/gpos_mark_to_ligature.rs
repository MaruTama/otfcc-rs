//! GPOS lookup type 5, mark-to-ligature: like mark-to-base, but each
//! ligature has one set of anchors per component.

use crate::logger::ByteStr;
use crate::support::handle::{GlyphHandle, Handle, HandleState, handle_from_name};
use crate::table::otl::budget::OtlReadBudget;
use crate::table::otl::coverage::{Coverage, push_to_coverage};
use otfcc_json::ParsedValue;

use crate::support::primitives::{GlyphClass, GlyphId, count_u16};
use crate::table::otl::coverage::build_coverage;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::subtables::gpos_common::{
    bk_from_anchor, otl_anchor_absent, otl_parse_anchor, push_anchor_point,
};
use crate::table::otl::subtables::gpos_mark_common::{
    MarkAttachHeader, otl_parse_mark_array, otl_read_mark_array, read_anchor_row,
    read_mark_attach_header,
};
use crate::table::otl::{
    Anchor, GposMarkToLigatureSubtable, LigatureArray, LigatureBaseRecord, MarkArray, Subtable,
};
use otfcc_binary::Buffer;
use otfcc_binary::FontReader;
use otfcc_binary::bk::block::bk_new_block_from_buffer;
use otfcc_binary::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use otfcc_binary::bk::graph::bk_build_block;
use otfcc_json::BuiltValue;
use otfcc_json::JsonType;
// A LigatureAttach's byte length, `2 * component_count * class_count`, is
// computed with `checked_mul` before `require_room`: both factors are `u16`
// fields of the font, and the product (up to ~8.6 billion) would otherwise
// overflow and defeat the check.
/// Reads a mark-to-ligature subtable (GPOS 5): the marks with their classes
/// and anchors, and for each ligature one anchor per mark class for each of
/// its components. Returns `None` if the header, either coverage or the
/// ligature array is malformed.
pub fn otl_read_gpos_mark_to_ligature(
    data: &[u8],
    offset: u32,
    _max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    let header = read_mark_attach_header(data, offset, budget)?;
    let mut mark_array = Vec::new();
    otl_read_mark_array(
        &mut mark_array,
        &header.marks,
        data,
        header.mark_array_offset,
    );
    let lig_array = read_ligature_array(data, &header, budget)?;
    Some(Subtable::GposMarkToLigature(GposMarkToLigatureSubtable {
        class_count: header.class_count as GlyphClass,
        mark_array,
        lig_array,
    }))
}
/// Reads the LigatureArray: for each ligature, one anchor per mark class
/// for each of its components. Returns `None` if its count differs from the
/// ligature coverage, or any LigatureAttach does not fit. Stops early,
/// keeping the ligatures read so far, once the table's anchor budget runs
/// out.
fn read_ligature_array(
    data: &[u8],
    header: &MarkAttachHeader,
    budget: &mut OtlReadBudget,
) -> Option<LigatureArray> {
    let class_count = header.class_count;
    let lig_array_offset = header.base_array_offset;
    let mut lr = FontReader::new(data).at(lig_array_offset as usize).ok()?;
    let lig_count = lr.u16().ok()?;
    if lig_count as usize != header.bases.len() {
        return None;
    }
    lr.require_room(lig_count as usize, 2).ok()?;
    let mut lig_attach_offsets = Vec::with_capacity(lig_count as usize);
    for _ in 0..lig_count {
        lig_attach_offsets.push(lig_array_offset.wrapping_add(u32::from(
            lr.u16().expect("require_room above covered this read"),
        )));
    }

    let mut lig_array: LigatureArray = Vec::new();
    for (j, &lig_attach_offset) in lig_attach_offsets.iter().enumerate() {
        let mut ar = FontReader::new(data).at(lig_attach_offset as usize).ok()?;
        let component_count = ar.u16().ok()?;
        let total_anchors = (component_count as usize).checked_mul(class_count as usize)?;
        ar.require_room(total_anchors, 2).ok()?;
        // See `MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE`'s doc comment
        // (`gpos_mark_common.rs`): each ligAttachOffset is read via its own
        // fresh reader, so `require_room` above only ever bounds THIS
        // entry's cost against the buffer -- it can't see that an
        // earlier entry already claimed (and, for a crafted font
        // aliasing the same bytes, already fully accounted for) the
        // same allowance. This table-wide budget is what actually
        // stops `lig_count` separate entries each maxing out that
        // per-entry allowance from multiplying into an unbounded
        // total. Exhausting it stops processing further lig_attach
        // entries (not just this one) -- there's no reason to expect
        // a later entry to fare any better, and the entries already
        // built stay valid.
        if !budget.try_spend_mark_attach_anchors(total_anchors) {
            break;
        }
        let mut lig = LigatureBaseRecord {
            glyph: header.bases[j].clone(),
            component_count,
            anchors: Vec::with_capacity(component_count as usize),
        };
        for _ in 0..component_count {
            lig.anchors.push(read_anchor_row(
                &mut ar,
                data,
                lig_attach_offset,
                class_count,
            ));
        }
        lig_array.push(lig);
    }
    Some(lig_array)
}
/// Writes the subtable as `{ classCount, marks, bases }`. Mark classes are
/// named `ac_<N>`; each ligature maps to an array with one object per
/// component, from class name to `{ x, y }`.
pub fn otl_gpos_dump_mark_to_ligature(st: &Subtable) -> BuiltValue {
    let Subtable::GposMarkToLigature(subtable) = st else {
        unreachable!()
    };
    let mut _subtable = BuiltValue::new_object(3);
    let mut _marks = BuiltValue::new_object(subtable.mark_array.len());
    let mut _bases = BuiltValue::new_object(subtable.lig_array.len());
    for mark in subtable.mark_array.iter() {
        let mut _mark = BuiltValue::new_object(3);
        let mark_class_name: Vec<u8> = crate::bytesbuild!(b"ac_", i32::from(mark.mark_class),);
        _mark.push_field(b"class", BuiltValue::str_truncated_at_nul(&mark_class_name));
        _mark.push_field(b"x", BuiltValue::Int(mark.anchor.x as i64));
        _mark.push_field(b"y", BuiltValue::Int(mark.anchor.y as i64));
        push_anchor_point(&mut _mark, &mark.anchor);
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
                    push_anchor_point(&mut _anchor, anchor);
                    let anchor_key: Vec<u8> = crate::bytesbuild!(b"ac_", m as i32);
                    _bk.push_field_bytes_key(&anchor_key, _anchor);
                }
            }
            _base.push_item(_bk);
        }
        _bases.push_field_bytes_key(&base.glyph.name, _base.preserialize());
    }
    _subtable.push_field(
        b"classCount",
        BuiltValue::Int(i64::from(subtable.class_count)),
    );
    _subtable.push_field(b"marks", _marks);
    _subtable.push_field(b"bases", _bases);
    _subtable
}
fn parse_bases(
    bases: Option<&ParsedValue>,
    lig_array: &mut LigatureArray,
    h: &std::collections::BTreeMap<Vec<u8>, GlyphClass>,
) {
    let class_count: GlyphClass = count_u16(h.len());
    let Some(fields) = bases.and_then(ParsedValue::as_object) else {
        return;
    };
    for (key, base_record) in fields {
        let gname = &key[..key.len() - 1];
        let mut lig: LigatureBaseRecord = LigatureBaseRecord {
            glyph: Handle::new(HandleState::Empty, 0, Vec::new()),
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
                lig.component_count = count_u16(components.len());
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
                                    tracing::warn!(
                                        "[OTFCC-fea] Invalid anchor class name <{}> for /{}. This base anchor is ignored.\n",
                                        ByteStr(class_name),
                                        ByteStr(gname)
                                    );
                                }
                                Some(&class_id) => {
                                    lig.anchors[k][class_id as usize] = otl_parse_anchor(Some(val));
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
/// Reads the object written by `otl_gpos_dump_mark_to_ligature`, or `None`
/// if `marks` or `bases` is missing.
pub fn otl_gpos_parse_mark_to_ligature(_subtable: Option<&ParsedValue>) -> Option<Subtable> {
    let marks = _subtable.and_then(|v| v.get_typed(b"marks", JsonType::Object));
    let bases = _subtable.and_then(|v| v.get_typed(b"bases", JsonType::Object));
    let (Some(marks), Some(bases)) = (marks, bases) else {
        return None;
    };
    let mut mark_array: MarkArray = Vec::new();
    let mut h: std::collections::BTreeMap<Vec<u8>, GlyphClass> = std::collections::BTreeMap::new();
    let class_count = otl_parse_mark_array(Some(marks), &mut mark_array, &mut h);
    let mut lig_array: LigatureArray = Vec::new();
    parse_bases(Some(bases), &mut lig_array, &h);
    Some(Subtable::GposMarkToLigature(GposMarkToLigatureSubtable {
        class_count,
        mark_array,
        lig_array,
    }))
}
/// Writes the subtable as MarkLigPos format 1.
pub fn build_gpos_mark_to_ligature(
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
        bk_int(BkCellType::B16, 1),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(&marks))),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(&bases))),
        ),
        bk_int(BkCellType::B16, i32::from(subtable.class_count) as u32),
    ]);
    let mut mark_array: BkBlock = bk_new_block(vec![bk_int(
        BkCellType::B16,
        (subtable.mark_array.len()) as u32,
    )]);
    for mark in subtable.mark_array.iter() {
        bk_push(
            &mut mark_array,
            vec![
                bk_int(BkCellType::B16, i32::from(mark.mark_class) as u32),
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
            i32::from(lig.component_count) as u32,
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
        bk_push(
            &mut ligature_array,
            vec![bk_ptr(BkCellType::P16, Some(attach))],
        );
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
    use crate::table::otl::subtables::gpos_mark_common::MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE;

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
        let result = otl_read_gpos_mark_to_ligature(&data, 0, 0, &mut OtlReadBudget::new());
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
        let result = otl_read_gpos_mark_to_ligature(&data, 0, 0, &mut OtlReadBudget::new());
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
        let result = otl_read_gpos_mark_to_ligature(&data, 0, 0, &mut OtlReadBudget::new());
        assert!(result.is_none());
    }

    // Builds a table with `n` ligatureCoverage entries (a single format-2
    // Coverage range, so the coverage table itself stays a few bytes
    // regardless of `n`) whose `n` ligAttachOffsets ALL alias one shared
    // LigatureAttach blob (`componentCount` components, one class each).
    // See `MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE`'s doc comment
    // (`gpos_mark_common.rs`) for why this specific shape -- every entry's own
    // `require_room` check is individually satisfied against the same
    // small blob, so nothing but the table-wide budget bounds the total.
    fn mark_to_ligature_aliased_data(n: u16, component_count: u16) -> Vec<u8> {
        let mut d = vec![0u8; 12];
        d[2..4].copy_from_slice(&12u16.to_be_bytes()); // marks coverage @12
        d[4..6].copy_from_slice(&18u16.to_be_bytes()); // bases coverage @18
        d[6..8].copy_from_slice(&1u16.to_be_bytes()); // classCount = 1
        d[8..10].copy_from_slice(&28u16.to_be_bytes()); // mark array @28
        d[10..12].copy_from_slice(&30u16.to_be_bytes()); // lig array @30

        // marks coverage @12: format 1, 1 glyph.
        d.extend_from_slice(&1u16.to_be_bytes());
        d.extend_from_slice(&1u16.to_be_bytes());
        d.extend_from_slice(&0u16.to_be_bytes());

        // bases coverage @18: format 2, one range covering gids 0..n-1.
        d.extend_from_slice(&2u16.to_be_bytes());
        d.extend_from_slice(&1u16.to_be_bytes()); // rangeCount
        d.extend_from_slice(&0u16.to_be_bytes()); // start
        d.extend_from_slice(&(n - 1).to_be_bytes()); // end
        d.extend_from_slice(&0u16.to_be_bytes()); // startCoverageIndex

        // mark array @28: markCount = 0.
        d.extend_from_slice(&0u16.to_be_bytes());

        // lig array @30: ligCount = n, then n identical ligAttachOffsets
        // pointing at one shared blob right after this header.
        let lig_array_offset: u32 = 30;
        d.extend_from_slice(&n.to_be_bytes());
        let blob_offset = d.len() as u32 + 2 * u32::from(n);
        let blob_rel = (blob_offset - lig_array_offset) as u16;
        for _ in 0..n {
            d.extend_from_slice(&blob_rel.to_be_bytes());
        }

        // Shared LigatureAttach blob: componentCount, then that many
        // absent (0) anchorOffset entries.
        d.extend_from_slice(&component_count.to_be_bytes());
        for _ in 0..component_count {
            d.extend_from_slice(&0u16.to_be_bytes());
        }
        d
    }

    #[test]
    // This test has to actually materialize close to the real
    // `MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE` (2,000,000) anchors for
    // it to mean anything -- proving the cap is enforced at its real
    // production value, not a shrunk stand-in, the same reasoning `otl/
    // read.rs`'s `total_language_count_across_the_whole_table_is_capped`
    // gives for its own `#[cfg_attr(miri, ignore)]`. Under a normal
    // `cargo test` this completes in well under a second; under Miri's
    // interpreter, 2,000,000 individual small `Vec` allocations does not.
    #[cfg_attr(
        miri,
        ignore = "far too slow to run meaningfully under Miri's interpreter; needs ~2,000,000 individual Vec<Anchor> allocations to exceed MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE"
    )]
    fn aliased_lig_attach_offsets_are_capped_by_table_wide_anchor_budget() {
        // Regression test for the OOM this crate's `otf_dump` fuzz target
        // hit intermittently in CI (see `fuzz/README.md`'s "Known
        // findings"): a crafted font whose LigatureArray has many
        // ligCount entries, every one of them pointing its ligAttachOffset
        // at the SAME small LigatureAttach blob. Each individual entry's
        // own `require_room` check passes (it only ever sees the shared
        // blob's own small size), so before this fix nothing stopped
        // 60,000 such entries from each independently allocating a fresh
        // `componentCount`-sized `Vec<Vec<Anchor>>` -- confirmed via a
        // standalone harness driving this exact function directly: a
        // 137,104-byte crafted table (n=65,535, componentCount=3,000) was
        // OOM-killed after exhausting 15GB of RAM. This table is smaller
        // (chosen so `n * component_count` clears `MAX_TOTAL_MARK_ATTACH_
        // ANCHORS_PER_TABLE`'s 2,000,000-anchor ceiling by a comfortable
        // margin, exercising the same aliasing shape without the test
        // itself needing anywhere near that much memory) and must
        // terminate as a merely-truncated result, not a hang or a
        // multi-hundred-MB allocation.
        let mut budget = OtlReadBudget::new();
        let n: u16 = 200;
        let component_count: u16 = 20_000; // n * component_count = 4,000,000
        let data = mark_to_ligature_aliased_data(n, component_count);
        let result = otl_read_gpos_mark_to_ligature(&data, 0, 0, &mut budget);
        let Some(Subtable::GposMarkToLigature(ref subtable)) = result else {
            unreachable!()
        };
        // Every entry built (however many the budget allowed) is still a
        // real, fully-formed record -- the budget stops further entries
        // from being built at all, it never truncates one mid-flight.
        assert!(!subtable.lig_array.is_empty());
        assert!((subtable.lig_array.len() as u32) < u32::from(n));
        for lig in &subtable.lig_array {
            assert_eq!(lig.component_count, component_count);
            assert_eq!(lig.anchors.len(), component_count as usize);
        }
        // Total anchors actually materialized never exceeds the budget.
        let total_anchors: u64 = subtable
            .lig_array
            .iter()
            .map(|lig| lig.anchors.len() as u64)
            .sum();
        assert!(total_anchors <= u64::from(MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE));
    }
}
