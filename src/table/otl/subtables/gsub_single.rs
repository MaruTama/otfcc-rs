use crate::support::handle::{
    GlyphHandle, handle_from_index, handle_from_name,
};
use crate::support::parsed_json::ParsedValue;
use crate::table::otl::coverage::{Coverage, push_to_coverage, read_coverage};

use crate::support::font_reader::FontReader;

use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::support::buffer::Buffer;
use crate::support::options::Options;
use crate::support::primitives::GlyphId;

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkgraph::bk_build_block;
use crate::support::built_json::BuiltValue;
use crate::table::otl::coverage::build_coverage_format;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::{GsubSingleEntry, GsubSingleSubtable, Subtable};
// `GsubSingleEntry` holds only two `GlyphHandle`s, so dropping the `Vec`
// runs `Handle`'s own `Drop` for every entry -- no per-element dtor needed
// anymore.
pub(crate) fn dispose_gsub_single_subtable(arr: &mut GsubSingleSubtable) {
    *arr = Vec::new();
}
pub fn otl_read_gsub_single(
    data: &[u8],
    subtable_offset: u32,
    _max_glyphs: GlyphId,
) -> Option<Subtable> {
    let mut subtable: GsubSingleSubtable = Vec::new();

    'parse: {
        let mut header = match FontReader::new(data).at(subtable_offset as usize) {
            Ok(r) => r,
            Err(_) => break 'parse,
        };
        let Ok(subtable_format) = header.u16() else {
            break 'parse;
        };
        let Ok(from_rel) = header.u16() else {
            break 'parse;
        };

        let from: Coverage = read_coverage(data, subtable_offset.wrapping_add(from_rel as u32));
        if from.is_empty() {
            break 'parse;
        }

        let to: Coverage = if subtable_format == 1 {
            // `header`'s cursor is already at `subtable_offset + 4` here.
            let Ok(delta) = header.u16() else {
                break 'parse;
            };
            from.iter()
                .map(|h| handle_from_index((h.index as i32 + delta as i32) as GlyphId) as GlyphHandle)
                .collect()
        } else {
            let Ok(toglyphs) = header.u16() else {
                break 'parse;
            };
            if toglyphs as usize != from.len() {
                break 'parse;
            }
            if header.require_room(toglyphs as usize, 2).is_err() {
                break 'parse;
            }
            (0..toglyphs)
                .map(|_| handle_from_index(header.u16().unwrap() as GlyphId) as GlyphHandle)
                .collect()
        };

        for j_1 in 0..from.len() {
            subtable.push(GsubSingleEntry {
                from: from[j_1].clone(),
                to: to[j_1].clone(),
            });
        }
        return Some(Subtable::GsubSingle(subtable));
    }

    None
}
pub fn otl_gsub_dump_single(_subtable: &Subtable) -> BuiltValue {
    let Subtable::GsubSingle(subtable) = _subtable else {
        unreachable!()
    };
    let mut st = BuiltValue::new_object(subtable.len());
    for entry in subtable.iter() {
        st.push_field_bytes_key(
            &entry.from.name,
            BuiltValue::str_truncated_at_nul(&entry.to.name),
        );
    }
    st
}
pub fn otl_gsub_parse_single(
    _subtable: Option<&ParsedValue>,
    _options: &Options,
) -> Option<Subtable> {
    let mut subtable: GsubSingleSubtable = Vec::new();
    if let Some(fields) = _subtable.and_then(ParsedValue::as_object) {
        for (key, val) in fields {
            if let Some(to_bytes) = val.as_str_bytes() {
                let from: GlyphHandle =
                    handle_from_name(Some(key[..key.len() - 1].to_vec())) as GlyphHandle;
                let to: GlyphHandle = handle_from_name(Some(to_bytes.to_vec())) as GlyphHandle;
                subtable.push(GsubSingleEntry { from, to });
            }
        }
    }
    Some(Subtable::GsubSingle(subtable))
}
pub fn otfcc_build_gsub_single_subtable(
    _subtable: &Subtable,
    heuristics: BuildHeuristics,
) -> Buffer {
    let Subtable::GsubSingle(subtable) = _subtable else {
        unreachable!()
    };
    let mut is_constant_difference: bool = !subtable.is_empty();
    if is_constant_difference {
        let difference: i32 =
            subtable[0].to.index as i32 - subtable[0].from.index as i32;
        is_constant_difference = is_constant_difference as i32 != 0
            && difference < 0x8000_i32
            && difference > -0x8000_i32;
        for entry in subtable.iter().skip(1) {
            let diff_j: i32 = entry.to.index as i32 - entry.from.index as i32;
            is_constant_difference = is_constant_difference
                && diff_j == difference
                && diff_j < 0x8000_i32
                && diff_j > -0x8000_i32;
        }
    }
    let mut cov: Coverage = Vec::new();
    for entry in subtable.iter() {
        push_to_coverage(&mut cov, entry.from.clone());
    }
    let coverage_buf: Buffer =
        build_coverage_format(&cov, heuristics.contains(BuildHeuristics::GSUB_VERT) as u16);
    if is_constant_difference as i32 != 0
        && !heuristics.contains(BuildHeuristics::GSUB_VERT)
    {
        let b: BkBlock = bk_new_block(vec![
            bk_int(BkCellType::B16, 1_u32),
            bk_ptr(BkCellType::P16, bk_new_block_from_buffer(Some(coverage_buf))),
            bk_int(
                BkCellType::B16,
                (subtable[0].to.index as i32
                    - subtable[0].from.index as i32) as u32,
            ),
        ]);
        return bk_build_block(b);
    } else {
        let mut b_0: BkBlock = bk_new_block(vec![
            bk_int(BkCellType::B16, 2_u32),
            bk_ptr(BkCellType::P16, bk_new_block_from_buffer(Some(coverage_buf))),
            bk_int(BkCellType::B16, (subtable.len()) as u32),
        ]);
        for entry in subtable.iter() {
            bk_push(
                &mut b_0,
                vec![bk_int(BkCellType::B16, (entry.to.index as i32) as u32)],
            );
        }
        return bk_build_block(b_0);
    };
}

