//! Reading the binary `cmap` table: formats 4 and 12 for code point
//! mappings, format 14 for variation sequences.

use crate::font::sfnt::Packet;
use crate::support::primitives::{TableId, Unicode};
use crate::table::cmap::{
    CmapTable, CmapUvsKey, cmap_lookup, encode_cmap_by_index, encode_cmap_uvs_by_index,
};
use otfcc_binary::{FontReader, ReadError};

// Every reader below takes the whole cmap table's bytes (`data`) and an
// absolute offset into it, so "how much is left" is always `data.len() -
// offset`, checked by `FontReader::at`, and an offset past the table cannot
// underflow anything. Counts read from the file (`n_groups` and friends)
// are checked with `require_room`, which cannot overflow either.
//
// The total number of code point mappings across the whole table is
// budgeted too: each group or range is bounded on its own, but a small
// subtable can pack enough of them to take minutes (a fuzzed format 12
// subtable did), and many variation selector records can share one
// non-default UVS subtable, each adding new entries (a fuzzed 70 KB input
// reached 2.1 GB; `tests/fuzz-corpus/known-issues/
// otf-dump-cmap-uvs-non-default-aliasing-oom.bin`). Even several
// subtables covering all of Unicode total only a few million mappings.
//
// `pub(crate)` for `otf_reader.rs`'s regression test.
pub(crate) const MAX_TOTAL_CMAP_MAPPINGS: u32 = 4_000_000;
fn read_format12(data: &[u8], offset: usize, cmap: &mut CmapTable, budget: &mut u32) {
    let mut r = match FontReader::new(data).at(offset) {
        Ok(r) => r,
        Err(_) => return,
    };
    if r.skip(12).is_err() {
        return; // format, reserved, length, language
    }
    let Ok(n_groups) = r.u32() else { return };
    if r.require_room(n_groups as usize, 12).is_err() {
        return;
    }
    for _ in 0..n_groups {
        let start_code = r.u32().expect("require_room checked the remaining bytes");
        let end_code = r.u32().expect("require_room checked the remaining bytes");
        let start_gid = r.u32().expect("require_room checked the remaining bytes");
        // `startCharCode`/`endCharCode` are raw 32-bit fields with no
        // structural guard tying them to a sane range: a single 12-byte
        // group can claim `endCharCode = 0xFFFFFFFF`, which without a
        // clamp is not just slow but a genuine infinite loop --
        // `c.wrapping_add(1)` at `0xFFFFFFFF` wraps back to `0`, and
        // `0 <= 0xFFFFFFFF` is true forever. Even short of that, a group
        // like `0x0..0xffff0000` forces ~4 billion iterations from 12
        // bytes of input. No real cmap needs to enumerate past the
        // Unicode codepoint ceiling, so clamp the walked range to it --
        // this changes nothing for any well-formed group (real codepoints
        // are always <= 0x10FFFF) and turns the unbounded/infinite cases
        // into a bounded, still-correct partial read of the group.
        let clamped_end = end_code.min(0x10ffff);
        for c in start_code..=clamped_end {
            if *budget == 0 {
                break;
            }
            *budget -= 1;
            encode_cmap_by_index(
                cmap,
                c as i32,
                c.wrapping_sub(start_code).wrapping_add(start_gid) as u16,
            );
        }
    }
}
fn read_format4(data: &[u8], offset: usize, cmap: &mut CmapTable, budget: &mut u32) {
    let available = data.len().saturating_sub(offset);
    if available < 14 {
        return;
    }
    let mut header = match FontReader::new(data).at(offset) {
        Ok(r) => r,
        Err(_) => return,
    };
    if header.skip(6).is_err() {
        return; // format, length, language
    }
    let Ok(seg_count_x2) = header.u16() else {
        return;
    };
    let segments_count = (seg_count_x2 / 2) as usize;
    let Some(needed) = segments_count
        .checked_mul(8)
        .and_then(|n| n.checked_add(16))
    else {
        return;
    };
    if available < needed {
        return;
    }

    // All four parallel arrays are subtable-relative, matching the
    // original's `start.offset(...)` layout: endCode[] at 14, a 2-byte
    // reserved pad, then startCode[]/idDelta[]/idRangeOffset[], each
    // `segments_count` u16s.
    let end_code_rel = 14usize;
    let start_code_rel = end_code_rel + segments_count * 2 + 2;
    let id_delta_rel = start_code_rel + segments_count * 2;
    let id_range_offset_rel = id_delta_rel + segments_count * 2;

    let read_u16 = |rel: usize| -> Option<u16> {
        FontReader::new(data)
            .at(offset + rel)
            .ok()
            .and_then(|mut r| r.u16().ok())
    };

    for j in 0..segments_count {
        let Some(end_code) = read_u16(end_code_rel + j * 2) else {
            return;
        };
        let Some(start_code) = read_u16(start_code_rel + j * 2) else {
            return;
        };
        let Some(id_delta_raw) = read_u16(id_delta_rel + j * 2) else {
            return;
        };
        let id_delta = id_delta_raw as i16;
        let id_range_offset_entry_rel = id_range_offset_rel + j * 2;
        let Some(id_range_offset) = read_u16(id_range_offset_entry_rel) else {
            return;
        };
        // `c < 0xffff && c <= end_code` combine into one fixed upper bound,
        // computed once here rather than re-checked every step: neither
        // half depends on `budget`, so `start_code..=upper` walks exactly
        // the same segment the `while` did (empty when `start_code as u32
        // > upper`, same as the `while`'s own zero-iteration case).
        // `budget` only ever causes an early `break`, checked first in the
        // body, the same position the `while`'s `&& *budget > 0` checked
        // it in.
        let upper = u32::from(end_code).min(0xfffe);
        if id_range_offset == 0 {
            for c in u32::from(start_code)..=upper {
                if *budget == 0 {
                    break;
                }
                *budget -= 1;
                let gid = (c.wrapping_add(id_delta as u32) & 0xffff) as u16;
                encode_cmap_by_index(cmap, c as i32, gid);
            }
        } else {
            for c in u32::from(start_code)..=upper {
                if *budget == 0 {
                    break;
                }
                *budget -= 1;
                // idRangeOffset's value is a byte distance measured from the
                // idRangeOffset array *entry itself* -- matches the
                // original's `.wrapping_add(id_range_offset_offset)`.
                let glyph_offset_rel = u32::from(id_range_offset)
                    .wrapping_add(c.wrapping_sub(u32::from(start_code)).wrapping_mul(2))
                    .wrapping_add(id_range_offset_entry_rel as u32);
                if let Some(raw) = read_u16(glyph_offset_rel as usize) {
                    let gid = ((i32::from(raw) + i32::from(id_delta)) & 0xffff) as u16;
                    encode_cmap_by_index(cmap, c as i32, gid);
                }
            }
        }
    }
}
fn read_uvs_default(
    data: &[u8],
    offset: usize,
    selector: Unicode,
    cmap: &mut CmapTable,
    budget: &mut u32,
) {
    let mut r = match FontReader::new(data).at(offset) {
        Ok(r) => r,
        Err(_) => return,
    };
    let Ok(num_ranges) = r.u32() else { return };
    if r.require_room(num_ranges as usize, 4).is_err() {
        return;
    }
    for _ in 0..num_ranges {
        let start_unicode_value = r.u24().expect("require_room checked the remaining bytes");
        let additional_count = r.u8().expect("require_room checked the remaining bytes");
        let end = start_unicode_value.wrapping_add(u32::from(additional_count));
        // `start_unicode_value..=end`: `end` is fixed above (`u24` plus a
        // `u8` count, never wrapping in practice since both are far below
        // `u32::MAX`), and doesn't depend on `budget` -- an empty range
        // when `start_unicode_value > end` matches the `while`'s own
        // zero-iteration case exactly. `budget` only ever causes an early
        // `break`, checked first in the body, the same position the
        // `while`'s own `&& *budget > 0` checked it in.
        for u in start_unicode_value..=end {
            if *budget == 0 {
                break;
            }
            *budget -= 1;
            if let Some(gid) = cmap_lookup(cmap, u as i32).map(|g| g.index) {
                encode_cmap_uvs_by_index(
                    cmap,
                    CmapUvsKey {
                        unicode: u,
                        selector,
                    },
                    gid,
                );
            }
        }
    }
}
fn read_uvs_non_default(
    data: &[u8],
    offset: usize,
    selector: Unicode,
    cmap: &mut CmapTable,
    budget: &mut u32,
) {
    let mut r = match FontReader::new(data).at(offset) {
        Ok(r) => r,
        Err(_) => return,
    };
    let Ok(num_mappings) = r.u32() else { return };
    if r.require_room(num_mappings as usize, 5).is_err() {
        return;
    }
    for _ in 0..num_mappings {
        if *budget == 0 {
            break;
        }
        *budget -= 1;
        let unicode_value = r.u24().expect("require_room checked the remaining bytes");
        let glyph_id = r.u16().expect("require_room checked the remaining bytes");
        encode_cmap_uvs_by_index(
            cmap,
            CmapUvsKey {
                unicode: unicode_value,
                selector,
            },
            glyph_id,
        );
    }
}
fn read_format14(data: &[u8], offset: usize, cmap: &mut CmapTable, budget: &mut u32) {
    let mut r = match FontReader::new(data).at(offset) {
        Ok(r) => r,
        Err(_) => return,
    };
    if r.skip(6).is_err() {
        return; // format, length
    }
    let Ok(n_groups) = r.u32() else { return }; // numVarSelectorRecords, at offset+6
    // Requires one byte more than the VarSelectorRecord array (10 + 11 *
    // n_groups) needs, as the C code did; stricter, so kept.
    let Some(needed) = (n_groups as usize)
        .checked_mul(11)
        .and_then(|n| n.checked_add(11))
    else {
        return;
    };
    if data.len().saturating_sub(offset) < needed {
        return;
    }
    for j in 0..n_groups as usize {
        let record_rel = 10 + 11 * j;
        let Ok(selector) = FontReader::new(data)
            .at(offset + record_rel)
            .and_then(|mut r| r.u24())
        else {
            return;
        };
        let Ok(default_uvs_offset) = FontReader::new(data)
            .at(offset + record_rel + 3)
            .and_then(|mut r| r.u32())
        else {
            return;
        };
        let Ok(non_default_uvs_offset) = FontReader::new(data)
            .at(offset + record_rel + 7)
            .and_then(|mut r| r.u32())
        else {
            return;
        };
        if default_uvs_offset != 0
            && let Some(sub_offset) = offset.checked_add(default_uvs_offset as usize)
        {
            read_uvs_default(data, sub_offset, selector, cmap, budget);
        }
        if non_default_uvs_offset != 0
            && let Some(sub_offset) = offset.checked_add(non_default_uvs_offset as usize)
        {
            read_uvs_non_default(data, sub_offset, selector, cmap, budget);
        }
    }
}
fn read_cmap_mapping_table(
    data: &[u8],
    offset: usize,
    cmap: &mut CmapTable,
    required_format: TableId,
    budget: &mut u32,
) {
    let Some(format) = FontReader::new(data)
        .at(offset)
        .ok()
        .and_then(|mut r| r.u16().ok())
    else {
        return;
    };
    if format == required_format {
        if format == 4 {
            read_format4(data, offset, cmap, budget);
        } else if format == 12 {
            read_format12(data, offset, cmap, budget);
        }
    }
}
fn read_cmap_mapping_table_uvs(data: &[u8], offset: usize, cmap: &mut CmapTable, budget: &mut u32) {
    let Some(format) = FontReader::new(data)
        .at(offset)
        .ok()
        .and_then(|mut r| r.u16().ok())
    else {
        return;
    };
    if format == 14 {
        read_format14(data, offset, cmap, budget);
    }
}
#[inline]
fn is_valid_cmap_encoding(platform: u16, encoding: u16) -> bool {
    matches!(
        (platform, encoding),
        (0, 3) | (0, 4) | (0, 5) | (3, 1) | (3, 10)
    )
}
pub static FORMAT_PRIORITIES: [TableId; 3] = [12, 4, 0];
// `FORMAT_PRIORITIES` ends with a `0` sentinel that is also, confusingly,
// a real cmap subtable format number (Apple standard byte encoding) --
// `take_while(|&f| f != 0)` stops before reaching it, matching the
// original's `while FORMAT_PRIORITIES[k] != 0` loop exactly: only formats
// 12 and 4 are ever dispatched as a `required_format`, never 0.
fn decode_cmap(data: &[u8]) -> Result<Box<CmapTable>, ReadError> {
    let mut header = FontReader::new(data);
    header.skip(2)?; // version
    let num_tables = header.u16()? as usize;
    header.require_room(num_tables, 8)?;

    let mut cmap_box = Box::new(CmapTable {
        unicodes: std::collections::BTreeMap::new(),
        uvs: std::collections::BTreeMap::new(),
    });
    let cmap: &mut CmapTable = cmap_box.as_mut();

    // Nothing in the cmap directory requires each entry's subtable offset
    // to be distinct -- the spec explicitly allows encoding records to
    // share a subtable (e.g. the Windows Unicode BMP and full-repertoire
    // records legitimately pointing at the same data). `num_tables` valid
    // entries can all alias one large format 4/12 subtable, and without
    // dedup every alias re-parses (and re-walks every codepoint of) that
    // same subtable from scratch -- an offset-aliasing amplification of
    // the same shape as the OTL script/language one (see
    // `MAX_TOTAL_LANGUAGES` in `table/otl/read.rs`), except here the
    // multiplier is `num_tables` itself (bounded only by table size / 8
    // bytes per entry) rather than a fixed cap. A fuzz session found a
    // small font with just two aliasing pairs already pushing a single
    // parse past 47 seconds under the fuzzer's instrumentation.
    //
    // Deduping by raw subtable offset is safe for well-formed fonts:
    // re-parsing the same bytes at the same offset is idempotent (every
    // insert into `cmap.unicodes`/`cmap.uvs` produces the same mapping
    // each time), so skipping the repeat parses never changes the final
    // table -- only the wasted work is removed.
    let mut budget: u32 = MAX_TOTAL_CMAP_MAPPINGS;
    let mut processed_offsets: std::collections::BTreeSet<usize> =
        std::collections::BTreeSet::new();
    for &required_format in FORMAT_PRIORITIES.iter().take_while(|&&f| f != 0) {
        for j in 0..num_tables {
            let entry_rel = 4 + 8 * j;
            let mut entry = FontReader::new(data).at(entry_rel)?;
            let platform = entry.u16()?;
            let encoding = entry.u16()?;
            if is_valid_cmap_encoding(platform, encoding) {
                let table_offset = FontReader::new(data).at(entry_rel + 4)?.u32()? as usize;
                let Some(format) = FontReader::new(data)
                    .at(table_offset)
                    .ok()
                    .and_then(|mut r| r.u16().ok())
                else {
                    continue;
                };
                if format == required_format && processed_offsets.insert(table_offset) {
                    read_cmap_mapping_table(data, table_offset, cmap, required_format, &mut budget);
                }
            }
        }
    }
    let mut processed_uvs_offsets: std::collections::BTreeSet<usize> =
        std::collections::BTreeSet::new();
    for j in 0..num_tables {
        let entry_rel = 4 + 8 * j;
        let mut entry = FontReader::new(data).at(entry_rel)?;
        let platform = entry.u16()?;
        let encoding = entry.u16()?;
        if is_valid_cmap_encoding(platform, encoding) {
            let table_offset = FontReader::new(data).at(entry_rel + 4)?.u32()? as usize;
            if processed_uvs_offsets.insert(table_offset) {
                read_cmap_mapping_table_uvs(data, table_offset, cmap, &mut budget);
            }
        }
    }
    Ok(cmap_box)
}
/// Reads the 'cmap' table from `packet`. Returns `None` when the table is absent
/// or corrupted (the warning "table 'cmap' corrupted" is logged).
///
/// Format 12 subtables are read before format 4 ones, and every subtable offset is
/// read at most once. When a code point is mapped by several subtables, the first
/// mapping read wins. The total number of mappings is capped
/// (`MAX_TOTAL_CMAP_MAPPINGS`). Format 14 subtables give the variation
/// sequences.
pub fn read_cmap(packet: &Packet) -> Option<Box<CmapTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_CMAP)?;
    match decode_cmap(&table.data) {
        Ok(cmap) => Some(cmap),
        Err(_) => {
            tracing::warn!("table 'cmap' corrupted.\n");
            None
        }
    }
}

