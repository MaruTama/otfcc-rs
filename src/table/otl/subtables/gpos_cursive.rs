use crate::support::font_reader::FontReader;
use crate::support::handle::{GlyphHandle, handle_from_name};
use crate::support::parsed_json::ParsedValue;
use crate::table::otl::coverage::{Coverage, push_to_coverage, read_coverage};

use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::support::buffer::Buffer;
use crate::support::options::Options;
use crate::support::primitives::GlyphId;

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkgraph::bk_build_block;
use crate::support::built_json::BuiltValue;
use crate::table::otl::coverage::build_coverage;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::subtables::gpos_common::{
    bk_from_anchor, otl_anchor_absent, otl_dump_anchor, otl_parse_anchor, otl_read_anchor,
};
use crate::table::otl::{GposCursiveEntry, GposCursiveSubtable, Subtable};
// `GposCursiveEntry` holds only a `GlyphHandle` plus two plain `Anchor`
// values, so dropping the `Vec` runs `Handle`'s own `Drop` for every entry --
// no per-element dtor needed anymore.
pub(crate) fn dispose_gpos_cursive_subtable(arr: &mut GposCursiveSubtable) {
    *arr = Vec::new();
}
pub fn otl_read_gpos_cursive(data: &[u8], offset: u32, _max_glyphs: GlyphId) -> Option<Subtable> {
    let mut subtable: GposCursiveSubtable = Vec::new();

    'parse: {
        let mut header = match FontReader::new(data).at(offset as usize) {
            Ok(r) => r,
            Err(_) => break 'parse,
        };
        if header.skip(2).is_err() {
            break 'parse; // format, unused
        }
        let Ok(from_rel) = header.u16() else {
            break 'parse;
        };
        let Ok(value_count) = header.u16() else {
            break 'parse;
        };

        let targets: Coverage = read_coverage(data, offset.wrapping_add(from_rel as u32));
        if targets.is_empty() {
            break 'parse;
        }
        if header.require_room(value_count as usize, 4).is_err() {
            break 'parse;
        }
        if value_count as usize != targets.len() {
            break 'parse;
        }

        for j in 0..value_count {
            let Ok(enter_offset) = header.u16() else {
                break 'parse;
            };
            let Ok(exit_offset) = header.u16() else {
                break 'parse;
            };
            let enter = if enter_offset != 0 {
                otl_read_anchor(data, offset.wrapping_add(enter_offset as u32))
            } else {
                otl_anchor_absent()
            };
            let exit = if exit_offset != 0 {
                otl_read_anchor(data, offset.wrapping_add(exit_offset as u32))
            } else {
                otl_anchor_absent()
            };
            subtable.push(GposCursiveEntry {
                target: targets[j as usize].clone(),
                enter,
                exit,
            });
        }
        return Some(Subtable::GposCursive(subtable));
    }

    None
}
pub fn otl_gpos_dump_cursive(_subtable: &Subtable) -> BuiltValue {
    let Subtable::GposCursive(subtable) = _subtable else {
        unreachable!()
    };
    let mut st = BuiltValue::new_object(subtable.len());
    for entry in subtable.iter() {
        let mut rec = BuiltValue::new_object(2);
        rec.push_field(b"enter", otl_dump_anchor(entry.enter));
        rec.push_field(b"exit", otl_dump_anchor(entry.exit));
        st.push_field_bytes_key(&entry.target.name, rec.preserialize());
    }
    st
}
pub fn otl_gpos_parse_cursive(
    _subtable: Option<&ParsedValue>,
    _options: &Options,
) -> Option<Subtable> {
    let mut subtable: GposCursiveSubtable = Vec::new();
    if let Some(fields) = _subtable.and_then(ParsedValue::as_object) {
        for (key, val) in fields {
            if val.as_object().is_some() {
                subtable.push(GposCursiveEntry {
                    target: handle_from_name(Some(key[..key.len() - 1].to_vec())) as GlyphHandle,
                    enter: otl_parse_anchor(val.get(b"enter")),
                    exit: otl_parse_anchor(val.get(b"exit")),
                });
            }
        }
    }
    Some(Subtable::GposCursive(subtable))
}
pub fn otfcc_build_gpos_cursive(
    _subtable: &Subtable,
    mut _heuristics: BuildHeuristics,
) -> Buffer {
    let Subtable::GposCursive(subtable) = _subtable else {
        unreachable!()
    };
    let mut cov: Coverage = Vec::new();
    for entry in subtable.iter() {
        push_to_coverage(&mut cov, entry.target.clone());
    }
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, 1_u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(&cov))),
        ),
        bk_int(BkCellType::B16, (subtable.len()) as u32),
    ]);
    for entry in subtable.iter() {
        bk_push(
            &mut root,
            vec![
                bk_ptr(BkCellType::P16, bk_from_anchor(entry.enter)),
                bk_ptr(BkCellType::P16, bk_from_anchor(entry.exit)),
            ],
        );
    }
    return bk_build_block(root);
}

#[cfg(test)]
mod otl_read_gpos_cursive_tests {
    use super::*;

    #[test]
    fn well_formed_table_with_absent_anchors_reads_the_target() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes()); // format
        data.extend_from_slice(&10u16.to_be_bytes()); // coverageOffset -> 10
        data.extend_from_slice(&1u16.to_be_bytes()); // entryExitCount
        data.extend_from_slice(&0u16.to_be_bytes()); // entryAnchorOffset (absent)
        data.extend_from_slice(&0u16.to_be_bytes()); // exitAnchorOffset (absent)
        // Coverage format 1 at byte 10: one glyph, id 5.
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&5u16.to_be_bytes());
        let result = otl_read_gpos_cursive(&data, 0, 0);
        let Some(Subtable::GposCursive(ref entries)) = result else {
            unreachable!()
        };
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].target.index, 5);
    }

    #[test]
    fn entry_exit_count_mismatch_with_coverage_is_rejected() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&14u16.to_be_bytes()); // coverageOffset -> 14 (after the 2-record array)
        data.extend_from_slice(&2u16.to_be_bytes()); // entryExitCount claims 2
        data.extend_from_slice(&0u16.to_be_bytes()); // record0.enter
        data.extend_from_slice(&0u16.to_be_bytes()); // record0.exit
        data.extend_from_slice(&0u16.to_be_bytes()); // record1.enter
        data.extend_from_slice(&0u16.to_be_bytes()); // record1.exit
        // Coverage format 1 at byte 14: only 1 glyph, not 2.
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&5u16.to_be_bytes());
        let result = otl_read_gpos_cursive(&data, 0, 0);
        assert!(result.is_none());
    }
}
