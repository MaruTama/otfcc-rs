#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::font_reader::FontReader;
use crate::support::handle::{GlyphHandle, Handle, handle_from_name, otfcc_handle_dup};
use crate::support::parsed_json::{
    ParsedValue, json_obj_get, json_obj_key_bytes_at, json_obj_len, json_obj_val_at, json_type_of,
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
use crate::support::built_json::{
    BuiltValue, json_object_new, json_object_push, json_object_push_bytes_key, preserialize,
};
use crate::table::otl::coverage::build_coverage;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::subtables::gpos_common::{
    bk_from_anchor, otl_anchor_absent, otl_dump_anchor, otl_parse_anchor, otl_read_anchor,
};
use crate::table::otl::{GposCursiveEntry, GposCursiveSubtable, Subtable, subtable_from_raw};
// `GposCursiveEntry` holds only a `GlyphHandle` plus two plain `Anchor`
// values, so dropping the `Vec` runs `Handle`'s own `Drop` for every entry --
// no per-element dtor needed anymore.
pub(crate) unsafe fn dispose_gpos_cursive_subtable(arr: *mut GposCursiveSubtable) {
    *arr = Vec::new();
}
pub(crate) unsafe fn subtable_gpos_cursive_free(x: *mut GposCursiveSubtable) {
    if x.is_null() {
        return;
    }
    // `Box::from_raw` reclaims exactly the allocation `_create()` made below
    // and runs the `Vec`'s own drop glue -- no separate dispose-then-`free`
    // needed (Stage 7-2-d; `dispose_gpos_cursive_subtable` stays, it is still
    // used by `table/otl.rs`'s `Drop for Subtable` and `consolidate/otl/
    // gpos_cursive.rs`, just no longer from here).
    drop(Box::from_raw(x));
}
unsafe fn subtable_gpos_cursive_create() -> *mut GposCursiveSubtable {
    Box::into_raw(Box::new(Vec::new()))
}
pub unsafe fn otl_read_gpos_cursive(
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    _max_glyphs: GlyphId,
) -> *mut Subtable {
    let subtable: *mut GposCursiveSubtable = subtable_gpos_cursive_create();
    let mut targets: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
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
        let Ok(value_count) = header.u16() else {
            break 'parse;
        };

        targets = read_coverage(data, table_length, offset.wrapping_add(from_rel as u32));
        if targets.is_null() || (*targets).is_empty() {
            break 'parse;
        }
        if header.require_room(value_count as usize, 4).is_err() {
            break 'parse;
        }
        if value_count as usize != (*targets).len() {
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
                otl_read_anchor(data, table_length, offset.wrapping_add(enter_offset as u32))
            } else {
                otl_anchor_absent()
            };
            let exit = if exit_offset != 0 {
                otl_read_anchor(data, table_length, offset.wrapping_add(exit_offset as u32))
            } else {
                otl_anchor_absent()
            };
            (*subtable).push(GposCursiveEntry {
                target: otfcc_handle_dup((&(*targets))[j as usize].clone() as Handle)
                    as GlyphHandle,
                enter,
                exit,
            });
        }
        otl_coverage_free(targets);
        return subtable_from_raw(subtable, Subtable::GposCursive);
    }

    if !targets.is_null() {
        otl_coverage_free(targets);
    }
    subtable_gpos_cursive_free(subtable);
    ::core::ptr::null_mut::<Subtable>()
}
pub unsafe fn otl_gpos_dump_cursive(mut _subtable: *const Subtable) -> *mut BuiltValue {
    let Subtable::GposCursive(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GposCursiveSubtable = mut_subtable;
    let st: *mut BuiltValue = json_object_new((*subtable).len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).len() {
        let rec: *mut BuiltValue = json_object_new(2_usize);
        json_object_push(
            rec,
            b"enter\0" as *const u8 as *const ::core::ffi::c_char,
            otl_dump_anchor((&(*subtable))[j as usize].enter),
        );
        json_object_push(
            rec,
            b"exit\0" as *const u8 as *const ::core::ffi::c_char,
            otl_dump_anchor((&(*subtable))[j as usize].exit),
        );
        json_object_push_bytes_key(
            st,
            &(&(*subtable))[j as usize].target.name,
            preserialize(rec),
        );
        j = j.wrapping_add(1);
    }
    return st;
}
pub unsafe fn otl_gpos_parse_cursive(
    mut _subtable: *const ParsedValue,
    mut _options: &Options,
) -> *mut Subtable {
    let subtable: *mut GposCursiveSubtable = subtable_gpos_cursive_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < json_obj_len(_subtable) {
        let val = json_obj_val_at(_subtable, j as u32);
        if !val.is_null()
            && json_type_of(val) as ::core::ffi::c_uint
                == JsonType::Object as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*subtable).push(GposCursiveEntry {
                target: handle_from_name(Some(json_obj_key_bytes_at(_subtable, j as u32)))
                    as GlyphHandle,
                enter: otl_parse_anchor(json_obj_get(
                    val,
                    b"enter\0" as *const u8 as *const ::core::ffi::c_char,
                )),
                exit: otl_parse_anchor(json_obj_get(
                    val,
                    b"exit\0" as *const u8 as *const ::core::ffi::c_char,
                )),
            });
        }
        j = j.wrapping_add(1);
    }
    return subtable_from_raw(subtable, Subtable::GposCursive);
}
pub unsafe fn otfcc_build_gpos_cursive(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let Subtable::GposCursive(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GposCursiveSubtable = mut_subtable;
    let cov: *mut Coverage = otl_coverage_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).len() {
        push_to_coverage(
            cov,
            otfcc_handle_dup((&(*subtable))[j as usize].target.clone() as Handle) as GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B16, 1_u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(build_coverage(cov)),
        ),
        bk_int(BkCellType::B16, ((*subtable).len()) as u32),
    ]);
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).len() {
        bk_push(
            root,
            &[
                bk_ptr(
                    BkCellType::P16,
                    bk_from_anchor((&(*subtable))[j_0 as usize].enter),
                ),
                bk_ptr(
                    BkCellType::P16,
                    bk_from_anchor((&(*subtable))[j_0 as usize].exit),
                ),
            ],
        );
        j_0 = j_0.wrapping_add(1);
    }
    otl_coverage_free(cov);
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
        unsafe {
            let raw =
                otl_read_gpos_cursive(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::GposCursive(entries) = &*boxed else {
                unreachable!()
            };
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].target.index, 5);
        }
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
        unsafe {
            let raw =
                otl_read_gpos_cursive(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(raw.is_null());
        }
    }
}
