//! Pieces shared by the mark attachment subtables (GPOS 4, 5 and 6): the
//! subtable header, the MarkArray, rows of anchor offsets, and the
//! table-wide budget on anchor slots.

use crate::support::handle::{GlyphHandle, Handle, HandleState, handle_from_name};
use crate::table::otl::budget::OtlReadBudget;
use crate::table::otl::coverage::{Coverage, read_coverage};
use otfcc_json::ParsedValue;

use otfcc_binary::FontReader;

use crate::support::primitives::{GlyphClass, Pos, count_u16};
use crate::table::otl::subtables::gpos_common::{otl_anchor_absent, otl_read_anchor};
use crate::table::otl::{Anchor, MarkArray, MarkRecord};
use otfcc_json::JsonType;
/// Bounds mark-attachment anchor-slot construction (`gpos_mark_to_single.rs`'s
/// BaseArray, `gpos_mark_to_ligature.rs`'s LigatureArray) across a WHOLE
/// GSUB/GPOS table, the same "individually bounded per call, unbounded in
/// aggregate" shape `OtlReadBudget`'s `coverage_entries` and
/// `class_coverage_calls` limits already close for their
/// own call sites.
///
/// `gpos_mark_to_ligature.rs`'s `otl_read_gpos_mark_to_ligature` reads its
/// LigatureArray's `lig_count` (attacker-controlled, up to 65,535 -- and
/// cheap to reach: a format-2 Coverage range expresses that many glyphs in
/// 8 bytes) ligAttachOffsets, each pointing anywhere in the table's data --
/// so each is read via its OWN fresh `FontReader` seeked to that offset,
/// not a single reader advancing sequentially the way `gpos_mark_to_
/// single.rs`'s BaseArray loop does. Each individual ligAttachOffset's own
/// `componentCount * classCount` is bounds-checked against the buffer
/// remaining from THAT offset (`require_room`, same discipline as
/// everywhere else in this crate) -- but nothing tracks how much of the
/// buffer earlier ligAttachOffsets already "spent", so a crafted font can
/// point every one of `lig_count`'s offsets at the SAME small blob, and
/// each one independently re-claims the full componentCount*classCount
/// allowance the buffer permits from there. A 137KB crafted table (n =
/// 65,535 ligAttachOffsets aliasing one ~6KB LigatureAttach blob with
/// componentCount = 3,000, so no single call's own `require_room` check
/// ever sees more than ~6KB behind it) exhausted 15GB of RAM in a
/// standalone repro harness driving `otl_read_gpos_mark_to_ligature`
/// directly -- confirming this is a real per-subtable amplification, not
/// theoretical. Shared with `gpos_mark_to_single.rs` too (defense in depth:
/// its own single-reader BaseArray loop cannot alias bytes *within* one
/// subtable, but nothing before this budget stopped many MarkToSingle/
/// MarkToLigature subtables -- up to `otl/read.rs`'s own `MAX_TOTAL_
/// SUBTABLES_PER_TABLE` ceiling (300,000 when this was written) --
/// from each independently pointing their BaseArray/LigatureArray at the
/// same maximal-cost bytes).
///
/// This is the `mark_attach_anchors` limit of `OtlReadBudget`, which
/// `otl/read.rs`'s `read_otl` creates once per table, not per
/// subtable -- a table-wide ceiling closes the many-subtables variant
/// above too, the same reasoning the other `OtlReadBudget` limits give.
/// `OtlReadBudget::try_spend_mark_attach_anchors` charges a whole request
/// or none of it, so a refusal means "this call changed nothing" and the
/// caller should stop adding records, keeping whatever was already built.
/// 2,000,000 anchor slots is generously above any legitimate font's mark
/// attachment count (each slot is one `Anchor`, a few bytes) while still
/// bounding worst-case memory to a few tens of MB instead of exhausting
/// all available RAM.
pub(crate) const MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE: u32 = 2_000_000;
/// The start shared by MarkBasePos, MarkMarkPos and MarkLigPos: the mark
/// and base (or ligature) coverages, the mark class count, and where the
/// MarkArray and the BaseArray (or LigatureArray) start in `data`.
pub(crate) struct MarkAttachHeader {
    pub marks: Coverage,
    pub bases: Coverage,
    pub class_count: u16,
    pub mark_array_offset: u32,
    pub base_array_offset: u32,
}
/// Reads the header of a mark attachment subtable at `offset` and its two
/// coverages. Returns `None` if the header does not fit or either coverage
/// is empty.
pub(crate) fn read_mark_attach_header(
    data: &[u8],
    offset: u32,
    budget: &mut OtlReadBudget,
) -> Option<MarkAttachHeader> {
    let mut header = FontReader::new(data).at(offset as usize).ok()?;
    header.skip(2).ok()?; // format, unused
    let marks_rel = header.u16().ok()?;
    let bases_rel = header.u16().ok()?;
    let class_count = header.u16().ok()?;
    let mark_array_rel = header.u16().ok()?;
    let base_array_rel = header.u16().ok()?;
    let marks = read_coverage(data, offset.wrapping_add(u32::from(marks_rel)), budget);
    let bases = read_coverage(data, offset.wrapping_add(u32::from(bases_rel)), budget);
    if marks.is_empty() || bases.is_empty() {
        return None;
    }
    Some(MarkAttachHeader {
        marks,
        bases,
        class_count,
        mark_array_offset: offset.wrapping_add(u32::from(mark_array_rel)),
        base_array_offset: offset.wrapping_add(u32::from(base_array_rel)),
    })
}
/// Reads `count` anchor offsets from `r`, each relative to `base`, and the
/// anchors they point at; a null offset is an absent anchor. The caller
/// has already checked with `require_room` that the offsets fit.
pub(crate) fn read_anchor_row(
    r: &mut FontReader<'_>,
    data: &[u8],
    base: u32,
    count: u16,
) -> Vec<Anchor> {
    let mut row: Vec<Anchor> = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let anchor_rel = r.u16().expect("require_room above covered this read");
        if anchor_rel != 0 {
            row.push(otl_read_anchor(
                data,
                base.wrapping_add(u32::from(anchor_rel)),
            ));
        } else {
            row.push(otl_anchor_absent());
        }
    }
    row
}
/// Reads a MarkArray at `offset`. Its record count is checked against the
/// table, and the marks are capped at `cov.len()`: the MarkArray's count and
/// the Coverage's glyph count come from the font independently and need not
/// agree.
pub fn otl_read_mark_array(array: &mut MarkArray, cov: &Coverage, data: &[u8], offset: u32) {
    let Ok(mut r) = FontReader::new(data).at(offset as usize) else {
        return;
    };
    let Ok(mark_count) = r.u16() else { return };
    if r.require_room(mark_count as usize, 4).is_err() {
        return;
    }
    let n = (mark_count as usize).min(cov.len());
    for glyph in cov.iter().take(n) {
        let mark_class = r.u16().expect("require_room above covered this read") as GlyphClass;
        let delta = r.u16().expect("require_room above covered this read");
        let anchor = if delta != 0 {
            otl_read_anchor(data, offset.wrapping_add(u32::from(delta)))
        } else {
            otl_anchor_absent()
        };
        array.push(MarkRecord {
            glyph: glyph.clone(),
            mark_class,
            anchor,
        });
    }
}
/// Reads the JSON `marks` object (glyph name to `{ class, x, y }`) into
/// `array`. Class names are numbered in alphabetical order; `h` receives
/// the name-to-number map, and the number of classes is returned. A mark
/// without a `class` is kept with no anchor.
pub fn otl_parse_mark_array(
    marks: Option<&ParsedValue>,
    array: &mut MarkArray,
    h: &mut std::collections::BTreeMap<Vec<u8>, GlyphClass>,
) -> GlyphClass {
    let Some(fields) = marks.and_then(ParsedValue::as_object) else {
        return 0;
    };
    for (key, anchor_record) in fields {
        let mut mark: MarkRecord = MarkRecord {
            glyph: Handle::new(HandleState::Empty, 0, Vec::new()),
            mark_class: 0,
            anchor: Anchor {
                present: false,
                x: 0.,
                y: 0.,
            },
        };
        mark.glyph = handle_from_name(Some(key[..key.len() - 1].to_vec())) as GlyphHandle;
        mark.mark_class = 0 as GlyphClass;
        mark.anchor = otl_anchor_absent();
        match anchor_record.get_typed(b"class", JsonType::String) {
            None => {
                array.push(mark);
            }
            Some(class_name_val) => {
                // Classes are keyed by name; the id given here is replaced
                // below once all names are known and numbered alphabetically.
                let class_name = class_name_val.as_str_bytes().unwrap_or(&[]).to_vec();
                h.entry(class_name).or_insert(0 as GlyphClass);
                mark.anchor.present = true;
                mark.anchor.x = anchor_record.get_num(b"x") as Pos;
                mark.anchor.y = anchor_record.get_num(b"y") as Pos;
                array.push(mark);
            }
        }
    }
    // Number the classes in alphabetical order of their names (the
    // `BTreeMap`'s order).
    // Class ids and the class count are 16-bit. Each mark names one class and
    // a `marks` object is bounded by `support::json_limits`, so the distinct
    // classes cannot outnumber 65,535.
    let class_count = count_u16(h.len());
    for (rank, id) in h.values_mut().enumerate() {
        *id = rank as GlyphClass;
    }
    // Give each mark its final class id, re-reading its class name from the
    // JSON (marks keep only the id). Every mark with an anchor registered
    // its class above, and the loop above pushed one mark per field.
    for (idx, (_, anchor_record)) in fields.iter().enumerate() {
        if array[idx].anchor.present {
            let class_name = anchor_record
                .get_typed(b"class", JsonType::String)
                .and_then(ParsedValue::as_str_bytes)
                .unwrap_or(&[])
                .to_vec();
            array[idx].mark_class = match h.get(&class_name) {
                Some(&id) => id,
                None => 0 as GlyphClass,
            };
        }
    }
    class_count
}

