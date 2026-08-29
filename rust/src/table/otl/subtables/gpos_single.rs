#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::font_reader::FontReader;
use crate::support::handle::{GlyphHandle, Handle, handle_from_name, otfcc_handle_dup};
use crate::support::parsed_json::{
    ParsedValue, json_obj_key_bytes_at, json_obj_len, json_obj_val_at, json_type_of,
};
use crate::table::otl::coverage::{
    Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage,
};

use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::support::buffer::Buffer;
use crate::support::options::Options;
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::json::JsonType;

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkgraph::bk_build_block;
use crate::support::built_json::{BuiltValue, json_object_new, json_object_push_bytes_key};
use crate::table::otl::coverage::build_coverage;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::subtables::gpos_common::{
    bk_gpos_value, gpos_dump_value, gpos_parse_value, position_format_length, read_gpos_value,
    required_position_format,
};
use crate::table::otl::{
    GposSingleEntry, GposSingleSubtable, PositionValue, Subtable, subtable_from_raw,
};
// `GposSingleEntry` holds only a `GlyphHandle` plus a plain `PositionValue`,
// so dropping the `Vec` runs `Handle`'s own `Drop` for every entry -- no
// per-element dtor needed anymore.
pub(crate) unsafe fn dispose_gpos_single_subtable(arr: *mut GposSingleSubtable) {
    *arr = Vec::new();
}
pub(crate) unsafe fn subtable_gpos_single_free(x: *mut GposSingleSubtable) {
    if x.is_null() {
        return;
    }
    // `Box::from_raw` reclaims exactly the allocation `_create()` made below
    // and runs the `Vec`'s own drop glue -- no separate dispose-then-`free`
    // needed (Stage 7-2-d; `dispose_gpos_single_subtable` stays, it is still
    // used by `table/otl.rs`'s `Drop for Subtable` and `consolidate/otl/
    // gpos_single.rs`, just no longer from here).
    drop(Box::from_raw(x));
}
unsafe fn subtable_gpos_single_create() -> *mut GposSingleSubtable {
    Box::into_raw(Box::new(Vec::new()))
}
pub unsafe fn otl_read_gpos_single(
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    _max_glyphs: GlyphId,
) -> *mut Subtable {
    let subtable: *mut GposSingleSubtable = subtable_gpos_single_create();
    let mut targets: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);

    'parse: {
        let mut header = match FontReader::new(slice).at(offset as usize) {
            Ok(r) => r,
            Err(_) => break 'parse,
        };
        let Ok(subtable_format) = header.u16() else {
            break 'parse;
        };
        let Ok(from_rel) = header.u16() else {
            break 'parse;
        };

        targets = read_coverage(data, table_length, offset.wrapping_add(from_rel as u32));
        if targets.is_null() || (*targets).is_empty() {
            break 'parse;
        }

        if subtable_format == 1 {
            let Ok(value_format) = header.u16() else {
                break 'parse;
            };
            let v: PositionValue =
                read_gpos_value(data, table_length, offset.wrapping_add(6), value_format);
            for j in 0..(*targets).len() {
                (*subtable).push(GposSingleEntry {
                    target: otfcc_handle_dup((&(*targets))[j].clone() as Handle) as GlyphHandle,
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
            if value_count as usize != (*targets).len() {
                break 'parse;
            }
            for j in 0..(*targets).len() {
                (*subtable).push(GposSingleEntry {
                    target: otfcc_handle_dup((&(*targets))[j].clone() as Handle) as GlyphHandle,
                    value: read_gpos_value(
                        data,
                        table_length,
                        offset.wrapping_add(8).wrapping_add((j * stride) as u32),
                        value_format,
                    ),
                });
            }
        }

        if !targets.is_null() {
            otl_coverage_free(targets);
        }
        return subtable_from_raw(subtable, Subtable::GposSingle);
    }

    if !targets.is_null() {
        otl_coverage_free(targets);
    }
    subtable_gpos_single_free(subtable);
    ::core::ptr::null_mut::<Subtable>()
}
pub unsafe fn otl_gpos_dump_single(mut _subtable: *const Subtable) -> *mut BuiltValue {
    let Subtable::GposSingle(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GposSingleSubtable = mut_subtable;
    let st: *mut BuiltValue = json_object_new((*subtable).len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).len() {
        json_object_push_bytes_key(
            st,
            &(&(*subtable))[j as usize].target.name,
            gpos_dump_value((&(*subtable))[j as usize].value),
        );
        j = j.wrapping_add(1);
    }
    return st;
}
pub unsafe fn otl_gpos_parse_single(
    mut _subtable: *const ParsedValue,
    mut _options: &Options,
) -> *mut Subtable {
    let subtable: *mut GposSingleSubtable = subtable_gpos_single_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < json_obj_len(_subtable) {
        let val = json_obj_val_at(_subtable, j as u32);
        if !val.is_null()
            && json_type_of(val) as ::core::ffi::c_uint
                == JsonType::Object as i32 as ::core::ffi::c_uint
        {
            (*subtable).push(GposSingleEntry {
                target: handle_from_name(Some(json_obj_key_bytes_at(_subtable, j as u32)))
                    as GlyphHandle,
                value: gpos_parse_value(val),
            });
        }
        j = j.wrapping_add(1);
    }
    return subtable_from_raw(subtable, Subtable::GposSingle);
}
pub unsafe fn otfcc_build_gpos_single(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let Subtable::GposSingle(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GposSingleSubtable = mut_subtable;
    let mut is_const: bool = (*subtable).len() > 0_usize;
    let mut format: u16 = 0_u16;
    if (*subtable).len() > 0_usize {
        let mut j: GlyphId = 0 as GlyphId;
        while (j as usize) < (*subtable).len() {
            is_const = is_const as i32 != 0
                && (&(*subtable))[j as usize].value.dx == (&(*subtable))[0].value.dx
                && (&(*subtable))[j as usize].value.dy == (&(*subtable))[0].value.dy
                && (&(*subtable))[j as usize].value.d_width == (&(*subtable))[0].value.d_width
                && (&(*subtable))[j as usize].value.d_height == (&(*subtable))[0].value.d_height;
            format = (format as i32
                | required_position_format((&(*subtable))[j as usize].value) as i32)
                as u16;
            j = j.wrapping_add(1);
        }
    }
    let cov: *mut Coverage = otl_coverage_create();
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).len() {
        push_to_coverage(
            cov,
            otfcc_handle_dup((&(*subtable))[j_0 as usize].target.clone() as Handle) as GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let coverage_buf: *mut Buffer = build_coverage(cov);
    if is_const {
        let b: *mut BkBlock = bk_new_block(&[
            bk_int(BkCellType::B16, 1_u32),
            bk_ptr(BkCellType::P16, bk_new_block_from_buffer(coverage_buf)),
            bk_int(BkCellType::B16, (format as i32) as u32),
            bk_ptr(
                BkCellType::Embed,
                bk_gpos_value((&(*subtable))[0].value, format),
            ),
        ]);
        otl_coverage_free(cov);
        return bk_build_block(b);
    } else {
        let b_0: *mut BkBlock = bk_new_block(&[
            bk_int(BkCellType::B16, 2_u32),
            bk_ptr(BkCellType::P16, bk_new_block_from_buffer(coverage_buf)),
            bk_int(BkCellType::B16, (format as i32) as u32),
            bk_int(BkCellType::B16, ((*subtable).len()) as u32),
        ]);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as usize) < (*subtable).len() {
            bk_push(
                b_0,
                &[bk_ptr(
                    BkCellType::Embed,
                    bk_gpos_value((&(*subtable))[k as usize].value, format),
                )],
            );
            k = k.wrapping_add(1);
        }
        otl_coverage_free(cov);
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
        unsafe {
            let raw =
                otl_read_gpos_single(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::GposSingle(entries) = &*boxed else {
                unreachable!()
            };
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].target.index, 9);
            assert_eq!(entries[0].value.dx, 77.0);
        }
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
        unsafe {
            let raw =
                otl_read_gpos_single(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::GposSingle(entries) = &*boxed else {
                unreachable!()
            };
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].target.index, 5);
            assert_eq!(entries[0].value.dx, 50.0);
        }
    }
}
