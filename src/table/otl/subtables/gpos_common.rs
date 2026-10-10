use crate::support::handle::{
    GlyphHandle, Handle, HandleState, handle_from_name,
};
use otfcc_json::ParsedValue;
use crate::table::otl::coverage::Coverage;

use otfcc_binary::FontReader;

use otfcc_binary::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_push};
use otfcc_binary::Buffer;
use otfcc_json::BuiltValue;
use crate::support::primitives::{GlyphClass, Pos, count_u16};
use crate::table::otl::{Anchor, MarkArray, MarkRecord, PositionValue};
use otfcc_json::JsonType;
// `MarkRecord` holds only a `GlyphHandle` plus a plain `Anchor`, so dropping
// the `Vec` runs `Handle`'s own `Drop` for every entry -- no per-element
// dtor needed anymore.
pub(crate) fn dispose_mark_array(arr: &mut MarkArray) {
    *arr = Vec::new();
}
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
/// SUBTABLES_PER_LOOKUP`/`MAX_TOTAL_LOOKUPS_PER_TABLE` ceiling of 300,000 --
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
        let mark_class = r.u16().unwrap() as GlyphClass;
        let delta = r.u16().unwrap();
        let anchor = if delta != 0 {
            otl_read_anchor(data, offset.wrapping_add(delta as u32))
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
pub fn otl_anchor_absent() -> Anchor {
    let anchor: Anchor = Anchor {
        present: false,
        x: 0_i32 as Pos,
        y: 0_i32 as Pos,
    };
    return anchor;
}
pub fn otl_read_anchor(data: &[u8], offset: u32) -> Anchor {
    let mut anchor: Anchor = Anchor {
        present: false,
        x: 0_i32 as Pos,
        y: 0_i32 as Pos,
    };
    let Ok(bytes) = FontReader::new(data).at(offset as usize).and_then(|mut r| r.bytes(6)) else {
        return anchor;
    };
    anchor.present = true;
    anchor.x = i16::from_be_bytes([bytes[2], bytes[3]]) as Pos;
    anchor.y = i16::from_be_bytes([bytes[4], bytes[5]]) as Pos;
    anchor
}
pub fn otl_dump_anchor(a: Anchor) -> BuiltValue {
    if a.present {
        let mut v = BuiltValue::new_object(2);
        v.push_field(b"x", BuiltValue::position(a.x));
        v.push_field(b"y", BuiltValue::position(a.y));
        v
    } else {
        BuiltValue::Null
    }
}
pub fn otl_parse_anchor(v: Option<&ParsedValue>) -> Anchor {
    let mut anchor: Anchor = Anchor {
        present: false,
        x: 0_i32 as Pos,
        y: 0_i32 as Pos,
    };
    let Some(v) = v.filter(|v| v.as_object().is_some()) else {
        return anchor;
    };
    anchor.present = true;
    anchor.x = v.get_num_or(b"x", 0.0) as Pos;
    anchor.y = v.get_num_or(b"y", 0.0) as Pos;
    return anchor;
}
pub fn bk_from_anchor(a: Anchor) -> Option<BkBlock> {
    if !a.present {
        return None;
    }
    return Some(bk_new_block(vec![
        bk_int(BkCellType::B16, 1_u32),
        bk_int(BkCellType::B16, (a.x as i16 as i32) as u32),
        bk_int(BkCellType::B16, (a.y as i16 as i32) as u32),
    ]));
}
pub static FORMAT_DX: u8 = 1_u8;
pub static FORMAT_DY: u8 = 2_u8;
pub static FORMAT_DWIDTH: u8 = 4_u8;
pub static FORMAT_DHEIGHT: u8 = 8_u8;
/// Size in bytes of a GPOS ValueRecord with this ValueFormat: 2 bytes per
/// field whose bit is set among the low 8 bits (`XPlacement` through
/// `YAdvDevice`); the reserved high bits never contribute a field.
pub fn position_format_length(format: u16) -> u8 {
    return ((format & 0xff).count_ones() << 1) as u8;
}
pub fn position_zero() -> PositionValue {
    let v: PositionValue = PositionValue {
        dx: 0.0f64,
        dy: 0.0f64,
        d_width: 0.0f64,
        d_height: 0.0f64,
    };
    return v;
}
pub fn read_gpos_value(data: &[u8], offset: u32, format: u16) -> PositionValue {
    let mut v: PositionValue = PositionValue {
        dx: 0.0f64,
        dy: 0.0f64,
        d_width: 0.0f64,
        d_height: 0.0f64,
    };
    let len = position_format_length(format) as usize;
    let Ok(bytes) = FontReader::new(data).at(offset as usize).and_then(|mut r| r.bytes(len))
    else {
        return v;
    };
    let mut pos = 0;
    if format & FORMAT_DX as u16 != 0 {
        v.dx = i16::from_be_bytes([bytes[pos], bytes[pos + 1]]) as Pos;
        pos += 2;
    }
    if format & FORMAT_DY as u16 != 0 {
        v.dy = i16::from_be_bytes([bytes[pos], bytes[pos + 1]]) as Pos;
        pos += 2;
    }
    if format & FORMAT_DWIDTH as u16 != 0 {
        v.d_width = i16::from_be_bytes([bytes[pos], bytes[pos + 1]]) as Pos;
        pos += 2;
    }
    if format & FORMAT_DHEIGHT as u16 != 0 {
        v.d_height = i16::from_be_bytes([bytes[pos], bytes[pos + 1]]) as Pos;
    }
    v
}
pub fn gpos_dump_value(value: PositionValue) -> BuiltValue {
    let mut v = BuiltValue::new_object(4);
    if value.dx != 0. {
        v.push_field(b"dx", BuiltValue::position(value.dx));
    }
    if value.dy != 0. {
        v.push_field(b"dy", BuiltValue::position(value.dy));
    }
    if value.d_width != 0. {
        v.push_field(b"dWidth", BuiltValue::position(value.d_width));
    }
    if value.d_height != 0. {
        v.push_field(b"dHeight", BuiltValue::position(value.d_height));
    }
    v.preserialize()
}
pub fn gpos_parse_value(pos: Option<&ParsedValue>) -> PositionValue {
    let mut v: PositionValue = PositionValue {
        dx: 0.0f64,
        dy: 0.0f64,
        d_width: 0.0f64,
        d_height: 0.0f64,
    };
    let Some(pos) = pos.filter(|p| p.as_object().is_some()) else {
        return v;
    };
    v.dx = pos.get_num(b"dx") as Pos;
    v.dy = pos.get_num(b"dy") as Pos;
    v.d_width = pos.get_num(b"dWidth") as Pos;
    v.d_height = pos.get_num(b"dHeight") as Pos;
    return v;
}
pub fn required_position_format(v: PositionValue) -> u8 {
    return ((if v.dx != 0. {
        FORMAT_DX as i32
    } else {
        0_i32
    }) | (if v.dy != 0. {
        FORMAT_DY as i32
    } else {
        0_i32
    }) | (if v.d_width != 0. {
        FORMAT_DWIDTH as i32
    } else {
        0_i32
    }) | (if v.d_height != 0. {
        FORMAT_DHEIGHT as i32
    } else {
        0_i32
    })) as u8;
}
pub fn write_gpos_value(buf: &mut Buffer, v: PositionValue, format: u16) {
    if format as i32 & FORMAT_DX as i32 != 0 {
        buf.write_i16be(v.dx as i16);
    }
    if format as i32 & FORMAT_DY as i32 != 0 {
        buf.write_i16be(v.dy as i16);
    }
    if format as i32 & FORMAT_DWIDTH as i32 != 0 {
        buf.write_i16be(v.d_width as i16);
    }
    if format as i32 & FORMAT_DHEIGHT as i32 != 0 {
        buf.write_i16be(v.d_height as i16);
    }
}
pub fn bk_gpos_value(v: PositionValue, format: u16) -> BkBlock {
    let mut b: BkBlock = bk_new_block(Vec::new());
    if format as i32 & FORMAT_DX as i32 != 0 {
        bk_push(
            &mut b,
            vec![bk_int(
                BkCellType::B16,
                (v.dx as i16 as i32) as u32,
            )],
        );
    }
    if format as i32 & FORMAT_DY as i32 != 0 {
        bk_push(
            &mut b,
            vec![bk_int(
                BkCellType::B16,
                (v.dy as i16 as i32) as u32,
            )],
        );
    }
    if format as i32 & FORMAT_DWIDTH as i32 != 0 {
        bk_push(
            &mut b,
            vec![bk_int(
                BkCellType::B16,
                (v.d_width as i16 as i32) as u32,
            )],
        );
    }
    if format as i32 & FORMAT_DHEIGHT as i32 != 0 {
        bk_push(
            &mut b,
            vec![bk_int(
                BkCellType::B16,
                (v.d_height as i16 as i32) as u32,
            )],
        );
    }
    return b;
}

#[cfg(test)]
mod read_anchor_and_value_tests {
    use super::*;
    use crate::support::handle::handle_from_index;
    use crate::support::primitives::GlyphId;
    use crate::table::otl::coverage::push_to_coverage;

    #[test]
    fn otl_read_anchor_well_formed_reads_x_and_y() {
        let mut data = vec![0u8; 6];
        data[2..4].copy_from_slice(&100i16.to_be_bytes());
        data[4..6].copy_from_slice(&(-50i16).to_be_bytes());
        let anchor = otl_read_anchor(&data, 0);
        assert!(anchor.present);
        assert_eq!(anchor.x, 100.0);
        assert_eq!(anchor.y, -50.0);
    }

    #[test]
    fn otl_read_anchor_truncated_is_absent_not_oob() {
        let data = vec![0u8; 4];
        let anchor = otl_read_anchor(&data, 0);
        assert!(!anchor.present);
    }

    #[test]
    fn read_gpos_value_reads_only_the_fields_the_format_selects() {
        let format = FORMAT_DX as u16 | FORMAT_DY as u16;
        let mut data = vec![0u8; 4];
        data[0..2].copy_from_slice(&10i16.to_be_bytes());
        data[2..4].copy_from_slice(&20i16.to_be_bytes());
        let v = read_gpos_value(&data, 0, format);
        assert_eq!((v.dx, v.dy, v.d_width, v.d_height), (10.0, 20.0, 0.0, 0.0));
    }

    #[test]
    fn read_gpos_value_truncated_is_zero_not_oob() {
        let format = FORMAT_DX as u16 | FORMAT_DY as u16;
        let data = vec![0u8; 2]; // needs 4 bytes for dx+dy, only 2 present
        let v = read_gpos_value(&data, 0, format);
        assert_eq!((v.dx, v.dy), (0.0, 0.0));
    }

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

    #[test]
    fn position_format_length_is_two_bytes_per_low_format_bit() {
        // The same per-bit count the removed 256-entry `BITS_IN` table
        // encoded, checked across every possible format word.
        for format in 0..=u16::MAX {
            let fields = (0..8).filter(|bit| format & (1 << bit) != 0).count() as u8;
            assert_eq!(position_format_length(format), fields * 2, "format {format:#06x}");
        }
    }
}
