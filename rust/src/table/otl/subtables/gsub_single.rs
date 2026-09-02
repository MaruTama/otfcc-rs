#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{
    GlyphHandle, Handle, handle_from_index, handle_from_name, otfcc_handle_dup,
};
use crate::support::parsed_json::{
    ParsedValue, json_obj_key_bytes_at, json_obj_len, json_obj_val_at, json_str_bytes, json_type_of,
};
use crate::table::otl::coverage::{
    Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage,
};

use crate::support::font_reader::FontReader;

use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::support::buffer::Buffer;
use crate::support::options::Options;
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::json::JsonType;

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkgraph::bk_build_block;
use crate::support::built_json::BuiltValue;
use crate::table::otl::coverage::build_coverage_format;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::{GsubSingleEntry, GsubSingleSubtable, Subtable, subtable_from_raw};
// `GsubSingleEntry` holds only two `GlyphHandle`s, so dropping the `Vec`
// runs `Handle`'s own `Drop` for every entry -- no per-element dtor needed
// anymore.
pub(crate) unsafe fn dispose_gsub_single_subtable(arr: *mut GsubSingleSubtable) {
    *arr = Vec::new();
}
pub(crate) unsafe fn subtable_gsub_single_free(x: *mut GsubSingleSubtable) {
    if x.is_null() {
        return;
    }
    // `Box::from_raw` reclaims exactly the allocation `_create()` made below
    // and runs the `Vec`'s own drop glue -- no separate dispose-then-`free`
    // needed (Stage 7-2-d; `dispose_gsub_single_subtable` stays, it is still
    // used by `table/otl.rs`'s `Drop for Subtable` and `consolidate/otl/
    // gsub_single.rs`, just no longer from here).
    drop(Box::from_raw(x));
}
unsafe fn subtable_gsub_single_create() -> *mut GsubSingleSubtable {
    Box::into_raw(Box::new(Vec::new()))
}
// `Coverage`/`otl_coverage_create`/`read_coverage` are still raw-pointer-
// shaped (unconverted, out of this PR's scope), so this keeps interleaving
// them with `FontReader`-checked header reads rather than fully
// restructuring into a `Result`-returning helper -- the labeled block
// below is the same "any failure bails to shared cleanup" shape the
// original's `current_block` goto-emulation had, without the goto.
pub unsafe fn otl_read_gsub_single(
    data: FontFilePointer,
    table_length: u32,
    subtable_offset: u32,
    _max_glyphs: GlyphId,
) -> *mut Subtable {
    let subtable: *mut GsubSingleSubtable = subtable_gsub_single_create();
    let mut from: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut to: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);

    'parse: {
        let mut header = match FontReader::new(slice).at(subtable_offset as usize) {
            Ok(r) => r,
            Err(_) => break 'parse,
        };
        let Ok(subtable_format) = header.u16() else {
            break 'parse;
        };
        let Ok(from_rel) = header.u16() else {
            break 'parse;
        };

        from = read_coverage(
            data,
            table_length,
            subtable_offset.wrapping_add(from_rel as u32),
        );
        if from.is_null() || (*from).is_empty() {
            break 'parse;
        }

        if subtable_format == 1 {
            // `header`'s cursor is already at `subtable_offset + 4` here.
            let Ok(delta) = header.u16() else {
                break 'parse;
            };
            to = otl_coverage_create();
            for j in 0..(*from).len() {
                (*to).push(
                    handle_from_index(((&(*from))[j].index as i32 + delta as i32) as GlyphId)
                        as GlyphHandle,
                );
            }
        } else {
            let Ok(toglyphs) = header.u16() else {
                break 'parse;
            };
            if toglyphs as usize != (*from).len() {
                break 'parse;
            }
            if header.require_room(toglyphs as usize, 2).is_err() {
                break 'parse;
            }
            to = otl_coverage_create();
            for _ in 0..toglyphs {
                (*to).push(handle_from_index(header.u16().unwrap() as GlyphId) as GlyphHandle);
            }
        }

        for j_1 in 0..(*from).len() {
            (*subtable).push(GsubSingleEntry {
                from: otfcc_handle_dup((&(*from))[j_1].clone() as Handle) as GlyphHandle,
                to: otfcc_handle_dup((&(*to))[j_1].clone() as Handle) as GlyphHandle,
            });
        }
        if !from.is_null() {
            otl_coverage_free(from);
        }
        if !to.is_null() {
            otl_coverage_free(to);
        }
        return subtable_from_raw(subtable, Subtable::GsubSingle);
    }

    subtable_gsub_single_free(subtable);
    if !from.is_null() {
        otl_coverage_free(from);
    }
    if !to.is_null() {
        otl_coverage_free(to);
    }
    ::core::ptr::null_mut::<Subtable>()
}
pub unsafe fn otl_gsub_dump_single(mut _subtable: *const Subtable) -> BuiltValue {
    let Subtable::GsubSingle(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GsubSingleSubtable = mut_subtable;
    let mut st = BuiltValue::new_object((*subtable).len());
    let mut j: usize = 0_usize;
    while j < (*subtable).len() {
        st.push_field_bytes_key(
            &(&(*subtable))[j].from.name,
            BuiltValue::str_truncated_at_nul(&(&(*subtable))[j].to.name),
        );
        j = j.wrapping_add(1);
    }
    st
}
pub unsafe fn otl_gsub_parse_single(
    mut _subtable: *const ParsedValue,
    mut _options: &Options,
) -> *mut Subtable {
    let subtable: *mut GsubSingleSubtable = subtable_gsub_single_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < json_obj_len(_subtable) {
        let val = json_obj_val_at(_subtable, j as u32);
        if !val.is_null()
            && json_type_of(val) as ::core::ffi::c_uint
                == JsonType::String as i32 as ::core::ffi::c_uint
        {
            let from: GlyphHandle =
                handle_from_name(Some(json_obj_key_bytes_at(_subtable, j as u32))) as GlyphHandle;
            let to: GlyphHandle = handle_from_name(Some(json_str_bytes(val))) as GlyphHandle;
            (*subtable).push(GsubSingleEntry {
                from: from as GlyphHandle,
                to: to as GlyphHandle,
            });
        }
        j = j.wrapping_add(1);
    }
    return subtable_from_raw(subtable, Subtable::GsubSingle);
}
pub unsafe fn otfcc_build_gsub_single_subtable(
    mut _subtable: *const Subtable,
    heuristics: BuildHeuristics,
) -> Buffer {
    let Subtable::GsubSingle(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GsubSingleSubtable = mut_subtable;
    let mut is_constant_difference: bool = (*subtable).len() > 0_usize;
    if is_constant_difference {
        let difference: i32 =
            (&(*subtable))[0].to.index as i32 - (&(*subtable))[0].from.index as i32;
        is_constant_difference = is_constant_difference as i32 != 0
            && difference < 0x8000_i32
            && difference > -0x8000_i32;
        let mut j: GlyphId = 1 as GlyphId;
        while (j as usize) < (*subtable).len() {
            let diff_j: i32 = (&(*subtable))[j as usize].to.index as i32
                - (&(*subtable))[j as usize].from.index as i32;
            is_constant_difference = is_constant_difference as i32 != 0
                && diff_j == difference
                && diff_j < 0x8000_i32
                && diff_j > -0x8000_i32;
            j = j.wrapping_add(1);
        }
    }
    let cov: *mut Coverage = otl_coverage_create();
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).len() {
        push_to_coverage(
            cov,
            otfcc_handle_dup((&(*subtable))[j_0 as usize].from.clone() as Handle) as GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let coverage_buf: Buffer =
        build_coverage_format(cov, heuristics.contains(BuildHeuristics::GSUB_VERT) as u16);
    if is_constant_difference as i32 != 0
        && !heuristics.contains(BuildHeuristics::GSUB_VERT)
    {
        let b: *mut BkBlock = bk_new_block(&[
            bk_int(BkCellType::B16, 1_u32),
            bk_ptr(BkCellType::P16, bk_new_block_from_buffer(Some(coverage_buf))),
            bk_int(
                BkCellType::B16,
                ((&(*subtable))[0].to.index as i32
                    - (&(*subtable))[0].from.index as i32) as u32,
            ),
        ]);
        otl_coverage_free(cov);
        return bk_build_block(b);
    } else {
        let b_0: *mut BkBlock = bk_new_block(&[
            bk_int(BkCellType::B16, 2_u32),
            bk_ptr(BkCellType::P16, bk_new_block_from_buffer(Some(coverage_buf))),
            bk_int(BkCellType::B16, ((*subtable).len()) as u32),
        ]);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as usize) < (*subtable).len() {
            bk_push(
                b_0,
                &[bk_int(
                    BkCellType::B16,
                    ((&(*subtable))[k as usize].to.index as i32) as u32,
                )],
            );
            k = k.wrapping_add(1);
        }
        otl_coverage_free(cov);
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
        unsafe {
            let raw =
                otl_read_gsub_single(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::GsubSingle(entries) = &*boxed else {
                unreachable!()
            };
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].from.index, 5);
            assert_eq!(entries[0].to.index, 105);
        }
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
        unsafe {
            let raw =
                otl_read_gsub_single(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::GsubSingle(entries) = &*boxed else {
                unreachable!()
            };
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].from.index, 5);
            assert_eq!(entries[0].to.index, 42);
        }
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
        unsafe {
            let raw =
                otl_read_gsub_single(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(raw.is_null());
        }
    }
}
