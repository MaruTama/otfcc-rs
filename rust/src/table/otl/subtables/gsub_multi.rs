#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{
    GlyphHandle, Handle, handle_from_index, handle_from_name, otfcc_handle_dup,
};
use crate::support::parsed_json::{
    ParsedValue, json_obj_key_bytes_at, json_obj_len, json_obj_val_at, json_type_of,
};
use crate::table::otl::coverage::{
    Coverage, coverage_from_raw, otl_coverage_create, otl_coverage_free, push_to_coverage,
    read_coverage,
};

use crate::support::alloc::__caryll_reallocate;
use crate::support::font_reader::FontReader;

use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::support::buffer::Buffer;
use crate::support::options::Options;
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};
use crate::vendor::json::JsonType;

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkgraph::bk_build_block;
use crate::support::built_json::{BuiltValue, json_object_new, json_object_push_bytes_key};
use crate::table::otl::coverage::{build_coverage, dump_coverage, parse_coverage};
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::{GsubMultiEntry, GsubMultiSubtable, Subtable, subtable_from_raw};
// `to: Coverage` and `from: GlyphHandle` both self-drop now, so a
// `GsubMultiSubtable` (`Vec<GsubMultiEntry>`) fully self-drops -- no
// per-element dtor needed anymore.
pub(crate) unsafe fn dispose_gsub_multi_subtable(arr: *mut GsubMultiSubtable) {
    *arr = Vec::new();
}
pub(crate) unsafe fn subtable_gsub_multi_free(x: *mut GsubMultiSubtable) {
    if x.is_null() {
        return;
    }
    // `Box::from_raw` reclaims exactly the allocation `_create()` made below
    // and runs the `Vec`'s own drop glue -- no separate dispose-then-`free`
    // needed (Stage 7-2-d; `dispose_gsub_multi_subtable` stays, it is still
    // used by `table/otl.rs`'s `Drop for Subtable` and `consolidate/otl/
    // gsub_multi.rs`, just no longer from here).
    drop(Box::from_raw(x));
}
unsafe fn subtable_gsub_multi_create() -> *mut GsubMultiSubtable {
    Box::into_raw(Box::new(Vec::new()))
}
// Each Sequence subtable (`seq_offset`, resolved from the per-entry
// `sequenceOffsets[]` array) had *no* length guard at all before this
// rewrite: neither `seq_offset` itself nor `n` (the Sequence's own
// glyphCount, a full attacker-controlled u16) were checked against the
// table's actual length before reading `n` glyph IDs from it -- a real,
// previously-undocumented unchecked read, same class as `otl/read.rs`'s
// `langSysRecords` bug. `FontReader::at`/`require_room` close both.
pub unsafe fn otl_read_gsub_multi(
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    _max_glyphs: GlyphId,
) -> *mut Subtable {
    let subtable: *mut GsubMultiSubtable = subtable_gsub_multi_create();
    let mut from: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);

    'parse: {
        let mut header = match FontReader::new(slice).at(offset as usize) {
            Ok(r) => r,
            Err(_) => break 'parse,
        };
        if header.skip(2).is_err() {
            break 'parse; // format, unused
        }
        let Ok(from_rel) = header.u16() else {
            break 'parse;
        };
        let Ok(seq_count) = header.u16() else {
            break 'parse;
        };

        from = read_coverage(data, table_length, offset.wrapping_add(from_rel as u32));
        if seq_count as usize != (*from).len() {
            break 'parse;
        }
        if header.require_room(seq_count as usize, 2).is_err() {
            break 'parse;
        }

        for j in 0..seq_count {
            let seq_offset = offset.wrapping_add(header.u16().unwrap() as u32);
            let Ok(mut sr) = FontReader::new(slice).at(seq_offset as usize) else {
                break 'parse;
            };
            let Ok(n) = sr.u16() else { break 'parse };
            if sr.require_room(n as usize, 2).is_err() {
                break 'parse;
            }
            let cov: *mut Coverage = otl_coverage_create();
            for _ in 0..n {
                push_to_coverage(
                    cov,
                    handle_from_index(sr.u16().unwrap() as GlyphId) as GlyphHandle,
                );
            }
            (*subtable).push(GsubMultiEntry {
                from: otfcc_handle_dup((&(*from))[j as usize].clone() as Handle) as GlyphHandle,
                to: coverage_from_raw(cov),
            });
        }
        otl_coverage_free(from);
        return subtable_from_raw(subtable, Subtable::GsubMulti);
    }

    if !from.is_null() {
        otl_coverage_free(from);
    }
    subtable_gsub_multi_free(subtable);
    ::core::ptr::null_mut::<Subtable>()
}
pub unsafe extern "C" fn otl_gsub_dump_multi(mut _subtable: *const Subtable) -> *mut BuiltValue {
    let Subtable::GsubMulti(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GsubMultiSubtable = mut_subtable;
    let st: *mut BuiltValue = json_object_new((*subtable).len());
    for j in 0..(*subtable).len() as GlyphId {
        let entry = &(&(*subtable))[j as usize];
        json_object_push_bytes_key(
            st,
            &(*entry).from.name,
            dump_coverage(&(*entry).to as *const Coverage),
        );
    }
    return st;
}
pub unsafe fn otl_gsub_parse_multi(
    mut _subtable: *const ParsedValue,
    mut _options: &Options,
) -> *mut Subtable {
    let st: *mut GsubMultiSubtable = subtable_gsub_multi_create();
    for k in 0..json_obj_len(_subtable) as GlyphId {
        let _to: *const ParsedValue = json_obj_val_at(_subtable, k as u32);
        if !_to.is_null() && json_type_of(_to) == JsonType::Array {
            (*st).push(GsubMultiEntry {
                from: handle_from_name(Some(json_obj_key_bytes_at(_subtable, k as u32)))
                    as GlyphHandle,
                to: coverage_from_raw(parse_coverage(_to)),
            });
        }
    }
    return subtable_from_raw(st, Subtable::GsubMulti);
}
unsafe fn build_gsub_multi_subtable_range(
    subtable: *const GsubMultiSubtable,
    start: GlyphId,
    end: GlyphId,
) -> *mut Buffer {
    let cov: *mut Coverage = otl_coverage_create();
    for j in start..end {
        push_to_coverage(
            cov,
            otfcc_handle_dup((&(*subtable))[j as usize].from.clone() as Handle) as GlyphHandle,
        );
    }
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B16, 1 as u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(build_coverage(cov)),
        ),
        bk_int(
            BkCellType::B16,
            (end as ::core::ffi::c_int - start as ::core::ffi::c_int) as u32,
        ),
    ]);
    for j_0 in start..end {
        let to: *const Coverage = &(&(*subtable))[j_0 as usize].to;
        let b: *mut BkBlock = bk_new_block(&[bk_int(
            BkCellType::B16,
            ((*to).len() as ::core::ffi::c_int) as u32,
        )]);
        for k in 0..(*to).len() {
            bk_push(
                b,
                &[bk_int(
                    BkCellType::B16,
                    ((&(*to))[k].index as ::core::ffi::c_int) as u32,
                )],
            );
        }
        bk_push(root, &[bk_ptr(BkCellType::P16, b)]);
    }
    otl_coverage_free(cov);
    return bk_build_block(root);
}
pub const GSUB_MULTI_SUBTABLE_SIZE_LIMIT: ::core::ffi::c_int = 0xff00 as ::core::ffi::c_int;
pub unsafe extern "C" fn otfcc_build_gsub_multi_subtable_split(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
    count: *mut TableId,
) -> *mut *mut Buffer {
    let Subtable::GsubMulti(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GsubMultiSubtable = mut_subtable;
    let mut parts: *mut *mut Buffer = ::core::ptr::null_mut::<*mut Buffer>();
    let mut n_parts: TableId = 0 as TableId;
    let mut start: GlyphId = 0 as GlyphId;
    while (start as usize) < (*subtable).len() {
        let mut size: usize = (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize;
        let mut end: GlyphId = start;
        while (end as usize) < (*subtable).len() {
            let entry_size: usize = ((2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize)
                .wrapping_add(((&(*subtable))[end as usize].to.len()).wrapping_mul(2 as usize));
            if end as ::core::ffi::c_int > start as ::core::ffi::c_int
                && size.wrapping_add(entry_size) > GSUB_MULTI_SUBTABLE_SIZE_LIMIT as usize
            {
                break;
            }
            size = size.wrapping_add(entry_size);
            end = end.wrapping_add(1);
        }
        parts = __caryll_reallocate(
            parts as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut Buffer>() as usize)
                .wrapping_mul((n_parts as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize),
            125 as ::core::ffi::c_ulong,
        ) as *mut *mut Buffer;
        let ref mut fresh2 = *parts.offset(n_parts as isize);
        *fresh2 = build_gsub_multi_subtable_range(subtable, start, end);
        n_parts = n_parts.wrapping_add(1);
        start = end;
    }
    if n_parts == 0 {
        parts = __caryll_reallocate(
            parts as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut Buffer>() as usize).wrapping_mul(1 as usize),
            132 as ::core::ffi::c_ulong,
        ) as *mut *mut Buffer;
        let ref mut fresh3 = *parts.offset(0 as ::core::ffi::c_int as isize);
        *fresh3 = build_gsub_multi_subtable_range(subtable, 0 as GlyphId, 0 as GlyphId);
        n_parts = 1 as TableId;
    }
    *count = n_parts;
    return parts;
}
pub unsafe fn otfcc_build_gsub_multi_subtable(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let Subtable::GsubMulti(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GsubMultiSubtable = mut_subtable;
    return build_gsub_multi_subtable_range(subtable, 0 as GlyphId, (*subtable).len() as GlyphId);
}

#[cfg(test)]
mod otl_read_gsub_multi_tests {
    use super::*;

    fn well_formed_data() -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_be_bytes()); // format
        data.extend_from_slice(&8u16.to_be_bytes()); // coverageOffset -> 8
        data.extend_from_slice(&1u16.to_be_bytes()); // sequenceCount
        data.extend_from_slice(&14u16.to_be_bytes()); // sequenceOffsets[0] -> 14
        // Coverage format 1 at byte 8: one glyph, id 5.
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&5u16.to_be_bytes());
        // Sequence table at byte 14: 2 substitute glyphs.
        data.extend_from_slice(&2u16.to_be_bytes());
        data.extend_from_slice(&10u16.to_be_bytes());
        data.extend_from_slice(&11u16.to_be_bytes());
        data
    }

    #[test]
    fn well_formed_table_reads_the_sequence() {
        let data = well_formed_data();
        unsafe {
            let raw =
                otl_read_gsub_multi(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::GsubMulti(entries) = &*boxed else {
                unreachable!()
            };
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].from.index, 5);
            let to: Vec<GlyphId> = entries[0].to.iter().map(|h| h.index).collect();
            assert_eq!(to, vec![10, 11]);
        }
    }

    #[test]
    fn sequence_glyph_count_larger_than_available_is_rejected_instead_of_reading_oob() {
        // The original had *no* length check on a Sequence subtable at
        // all -- neither `seq_offset` nor its own `glyphCount` (a full
        // attacker-controlled u16) were validated before reading that
        // many glyph IDs.
        let mut data = well_formed_data();
        data[14..16].copy_from_slice(&100u16.to_be_bytes()); // glyphCount claims 100, far more than fits
        unsafe {
            let raw =
                otl_read_gsub_multi(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(raw.is_null());
        }
    }

    #[test]
    fn sequence_offset_past_the_table_end_is_rejected_instead_of_reading_oob() {
        let mut data = well_formed_data();
        data[6..8].copy_from_slice(&9000u16.to_be_bytes()); // sequenceOffsets[0]: far past the table
        unsafe {
            let raw =
                otl_read_gsub_multi(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(raw.is_null());
        }
    }
}
