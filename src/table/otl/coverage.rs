use crate::support::handle::{GlyphHandle, Handle, handle_from_index, handle_from_name};
use otfcc_json::ParsedValue;

use otfcc_binary::Buffer;
use otfcc_json::BuiltValue;
use otfcc_binary::FontReader;
use crate::support::primitives::{GlyphId, count_u16};
use crate::table::otl::budget::OtlReadBudget;
/// A glyph coverage set, in coverage-index order.
pub type Coverage = Vec<GlyphHandle>;
/// The `coverage_entries` limit of `OtlReadBudget`. It bounds the total
/// cost of *building* a `Coverage` -- format 1's per-glyph loop and format
/// 2's range expansion, both in `read_coverage` below -- across a *whole*
/// GSUB/GPOS/GDEF table, every `read_coverage` call combined.
///
/// Each call is individually bounded (at most 65,536 glyphs), but many
/// lookups' subtables can point at the same maximal coverage bytes, so a
/// per-call cap alone still multiplies into a table-wide hang or OOM (both
/// formats were found this way by `cargo fuzz`). This is the same shape
/// `OtlReadBudget`'s `class_zero_glyphs`/`class_coverage_calls` handle for
/// `chaining/read.rs`'s `class_coverage`. Sized at ~76x the largest amount
/// of work a single legitimate coverage table can need (65,536 distinct
/// glyphs), leaving headroom for fonts with many real coverage tables.
pub(crate) const MAX_TOTAL_COVERAGE_ENTRY_BUILDS_PER_TABLE: u32 = 5_000_000;
pub(crate) fn push_to_coverage(coverage: &mut Coverage, h: GlyphHandle) {
    coverage.push(h);
}
// `data` is always the whole owning GSUB/GPOS/GDEF table; only `offset`
// grows as reading descends into subtables. `offset` comes from the file,
// so every position is computed through `FontReader::at`/`require_room`
// (`checked_add`/`checked_mul`), never by plain addition that could wrap.
pub(crate) fn read_coverage(data: &[u8], offset: u32, budget: &mut OtlReadBudget) -> Coverage {
    let mut coverage: Coverage = Vec::new();
    let Ok(mut r) = FontReader::new(data).at(offset as usize) else {
        return coverage;
    };
    let Ok(format) = r.u16() else {
        return coverage;
    };
    match format {
        1 => {
            let Ok(glyph_count) = r.u16() else {
                return coverage;
            };
            if r.require_room(glyph_count as usize, 2).is_err() {
                return coverage;
            }
            // Each glyph's coverage index is its position `j`, and only a
            // first occurrence of each `gid` is inserted, so insertion
            // order is coverage-index order. `IndexSet` dedups on
            // `.insert()` and needs no sort step.
            //
            // `glyph_count` is individually bounded (`require_room`
            // against the table, at most 65,536 entries) but, like format
            // 2's range expansion below, many `read_coverage` calls across
            // a table's lookups/subtables could each pay that cost -- see
            // `MAX_TOTAL_COVERAGE_ENTRY_BUILDS_PER_TABLE`'s doc comment.
            let mut h: indexmap::IndexSet<GlyphId> = indexmap::IndexSet::new();
            'glyphs: for _ in 0..glyph_count {
                if !budget.take_coverage_entry() {
                    break 'glyphs;
                }
                h.insert(r.u16().unwrap());
            }
            for gid in h.into_iter() {
                push_to_coverage(&mut coverage, handle_from_index(gid) as GlyphHandle);
            }
        }
        2 => {
            let Ok(range_count) = r.u16() else {
                return coverage;
            };
            if r.require_room(range_count as usize, 6).is_err() {
                return coverage;
            }
            // Unlike format 1, `covIndex` here is `startCoverageIndex + k`
            // (`k` the absolute gid -- see RUST_MIGRATION.md), which is
            // *not* generally monotonic with insertion order once ranges
            // overlap or run out of order. So: dedup-by-gid (first
            // occurrence wins) via `IndexMap`, then a stable sort by the
            // stored `covIndex`.
            //
            // `range_count` and each range's `start..=end` span are each
            // bounded, but their product is not: ~65,535 ranges each
            // spanning the whole glyph space expand into billions of
            // `IndexMap::entry` calls although `h` never holds more than
            // 65,536 entries. The table-wide
            // `OtlReadBudget::coverage_entries` (shared with format 1's
            // loop above) caps that; it only ever discards redundant range
            // expansion, never real coverage.
            let mut h: indexmap::IndexMap<GlyphId, i32> = indexmap::IndexMap::new();
            'ranges: for _ in 0..range_count {
                let start = r.u16().unwrap();
                let end = r.u16().unwrap();
                let start_coverage_index = r.u16().unwrap();
                // `start as i32..=end as i32`: both ends are fixed once
                // read above and don't depend on the budget, so this walks
                // exactly the same range the `while` did one step at a
                // time (empty when `start > end`, matching the `while`'s
                // own zero-iteration case). The budget check keeps its
                // exact original position -- first thing in the body,
                // still `break 'ranges` (the *outer* per-range loop, not
                // just this expanded range) on exhaustion, unchanged by
                // this loop's own shape becoming a `for`.
                for k in start as i32..=end as i32 {
                    if !budget.take_coverage_entry() {
                        break 'ranges;
                    }
                    let cov_index = start_coverage_index as i32 + k;
                    h.entry(k as GlyphId).or_insert(cov_index);
                }
            }
            let mut entries: Vec<(GlyphId, i32)> = h.into_iter().collect();
            entries.sort_by_key(|&(_, cov_index)| cov_index);
            for (gid, _) in entries {
                push_to_coverage(&mut coverage, handle_from_index(gid) as GlyphHandle);
            }
        }
        _ => {}
    }
    coverage
}
pub(crate) fn dump_coverage(coverage: &Coverage) -> BuiltValue {
    let mut a = BuiltValue::new_array(coverage.len());
    for h in coverage {
        a.push_item(BuiltValue::str_truncated_at_nul(&h.name));
    }
    a.preserialize()
}
pub(crate) fn parse_coverage(cov: Option<&ParsedValue>) -> Coverage {
    let mut c: Coverage = Vec::new();
    let Some(items) = cov.and_then(ParsedValue::as_array) else {
        return c;
    };
    for item in items {
        if let Some(name) = item.as_str_bytes() {
            push_to_coverage(&mut c, handle_from_name(Some(name.to_vec())) as GlyphHandle);
        }
    }
    c
}
pub(crate) fn build_coverage_format(coverage: &Coverage, format: u16) -> Buffer {
    if coverage.is_empty() {
        let mut buf = Buffer::new();
        buf.write_u16be(2_u16);
        buf.write_u16be(0_u16);
        return buf;
    }
    // `sort_by_key` is stable, like every other sort in this file.
    let mut r: Vec<GlyphId> = coverage.iter().map(|h| h.index).collect();
    r.sort_by_key(|&gid| gid);
    let jj: GlyphId = count_u16(r.len());
    let mut format1 = Buffer::new();
    format1.write_u16be(1_u16);
    format1.write_u16be(jj as u16);
    for &gid in &r {
        format1.write_u16be(gid);
    }
    if (jj as i32) < 2_i32 {
        return format1;
    }
    let mut format2 = Buffer::new();
    format2.write_u16be(2_u16);
    let mut ranges = Buffer::new();
    let mut start_gid: GlyphId = r[0];
    let mut end_gid: GlyphId = start_gid;
    let mut last_gid: GlyphId = start_gid;
    let mut n_ranges: GlyphId = 0 as GlyphId;
    for (j_1, &current) in r.iter().enumerate().skip(1) {
        if !(current as i32 <= last_gid as i32) {
            if current as i32 == end_gid as i32 + 1_i32 {
                end_gid = current;
            } else {
                ranges.write_u16be(start_gid as u16);
                ranges.write_u16be(end_gid as u16);
                ranges.write_u16be((j_1 as i32 + start_gid as i32 - end_gid as i32 - 1_i32) as u16);
                n_ranges = (n_ranges as i32 + 1_i32) as GlyphId;
                end_gid = current;
                start_gid = end_gid;
            }
            last_gid = current;
        }
    }
    ranges.write_u16be(start_gid as u16);
    ranges.write_u16be(end_gid as u16);
    ranges.write_u16be(
        (jj as i32 + start_gid as i32
            - end_gid as i32
            - 1_i32) as u16,
    );
    n_ranges = (n_ranges as i32 + 1_i32) as GlyphId;
    format2.write_u16be(n_ranges as u16);
    format2.write_buffer_owned(ranges);
    match format {
        1 => format1,
        2 => format2,
        _ => {
            if format1.len() < format2.len() {
                format1
            } else {
                format2
            }
        }
    }
}
pub(crate) fn build_coverage(coverage: &Coverage) -> Buffer {
    build_coverage_format(coverage, 0_u16)
}
pub(crate) fn shrink_coverage(coverage: &mut Coverage, dosort: bool) {
    // `truncate` drops every handle past the compacted length, including
    // survivors that were superseded but never overwritten themselves.
    let mut k: usize = 0;
    for j in 0..coverage.len() {
        if !coverage[j].name.is_empty() {
            let elem = coverage[j].clone();
            coverage[k] = elem;
            k += 1;
        } else {
            coverage[j] = Handle::default();
        }
    }
    coverage.truncate(k);
    if dosort {
        coverage.sort_by_key(|h| h.index);
        let mut skip: usize = 0;
        for rear in 1..coverage.len() {
            if coverage[rear].index == coverage[rear - skip - 1].index {
                coverage[rear] = Handle::default();
                skip += 1;
            } else {
                let elem = coverage[rear].clone();
                coverage[rear - skip] = elem;
            }
        }
        let new_len = coverage.len() - skip;
        coverage.truncate(new_len);
    }
}

