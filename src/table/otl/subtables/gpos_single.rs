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
    bk_gpos_value, gpos_dump_value, gpos_parse_value, position_format_length, read_gpos_value,
    required_position_format,
};
use crate::table::otl::{GposSingleEntry, GposSingleSubtable, PositionValue, Subtable};
// `GposSingleEntry` holds only a `GlyphHandle` plus a plain `PositionValue`,
// so dropping the `Vec` runs `Handle`'s own `Drop` for every entry -- no
// per-element dtor needed anymore.
pub(crate) fn dispose_gpos_single_subtable(arr: &mut GposSingleSubtable) {
    *arr = Vec::new();
}
pub fn otl_read_gpos_single(data: &[u8], offset: u32, _max_glyphs: GlyphId) -> Option<Subtable> {
    let mut subtable: GposSingleSubtable = Vec::new();

    'parse: {
        let mut header = match FontReader::new(data).at(offset as usize) {
            Ok(r) => r,
            Err(_) => break 'parse,
        };
        let Ok(subtable_format) = header.u16() else {
            break 'parse;
        };
        let Ok(from_rel) = header.u16() else {
            break 'parse;
        };

        let targets: Coverage = read_coverage(data, offset.wrapping_add(from_rel as u32));
        if targets.is_empty() {
            break 'parse;
        }

        if subtable_format == 1 {
            let Ok(value_format) = header.u16() else {
                break 'parse;
            };
            let v: PositionValue = read_gpos_value(data, offset.wrapping_add(6), value_format);
            for target in &targets {
                subtable.push(GposSingleEntry {
                    target: target.clone(),
                    value: v,
                });
            }
        } else {
            let Ok(value_format) = header.u16() else {
                break 'parse;
            };
            let Ok(value_count) = header.u16() else {
                break 'parse;
            };
            let stride = position_format_length(value_format) as usize;
            if header.require_room(value_count as usize, stride).is_err() {
                break 'parse;
            }
            if value_count as usize != targets.len() {
                break 'parse;
            }
            for (j, target) in targets.iter().enumerate() {
                subtable.push(GposSingleEntry {
                    target: target.clone(),
                    value: read_gpos_value(
                        data,
                        offset.wrapping_add(8).wrapping_add((j * stride) as u32),
                        value_format,
                    ),
                });
            }
        }

        return Some(Subtable::GposSingle(subtable));
    }

    None
}
pub fn otl_gpos_dump_single(_subtable: &Subtable) -> BuiltValue {
    let Subtable::GposSingle(subtable) = _subtable else {
        unreachable!()
    };
    let mut st = BuiltValue::new_object(subtable.len());
    for entry in subtable.iter() {
        st.push_field_bytes_key(&entry.target.name, gpos_dump_value(entry.value));
    }
    st
}
pub fn otl_gpos_parse_single(
    _subtable: Option<&ParsedValue>,
    _options: &Options,
) -> Option<Subtable> {
    let mut subtable: GposSingleSubtable = Vec::new();
    if let Some(fields) = _subtable.and_then(ParsedValue::as_object) {
        for (key, val) in fields {
            if val.as_object().is_some() {
                subtable.push(GposSingleEntry {
                    target: handle_from_name(Some(key[..key.len() - 1].to_vec())) as GlyphHandle,
                    value: gpos_parse_value(Some(val)),
                });
            }
        }
    }
    Some(Subtable::GposSingle(subtable))
}
pub fn otfcc_build_gpos_single(
    _subtable: &Subtable,
    mut _heuristics: BuildHeuristics,
) -> Buffer {
    let Subtable::GposSingle(subtable) = _subtable else {
        unreachable!()
    };
    let mut is_const: bool = !subtable.is_empty();
    let mut format: u16 = 0_u16;
    if !subtable.is_empty() {
        for entry in subtable.iter() {
            is_const = is_const
                && entry.value.dx == subtable[0].value.dx
                && entry.value.dy == subtable[0].value.dy
                && entry.value.d_width == subtable[0].value.d_width
                && entry.value.d_height == subtable[0].value.d_height;
            format |= required_position_format(entry.value) as u16;
        }
    }
    let mut cov: Coverage = Vec::new();
    for entry in subtable.iter() {
        push_to_coverage(&mut cov, entry.target.clone());
    }
    let coverage_buf: Buffer = build_coverage(&cov);
    if is_const {
        let b: BkBlock = bk_new_block(vec![
            bk_int(BkCellType::B16, 1_u32),
            bk_ptr(BkCellType::P16, bk_new_block_from_buffer(Some(coverage_buf))),
            bk_int(BkCellType::B16, (format as i32) as u32),
            bk_ptr(
                BkCellType::Embed,
                Some(bk_gpos_value(subtable[0].value, format)),
            ),
        ]);
        return bk_build_block(b);
    } else {
        let mut b_0: BkBlock = bk_new_block(vec![
            bk_int(BkCellType::B16, 2_u32),
            bk_ptr(BkCellType::P16, bk_new_block_from_buffer(Some(coverage_buf))),
            bk_int(BkCellType::B16, (format as i32) as u32),
            bk_int(BkCellType::B16, (subtable.len()) as u32),
        ]);
        for entry in subtable.iter() {
            bk_push(
                &mut b_0,
                vec![bk_ptr(
                    BkCellType::Embed,
                    Some(bk_gpos_value(entry.value, format)),
                )],
            );
        }
        return bk_build_block(b_0);
    };
}

#[cfg(test)]
mod otl_read_gpos_single_tests {
    use super::*;

    #[test]
    fn format1_applies_one_shared_value_to_every_glyph() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes()); // format
        data.extend_from_slice(&8u16.to_be_bytes()); // coverageOffset -> 8
        data.extend_from_slice(&1u16.to_be_bytes()); // valueFormat: FORMAT_DX only
        data.extend_from_slice(&77i16.to_be_bytes()); // Value.dx
        // Coverage format 1 at byte 8: one glyph, id 9.
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&9u16.to_be_bytes());
        let result = otl_read_gpos_single(&data, 0, 0);
        let Some(Subtable::GposSingle(ref entries)) = result else {
            unreachable!()
        };
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].target.index, 9);
        assert_eq!(entries[0].value.dx, 77.0);
    }

    #[test]
    fn format2_reads_a_per_glyph_value() {
        let mut data = Vec::new();
        data.extend_from_slice(&2u16.to_be_bytes()); // format
        data.extend_from_slice(&10u16.to_be_bytes()); // coverageOffset -> 10
        data.extend_from_slice(&1u16.to_be_bytes()); // valueFormat: FORMAT_DX only
        data.extend_from_slice(&1u16.to_be_bytes()); // valueCount
        data.extend_from_slice(&50i16.to_be_bytes()); // value[0].dx
        // Coverage format 1 at byte 10: one glyph, id 5.
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&5u16.to_be_bytes());
        let result = otl_read_gpos_single(&data, 0, 0);
        let Some(Subtable::GposSingle(ref entries)) = result else {
            unreachable!()
        };
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].target.index, 5);
        assert_eq!(entries[0].value.dx, 50.0);
    }
}