#[cfg(test)]
mod otl_read_gsub_single_tests {
    use super::*;

    #[test]
    fn format1_applies_a_constant_delta() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes()); // format
        data.extend_from_slice(&6u16.to_be_bytes()); // coverageOffset -> 6
        data.extend_from_slice(&100i16.to_be_bytes()); // deltaGlyphID
        // Coverage format 1 at byte 6: one glyph, id 5.
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&5u16.to_be_bytes());
        let result = otl_read_gsub_single(&data, 0, 0);
        let Some(Subtable::GsubSingle(ref entries)) = result else {
            unreachable!()
        };
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].from.index, 5);
        assert_eq!(entries[0].to.index, 105);
    }

    #[test]
    fn format2_uses_an_explicit_glyph_array() {
        let mut data = Vec::new();
        data.extend_from_slice(&2u16.to_be_bytes()); // format
        data.extend_from_slice(&8u16.to_be_bytes()); // coverageOffset -> 8
        data.extend_from_slice(&1u16.to_be_bytes()); // glyphCount
        data.extend_from_slice(&42u16.to_be_bytes()); // substituteGlyphIDs[0]
        // Coverage format 1 at byte 8: one glyph, id 5.
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&5u16.to_be_bytes());
        let result = otl_read_gsub_single(&data, 0, 0);
        let Some(Subtable::GsubSingle(ref entries)) = result else {
            unreachable!()
        };
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].from.index, 5);
        assert_eq!(entries[0].to.index, 42);
    }

    #[test]
    fn glyph_count_mismatch_with_coverage_is_rejected() {
        let mut data = Vec::new();
        data.extend_from_slice(&2u16.to_be_bytes()); // format
        data.extend_from_slice(&10u16.to_be_bytes()); // coverageOffset -> 10, after the 2-entry substitute array
        data.extend_from_slice(&2u16.to_be_bytes()); // glyphCount claims 2
        data.extend_from_slice(&42u16.to_be_bytes());
        data.extend_from_slice(&43u16.to_be_bytes());
        // Coverage format 1 at byte 10: only 1 glyph, not 2.
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&5u16.to_be_bytes());
        let result = otl_read_gsub_single(&data, 0, 0);
        assert!(result.is_none());
    }
}
