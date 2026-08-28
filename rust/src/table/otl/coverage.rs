#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::handle::{
    GlyphHandle, Handle, handle_from_index, handle_from_name, otfcc_handle_dispose,
};
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_str_bytes, json_type_of,
};

use crate::support::buffer::Buffer;
use crate::support::buffer::{buffree, buflen, bufnew, bufwrite_bufdel, bufwrite16b};
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_string_new_from_bytes, preserialize,
};
use crate::support::font_reader::FontReader;
use crate::support::primitives::GlyphId;
use crate::vendor::json::JsonType;
/// A glyph coverage set: C by way of c2rust had this as a hand-rolled
/// `malloc`/`realloc` array (`num_glyphs`/`capacity`/`glyphs: *mut
/// GlyphHandle`); it was never anything but a growable array of
/// `GlyphHandle`, so `Vec<GlyphHandle>` *is* `Coverage` now, not a struct
/// wrapping one -- same "C-native vector shape becomes a bare `pub type`"
/// call as `ColrTable`/`TsiTable` earlier in this migration.
pub type Coverage = Vec<GlyphHandle>;
pub(crate) unsafe fn otl_coverage_create() -> *mut Coverage {
    // A real Rust allocation now, not a `malloc`'d shell: `Box::into_raw`
    // gives back a pointer with the same shape (`*mut Coverage`) every
    // caller already expects, but it must from here on only ever be
    // reclaimed with `Box::from_raw` (`otl_coverage_free`/
    // `coverage_from_raw` below), never a bare `free` -- mixing the two
    // is exactly the hazard this conversion removes.
    Box::into_raw(Box::new(Vec::new()))
}
pub(crate) unsafe fn otl_coverage_free(x: *mut Coverage) {
    if x.is_null() {
        return;
    }
    // Dropping the reclaimed `Box` drops the `Vec` first (running every
    // element's `Handle::drop`, freeing each glyph name -- the explicit
    // per-element `otfcc_handle_dispose` loop this used to need is
    // redundant, same finding as the `Handle` Drop/Clone PR), then
    // deallocates the shell. One step where there used to be two.
    drop(Box::from_raw(x));
}
/// Adopt a `otl_coverage_create()`/vtable-`.parse()`-style raw `*mut
/// Coverage` into an owned `Coverage` (`Vec<GlyphHandle>`) value -- the
/// "unwrap_X_table" idiom used throughout Stage 6-4, for the many
/// `XxxSubtable`/`XxxEntry` fields that hold a coverage table by value now
/// instead of by raw pointer. `Box::from_raw` reclaims the allocation
/// `otl_coverage_create` made; dereferencing it moves the `Vec` out and
/// drops the now-empty shell in the same step.
pub(crate) unsafe fn coverage_from_raw(raw: *mut Coverage) -> Coverage {
    *Box::from_raw(raw)
}
// `Handle` (aliased `GlyphHandle`) now owns a `Vec<u8>` name, so passing it
// by value trips `improper_ctypes_definitions`; this is never called across
// a real FFI boundary (c2rust artifact, not `#[no_mangle]`).
#[allow(improper_ctypes_definitions)]
pub(crate) unsafe fn push_to_coverage(coverage: *mut Coverage, h: GlyphHandle) {
    (*coverage).push(h);
}
// `data`/`table_length` are always the untouched pointer/length of the
// whole owning GSUB/GPOS/GDEF table (confirmed by tracing every call site
// up to `otfcc_read_otl`/`otfcc_read_gdef`, which read `table.length` once
// from the `PacketPiece` and thread it unchanged through every layer down
// to here -- only `offset` grows as recursion descends into subtables).
// That means `slice::from_raw_parts(data, table_length as usize)` below
// really does describe the same allocation the top-level table reader
// validated, so every bounds check downstream of it is real.
//
// The original's own guards here used `wrapping_add` on `offset` (a `u32`
// read from the file, unbounded) plus a small constant: `offset` close to
// `u32::MAX` could wrap the whole comparison back down to something
// small, passing a guard that should have failed -- the same overflow-
// defeats-guard shape as `cmap.rs`'s bugs, just via addition instead of
// multiplication. `FontReader::at`/`require_room` use `checked_add`/
// `checked_mul` throughout, closing this.
pub(crate) unsafe fn read_coverage(
    data: *const u8,
    table_length: u32,
    offset: u32,
) -> *mut Coverage {
    let coverage = otl_coverage_create();
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let Ok(mut r) = FontReader::new(slice).at(offset as usize) else {
        return coverage;
    };
    let Ok(format) = r.u16() else { return coverage };
    match format {
        1 => {
            let Ok(glyph_count) = r.u16() else {
                return coverage;
            };
            if r.require_room(glyph_count as usize, 2).is_err() {
                return coverage;
            }
            // `HASH_SORT`-by-`covIndex` is a no-op here: `covIndex` was
            // assigned `j` (this loop's own position) at insert time, and
            // only a first occurrence of each `gid` is ever inserted, so
            // the sequence of inserted `covIndex` values is already
            // strictly increasing -- sorting by it reproduces insertion
            // order exactly. `IndexSet` (insertion-order-preserving,
            // dedups on `.insert()`) needs no explicit sort step at all.
            let mut h: indexmap::IndexSet<GlyphId> = indexmap::IndexSet::new();
            for _ in 0..glyph_count {
                h.insert(r.u16().unwrap());
            }
            for gid in h.into_iter() {
                push_to_coverage(coverage, handle_from_index(gid) as GlyphHandle);
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
            // (`k` the absolute gid, per the original C -- see
            // rust/README.md) which is *not* generally monotonic with
            // insertion order once ranges overlap or run out of order, so
            // the `HASH_SORT`-by-`covIndex` step is not a no-op and must
            // be reproduced explicitly: dedup-by-gid (first occurrence
            // wins) via `IndexMap`, then a stable sort by the stored
            // `covIndex`, matching `HASH_SORT`'s documented mergesort
            // stability for ties.
            let mut h: indexmap::IndexMap<GlyphId, i32> = indexmap::IndexMap::new();
            for _ in 0..range_count {
                let start = r.u16().unwrap();
                let end = r.u16().unwrap();
                let start_coverage_index = r.u16().unwrap();
                let mut k = start as i32;
                while k <= end as i32 {
                    let cov_index = start_coverage_index as i32 + k;
                    h.entry(k as GlyphId).or_insert(cov_index);
                    k += 1;
                }
            }
            let mut entries: Vec<(GlyphId, i32)> = h.into_iter().collect();
            entries.sort_by_key(|&(_, cov_index)| cov_index);
            for (gid, _) in entries {
                push_to_coverage(coverage, handle_from_index(gid) as GlyphHandle);
            }
        }
        _ => {}
    }
    coverage
}
// No longer `extern "C"`: every call site (`gsub_multi.rs`, `gsub_ligature.rs`,
// `gsub_reverse.rs`, `chaining/dump.rs`) calls this directly by name, never
// through a function-pointer value -- confirmed by grep across the crate.
// Same for `parse_coverage`/`build_coverage_format`/`build_coverage` below.
pub(crate) unsafe fn dump_coverage(coverage: *const Coverage) -> *mut BuiltValue {
    let a: *mut BuiltValue = json_array_new((*coverage).len());
    for j in 0..(*coverage).len() {
        json_array_push(a, json_string_new_from_bytes(&(&(*coverage))[j].name));
    }
    return preserialize(a);
}
pub(crate) unsafe fn parse_coverage(cov: *const ParsedValue) -> *mut Coverage {
    let c: *mut Coverage = otl_coverage_create();
    if cov.is_null() || json_type_of(cov) != JsonType::Array {
        return c;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < json_arr_len(cov) {
        if json_type_of(json_arr_at(cov, j as u32)) == JsonType::String {
            push_to_coverage(
                c,
                handle_from_name(Some(json_str_bytes(json_arr_at(cov, j as u32)))) as GlyphHandle,
            );
        }
        j = j.wrapping_add(1);
    }
    return c;
}
pub(crate) unsafe fn build_coverage_format(
    coverage: *const Coverage,
    format: u16,
) -> *mut Buffer {
    if (*coverage).is_empty() {
        let buf: *mut Buffer = bufnew();
        bufwrite16b(buf, 2 as u16);
        bufwrite16b(buf, 0 as u16);
        return buf;
    }
    // A local `Vec` scratch buffer, not a `__caryll_allocate_clean`/`qsort`/
    // `free` trio: `sort_by_key` (stable, matching the conservative choice
    // made everywhere else in this file) reproduces `by_gid`'s ordering,
    // and the `Vec` drops itself at every one of this function's several
    // return points instead of needing a matching `free` at each.
    let mut r: Vec<GlyphId> = (*coverage).iter().map(|h| h.index).collect();
    r.sort_by_key(|&gid| gid);
    let jj: GlyphId = r.len() as GlyphId;
    let format1: *mut Buffer = bufnew();
    bufwrite16b(format1, 1 as u16);
    bufwrite16b(format1, jj as u16);
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as ::core::ffi::c_int) < jj as ::core::ffi::c_int {
        bufwrite16b(format1, r[j_0 as usize] as u16);
        j_0 = j_0.wrapping_add(1);
    }
    if (jj as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        return format1;
    }
    let format2: *mut Buffer = bufnew();
    bufwrite16b(format2, 2 as u16);
    let ranges: *mut Buffer = bufnew();
    let mut start_gid: GlyphId = r[0];
    let mut end_gid: GlyphId = start_gid;
    let mut last_gid: GlyphId = start_gid;
    let mut n_ranges: GlyphId = 0 as GlyphId;
    let mut j_1: GlyphId = 1 as GlyphId;
    while (j_1 as ::core::ffi::c_int) < jj as ::core::ffi::c_int {
        let current: GlyphId = r[j_1 as usize];
        if !(current as ::core::ffi::c_int <= last_gid as ::core::ffi::c_int) {
            if current as ::core::ffi::c_int
                == end_gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            {
                end_gid = current;
            } else {
                bufwrite16b(ranges, start_gid as u16);
                bufwrite16b(ranges, end_gid as u16);
                bufwrite16b(
                    ranges,
                    (j_1 as ::core::ffi::c_int + start_gid as ::core::ffi::c_int
                        - end_gid as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as u16,
                );
                n_ranges = (n_ranges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
                end_gid = current;
                start_gid = end_gid;
            }
            last_gid = current;
        }
        j_1 = j_1.wrapping_add(1);
    }
    bufwrite16b(ranges, start_gid as u16);
    bufwrite16b(ranges, end_gid as u16);
    bufwrite16b(
        ranges,
        (jj as ::core::ffi::c_int + start_gid as ::core::ffi::c_int
            - end_gid as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as u16,
    );
    n_ranges = (n_ranges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
    bufwrite16b(format2, n_ranges as u16);
    bufwrite_bufdel(format2, ranges);
    if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        buffree(format2);
        return format1;
    } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        buffree(format1);
        return format2;
    } else if buflen(format1) < buflen(format2) {
        buffree(format2);
        return format1;
    } else {
        buffree(format1);
        return format2;
    };
}
pub(crate) unsafe fn build_coverage(coverage: *const Coverage) -> *mut Buffer {
    return build_coverage_format(coverage, 0 as u16);
}
pub(crate) unsafe fn shrink_coverage(coverage: *mut Coverage, dosort: bool) {
    if coverage.is_null() {
        return;
    }
    // Two `truncate`s, not one `num_glyphs = k` at the end as the original
    // did: each `truncate` lets `Vec`'s own drop glue free every handle
    // past the new length, including ones this function's own compaction
    // loops never got around to calling `otfcc_handle_dispose` on directly
    // (a survivor that gets superseded by a *later* compaction write, but
    // never becomes a write target itself, is exactly that case) -- the
    // original leaked that name; `truncate` doesn't.
    let mut k: usize = 0;
    for j in 0..(*coverage).len() {
        if !(&(*coverage))[j].name.is_empty() {
            let elem = (&(*coverage))[j].clone();
            (&mut (*coverage))[k] = elem;
            k += 1;
        } else {
            otfcc_handle_dispose(&raw mut (&mut (*coverage))[j] as *mut Handle);
        }
    }
    (*coverage).truncate(k);
    if dosort {
        (*coverage).sort_by_key(|h| h.index);
        let mut skip: usize = 0;
        let mut rear: usize = 1;
        while rear < (*coverage).len() {
            if (&(*coverage))[rear].index == (&(*coverage))[rear - skip - 1].index {
                otfcc_handle_dispose(&raw mut (&mut (*coverage))[rear] as *mut Handle);
                skip += 1;
            } else {
                let elem = (&(*coverage))[rear].clone();
                (&mut (*coverage))[rear - skip] = elem;
            }
            rear += 1;
        }
        let new_len = (*coverage).len() - skip;
        (*coverage).truncate(new_len);
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
        unsafe {
            let raw = read_coverage(data.as_ptr(), data.len() as u32, 0);
            let cov = coverage_from_raw(raw);
            assert_eq!(cov.iter().map(|h| h.index).collect::<Vec<_>>(), vec![5, 9]);
        }
    }

    #[test]
    fn format2_ranges_expand_and_sort_by_coverage_index() {
        let mut data = Vec::new();
        data.extend_from_slice(&2u16.to_be_bytes()); // format
        data.extend_from_slice(&1u16.to_be_bytes()); // rangeCount
        data.extend_from_slice(&10u16.to_be_bytes()); // startGlyphID
        data.extend_from_slice(&12u16.to_be_bytes()); // endGlyphID
        data.extend_from_slice(&0u16.to_be_bytes()); // startCoverageIndex
        unsafe {
            let raw = read_coverage(data.as_ptr(), data.len() as u32, 0);
            let cov = coverage_from_raw(raw);
            assert_eq!(
                cov.iter().map(|h| h.index).collect::<Vec<_>>(),
                vec![10, 11, 12]
            );
        }
    }

    #[test]
    fn offset_near_u32_max_does_not_wrap_the_guard() {
        // The original computed `offset.wrapping_add(4)` -- an offset this
        // close to u32::MAX wraps that addition back down to a small
        // number, which could pass the `table_length < ...` guard even
        // though `offset` itself points nowhere near the table.
        let data = [0u8; 8];
        unsafe {
            let raw = read_coverage(data.as_ptr(), data.len() as u32, 0xFFFF_FFF0);
            let cov = coverage_from_raw(raw);
            assert!(cov.is_empty());
        }
    }

    #[test]
    fn truncated_header_is_empty_not_oob() {
        let data = [0u8; 1];
        unsafe {
            let raw = read_coverage(data.as_ptr(), data.len() as u32, 0);
            let cov = coverage_from_raw(raw);
            assert!(cov.is_empty());
        }
    }
}
