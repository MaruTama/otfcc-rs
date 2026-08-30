#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{GlyphHandle, handle_from_index};
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_getnum_fallback,
};
use crate::table::otl::coverage::{Coverage, coverage_from_raw, push_to_coverage, read_coverage};

use crate::support::font_reader::FontReader;

use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::support::buffer::Buffer;
use crate::support::options::Options;
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};
use crate::vendor::json::JsonType;

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkgraph::bk_build_block;
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new,
    json_object_push,
};
use crate::table::otl::coverage::{build_coverage, dump_coverage, parse_coverage};
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::{GsubReverseSubtable, Subtable, subtable_from_raw};

#[inline]
unsafe fn subtable_gsub_reverse_free(x: *mut GsubReverseSubtable) {
    if x.is_null() {
        return;
    }
    // `Box::from_raw` reclaims exactly the allocation `_create()` made below
    // and runs `match_0`/`to`'s own drop glue directly -- no separate
    // dispose-then-`free` needed (Stage 7-2-d). `dispose_gsub_reverse`/
    // `subtable_gsub_reverse_dispose` had no other callers, so they're gone
    // along with `init_gsub_reverse`/`subtable_gsub_reverse_init`.
    drop(Box::from_raw(x));
}
#[inline]
unsafe fn subtable_gsub_reverse_create() -> *mut GsubReverseSubtable {
    Box::into_raw(Box::new(GsubReverseSubtable {
        match_count: 0,
        input_index: 0,
        match_0: Vec::new(),
        to: Coverage::new(),
    }))
}
// Was a manual index-swapping loop over `start..end`, meeting in the
// middle -- exactly what `[T]::reverse` does, now that `match_0` is a real
// `Vec<Coverage>` slice instead of an array of raw pointers to swap by
// value. `input_index == 0` (nothing to reverse) falls out of slicing an
// empty range, no separate guard needed.
unsafe fn reverse_backtracks(match_0: &mut [Coverage], input_index: TableId) {
    match_0[..input_index as usize].reverse();
}
pub unsafe fn otl_read_gsub_reverse(
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    _max_glyphs: GlyphId,
) -> *mut Subtable {
    let subtable: *mut GsubReverseSubtable = subtable_gsub_reverse_create();
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);

    'parse: {
        let mut header = match FontReader::new(slice).at(offset as usize) {
            Ok(r) => r,
            Err(_) => break 'parse,
        };
        if header.skip(2).is_err() {
            break 'parse; // format, unused (this reader is only ever called for format 1)
        }
        let Ok(input_cov_rel) = header.u16() else {
            break 'parse;
        };
        let Ok(n_backtrack) = header.u16() else {
            break 'parse;
        };
        if header.require_room(n_backtrack as usize, 2).is_err() {
            break 'parse;
        }
        let mut backtrack_offsets = Vec::with_capacity(n_backtrack as usize);
        for _ in 0..n_backtrack {
            backtrack_offsets.push(offset.wrapping_add(header.u16().unwrap() as u32));
        }

        let Ok(n_forward) = header.u16() else {
            break 'parse;
        };
        if header.require_room(n_forward as usize, 2).is_err() {
            break 'parse;
        }
        let mut forward_offsets = Vec::with_capacity(n_forward as usize);
        for _ in 0..n_forward {
            forward_offsets.push(offset.wrapping_add(header.u16().unwrap() as u32));
        }

        let Ok(n_replacement) = header.u16() else {
            break 'parse;
        };
        if header.require_room(n_replacement as usize, 2).is_err() {
            break 'parse;
        }

        // `match_count` (a `TableId`/u16 field) is `n_backtrack + n_forward
        // + 1` -- each addend is individually bounded to u16, but their
        // sum is not, and the original's implicit `as TableId` cast just
        // truncated it. A truncated `match_count` here would go on to
        // index `match_0` (sized to the truncated count) with the *real*
        // `n_backtrack`/`n_forward` below and panic out of bounds, rather
        // than merely produce a wrong-but-safe result -- rejected instead.
        let Some(match_count_u32) = (n_backtrack as u32)
            .checked_add(n_forward as u32)
            .and_then(|s| s.checked_add(1))
        else {
            break 'parse;
        };
        if match_count_u32 > u16::MAX as u32 {
            break 'parse;
        }
        let match_count = match_count_u32 as TableId;

        (*subtable).match_count = match_count;
        // Filled out of sequential order below (backtrack slots, then the
        // input slot at `input_index`, then forward slots) -- every one of
        // the `match_count` slots is written exactly once by the time this
        // subtable is returned, so pre-sizing with placeholder empty
        // `Coverage`s and index-assigning is the direct replacement for
        // the old `offset`-indexed writes into `__caryll_allocate_clean`'d
        // memory.
        (*subtable).match_0 = vec![Coverage::new(); match_count as usize];
        (*subtable).input_index = n_backtrack;

        for (j, &cov_offset) in backtrack_offsets.iter().enumerate() {
            (&mut (*subtable).match_0)[j] =
                coverage_from_raw(read_coverage(data, table_length, cov_offset));
        }

        let input_cov_offset = offset.wrapping_add(input_cov_rel as u32);
        (&mut (*subtable).match_0)[(*subtable).input_index as usize] =
            coverage_from_raw(read_coverage(data, table_length, input_cov_offset));

        if n_replacement as usize != (&(*subtable).match_0)[(*subtable).input_index as usize].len()
        {
            break 'parse;
        }

        for (j, &cov_offset) in forward_offsets.iter().enumerate() {
            let fwd_idx = n_backtrack as usize + 1 + j;
            (&mut (*subtable).match_0)[fwd_idx] =
                coverage_from_raw(read_coverage(data, table_length, cov_offset));
        }

        (*subtable).to = Coverage::new();
        for _ in 0..n_replacement {
            push_to_coverage(
                &mut (*subtable).to as *mut Coverage,
                handle_from_index(header.u16().unwrap() as GlyphId) as GlyphHandle,
            );
        }
        reverse_backtracks(&mut (*subtable).match_0, (*subtable).input_index);
        return subtable_from_raw(subtable, Subtable::GsubReverse);
    }
    subtable_gsub_reverse_free(subtable);
    ::core::ptr::null_mut::<Subtable>()
}
pub unsafe fn otl_gsub_dump_reverse(mut _subtable: *const Subtable) -> *mut BuiltValue {
    let Subtable::GsubReverse(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GsubReverseSubtable = mut_subtable;
    let mut _st: *mut BuiltValue = json_object_new(3_usize);
    let mut _match: *mut BuiltValue = json_array_new((*subtable).match_count as usize);
    let mut j: TableId = 0 as TableId;
    while (j as i32) < (*subtable).match_count as i32 {
        json_array_push(
            _match,
            dump_coverage(&(&(*subtable).match_0)[j as usize] as *const Coverage),
        );
        j = j.wrapping_add(1);
    }
    json_object_push(
        _st,
        b"match\0" as *const u8 as *const ::core::ffi::c_char,
        _match,
    );
    json_object_push(
        _st,
        b"to\0" as *const u8 as *const ::core::ffi::c_char,
        dump_coverage(&(*subtable).to as *const Coverage),
    );
    json_object_push(
        _st,
        b"inputIndex\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*subtable).input_index as i64),
    );
    return _st;
}
pub unsafe fn otl_gsub_parse_reverse(
    mut _subtable: *const ParsedValue,
    mut _options: &Options,
) -> *mut Subtable {
    let mut _match: *const ParsedValue = json_obj_get_type(
        _subtable,
        b"match\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    let mut _to: *const ParsedValue = json_obj_get_type(
        _subtable,
        b"to\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _match.is_null() || _to.is_null() {
        return ::core::ptr::null_mut::<Subtable>();
    }
    let subtable: *mut GsubReverseSubtable = (subtable_gsub_reverse_create)();
    (*subtable).match_count = json_arr_len(_match) as TableId;
    (*subtable).match_0 = Vec::with_capacity((*subtable).match_count as usize);
    (*subtable).input_index = json_obj_getnum_fallback(
        _subtable,
        b"inputIndex\0" as *const u8 as *const ::core::ffi::c_char,
        0_i32 as ::core::ffi::c_double,
    ) as TableId;
    let mut j: TableId = 0 as TableId;
    while (j as i32) < (*subtable).match_count as i32 {
        (*subtable)
            .match_0
            .push(coverage_from_raw(parse_coverage(json_arr_at(
                _match, j as u32,
            ))));
        j = j.wrapping_add(1);
    }
    (*subtable).to = coverage_from_raw(parse_coverage(_to));
    return subtable_from_raw(subtable, Subtable::GsubReverse);
}
pub unsafe fn otfcc_build_gsub_reverse(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let Subtable::GsubReverse(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GsubReverseSubtable = mut_subtable;
    // `subtable` is `*const` because every other read in this function is
    // read-only, but sorting `match_0`'s backtrack portion into wire order
    // in place is pre-existing behavior (unchanged by this field's type),
    // and nothing else touches `_subtable` during a build pass -- sound to
    // cast away constness just for this one call.
    reverse_backtracks(
        &mut (*(subtable as *mut GsubReverseSubtable)).match_0,
        (*subtable).input_index,
    );
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B16, 1_u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(build_coverage(
                &(&(*subtable).match_0)[(*subtable).input_index as usize] as *const Coverage,
            )),
        ),
    ]);
    bk_push(
        root,
        &[bk_int(
            BkCellType::B16,
            ((*subtable).input_index as i32) as u32,
        )],
    );
    let mut j: TableId = 0 as TableId;
    while (j as i32) < (*subtable).input_index as i32 {
        bk_push(
            root,
            &[bk_ptr(
                BkCellType::P16,
                bk_new_block_from_buffer(build_coverage(
                    &(&(*subtable).match_0)[j as usize] as *const Coverage,
                )),
            )],
        );
        j = j.wrapping_add(1);
    }
    bk_push(
        root,
        &[bk_int(
            BkCellType::B16,
            ((*subtable).match_count as i32
                - (*subtable).input_index as i32
                - 1_i32) as u32,
        )],
    );
    let mut j_0: TableId =
        ((*subtable).input_index as i32 + 1_i32) as TableId;
    while (j_0 as i32) < (*subtable).match_count as i32 {
        bk_push(
            root,
            &[bk_ptr(
                BkCellType::P16,
                bk_new_block_from_buffer(build_coverage(
                    &(&(*subtable).match_0)[j_0 as usize] as *const Coverage,
                )),
            )],
        );
        j_0 = j_0.wrapping_add(1);
    }
    bk_push(
        root,
        &[bk_int(
            BkCellType::B16,
            ((*subtable).to.len() as i32) as u32,
        )],
    );
    let mut j_1: TableId = 0 as TableId;
    while (j_1 as usize) < (*subtable).to.len() {
        bk_push(
            root,
            &[bk_int(
                BkCellType::B16,
                ((&(*subtable).to)[j_1 as usize].index as i32) as u32,
            )],
        );
        j_1 = j_1.wrapping_add(1);
    }
    return bk_build_block(root).into_raw();
}