#[cfg(test)]
mod read_coverage_tests {
    use super::*;

    #[test]
    fn format1_glyph_array_dedups_and_preserves_insertion_order() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes()); // format
        data.extend_from_slice(&3u16.to_be_bytes()); // glyphCount
        data.extend_from_slice(&5u16.to_be_bytes());
        data.extend_from_slice(&9u16.to_be_bytes());
        data.extend_from_slice(&5u16.to_be_bytes()); // duplicate, deduped
        let cov = read_coverage(&data, 0, &mut OtlReadBudget::new());
        assert_eq!(cov.iter().map(|h| h.index).collect::<Vec<_>>(), vec![5, 9]);
    }

    #[test]
    fn format2_ranges_expand_and_sort_by_coverage_index() {
        let mut data = Vec::new();
        data.extend_from_slice(&2u16.to_be_bytes()); // format
        data.extend_from_slice(&1u16.to_be_bytes()); // rangeCount
        data.extend_from_slice(&10u16.to_be_bytes()); // startGlyphID
        data.extend_from_slice(&12u16.to_be_bytes()); // endGlyphID
        data.extend_from_slice(&0u16.to_be_bytes()); // startCoverageIndex
        let cov = read_coverage(&data, 0, &mut OtlReadBudget::new());
        assert_eq!(
            cov.iter().map(|h| h.index).collect::<Vec<_>>(),
            vec![10, 11, 12]
        );
    }

    #[test]
    fn format2_budget_stops_mid_range_at_the_exact_boundary() {
        // A single format-2 range spanning 16 glyphs (10..=25), given a
        // coverage budget of only 5, must expand exactly the first 5 glyphs
        // in the range's own order and leave the budget fully spent -- not
        // skip one, not expand one extra.
        let mut budget = OtlReadBudget { coverage_entries: 5, ..OtlReadBudget::new() };
        let mut data = Vec::new();
        data.extend_from_slice(&2u16.to_be_bytes()); // format
        data.extend_from_slice(&1u16.to_be_bytes()); // rangeCount
        data.extend_from_slice(&10u16.to_be_bytes()); // startGlyphID
        data.extend_from_slice(&25u16.to_be_bytes()); // endGlyphID (16 glyphs total)
        data.extend_from_slice(&0u16.to_be_bytes()); // startCoverageIndex
        let cov = read_coverage(&data, 0, &mut budget);

        assert_eq!(budget.coverage_entries, 0, "the budget must be fully consumed");
        assert_eq!(
            cov.iter().map(|h| h.index).collect::<Vec<_>>(),
            vec![10, 11, 12, 13, 14],
            "exactly `budget` glyphs expanded, in range order, not fewer or more"
        );
    }

    #[test]
    fn offset_near_u32_max_does_not_wrap_the_guard() {
        // An offset this close to u32::MAX must not wrap `offset + 4` back
        // down to a small number that passes the length check.
        let data = [0u8; 8];
        let cov = read_coverage(&data, 0xFFFF_FFF0, &mut OtlReadBudget::new());
        assert!(cov.is_empty());
    }

    #[test]
    fn truncated_header_is_empty_not_oob() {
        let data = [0u8; 1];
        let cov = read_coverage(&data, 0, &mut OtlReadBudget::new());
        assert!(cov.is_empty());
    }
}
