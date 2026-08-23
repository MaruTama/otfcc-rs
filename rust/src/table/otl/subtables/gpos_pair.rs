#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::handle::{GlyphHandle, Handle, handle_from_index, otfcc_handle_dup};
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_dbl_val, json_int_val, json_obj_get,
    json_obj_get_type, json_type_of,
};
use crate::table::otl::classdef::{
    ClassDef, classdef_from_raw, expand_class_def, otl_class_def_create, read_class_def,
};
use crate::table::otl::coverage::{
    Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage,
    shrink_coverage,
};

use crate::support::font_reader::FontReader;

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::bk::bkgraph::BkGraph;
use crate::bk::bkgraph::{
    bk_build_graph, bk_delete_graph, bk_estimate_size_of_graph, bk_minimize_graph,
    bk_new_graph_from_root_block, bk_untangle_graph,
};
use crate::support::buffer::Buffer;
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_new_position, json_object_new,
    json_object_push, preserialize,
};
use crate::support::options::Options;
use crate::support::primitives::{FontFilePointer, GlyphClass, GlyphId, Pos, TableId};
use crate::table::otl::classdef::{build_class_def, dump_class_def, parse_class_def};
use crate::table::otl::coverage::build_coverage;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::subtables::gpos_common::{
    FORMAT_DWIDTH, bk_gpos_value, gpos_dump_value, gpos_parse_value, position_format_length,
    position_zero, read_gpos_value, required_position_format,
};
use crate::table::otl::{GposPairSubtable, PositionValue, Subtable, subtable_from_raw};
use crate::vendor::json::JsonType;