#[cfg(test)]
mod cmap_read_tests {
    use super::*;

    fn empty_cmap() -> Box<CmapTable> {
        Box::new(CmapTable {
            unicodes: std::collections::BTreeMap::new(),
            uvs: std::collections::BTreeMap::new(),
        })
    }

    #[test]
    fn format4_direct_delta_segment_maps_one_codepoint() {
        let mut data = Vec::new();
        data.extend_from_slice(&4u16.to_be_bytes()); // format
        data.extend_from_slice(&32u16.to_be_bytes()); // length (informational)
        data.extend_from_slice(&0u16.to_be_bytes()); // language
        data.extend_from_slice(&4u16.to_be_bytes()); // segCountX2 (2 segments)
        data.extend_from_slice(&[0u8; 6]); // searchRange, entrySelector, rangeShift
        data.extend_from_slice(&0x0041u16.to_be_bytes()); // endCode[0]
        data.extend_from_slice(&0xFFFFu16.to_be_bytes()); // endCode[1]
        data.extend_from_slice(&0u16.to_be_bytes()); // reservedPad
        data.extend_from_slice(&0x0041u16.to_be_bytes()); // startCode[0]
        data.extend_from_slice(&0xFFFFu16.to_be_bytes()); // startCode[1]
        data.extend_from_slice(&((5i32 - 0x41i32) as i16).to_be_bytes()); // idDelta[0]
        data.extend_from_slice(&1i16.to_be_bytes()); // idDelta[1]
        data.extend_from_slice(&0u16.to_be_bytes()); // idRangeOffset[0]
        data.extend_from_slice(&0u16.to_be_bytes()); // idRangeOffset[1]
        assert_eq!(data.len(), 32);

        let mut cmap = empty_cmap();
        read_format4(&data, 0, cmap.as_mut(), &mut { MAX_TOTAL_CMAP_MAPPINGS });
        assert_eq!(cmap.unicodes.get(&0x41).unwrap().index, 5);
        assert!(!cmap.unicodes.contains_key(&0xFFFF));
    }