#[cfg(test)]
mod otl_read_gsub_reverse_tests {
    use super::*;

    #[test]
    fn well_formed_table_builds_match_0_in_backtrack_input_forward_order() {
        let mut data = [0u8; 26];
        data[2..4].copy_from_slice(&14u16.to_be_bytes()); // inputCoverageOffset -> 14
        data[4..6].copy_from_slice(&1u16.to_be_bytes()); // backtrackGlyphCount
        data[6..8].copy_from_slice(&20u16.to_be_bytes()); // backtrackCoverageOffsets[0] -> 20
        data[8..10].copy_from_slice(&0u16.to_be_bytes()); // lookaheadGlyphCount
        data[10..12].copy_from_slice(&1u16.to_be_bytes()); // glyphCount (substitute count)
        data[12..14].copy_from_slice(&99u16.to_be_bytes()); // substituteGlyphID[0]
        // Input coverage format 1 at byte 14: one glyph, id 20.
        data[14..16].copy_from_slice(&1u16.to_be_bytes());
        data[16..18].copy_from_slice(&1u16.to_be_bytes());
        data[18..20].copy_from_slice(&20u16.to_be_bytes());
        // Backtrack coverage format 1 at byte 20: one glyph, id 21.
        data[20..22].copy_from_slice(&1u16.to_be_bytes());
        data[22..24].copy_from_slice(&1u16.to_be_bytes());
        data[24..26].copy_from_slice(&21u16.to_be_bytes());
        unsafe {
            let raw =
                otl_read_gsub_reverse(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::GsubReverse(subtable) = &*boxed else {
                unreachable!()
            };
            assert_eq!(subtable.match_count, 2);
            assert_eq!(subtable.input_index, 1);
            assert_eq!(
                subtable.match_0[0]
                    .iter()
                    .map(|h| h.index)
                    .collect::<Vec<_>>(),
                vec![21]
            );
            assert_eq!(
                subtable.match_0[1]
                    .iter()
                    .map(|h| h.index)
                    .collect::<Vec<_>>(),
                vec![20]
            );
            assert_eq!(
                subtable.to.iter().map(|h| h.index).collect::<Vec<_>>(),
                vec![99]
            );
        }
    }

    #[test]
    // `n_backtrack` has to genuinely be `u16::MAX` for this test's sum
    // to actually overflow a u16 -- there's no smaller data that still
    // exercises this specific overflow, so this stays a ~131KB buffer
    // regardless. 44s under Miri (this crate's third-slowest test);
    // `cargo test` (native) is the real regression guard, this is just
    // advisory extra confidence that isn't worth the wall-clock cost.
    #[cfg_attr(
        miri,
        ignore = "far too slow to run meaningfully under Miri's interpreter; needs a genuinely u16::MAX-sized backtrack array to trigger the overflow being tested"
    )]
    fn match_count_overflow_is_rejected_instead_of_panicking() {
        // `match_count` (`n_backtrack + n_forward + 1`, a u16 field) can
        // overflow even though each addend is individually u16-bounded.
        // The original's implicit `as TableId` cast truncated it, which
        // would have gone on to index `match_0` (sized to the truncated
        // count) with the real, larger `n_backtrack` below and panic out
        // of bounds. `n_backtrack` here is pushed to `u16::MAX` and
        // `n_forward` to 1, so their sum plus the input slot overflows
        // `u16::MAX` by one.
        let mut data = vec![0u8; 131082];
        data[4..6].copy_from_slice(&u16::MAX.to_be_bytes()); // backtrackGlyphCount
        let n_forward_pos = 6 + u16::MAX as usize * 2;
        data[n_forward_pos..n_forward_pos + 2].copy_from_slice(&1u16.to_be_bytes()); // lookaheadGlyphCount
        let n_replacement_pos = n_forward_pos + 2 + 2;
        data[n_replacement_pos..n_replacement_pos + 2].copy_from_slice(&0u16.to_be_bytes()); // glyphCount
        assert_eq!(data.len(), n_replacement_pos + 2);
        unsafe {
            let raw =
                otl_read_gsub_reverse(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(raw.is_null());
        }
    }
}