// `fv`/`sv` hold the matched cell's value directly now, not a pointer into
// `first_values`/`second_values` -- `PositionValue` is `Copy`, and with the
// grid a real `Vec<Vec<PositionValue>>` there is no backing array to point
// into that outlives this struct's own use (a single build pass).
#[derive(Copy, Clone)]
pub struct IndividualGposPair {
    pub gid: GlyphId,
    pub fv: PositionValue,
    pub sv: PositionValue,
}
#[inline]
unsafe fn subtable_gpos_pair_create() -> *mut GposPairSubtable {
    Box::into_raw(Box::new(GposPairSubtable {
        first: None,
        second: None,
        first_values: Vec::new(),
        second_values: Vec::new(),
    }))
}
#[inline]
unsafe fn subtable_gpos_pair_free(mut x: *mut GposPairSubtable) {
    if x.is_null() {
        return;
    }
    // `Box::from_raw` reclaims exactly the allocation `_create()` made above
    // and runs `first`/`second`/`first_values`/`second_values`'s own drop
    // glue directly -- no separate dispose-then-`free` needed (Stage 7-2-d).
    // `dispose_gpos_pair`/`subtable_gpos_pair_dispose` had no other callers,
    // so they're gone along with `init_gpos_pair`/`subtable_gpos_pair_init`.
    drop(Box::from_raw(x));
}
// Two real bugs fixed, one per format:
//
// Format 1's initial `coverageOffset` (and everything after it) used to be
// read with *no* guard beyond the very first `table_length < offset + 2`
// (just the 2-byte format field itself) -- a 3-or-more-byte-short table
// claiming format 1 read straight past its own end before any of the
// per-field guards further down ever ran. `FontReader`'s sequential reads
// close this by construction: every field, not just the ones the original
// happened to guard explicitly, is checked.
//
// Format 2's final byte-length guard -- `class1_count * class2_count *
// (len1+len2)` -- is the same overflow-defeats-guard shape as
// `gpos_mark_to_ligature.rs`'s `component_count * class_count`:
// `class1_count`/`class2_count` are independently unbounded `u16` fields,
// so the product can reach 65535*65535*16 (~68.7 billion), far past
// `i32::MAX`. Fixed with two chained `checked_mul`s on `usize` (count*count,
// then that product against the per-cell stride via `require_room`).
pub unsafe fn otl_read_gpos_pair(
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    _max_glyphs: GlyphId,
) -> *mut Subtable {
    let subtable: *mut GposPairSubtable = subtable_gpos_pair_create();
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);

    'parse: {
        let Ok(subtable_format) = FontReader::new(slice)
            .at(offset as usize)
            .and_then(|mut r| r.u16())
        else {
            break 'parse;
        };

        if subtable_format == 1 {
            let Ok(mut header) = FontReader::new(slice).at(offset as usize + 2) else {
                break 'parse;
            };
            let Ok(cov_rel) = header.u16() else {
                break 'parse;
            };

            // Built through a local raw pointer first (matches
            // `otl_class_def_create`'s own raw-pointer API), then adopted
            // into `(*subtable).first` as soon as it's fully constructed --
            // matching the original's immediate field assignment, so every
            // exit path below still disposes it correctly via
            // `subtable_gpos_pair_free`'s `Box::from_raw`.
            let cov = read_coverage(data, table_length, offset.wrapping_add(cov_rel as u32));
            let first_raw: *mut ClassDef = otl_class_def_create();
            (*first_raw).glyphs = ::core::mem::take(&mut *cov);
            (*first_raw).maxclass = ((*first_raw).glyphs.len() as i32 - 1) as GlyphClass;
            (*first_raw).classes = (0..(*first_raw).glyphs.len())
                .map(|j| j as GlyphClass)
                .collect();
            otl_coverage_free(cov);
            (*subtable).first = classdef_from_raw(first_raw);
            let first_cd: *mut ClassDef = (*subtable).first.as_deref_mut().unwrap();

            let Ok(format1) = header.u16() else {
                break 'parse;
            };
            let Ok(format2) = header.u16() else {
                break 'parse;
            };
            let len1 = position_format_length(format1);
            let len2 = position_format_length(format2);
            let Ok(pair_set_count) = header.u16() else {
                break 'parse;
            };
            if pair_set_count as usize != (*first_cd).glyphs.len() {
                break 'parse;
            }
            if header.require_room(pair_set_count as usize, 2).is_err() {
                break 'parse;
            }
            let mut pair_set_offsets = Vec::with_capacity(pair_set_count as usize);
            for _ in 0..pair_set_count {
                pair_set_offsets.push(offset.wrapping_add(header.u16().unwrap() as u32));
            }

            // Validate every PairSet's own header + record array up front,
            // matching the original's separate validation pass.
            let stride = 2usize + len1 as usize + len2 as usize;
            let mut pair_counts = Vec::with_capacity(pair_set_count as usize);
            for &pso in &pair_set_offsets {
                let Ok(mut pr) = FontReader::new(slice).at(pso as usize) else {
                    break 'parse;
                };
                let Ok(pc) = pr.u16() else { break 'parse };
                if pr.require_room(pc as usize, stride).is_err() {
                    break 'parse;
                }
                pair_counts.push(pc);
            }

            // Deduplicates the "second" glyph of every pair by gid,
            // assigning each distinct one the next sequential class id
            // (1-based -- class 0 is reserved for "not covered" per the
            // OpenType format) -- synthesizing a class def for `second`
            // that Format 1 (individual pairs) doesn't carry on the wire,
            // the same way Format 2 already does explicitly. No
            // `HASH_SORT` is used before the original's `HASH_ITER` here,
            // so output order is insertion order; since cid is assigned as
            // `num_items + 1` at insert time, insertion order and
            // cid-ascending order are the same order by construction,
            // matching the `IndexSet`-not-a-map shape of
            // `LigatureAggregator` (see rust/README.md) but with cid
            // derived from position instead of tracked separately. This
            // one set is used across two phases within this branch: built
            // while first reading every pair (below), then looked up (not
            // rebuilt) while re-reading the same pairs a second time to
            // place position values into the `first_values`/
            // `second_values` grid, then walked once more at the end to
            // populate `(*subtable).second`'s `glyphs`/`classes`.
            let mut h: indexmap::IndexSet<i32> = indexmap::IndexSet::new();
            for (i, &pso) in pair_set_offsets.iter().enumerate() {
                for k in 0..pair_counts[i] {
                    let second_offset = pso as usize + 2 + stride * k as usize;
                    // Already validated by the pass above.
                    let second = FontReader::new(slice)
                        .at(second_offset)
                        .unwrap()
                        .u16()
                        .unwrap() as i32;
                    h.insert(second);
                }
            }

            let second_raw: *mut ClassDef = otl_class_def_create();
            let n_second = h.len();
            (*second_raw).maxclass = n_second as GlyphClass;
            (*second_raw).classes = vec![0 as GlyphClass; n_second];
            (*second_raw).glyphs = vec![GlyphHandle::default(); n_second];
            (*subtable).second = classdef_from_raw(second_raw);
            let second_cd: *mut ClassDef = (*subtable).second.as_deref_mut().unwrap();
            let class2_count = (*second_cd).maxclass as usize + 1;

            // Was a manual `__caryll_allocate_clean` + nested-loop-of-
            // `position_zero()` writes over `*mut *mut PositionValue` --
            // `PositionValue` is `Copy`, so pre-sizing the whole grid
            // collapses to one `vec![vec![..]; ..]` expression; the real
            // values below are then index-assigned directly.
            let first_class_count = (*first_cd).maxclass as usize + 1;
            let mut first_values: Vec<Vec<PositionValue>> =
                vec![vec![position_zero(); class2_count]; first_class_count];
            let mut second_values: Vec<Vec<PositionValue>> =
                vec![vec![position_zero(); class2_count]; first_class_count];

            for (j3, &pso) in pair_set_offsets.iter().enumerate() {
                for k1 in 0..pair_counts[j3] {
                    let second_offset = pso as usize + 2 + stride * k1 as usize;
                    let second = FontReader::new(slice)
                        .at(second_offset)
                        .unwrap()
                        .u16()
                        .unwrap() as i32;
                    if let Some(idx) = h.get_index_of(&second) {
                        let cid = idx + 1;
                        first_values[j3][cid] = read_gpos_value(
                            data,
                            table_length,
                            (second_offset + 2) as u32,
                            format1,
                        );
                        second_values[j3][cid] = read_gpos_value(
                            data,
                            table_length,
                            (second_offset + 2 + len1 as usize) as u32,
                            format2,
                        );
                    }
                }
            }
            (*subtable).first_values = first_values;
            (*subtable).second_values = second_values;
            for (jj, &gid) in h.iter().enumerate() {
                (&mut (*second_cd).glyphs)[jj] = handle_from_index(gid as GlyphId) as GlyphHandle;
                (&mut (*second_cd).classes)[jj] = (jj + 1) as GlyphClass;
            }
            return subtable_from_raw(subtable, Subtable::GposPair);
        } else if subtable_format == 2 {
            let Ok(mut header) = FontReader::new(slice).at(offset as usize + 2) else {
                break 'parse;
            };
            let Ok(cov_rel) = header.u16() else {
                break 'parse;
            };
            let Ok(format1_0) = header.u16() else {
                break 'parse;
            };
            let Ok(format2_0) = header.u16() else {
                break 'parse;
            };
            let Ok(cd1_rel) = header.u16() else {
                break 'parse;
            };
            let Ok(cd2_rel) = header.u16() else {
                break 'parse;
            };
            let Ok(class1_count) = header.u16() else {
                break 'parse;
            };
            let Ok(class2_count) = header.u16() else {
                break 'parse;
            };
            let len1_0 = position_format_length(format1_0);
            let len2_0 = position_format_length(format2_0);

            let cov_0 = read_coverage(data, table_length, offset.wrapping_add(cov_rel as u32));
            // `expand_class_def` consumes (and internally frees) the `ocd`
            // it's handed and returns a brand-new `*mut ClassDef` -- kept
            // as a plain local raw pointer through that consuming call,
            // then adopted into `(*subtable).first` only once settled.
            let mut first_raw: *mut ClassDef =
                read_class_def(data, table_length, offset.wrapping_add(cd1_rel as u32));
            first_raw = expand_class_def(cov_0, first_raw);
            otl_coverage_free(cov_0);
            (*subtable).first = classdef_from_raw(first_raw);
            (*subtable).second = classdef_from_raw(read_class_def(
                data,
                table_length,
                offset.wrapping_add(cd2_rel as u32),
            ));
            if (*subtable).first.is_none() || (*subtable).second.is_none() {
                break 'parse;
            }
            let first_cd: *mut ClassDef = (*subtable).first.as_deref_mut().unwrap();
            let second_cd: *mut ClassDef = (*subtable).second.as_deref_mut().unwrap();
            if (*first_cd).maxclass as usize + 1 != class1_count as usize {
                break 'parse;
            }
            if (*second_cd).maxclass as usize + 1 != class2_count as usize {
                break 'parse;
            }

            let stride = len1_0 as usize + len2_0 as usize;
            let Some(total_cells) = (class1_count as usize).checked_mul(class2_count as usize)
            else {
                break 'parse;
            };
            let Ok(matrix) = FontReader::new(slice).at(offset as usize + 16) else {
                break 'parse;
            };
            if matrix.require_room(total_cells, stride).is_err() {
                break 'parse;
            }

            // Format 2 fills every cell exhaustively and in order, so
            // (unlike Format 1's `cid`-indexed overwrite pass) no
            // pre-sized placeholder grid is needed -- each row is just
            // pushed as it's read.
            let mut first_values: Vec<Vec<PositionValue>> =
                Vec::with_capacity(class1_count as usize);
            let mut second_values: Vec<Vec<PositionValue>> =
                Vec::with_capacity(class1_count as usize);
            for j4 in 0..class1_count as u32 {
                let mut row1 = Vec::with_capacity(class2_count as usize);
                let mut row2 = Vec::with_capacity(class2_count as usize);
                for k2 in 0..class2_count as u32 {
                    // Safe from u32 overflow: `require_room` above already
                    // rejected any input where `total_cells * stride`
                    // wouldn't fit in `table_length` (itself a `u32`), so
                    // every offset computed here is bounded by that.
                    let cell_offset = offset
                        .wrapping_add(16)
                        .wrapping_add((j4 * class2_count as u32 + k2) * stride as u32);
                    row1.push(read_gpos_value(data, table_length, cell_offset, format1_0));
                    row2.push(read_gpos_value(
                        data,
                        table_length,
                        cell_offset + len1_0 as u32,
                        format2_0,
                    ));
                }
                first_values.push(row1);
                second_values.push(row2);
            }
            (*subtable).first_values = first_values;
            (*subtable).second_values = second_values;
            return subtable_from_raw(subtable, Subtable::GposPair);
        }
    }
    subtable_gpos_pair_free(subtable);
    ::core::ptr::null_mut::<Subtable>()
}
pub unsafe extern "C" fn otl_gpos_dump_pair(mut _subtable: *const Subtable) -> *mut BuiltValue {
    let Subtable::GposPair(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GposPairSubtable = mut_subtable;
    let first_cd: *const ClassDef = (*subtable).first.as_deref().unwrap();
    let second_cd: *const ClassDef = (*subtable).second.as_deref().unwrap();
    let mut st: *mut BuiltValue = json_object_new(3 as usize);
    json_object_push(
        st,
        b"first\0" as *const u8 as *const ::core::ffi::c_char,
        dump_class_def(first_cd),
    );
    json_object_push(
        st,
        b"second\0" as *const u8 as *const ::core::ffi::c_char,
        dump_class_def(second_cd),
    );
    let mut mat: *mut BuiltValue = json_array_new(
        ((*first_cd).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
    );
    let mut j: GlyphClass = 0 as GlyphClass;
    while j as ::core::ffi::c_int <= (*first_cd).maxclass as ::core::ffi::c_int {
        let mut row: *mut BuiltValue = json_array_new(
            ((*second_cd).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
        );
        let mut k: GlyphClass = 0 as GlyphClass;
        while k as ::core::ffi::c_int <= (*second_cd).maxclass as ::core::ffi::c_int {
            let mut f1: u8 =
                required_position_format((&(*subtable).first_values)[j as usize][k as usize]);
            let mut f2: u8 =
                required_position_format((&(*subtable).second_values)[j as usize][k as usize]);
            if f1 as ::core::ffi::c_int | f2 as ::core::ffi::c_int != 0 {
                if f1 as ::core::ffi::c_int == FORMAT_DWIDTH as ::core::ffi::c_int
                    && f2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    json_array_push(
                        row,
                        json_new_position(
                            (&(*subtable).first_values)[j as usize][k as usize].d_width,
                        ),
                    );
                } else {
                    let mut pair: *mut BuiltValue = json_object_new(2 as usize);
                    if f1 != 0 {
                        json_object_push(
                            pair,
                            b"first\0" as *const u8 as *const ::core::ffi::c_char,
                            gpos_dump_value((&(*subtable).first_values)[j as usize][k as usize]),
                        );
                    }
                    if f2 != 0 {
                        json_object_push(
                            pair,
                            b"second\0" as *const u8 as *const ::core::ffi::c_char,
                            gpos_dump_value((&(*subtable).second_values)[j as usize][k as usize]),
                        );
                    }
                    json_array_push(row, pair);
                }
            } else {
                json_array_push(row, json_new_position(0 as ::core::ffi::c_int as Pos));
            }
            k = k.wrapping_add(1);
        }
        json_array_push(mat, preserialize(row));
        j = j.wrapping_add(1);
    }
    json_object_push(
        st,
        b"matrix\0" as *const u8 as *const ::core::ffi::c_char,
        mat,
    );
    return st;
}
pub unsafe fn otl_gpos_parse_pair(
    mut _subtable: *const ParsedValue,
    mut _options: &Options,
) -> *mut Subtable {
    let mut class1_count: GlyphClass = 0;
    let mut class2_count: GlyphClass = 0;
    let mut subtable: *mut GposPairSubtable = (subtable_gpos_pair_create)();
    let mut _mat: *const ParsedValue = json_obj_get_type(
        _subtable,
        b"matrix\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    (*subtable).first = classdef_from_raw(parse_class_def(json_obj_get_type(
        _subtable,
        b"first\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    )));
    (*subtable).second = classdef_from_raw(parse_class_def(json_obj_get_type(
        _subtable,
        b"second\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    )));
    if _mat.is_null() || (*subtable).first.is_none() || (*subtable).second.is_none() {
        subtable_gpos_pair_free(subtable);
        return ::core::ptr::null_mut::<Subtable>();
    } else {
        let first_cd: *mut ClassDef = (*subtable).first.as_deref_mut().unwrap();
        let second_cd: *mut ClassDef = (*subtable).second.as_deref_mut().unwrap();
        class1_count =
            ((*first_cd).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphClass;
        class2_count =
            ((*second_cd).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphClass;
        let mut first_values: Vec<Vec<PositionValue>> =
            vec![vec![position_zero(); class2_count as usize]; class1_count as usize];
        let mut second_values: Vec<Vec<PositionValue>> =
            vec![vec![position_zero(); class2_count as usize]; class1_count as usize];
        let mut j_0: GlyphClass = 0 as GlyphClass;
        while (j_0 as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int
            && (j_0 as ::core::ffi::c_uint) < json_arr_len(_mat)
        {
            let mut _row: *const ParsedValue = json_arr_at(_mat, j_0 as u32);
            if !(_row.is_null() || json_type_of(_row) != JsonType::Array) {
                let mut k_0: GlyphClass = 0 as GlyphClass;
                while (k_0 as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int
                    && (k_0 as ::core::ffi::c_uint) < json_arr_len(_row)
                {
                    let mut _item: *const ParsedValue = json_arr_at(_row, k_0 as u32);
                    if json_type_of(_item) == JsonType::Integer {
                        first_values[j_0 as usize][k_0 as usize].d_width =
                            json_int_val(_item) as Pos;
                    } else if json_type_of(_item) == JsonType::Double {
                        first_values[j_0 as usize][k_0 as usize].d_width =
                            json_dbl_val(_item) as Pos;
                    } else if json_type_of(_item) == JsonType::Object {
                        first_values[j_0 as usize][k_0 as usize] = gpos_parse_value(json_obj_get(
                            _item,
                            b"first\0" as *const u8 as *const ::core::ffi::c_char,
                        ));
                        second_values[j_0 as usize][k_0 as usize] = gpos_parse_value(json_obj_get(
                            _item,
                            b"second\0" as *const u8 as *const ::core::ffi::c_char,
                        ));
                    }
                    k_0 = k_0.wrapping_add(1);
                }
            }
            j_0 = j_0.wrapping_add(1);
        }
        (*subtable).first_values = first_values;
        (*subtable).second_values = second_values;
        return subtable_from_raw(subtable, Subtable::GposPair);
    };
}
unsafe fn cov_from_cd(mut cd: *const ClassDef) -> *mut Coverage {
    let cov: *mut Coverage = otl_coverage_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*cd).glyphs.len() {
        push_to_coverage(
            cov,
            otfcc_handle_dup((&(*cd).glyphs)[j as usize].clone() as Handle) as GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    return cov;
}
pub unsafe fn otfcc_build_gpos_pair_individual(mut _subtable: *const Subtable) -> *mut BkBlock {
    let Subtable::GposPair(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GposPairSubtable = mut_subtable;
    let first_cd: *const ClassDef = (*subtable).first.as_deref().unwrap();
    let second_cd: *const ClassDef = (*subtable).second.as_deref().unwrap();
    let mut format1: u16 = 0 as u16;
    let mut format2: u16 = 0 as u16;
    let mut class1_count: GlyphClass =
        ((*first_cd).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphClass;
    let mut class2_count: GlyphClass =
        ((*second_cd).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphClass;
    let mut j: GlyphClass = 0 as GlyphClass;
    while (j as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int {
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int {
            format1 = (format1 as ::core::ffi::c_int
                | required_position_format((&(*subtable).first_values)[j as usize][k as usize])
                    as ::core::ffi::c_int) as u16;
            format2 = (format2 as ::core::ffi::c_int
                | required_position_format((&(*subtable).second_values)[j as usize][k as usize])
                    as ::core::ffi::c_int) as u16;
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    // A local `Vec`, not a `__caryll_allocate_clean`/`free` pair -- the
    // zero-fill this function relied on from `__caryll_allocate_clean` is
    // just `vec![0; ...]`, and the `Vec` drops itself at the end instead of
    // needing an explicit `free` to match.
    let mut pair_counts: Vec<GlyphId> = vec![0 as GlyphId; (*first_cd).glyphs.len()];
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*first_cd).glyphs.len() {
        let mut k_0: GlyphId = 0 as GlyphId;
        while (k_0 as usize) < (*second_cd).glyphs.len() {
            let mut c1: GlyphClass = (&(*first_cd).classes)[j_0 as usize];
            let mut c2: GlyphClass = (&(*second_cd).classes)[k_0 as usize];
            if required_position_format((&(*subtable).first_values)[c1 as usize][c2 as usize])
                as ::core::ffi::c_int
                | required_position_format((&(*subtable).second_values)[c1 as usize][c2 as usize])
                    as ::core::ffi::c_int
                != 0
            {
                let ref mut fresh10 = pair_counts[j_0 as usize];
                *fresh10 = (*fresh10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
            }
            k_0 = k_0.wrapping_add(1);
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut cov: *mut Coverage = cov_from_cd(first_cd);
    shrink_coverage(cov, true);
    let mut root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B16, 1 as u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(build_coverage(cov)),
        ),
        bk_int(BkCellType::B16, (format1 as ::core::ffi::c_int) as u32),
        bk_int(BkCellType::B16, (format2 as ::core::ffi::c_int) as u32),
        bk_int(
            BkCellType::B16,
            ((*first_cd).glyphs.len() as ::core::ffi::c_int) as u32,
        ),
    ]);
    let mut j_1: GlyphId = 0 as GlyphId;
    while (j_1 as usize) < (*cov).len() {
        let mut current_pair_count: TableId = 0 as TableId;
        let mut c1_0: GlyphClass = 0 as GlyphClass;
        let mut k_1: GlyphId = 0 as GlyphId;
        while (k_1 as usize) < (*first_cd).glyphs.len() {
            if (&(*first_cd).glyphs)[k_1 as usize].index as ::core::ffi::c_int
                == (&(*cov))[j_1 as usize].index as ::core::ffi::c_int
            {
                c1_0 = (&(*first_cd).classes)[k_1 as usize];
                current_pair_count = pair_counts[k_1 as usize] as TableId;
            }
            k_1 = k_1.wrapping_add(1);
        }
        let mut pair_set: *mut BkBlock = bk_new_block(&[bk_int(
            BkCellType::B16,
            (current_pair_count as ::core::ffi::c_int) as u32,
        )]);
        // A local `Vec`, not a `__caryll_allocate_clean`/`qsort`/`free`
        // trio: built with exactly `current_pair_count` entries by
        // construction (this loop applies the same predicate the earlier
        // counting pass used), sorted with `sort_by_key` (stable, same
        // conservative choice as the `Coverage`/`ClassDef` PR since `qsort`
        // itself gives no stability guarantee), and dropped automatically
        // at the end of this loop iteration instead of needing an explicit
        // `free` to match the explicit allocation.
        let mut pairs: Vec<IndividualGposPair> = Vec::with_capacity(current_pair_count as usize);
        let mut k_2: GlyphId = 0 as GlyphId;
        while (k_2 as usize) < (*second_cd).glyphs.len() {
            let mut c2_0: GlyphClass = (&(*second_cd).classes)[k_2 as usize];
            if required_position_format((&(*subtable).first_values)[c1_0 as usize][c2_0 as usize])
                as ::core::ffi::c_int
                | required_position_format(
                    (&(*subtable).second_values)[c1_0 as usize][c2_0 as usize],
                ) as ::core::ffi::c_int
                != 0
            {
                pairs.push(IndividualGposPair {
                    gid: (&(*second_cd).glyphs)[k_2 as usize].index,
                    fv: (&(*subtable).first_values)[c1_0 as usize][c2_0 as usize],
                    sv: (&(*subtable).second_values)[c1_0 as usize][c2_0 as usize],
                });
            }
            k_2 = k_2.wrapping_add(1);
        }
        pairs.sort_by_key(|p| p.gid);
        for pair in &pairs {
            bk_push(
                pair_set,
                &[
                    bk_int(BkCellType::B16, (pair.gid as ::core::ffi::c_int) as u32),
                    bk_ptr(BkCellType::Embed, bk_gpos_value(pair.fv, format1)),
                    bk_ptr(BkCellType::Embed, bk_gpos_value(pair.sv, format2)),
                ],
            );
        }
        bk_push(root, &[bk_ptr(BkCellType::P16, pair_set)]);
        j_1 = j_1.wrapping_add(1);
    }
    otl_coverage_free(cov);
    cov = ::core::ptr::null_mut::<Coverage>();
    return root;
}
pub unsafe fn otfcc_build_gpos_pair_classes(mut _subtable: *const Subtable) -> *mut BkBlock {
    let Subtable::GposPair(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GposPairSubtable = mut_subtable;
    let first_cd: *const ClassDef = (*subtable).first.as_deref().unwrap();
    let second_cd: *const ClassDef = (*subtable).second.as_deref().unwrap();
    let mut format1: u16 = 0 as u16;
    let mut format2: u16 = 0 as u16;
    let mut class1_count: GlyphClass =
        ((*first_cd).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphClass;
    let mut class2_count: GlyphClass =
        ((*second_cd).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphClass;
    let mut j: GlyphClass = 0 as GlyphClass;
    while (j as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int {
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int {
            format1 = (format1 as ::core::ffi::c_int
                | required_position_format((&(*subtable).first_values)[j as usize][k as usize])
                    as ::core::ffi::c_int) as u16;
            format2 = (format2 as ::core::ffi::c_int
                | required_position_format((&(*subtable).second_values)[j as usize][k as usize])
                    as ::core::ffi::c_int) as u16;
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    let mut cov: *mut Coverage = cov_from_cd(first_cd);
    let mut root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B16, 2 as u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(build_coverage(cov)),
        ),
        bk_int(BkCellType::B16, (format1 as ::core::ffi::c_int) as u32),
        bk_int(BkCellType::B16, (format2 as ::core::ffi::c_int) as u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(build_class_def(first_cd)),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(build_class_def(second_cd)),
        ),
        bk_int(BkCellType::B16, (class1_count as ::core::ffi::c_int) as u32),
        bk_int(BkCellType::B16, (class2_count as ::core::ffi::c_int) as u32),
    ]);
    let mut j_0: GlyphClass = 0 as GlyphClass;
    while (j_0 as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int {
        let mut k_0: GlyphClass = 0 as GlyphClass;
        while (k_0 as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int {
            bk_push(
                root,
                &[
                    bk_ptr(
                        BkCellType::Embed,
                        bk_gpos_value(
                            (&(*subtable).first_values)[j_0 as usize][k_0 as usize],
                            format1,
                        ),
                    ),
                    bk_ptr(
                        BkCellType::Embed,
                        bk_gpos_value(
                            (&(*subtable).second_values)[j_0 as usize][k_0 as usize],
                            format2,
                        ),
                    ),
                ],
            );
            k_0 = k_0.wrapping_add(1);
        }
        j_0 = j_0.wrapping_add(1);
    }
    otl_coverage_free(cov);
    cov = ::core::ptr::null_mut::<Coverage>();
    return root;
}
pub unsafe extern "C" fn otfcc_build_gpos_pair(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let mut format1: *mut BkBlock = otfcc_build_gpos_pair_individual(_subtable);
    let mut format2: *mut BkBlock = otfcc_build_gpos_pair_classes(_subtable);
    let mut g1: *mut BkGraph = bk_new_graph_from_root_block(format1);
    let mut g2: *mut BkGraph = bk_new_graph_from_root_block(format2);
    bk_minimize_graph(g1);
    bk_minimize_graph(g2);
    if bk_estimate_size_of_graph(g1) > bk_estimate_size_of_graph(g2) {
        bk_delete_graph(g1);
        bk_untangle_graph(g2);
        let mut buf: *mut Buffer = bk_build_graph(g2);
        bk_delete_graph(g2);
        return buf;
    } else {
        bk_delete_graph(g2);
        bk_untangle_graph(g1);
        let mut buf_0: *mut Buffer = bk_build_graph(g1);
        bk_delete_graph(g1);
        return buf_0;
    };
}

#[cfg(test)]
mod otl_read_gpos_pair_tests {
    use super::*;

    #[test]
    fn format1_reads_one_pair_and_synthesizes_the_second_class_def() {
        let mut data = [0u8; 24];
        data[0..2].copy_from_slice(&1u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&12u16.to_be_bytes()); // coverageOffset -> 12
        data[4..6].copy_from_slice(&1u16.to_be_bytes()); // valueFormat1: dx only
        data[6..8].copy_from_slice(&0u16.to_be_bytes()); // valueFormat2: none
        data[8..10].copy_from_slice(&1u16.to_be_bytes()); // pairSetCount
        data[10..12].copy_from_slice(&18u16.to_be_bytes()); // pairSetOffsets[0] -> 18
        // Coverage format 1 at byte 12: one glyph, id 10.
        data[12..14].copy_from_slice(&1u16.to_be_bytes());
        data[14..16].copy_from_slice(&1u16.to_be_bytes());
        data[16..18].copy_from_slice(&10u16.to_be_bytes());
        // PairSet at byte 18: one pair, second=20, value1.dx=50.
        data[18..20].copy_from_slice(&1u16.to_be_bytes());
        data[20..22].copy_from_slice(&20i16.to_be_bytes());
        data[22..24].copy_from_slice(&50i16.to_be_bytes());
        unsafe {
            let raw = otl_read_gpos_pair(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::GposPair(subtable) = &*boxed else {
                unreachable!()
            };
            let first = subtable.first.as_ref().unwrap();
            let second = subtable.second.as_ref().unwrap();
            assert_eq!(
                first.glyphs.iter().map(|h| h.index).collect::<Vec<_>>(),
                vec![10]
            );
            assert_eq!(
                second.glyphs.iter().map(|h| h.index).collect::<Vec<_>>(),
                vec![20]
            );
            assert_eq!(second.classes, vec![1]);
            assert_eq!(subtable.first_values[0][1].dx, 50.0);
        }
    }

    #[test]
    fn format1_table_too_short_for_the_header_is_rejected_instead_of_reading_oob() {
        // The original read `coverageOffset` (and every header field after
        // it) with no guard beyond the very first `table_length < offset +
        // 2` -- just the 2-byte format field itself -- so a table this
        // short claiming format 1 read straight past its own end.
        let data = [0u8, 1]; // format = 1, nothing else
        unsafe {
            let raw = otl_read_gpos_pair(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(raw.is_null());
        }
    }

    #[test]
    fn format2_reads_the_matrix_cell_for_each_class_pair() {
        let mut data = [0u8; 46];
        data[0..2].copy_from_slice(&2u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&24u16.to_be_bytes()); // coverageOffset -> 24
        data[4..6].copy_from_slice(&1u16.to_be_bytes()); // valueFormat1: dx only
        data[6..8].copy_from_slice(&0u16.to_be_bytes()); // valueFormat2: none
        data[8..10].copy_from_slice(&30u16.to_be_bytes()); // classDef1Offset -> 30
        data[10..12].copy_from_slice(&38u16.to_be_bytes()); // classDef2Offset -> 38
        data[12..14].copy_from_slice(&2u16.to_be_bytes()); // class1Count
        data[14..16].copy_from_slice(&2u16.to_be_bytes()); // class2Count
        // Matrix at byte 16: 2x2 cells, 2 bytes (dx) each; cell[1][1].dx=77.
        data[22..24].copy_from_slice(&77i16.to_be_bytes());
        // Coverage format 1 at byte 24: one glyph, id 10.
        data[24..26].copy_from_slice(&1u16.to_be_bytes());
        data[26..28].copy_from_slice(&1u16.to_be_bytes());
        data[28..30].copy_from_slice(&10u16.to_be_bytes());
        // ClassDef format 1 at byte 30: glyph 10 -> class 1.
        data[30..32].copy_from_slice(&1u16.to_be_bytes());
        data[32..34].copy_from_slice(&10u16.to_be_bytes());
        data[34..36].copy_from_slice(&1u16.to_be_bytes());
        data[36..38].copy_from_slice(&1u16.to_be_bytes());
        // ClassDef format 1 at byte 38: glyph 20 -> class 1.
        data[38..40].copy_from_slice(&1u16.to_be_bytes());
        data[40..42].copy_from_slice(&20u16.to_be_bytes());
        data[42..44].copy_from_slice(&1u16.to_be_bytes());
        data[44..46].copy_from_slice(&1u16.to_be_bytes());
        unsafe {
            let raw = otl_read_gpos_pair(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::GposPair(subtable) = &*boxed else {
                unreachable!()
            };
            assert_eq!(subtable.first_values[1][1].dx, 77.0);
            assert_eq!(subtable.first_values[0][0].dx, 0.0);
        }
    }

    #[test]
    fn format2_max_class_counts_are_rejected_not_read_oob() {
        // The original's final guard -- `class1_count * class2_count *
        // (len1+len2)` -- is computed as unchecked `i32` arithmetic;
        // `class1_count`/`class2_count` are independently unbounded u16
        // fields, so the product can reach ~68.7 billion, far past
        // `i32::MAX`. A single-range ClassDef format 2 entry cheaply
        // claims a `maxclass` of `u16::MAX - 1` without needing anywhere
        // near that many real entries, so this reaches the guard with a
        // tiny buffer.
        let mut data = [0u8; 42];
        data[0..2].copy_from_slice(&2u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&16u16.to_be_bytes()); // coverageOffset -> 16
        data[4..6].copy_from_slice(&1u16.to_be_bytes()); // valueFormat1
        data[6..8].copy_from_slice(&1u16.to_be_bytes()); // valueFormat2
        data[8..10].copy_from_slice(&22u16.to_be_bytes()); // classDef1Offset -> 22
        data[10..12].copy_from_slice(&32u16.to_be_bytes()); // classDef2Offset -> 32
        data[12..14].copy_from_slice(&u16::MAX.to_be_bytes()); // class1Count
        data[14..16].copy_from_slice(&u16::MAX.to_be_bytes()); // class2Count
        // Coverage format 1 at byte 16: one glyph, id 10.
        data[16..18].copy_from_slice(&1u16.to_be_bytes());
        data[18..20].copy_from_slice(&1u16.to_be_bytes());
        data[20..22].copy_from_slice(&10u16.to_be_bytes());
        // ClassDef format 2 at byte 22: one range, glyph 10, class u16::MAX-1.
        data[22..24].copy_from_slice(&2u16.to_be_bytes());
        data[24..26].copy_from_slice(&1u16.to_be_bytes());
        data[26..28].copy_from_slice(&10u16.to_be_bytes());
        data[28..30].copy_from_slice(&10u16.to_be_bytes());
        data[30..32].copy_from_slice(&(u16::MAX - 1).to_be_bytes());
        // ClassDef format 2 at byte 32: one range, glyph 20, class u16::MAX-1.
        data[32..34].copy_from_slice(&2u16.to_be_bytes());
        data[34..36].copy_from_slice(&1u16.to_be_bytes());
        data[36..38].copy_from_slice(&20u16.to_be_bytes());
        data[38..40].copy_from_slice(&20u16.to_be_bytes());
        data[40..42].copy_from_slice(&(u16::MAX - 1).to_be_bytes());
        unsafe {
            let raw = otl_read_gpos_pair(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(raw.is_null());
        }
    }
}