    #[test]
    fn format4_indirect_segment_follows_id_range_offset_into_the_glyph_array() {
        let mut data = Vec::new();
        data.extend_from_slice(&4u16.to_be_bytes());
        data.extend_from_slice(&36u16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&4u16.to_be_bytes()); // 2 segments
        data.extend_from_slice(&[0u8; 6]);
        data.extend_from_slice(&0x0042u16.to_be_bytes()); // endCode[0]
        data.extend_from_slice(&0xFFFFu16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes()); // reservedPad
        data.extend_from_slice(&0x0041u16.to_be_bytes()); // startCode[0]
        data.extend_from_slice(&0xFFFFu16.to_be_bytes());
        data.extend_from_slice(&0i16.to_be_bytes()); // idDelta[0]
        data.extend_from_slice(&1i16.to_be_bytes());
        data.extend_from_slice(&4u16.to_be_bytes()); // idRangeOffset[0]: 4 bytes ahead of its own entry
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&7u16.to_be_bytes()); // glyphIdArray[0] (for 0x41)
        data.extend_from_slice(&8u16.to_be_bytes()); // glyphIdArray[1] (for 0x42)
        assert_eq!(data.len(), 36);

        let mut cmap = empty_cmap();
        read_format4(&data, 0, cmap.as_mut(), &mut { MAX_TOTAL_CMAP_MAPPINGS });
        assert_eq!(cmap.unicodes.get(&0x41).unwrap().index, 7);
        assert_eq!(cmap.unicodes.get(&0x42).unwrap().index, 8);
    }

    #[test]
    fn format4_budget_stops_mid_segment_at_the_exact_boundary() {
        // Pins the budget check inside the segment loop: a single direct-delta segment
        // spanning 16 codepoints (0x41..=0x50), given a budget of only 5,
        // must map exactly the first 5 codepoints in the segment's own
        // order and leave the budget fully spent -- not skip one, not map
        // one extra, and not silently reset/re-grant the budget once the
        // segment's own range is exhausted.
        let mut data = Vec::new();
        data.extend_from_slice(&4u16.to_be_bytes()); // format
        data.extend_from_slice(&32u16.to_be_bytes()); // length (informational)
        data.extend_from_slice(&0u16.to_be_bytes()); // language
        data.extend_from_slice(&4u16.to_be_bytes()); // segCountX2 (2 segments)
        data.extend_from_slice(&[0u8; 6]);
        data.extend_from_slice(&0x0050u16.to_be_bytes()); // endCode[0]
        data.extend_from_slice(&0xFFFFu16.to_be_bytes()); // endCode[1]
        data.extend_from_slice(&0u16.to_be_bytes()); // reservedPad
        data.extend_from_slice(&0x0041u16.to_be_bytes()); // startCode[0]
        data.extend_from_slice(&0xFFFFu16.to_be_bytes()); // startCode[1]
        data.extend_from_slice(&0i16.to_be_bytes()); // idDelta[0]: gid == codepoint
        data.extend_from_slice(&1i16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes()); // idRangeOffset[0]: direct delta
        data.extend_from_slice(&0u16.to_be_bytes());
        assert_eq!(data.len(), 32);

        let mut cmap = empty_cmap();
        let mut budget: u32 = 5;
        read_format4(&data, 0, cmap.as_mut(), &mut budget);

        assert_eq!(budget, 0, "the budget must be fully consumed");
        assert_eq!(
            cmap.unicodes.len(),
            5,
            "exactly `budget` codepoints mapped, not fewer or more"
        );
        for cp in 0x41u32..0x46 {
            assert!(
                cmap.unicodes.contains_key(&(cp as i32)),
                "codepoint {cp:#x} within the budget must be mapped"
            );
        }
        assert!(
            !cmap.unicodes.contains_key(&0x46),
            "the first codepoint past the budget must not be mapped"
        );
    }

    #[test]
    fn format12_direct_group_maps_a_range() {
        let mut data = Vec::new();
        data.extend_from_slice(&12u16.to_be_bytes()); // format
        data.extend_from_slice(&0u16.to_be_bytes()); // reserved
        data.extend_from_slice(&28u32.to_be_bytes()); // length (informational)
        data.extend_from_slice(&0u32.to_be_bytes()); // language
        data.extend_from_slice(&1u32.to_be_bytes()); // nGroups
        data.extend_from_slice(&0x1F600u32.to_be_bytes()); // startCharCode
        data.extend_from_slice(&0x1F601u32.to_be_bytes()); // endCharCode
        data.extend_from_slice(&10u32.to_be_bytes()); // startGlyphID
        assert_eq!(data.len(), 28);

        let mut cmap = empty_cmap();
        read_format12(&data, 0, cmap.as_mut(), &mut { MAX_TOTAL_CMAP_MAPPINGS });
        assert_eq!(cmap.unicodes.get(&0x1F600).unwrap().index, 10);
        assert_eq!(cmap.unicodes.get(&0x1F601).unwrap().index, 11);
    }

    #[test]
    fn format12_n_groups_large_enough_to_overflow_the_multiplication_is_a_noop() {
        // Was `length_limit < 16.wrapping_add(12.wrapping_mul(n_groups))`:
        // an n_groups this large wraps `12 * n_groups` back down to a small
        // number, so the guard passed even though the real group array is
        // nowhere near that short, and the loop then read groups straight
        // past this 16-byte buffer's end. `require_room`'s `checked_mul`
        // must reject it instead.
        let mut data = Vec::new();
        data.extend_from_slice(&12u16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&16u32.to_be_bytes());
        data.extend_from_slice(&0u32.to_be_bytes());
        data.extend_from_slice(&0x1555_5556u32.to_be_bytes()); // nGroups
        assert_eq!(data.len(), 16);

        let mut cmap = empty_cmap();
        read_format12(&data, 0, cmap.as_mut(), &mut { MAX_TOTAL_CMAP_MAPPINGS });
        assert!(cmap.unicodes.is_empty());
    }

    #[test]
    fn format12_group_range_past_the_unicode_ceiling_is_clamped_not_walked_in_full() {
        // A single 12-byte group can claim an endCharCode anywhere in
        // the full u32 range: `0xFFFFFFFF` makes the walk a genuine
        // infinite loop (`c.wrapping_add(1)` wraps `0xFFFFFFFF` back to
        // `0`, so `c <= end_code` is true forever), and anything merely
        // huge (a fuzz-found font used `0xffff0000`) still forces
        // billions of iterations from those same 12 bytes. Real
        // codepoints never exceed the Unicode ceiling, so clamping the
        // walked range to it must both terminate promptly and still map
        // every codepoint up to (and including) the ceiling correctly.
        let mut data = Vec::new();
        data.extend_from_slice(&12u16.to_be_bytes()); // format
        data.extend_from_slice(&0u16.to_be_bytes()); // reserved
        data.extend_from_slice(&28u32.to_be_bytes()); // length (informational)
        data.extend_from_slice(&0u32.to_be_bytes()); // language
        data.extend_from_slice(&1u32.to_be_bytes()); // nGroups
        data.extend_from_slice(&0x10fffeu32.to_be_bytes()); // startCharCode: just below the ceiling
        data.extend_from_slice(&0xffffffffu32.to_be_bytes()); // endCharCode: the u32 max
        data.extend_from_slice(&10u32.to_be_bytes()); // startGlyphID
        assert_eq!(data.len(), 28);

        let mut cmap = empty_cmap();
        let start = std::time::Instant::now();
        read_format12(&data, 0, cmap.as_mut(), &mut { MAX_TOTAL_CMAP_MAPPINGS });
        assert!(start.elapsed() < std::time::Duration::from_secs(5));

        assert_eq!(cmap.unicodes.get(&0x10fffe).unwrap().index, 10);
        assert_eq!(cmap.unicodes.get(&0x10ffff).unwrap().index, 11);
        assert!(!cmap.unicodes.contains_key(&0x110000));
    }

    #[test]
    fn format12_budget_caps_the_total_across_many_groups_not_just_one() {
        // Each group's own range is already clamped to the Unicode ceiling
        // by `clamped_end` (see the test above), but that clamp alone
        // doesn't stop a subtable from packing many groups that each
        // independently claim a huge range -- a few thousand such groups,
        // well within any real subtable byte-size limit, still multiply
        // out to billions of iterations (found by `cargo fuzz run
        // otf_dump`: a single crafted format12 subtable pushed a parse
        // past several minutes and toward the fuzzer's memory limit). This
        // pins that the budget is shared *across* groups, not reset or
        // re-granted per group: three groups here, each individually able
        // to produce far more than the budget allows, must still total no
        // more than the budget handed in.
        let mut data = Vec::new();
        data.extend_from_slice(&12u16.to_be_bytes()); // format
        data.extend_from_slice(&0u16.to_be_bytes()); // reserved
        data.extend_from_slice(&52u32.to_be_bytes()); // length (informational)
        data.extend_from_slice(&0u32.to_be_bytes()); // language
        data.extend_from_slice(&3u32.to_be_bytes()); // nGroups
        for i in 0..3u32 {
            data.extend_from_slice(&0u32.to_be_bytes()); // startCharCode
            data.extend_from_slice(&0x10ffffu32.to_be_bytes()); // endCharCode: the whole Unicode range
            data.extend_from_slice(&(i * 1000).to_be_bytes()); // startGlyphID
        }
        assert_eq!(data.len(), 52);

        let mut cmap = empty_cmap();
        let mut budget: u32 = 5;
        let start = std::time::Instant::now();
        read_format12(&data, 0, cmap.as_mut(), &mut budget);
        assert!(start.elapsed() < std::time::Duration::from_secs(5));

        assert_eq!(
            budget, 0,
            "the shared budget must be fully consumed, not per-group"
        );
        assert_eq!(
            cmap.unicodes.len(),
            5,
            "no more mappings than the budget allows, even though each group alone claims far more"
        );
    }

    #[test]
    fn uvs_default_budget_stops_mid_range_at_the_exact_boundary() {
        // Pins the budget check inside the range loop: a single default-UVS range spanning 16
        // codepoints (0x41..=0x50, `additionalCount == 15`), given a
        // budget of only 5, must register a UVS mapping for exactly the
        // first 5 codepoints in the range's own order and leave the
        // budget fully spent. Every codepoint in the range must already
        // resolve via `cmap_lookup` -- `read_uvs_default` only
        // registers a UVS mapping when the plain cmap already maps that
        // codepoint -- so the budget is the only reason fewer than 16
        // entries land in `cmap.uvs`, not a lookup miss.
        let mut data = Vec::new();
        data.extend_from_slice(&1u32.to_be_bytes()); // numUnicodeValueRanges
        data.extend_from_slice(&[0x00, 0x00, 0x41]); // startUnicodeValue (24-bit): 0x41
        data.extend_from_slice(&15u8.to_be_bytes()); // additionalCount: 16 codepoints total
        assert_eq!(data.len(), 8);

        let mut cmap = empty_cmap();
        for cp in 0x41i32..=0x50 {
            encode_cmap_by_index(cmap.as_mut(), cp, cp as u16);
        }
        let mut budget: u32 = 5;
        read_uvs_default(&data, 0, 0xFE00, cmap.as_mut(), &mut budget);

        assert_eq!(budget, 0, "the budget must be fully consumed");
        assert_eq!(
            cmap.uvs.len(),
            5,
            "exactly `budget` UVS mappings registered, not fewer or more"
        );
        for cp in 0x41u32..0x46 {
            assert!(
                cmap.uvs.contains_key(&CmapUvsKey {
                    unicode: cp,
                    selector: 0xFE00
                }),
                "codepoint {cp:#x} within the budget must be registered"
            );
        }
        assert!(
            !cmap.uvs.contains_key(&CmapUvsKey {
                unicode: 0x46,
                selector: 0xFE00
            }),
            "the first codepoint past the budget must not be registered"
        );
    }

    #[test]
    fn uvs_default_num_ranges_overflow_is_a_noop_not_oob() {
        let mut data = Vec::new();
        data.extend_from_slice(&0x4000_0001u32.to_be_bytes()); // numUnicodeValueRanges
        assert_eq!(data.len(), 4);

        let mut cmap = empty_cmap();
        read_uvs_default(&data, 0, 0xFE00, cmap.as_mut(), &mut {
            MAX_TOTAL_CMAP_MAPPINGS
        });
        assert!(cmap.uvs.is_empty());
    }

    #[test]
    fn uvs_non_default_num_mappings_overflow_is_a_noop_not_oob() {
        let mut data = Vec::new();
        data.extend_from_slice(&0x3333_3334u32.to_be_bytes()); // numUVSMappings
        assert_eq!(data.len(), 4);

        let mut cmap = empty_cmap();
        read_uvs_non_default(&data, 0, 0xFE00, cmap.as_mut(), &mut {
            MAX_TOTAL_CMAP_MAPPINGS
        });
        assert!(cmap.uvs.is_empty());
    }

    #[test]
    fn uvs_non_default_well_formed_registers_a_mapping() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u32.to_be_bytes()); // numUVSMappings
        data.extend_from_slice(&[0x00, 0x00, 0x41]); // unicodeValue (24-bit): 0x41
        data.extend_from_slice(&9u16.to_be_bytes()); // glyphID
        assert_eq!(data.len(), 9);

        let mut cmap = empty_cmap();
        read_uvs_non_default(&data, 0, 0xFE00, cmap.as_mut(), &mut {
            MAX_TOTAL_CMAP_MAPPINGS
        });
        let g = cmap
            .uvs
            .get(&CmapUvsKey {
                unicode: 0x41,
                selector: 0xFE00,
            })
            .unwrap();
        assert_eq!(g.index, 9);
    }

    #[test]
    fn format14_non_default_uvs_offset_aliasing_respects_shared_budget() {
        // `tests/fuzz-corpus/known-issues/otf-dump-cmap-uvs-non-default-
        // aliasing-oom.bin`: `read_uvs_non_default`'s own guard
        // (`require_room(num_mappings, 5)`) only bounds one call's own
        // mapping count against its subtable's own bytes -- it says
        // nothing about how many *times* `read_format14` calls it. Many
        // `VarSelectorRecord`s, each with a distinct `varSelector` but
        // all sharing the same `nonDefaultUVSOffset`, each alias the same
        // small subtable -- and because `CmapUvsKey` includes `selector`,
        // every alias inserts a genuinely new, distinct set of `cmap.uvs`
        // entries (unlike the directory-level offset dedup in
        // `decode_cmap`, which is safe *because* re-parsing the same bytes
        // at the same offset is idempotent there). Three records here,
        // each individually able to add 10 more mappings than the budget
        // allows, must still total no more than the budget handed in --
        // the same shape `format12_budget_caps_the_total_across_many_
        // groups_not_just_one` already pins for format12's own group
        // aliasing.
        let mut data = Vec::new();
        data.extend_from_slice(&14u16.to_be_bytes()); // format
        data.extend_from_slice(&0u32.to_be_bytes()); // length (informational)
        data.extend_from_slice(&3u32.to_be_bytes()); // numVarSelectorRecords
        let shared_offset: u32 = 10 + 11 * 3; // right after the VarSelectorRecord array
        for i in 0..3u32 {
            data.extend_from_slice(&(i + 1).to_be_bytes()[1..]); // varSelector (24-bit), distinct per record
            data.extend_from_slice(&0u32.to_be_bytes()); // defaultUVSOffset: absent
            data.extend_from_slice(&shared_offset.to_be_bytes()); // nonDefaultUVSOffset: aliased
        }
        assert_eq!(data.len(), shared_offset as usize);
        data.extend_from_slice(&10u32.to_be_bytes()); // numUVSMappings
        for i in 0..10u32 {
            data.extend_from_slice(&(i + 1).to_be_bytes()[1..]); // unicodeValue (24-bit)
            data.extend_from_slice(&0u16.to_be_bytes()); // glyphID
        }

        let mut cmap = empty_cmap();
        let mut budget: u32 = 5;
        read_format14(&data, 0, cmap.as_mut(), &mut budget);

        assert_eq!(
            budget, 0,
            "the shared budget must be fully consumed, not per-record"
        );
        assert_eq!(
            cmap.uvs.len(),
            5,
            "no more mappings than the budget allows, even though each aliased record alone claims far more"
        );
    }

    #[test]
    fn format14_n_groups_overflow_is_a_noop_not_oob() {
        let mut data = Vec::new();
        data.extend_from_slice(&14u16.to_be_bytes()); // format
        data.extend_from_slice(&0u32.to_be_bytes()); // length
        data.extend_from_slice(&0x1999_999Au32.to_be_bytes()); // numVarSelectorRecords
        assert_eq!(data.len(), 10);

        let mut cmap = empty_cmap();
        read_format14(&data, 0, cmap.as_mut(), &mut { MAX_TOTAL_CMAP_MAPPINGS });
        assert!(cmap.uvs.is_empty());
    }

    #[test]
    fn format14_default_uvs_offset_past_the_table_end_is_a_noop_not_oob() {
        // The original computed the recursive call's remaining-length
        // budget as `length_limit.wrapping_sub(default_uvs_offset)` --
        // an `default_uvs_offset` larger than the whole table's own
        // length wrapped that subtraction into a huge number, so
        // `read_uvs_default`'s own guards, now looking at a bogus giant
        // budget, no longer protected anything. This reader has no
        // separate length_limit to underflow at all: the recursive call's
        // own `FontReader::at` rejects the out-of-range offset directly.
        let mut data = Vec::new();
        data.extend_from_slice(&14u16.to_be_bytes());
        data.extend_from_slice(&0u32.to_be_bytes());
        data.extend_from_slice(&1u32.to_be_bytes()); // numVarSelectorRecords
        data.extend_from_slice(&[0x00, 0xFE, 0x00]); // varSelector (24-bit): 0xFE00
        data.extend_from_slice(&0xFFFF_FFF0u32.to_be_bytes()); // defaultUVSOffset: far past the table
        data.extend_from_slice(&0u32.to_be_bytes()); // nonDefaultUVSOffset: absent
        assert_eq!(data.len(), 21);

        let mut cmap = empty_cmap();
        read_format14(&data, 0, cmap.as_mut(), &mut { MAX_TOTAL_CMAP_MAPPINGS });
        assert!(cmap.uvs.is_empty());
    }

    #[test]
    fn parse_cmap_directory_entry_pointing_past_the_table_end_skips_just_that_subtable() {
        // The headline bug this file's migration exists to fix: the
        // original derived each subtable's remaining-length budget as
        // `length.wrapping_sub(table_offset)`, so a `table_offset` larger
        // than the table's own length wrapped that subtraction into a
        // huge number, defeating every downstream guard in whichever
        // format reader ran next. `decode_cmap` never computes that
        // subtraction at all -- it just hands the format readers the same
        // `data` slice and the (unvalidated) absolute `table_offset`, and
        // each one's own `FontReader::at` rejects an out-of-range offset
        // on contact.
        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_be_bytes()); // version
        data.extend_from_slice(&1u16.to_be_bytes()); // numTables
        data.extend_from_slice(&3u16.to_be_bytes()); // platformID (Windows)
        data.extend_from_slice(&1u16.to_be_bytes()); // encodingID (Unicode BMP)
        data.extend_from_slice(&0xFFFF_FFF0u32.to_be_bytes()); // offset: far past the table
        assert_eq!(data.len(), 12);

        let cmap = decode_cmap(&data).unwrap();
        assert!(cmap.unicodes.is_empty());
    }

    #[test]
    fn parse_cmap_directory_shorter_than_declared_num_tables_errs() {
        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&5u16.to_be_bytes()); // numTables, but no entries follow
        assert!(decode_cmap(&data).is_err());
    }

    #[test]
    fn parse_cmap_end_to_end_finds_the_format4_subtable() {
        let mut subtable = Vec::new();
        subtable.extend_from_slice(&4u16.to_be_bytes());
        subtable.extend_from_slice(&32u16.to_be_bytes());
        subtable.extend_from_slice(&0u16.to_be_bytes());
        subtable.extend_from_slice(&4u16.to_be_bytes());
        subtable.extend_from_slice(&[0u8; 6]);
        subtable.extend_from_slice(&0x0041u16.to_be_bytes());
        subtable.extend_from_slice(&0xFFFFu16.to_be_bytes());
        subtable.extend_from_slice(&0u16.to_be_bytes());
        subtable.extend_from_slice(&0x0041u16.to_be_bytes());
        subtable.extend_from_slice(&0xFFFFu16.to_be_bytes());
        subtable.extend_from_slice(&((5i32 - 0x41i32) as i16).to_be_bytes());
        subtable.extend_from_slice(&1i16.to_be_bytes());
        subtable.extend_from_slice(&0u16.to_be_bytes());
        subtable.extend_from_slice(&0u16.to_be_bytes());
        assert_eq!(subtable.len(), 32);

        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_be_bytes()); // version
        data.extend_from_slice(&1u16.to_be_bytes()); // numTables
        data.extend_from_slice(&3u16.to_be_bytes()); // platformID
        data.extend_from_slice(&1u16.to_be_bytes()); // encodingID
        data.extend_from_slice(&12u32.to_be_bytes()); // offset: right after the 12-byte directory
        data.extend_from_slice(&subtable);

        let cmap = decode_cmap(&data).unwrap();
        assert_eq!(cmap.unicodes.get(&0x41).unwrap().index, 5);
    }

    #[test]
    #[cfg_attr(
        miri,
        ignore = "timing-based; 400,000 codepoint inserts is far too slow to run meaningfully under Miri's interpreter"
    )]
    fn parse_cmap_duplicate_directory_entries_aliasing_the_same_subtable_are_only_parsed_once() {
        // The cmap spec puts no requirement on encoding-record subtable
        // offsets being distinct -- real fonts legitimately have several
        // records point at the very same subtable. Without a dedup,
        // `decode_cmap` re-parses (and re-walks every codepoint of) the
        // aliased subtable once per record pointing at it, an
        // amplification whose multiplier is `numTables` itself, bounded
        // only by the cmap table's own size divided by 8 bytes per
        // record -- not by any fixed cap. This builds 200 duplicate,
        // individually-valid directory entries all pointing at one
        // format 12 subtable large enough that re-walking it 200 times
        // (undeduped) takes seconds, while parsing it once (deduped)
        // is near-instant -- proving the redundant records are actually
        // skipped, not just that the end result happens to be correct
        // (parsing the same bytes twice is idempotent either way).
        const NUM_ENTRIES: usize = 200;
        const NUM_GROUPS: u32 = 200;
        const GROUP_SPAN: u32 = 2000; // codepoints per group

        let mut subtable = Vec::new();
        subtable.extend_from_slice(&12u16.to_be_bytes()); // format
        subtable.extend_from_slice(&0u16.to_be_bytes()); // reserved
        subtable.extend_from_slice(&0u32.to_be_bytes()); // length (informational)
        subtable.extend_from_slice(&0u32.to_be_bytes()); // language
        subtable.extend_from_slice(&NUM_GROUPS.to_be_bytes());
        for g in 0..NUM_GROUPS {
            let start = g * GROUP_SPAN;
            let end = start + GROUP_SPAN - 1;
            subtable.extend_from_slice(&start.to_be_bytes());
            subtable.extend_from_slice(&end.to_be_bytes());
            subtable.extend_from_slice(&1u32.to_be_bytes()); // startGlyphID
        }

        let directory_len = 4 + NUM_ENTRIES * 8;
        let subtable_offset = directory_len as u32;

        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_be_bytes()); // version
        data.extend_from_slice(&(NUM_ENTRIES as u16).to_be_bytes()); // numTables
        for _ in 0..NUM_ENTRIES {
            data.extend_from_slice(&3u16.to_be_bytes()); // platformID (Windows)
            data.extend_from_slice(&10u16.to_be_bytes()); // encodingID (Unicode full repertoire)
            data.extend_from_slice(&subtable_offset.to_be_bytes());
        }
        data.extend_from_slice(&subtable);
        assert_eq!(data.len(), directory_len + subtable.len());

        let start = std::time::Instant::now();
        let cmap = decode_cmap(&data).unwrap();
        let elapsed = start.elapsed();

        assert_eq!(cmap.unicodes.len() as u32, NUM_GROUPS * GROUP_SPAN);
        assert_eq!(cmap.unicodes.get(&0).unwrap().index, 1);
        // The last codepoint of the last group: startGlyphID is 1 in
        // every group, so a codepoint's gid is `1 + its offset within
        // its own group` -- the last codepoint of any GROUP_SPAN-wide
        // group is GROUP_SPAN itself, not 1.
        assert_eq!(
            cmap.unicodes
                .get(&((NUM_GROUPS * GROUP_SPAN - 1) as i32))
                .unwrap()
                .index,
            GROUP_SPAN as u16
        );
        assert!(
            elapsed < std::time::Duration::from_secs(2),
            "parsing {NUM_ENTRIES} aliasing directory entries took {elapsed:?} -- \
             looks like the same subtable is being re-parsed per entry again"
        );
    }
}