#[cfg(test)]
mod read_mark_array_tests {
    use super::*;
    use crate::support::handle::handle_from_index;
    use crate::support::primitives::GlyphId;
    use crate::table::otl::coverage::push_to_coverage;

    fn coverage_of(gids: &[GlyphId]) -> Coverage {
        let mut cov = Coverage::new();
        for &gid in gids {
            push_to_coverage(&mut cov, handle_from_index(gid) as GlyphHandle);
        }
        cov
    }

    #[test]
    fn otl_read_mark_array_reads_one_record_per_covered_glyph() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes()); // MarkCount
        data.extend_from_slice(&2u16.to_be_bytes()); // Class
        data.extend_from_slice(&6u16.to_be_bytes()); // MarkAnchorOffset (rel to 0)
        data.extend_from_slice(&1u16.to_be_bytes()); // Anchor format (unread)
        data.extend_from_slice(&100i16.to_be_bytes()); // x
        data.extend_from_slice(&(-30i16).to_be_bytes()); // y
        let cov = coverage_of(&[5]);
        let mut array: MarkArray = Vec::new();
        otl_read_mark_array(&mut array, &cov, &data, 0);
        assert_eq!(array.len(), 1);
        assert_eq!(array[0].glyph.index, 5);
        assert_eq!(array[0].mark_class, 2);
        assert!(array[0].anchor.present);
        assert_eq!(array[0].anchor.x, 100.0);
        assert_eq!(array[0].anchor.y, -30.0);
    }

    #[test]
    fn otl_read_mark_array_zero_delta_is_an_absent_anchor() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes()); // MarkCount
        data.extend_from_slice(&0u16.to_be_bytes()); // Class
        data.extend_from_slice(&0u16.to_be_bytes()); // MarkAnchorOffset = 0
        let cov = coverage_of(&[5]);
        let mut array: MarkArray = Vec::new();
        otl_read_mark_array(&mut array, &cov, &data, 0);
        assert!(!array[0].anchor.present);
    }

    #[test]
    fn otl_read_mark_array_count_larger_than_coverage_is_capped_not_a_panic() {
        // MarkCount (2) claiming more marks than the sibling Coverage
        // table actually lists (1 glyph) used to index the Coverage
        // `Vec` out of bounds and panic -- must clamp to `cov.len()`
        // instead.
        let mut data = Vec::new();
        data.extend_from_slice(&2u16.to_be_bytes()); // MarkCount
        data.extend_from_slice(&0u16.to_be_bytes()); // record[0].Class
        data.extend_from_slice(&0u16.to_be_bytes()); // record[0].MarkAnchorOffset
        data.extend_from_slice(&0u16.to_be_bytes()); // record[1].Class
        data.extend_from_slice(&0u16.to_be_bytes()); // record[1].MarkAnchorOffset
        let cov = coverage_of(&[5]);
        let mut array: MarkArray = Vec::new();
        otl_read_mark_array(&mut array, &cov, &data, 0);
        assert_eq!(array.len(), 1);
    }

    #[test]
    fn otl_read_mark_array_count_larger_than_buffer_is_rejected_not_read_oob() {
        // The original had no room check at all for the `mark_count`
        // 4-byte records -- a `MarkCount` this large against a 2-byte
        // buffer used to read straight off the end.
        let data = 1000u16.to_be_bytes().to_vec();
        let cov = coverage_of(&[5]);
        let mut array: MarkArray = Vec::new();
        otl_read_mark_array(&mut array, &cov, &data, 0);
        assert!(array.is_empty());
    }
}
