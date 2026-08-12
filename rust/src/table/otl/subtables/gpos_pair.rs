#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, qsort};
use crate::support::json_funcs::{json_new_position, preserialize};
use crate::support::parsed_json::{ParsedValue, json_arr_at, json_arr_len, json_dbl_val, json_int_val, json_obj_get, json_obj_get_type, json_type_of};
use crate::table::otl::classdef::{expand_class_def, classdef_from_raw, ClassDef, otl_class_def_create, read_class_def};
use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage, shrink_coverage};
use crate::support::handle::{handle_from_index, otfcc_handle_dup, Handle, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphClass, GlyphId, Pos, TableId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::bk::bkgraph::{BkGraph};
use crate::table::otl::{GposPairSubtableElementInterface, PositionValue, Subtable, GposPairSubtable, subtable_from_raw};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_graph, bk_delete_graph, bk_estimate_size_of_graph, bk_minimize_graph, bk_new_graph_from_root_block, bk_untangle_graph};
use crate::table::otl::classdef::{OTL_I_CLASS_DEF};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::table::otl::subtables::gpos_common::{FORMAT_DWIDTH, bk_gpos_value, gpos_dump_value, gpos_parse_value, position_format_length, position_zero, read_gpos_value, required_position_format};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_object_new, json_object_push};

// `fv`/`sv` hold the matched cell's value directly now, not a pointer into
// `first_values`/`second_values` -- `PositionValue` is `Copy`, and with the
// grid a real `Vec<Vec<PositionValue>>` there is no backing array to point
// into that outlives this struct's own use (a single build pass).
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndividualGposPair {
    pub gid: GlyphId,
    pub fv: PositionValue,
    pub sv: PositionValue,
}
#[inline]
unsafe extern "C" fn init_gpos_pair(mut subtable: *mut GposPairSubtable) {
    // Placement-construct: `subtable` is fresh malloc'd (not calloc'd) by
    // `subtable_gpos_pair_create`, so there is nothing valid to drop first --
    // same reasoning as `otl_coverage_create`/`otl_class_def_create`.
    (&raw mut (*subtable).first).write(None);
    (&raw mut (*subtable).second).write(None);
    (&raw mut (*subtable).first_values).write(Vec::new());
    (&raw mut (*subtable).second_values).write(Vec::new());
}
#[inline]
pub(crate) unsafe extern "C" fn dispose_gpos_pair(mut subtable: *mut GposPairSubtable) {
    (*subtable).first = None;
    (*subtable).second = None;
    (*subtable).first_values = Vec::new();
    (*subtable).second_values = Vec::new();
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_dispose(mut x: *mut GposPairSubtable) {
    dispose_gpos_pair(x);
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_copy(
    mut dst: *mut GposPairSubtable,
    mut src: *const GposPairSubtable,
) {
    // Was a whole-struct `memcpy` -- unsound once any field owns a `Vec`/
    // `Box` (aliases two owners onto one allocation, a double-free once
    // either is dropped). This vtable slot is never invoked crate-wide
    // (`I_SUBTABLE_GPOS_PAIR.copy` has no callers, matching the
    // `otfcc-stage6-vtable-copy-move-mostly-dead` finding for every other
    // `XxxSubtableElementInterface.copy`), so a field-wise `Clone` is the
    // simplest correct body rather than a behavior-preserving concern.
    (*dst).first = (*src).first.clone();
    (*dst).second = (*src).second.clone();
    (*dst).first_values = (*src).first_values.clone();
    (*dst).second_values = (*src).second_values.clone();
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_create() -> *mut GposPairSubtable {
    let mut x: *mut GposPairSubtable =
        malloc(::core::mem::size_of::<GposPairSubtable>() as usize) as *mut GposPairSubtable;
    subtable_gpos_pair_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_init(mut x: *mut GposPairSubtable) {
    init_gpos_pair(x);
}
pub static I_SUBTABLE_GPOS_PAIR: GposPairSubtableElementInterface = {
    GposPairSubtableElementInterface {
        init: Some(subtable_gpos_pair_init as unsafe extern "C" fn(*mut GposPairSubtable) -> ()),
        copy: Some(
            subtable_gpos_pair_copy
                as unsafe extern "C" fn(*mut GposPairSubtable, *const GposPairSubtable) -> (),
        ),
        dispose: Some(
            subtable_gpos_pair_dispose as unsafe extern "C" fn(*mut GposPairSubtable) -> (),
        ),
        create: Some(subtable_gpos_pair_create),
        free: Some(subtable_gpos_pair_free as unsafe extern "C" fn(*mut GposPairSubtable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_pair_free(mut x: *mut GposPairSubtable) {
    if x.is_null() {
        return;
    }
    subtable_gpos_pair_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otl_read_gpos_pair(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    _max_glyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut subtable_format: u16 = 0;
    let mut current_block: u64;
    let mut subtable: *mut GposPairSubtable =
        (
            I_SUBTABLE_GPOS_PAIR
                .create
                .expect("non-null function pointer"))();
    if !(table_length < offset.wrapping_add(2 as u32)) {
        subtable_format = read_16u(data.offset(offset as isize) as *const u8);
        if subtable_format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            let mut cov: *mut Coverage = read_coverage(
                data as *const u8,
                table_length,
                offset.wrapping_add(read_16u(
                    data.offset(offset as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const u8,
                ) as u32),
            );
            // Built through a local raw pointer first (matches
            // `otl_class_def_create`'s own raw-pointer API), then adopted
            // into `(*subtable).first` as soon as it's fully constructed --
            // same timing as the original's immediate field assignment, so
            // every exit path below (including the length-check failures
            // that fall through to the bottom `I_SUBTABLE_GPOS_PAIR.free`)
            // still disposes it correctly.
            let first_raw: *mut ClassDef = otl_class_def_create();
            (*first_raw).glyphs = ::core::mem::take(&mut *cov);
            (*first_raw).maxclass = ((*first_raw).glyphs.len() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as GlyphClass;
            (*first_raw).classes = (0..(*first_raw).glyphs.len())
                .map(|j| j as GlyphClass)
                .collect();
            otl_coverage_free(cov);
            cov = ::core::ptr::null_mut::<Coverage>();
            (*subtable).first = classdef_from_raw(first_raw);
            let first_cd: *mut ClassDef = (*subtable).first.as_deref_mut().unwrap();
            let mut format1: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            );
            let mut format2: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize) as *const u8,
            );
            let mut len1: u8 = position_format_length(format1);
            let mut len2: u8 = position_format_length(format2);
            let mut pair_set_count: GlyphId = read_16u(
                data.offset(offset as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *const u8,
            ) as GlyphId;
            if !(pair_set_count as usize != (*first_cd).glyphs.len())
            {
                if !(table_length
                    < offset.wrapping_add(10 as u32).wrapping_add(
                        (2 as ::core::ffi::c_int * pair_set_count as ::core::ffi::c_int) as u32,
                    ))
                {
                    let mut j_0: GlyphId = 0 as GlyphId;
                    loop {
                        if !((j_0 as ::core::ffi::c_int) < pair_set_count as ::core::ffi::c_int) {
                            current_block = 14401909646449704462;
                            break;
                        }
                        let mut pair_set_offset: u32 = offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(10 as ::core::ffi::c_int as isize)
                                .offset(
                                    (2 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        )
                            as u32);
                        if table_length < pair_set_offset.wrapping_add(2 as u32) {
                            current_block = 1524425613423851471;
                            break;
                        }
                        let mut pair_count: GlyphId =
                            read_16u(data.offset(pair_set_offset as isize) as *const u8)
                                as GlyphId;
                        if table_length
                            < pair_set_offset.wrapping_add(2 as u32).wrapping_add(
                                ((2 as ::core::ffi::c_int
                                    + len1 as ::core::ffi::c_int
                                    + len2 as ::core::ffi::c_int)
                                    * pair_count as ::core::ffi::c_int)
                                    as u32,
                            )
                        {
                            current_block = 1524425613423851471;
                            break;
                        }
                        j_0 = j_0.wrapping_add(1);
                    }
                    match current_block {
                        1524425613423851471 => {}
                        _ => {
                            // Deduplicates the "second" glyph of every pair by gid,
                            // assigning each distinct one the next sequential class id
                            // (1-based -- class 0 is reserved for "not covered" per the
                            // OpenType format) -- synthesizing a class def for `second`
                            // that Format 1 (individual pairs) doesn't carry on the
                            // wire, the same way Format 2 already does explicitly. No
                            // `HASH_SORT` is used before the original's `HASH_ITER`
                            // here, so output order is insertion order; since cid is
                            // assigned as `num_items + 1` at insert time, insertion
                            // order and cid-ascending order are the same order by
                            // construction, matching the `IndexSet`-not-a-map shape of
                            // `LigatureAggregator` (see rust/README.md) but with cid
                            // derived from position instead of tracked separately.
                            // This one set is used across three phases within this
                            // branch: built while first reading every pair (below),
                            // then looked up (not rebuilt) while re-reading the same
                            // pairs a second time to place position values into the
                            // `first_values`/`second_values` grid, then walked once
                            // more at the end to populate `(*subtable).second`'s
                            // `glyphs`/`classes` before this branch returns.
                            let mut h: indexmap::IndexSet<::core::ffi::c_int> = indexmap::IndexSet::new();
                            let mut j_1: GlyphId = 0 as GlyphId;
                            while (j_1 as ::core::ffi::c_int) < pair_set_count as ::core::ffi::c_int {
                                let mut pair_set_offset_0: u32 = offset.wrapping_add(read_16u(
                                    data.offset(offset as isize)
                                        .offset(10 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int * j_1 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let mut pair_count_0: GlyphId = read_16u(
                                    data.offset(pair_set_offset_0 as isize) as *const u8,
                                )
                                    as GlyphId;
                                let mut k: GlyphId = 0 as GlyphId;
                                while (k as ::core::ffi::c_int) < pair_count_0 as ::core::ffi::c_int
                                {
                                    let mut second: ::core::ffi::c_int = read_16u(
                                        data.offset(pair_set_offset_0 as isize)
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            .offset(
                                                ((2 as ::core::ffi::c_int
                                                    + len1 as ::core::ffi::c_int
                                                    + len2 as ::core::ffi::c_int)
                                                    * k as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            as *const u8,
                                    )
                                        as ::core::ffi::c_int;
                                    h.insert(second);
                                    k = k.wrapping_add(1);
                                }
                                j_1 = j_1.wrapping_add(1);
                            }
                            let second_raw: *mut ClassDef = otl_class_def_create();
                            let n_second: usize = h.len();
                            (*second_raw).maxclass = n_second as GlyphClass;
                            (*second_raw).classes = vec![0 as GlyphClass; n_second];
                            (*second_raw).glyphs = vec![GlyphHandle::default(); n_second];
                            (*subtable).second = classdef_from_raw(second_raw);
                            let second_cd: *mut ClassDef = (*subtable).second.as_deref_mut().unwrap();
                            let mut class2_count: GlyphClass = ((*second_cd).maxclass
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as GlyphClass;
                            // Was a manual `__caryll_allocate_clean` +
                            // nested-loop-of-`position_zero()` writes over
                            // `*mut *mut PositionValue` -- `PositionValue` is
                            // `Copy`, so pre-sizing the whole grid collapses
                            // to one `vec![vec![..]; ..]` expression; the
                            // real values below are then index-assigned
                            // directly, the same shape B-3-1/B-3-4 used for
                            // out-of-order-indexed `Copy` element fills.
                            let mut first_values: Vec<Vec<PositionValue>> = vec![
                                vec![position_zero(); class2_count as usize];
                                ((*first_cd).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize
                            ];
                            let mut second_values: Vec<Vec<PositionValue>> = vec![
                                vec![position_zero(); class2_count as usize];
                                ((*first_cd).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize
                            ];
                            let mut j_3: GlyphClass = 0 as GlyphClass;
                            while j_3 as ::core::ffi::c_int
                                <= (*first_cd).maxclass as ::core::ffi::c_int
                            {
                                let mut pair_set_offset_1: u32 = offset.wrapping_add(read_16u(
                                    data.offset(offset as isize)
                                        .offset(10 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int * j_3 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let mut pair_count_1: GlyphId = read_16u(
                                    data.offset(pair_set_offset_1 as isize) as *const u8,
                                )
                                    as GlyphId;
                                let mut k_1: GlyphId = 0 as GlyphId;
                                while (k_1 as ::core::ffi::c_int)
                                    < pair_count_1 as ::core::ffi::c_int
                                {
                                    let mut second_0: ::core::ffi::c_int = read_16u(
                                        data.offset(pair_set_offset_1 as isize)
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            .offset(
                                                ((2 as ::core::ffi::c_int
                                                    + len1 as ::core::ffi::c_int
                                                    + len2 as ::core::ffi::c_int)
                                                    * k_1 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            as *const u8,
                                    )
                                        as ::core::ffi::c_int;
                                    if let Some(idx) = h.get_index_of(&second_0) {
                                        let cid: ::core::ffi::c_int =
                                            idx as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                                        first_values[j_3 as usize][cid as usize] = read_gpos_value(
                                            data,
                                            table_length,
                                            pair_set_offset_1
                                                .wrapping_add(2 as u32)
                                                .wrapping_add(
                                                    ((2 as ::core::ffi::c_int
                                                        + len1 as ::core::ffi::c_int
                                                        + len2 as ::core::ffi::c_int)
                                                        * k_1 as ::core::ffi::c_int)
                                                        as u32,
                                                )
                                                .wrapping_add(2 as u32),
                                            format1,
                                        );
                                        second_values[j_3 as usize][cid as usize] = read_gpos_value(
                                            data,
                                            table_length,
                                            pair_set_offset_1
                                                .wrapping_add(2 as u32)
                                                .wrapping_add(
                                                    ((2 as ::core::ffi::c_int
                                                        + len1 as ::core::ffi::c_int
                                                        + len2 as ::core::ffi::c_int)
                                                        * k_1 as ::core::ffi::c_int)
                                                        as u32,
                                                )
                                                .wrapping_add(2 as u32)
                                                .wrapping_add(len1 as u32),
                                            format2,
                                        );
                                    }
                                    k_1 = k_1.wrapping_add(1);
                                }
                                j_3 = j_3.wrapping_add(1);
                            }
                            (*subtable).first_values = first_values;
                            (*subtable).second_values = second_values;
                            for (jj, &gid) in h.iter().enumerate() {
                                (&mut (*second_cd).glyphs)[jj as usize] =
                                    handle_from_index(gid as GlyphId) as GlyphHandle;
                                (&mut (*second_cd).classes)[jj as usize] =
                                    (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphClass;
                            }
                            return subtable_from_raw(subtable, Subtable::GposPair);
                        }
                    }
                }
            }
        } else if subtable_format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            if !(table_length < offset.wrapping_add(16 as u32)) {
                let mut format1_0: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *const u8,
                );
                let mut format2_0: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(6 as ::core::ffi::c_int as isize)
                        as *const u8,
                );
                let mut len1_0: u8 = position_format_length(format1_0);
                let mut len2_0: u8 = position_format_length(format2_0);
                let mut cov_0: *mut Coverage =
                    read_coverage(
                        data as *const u8,
                        table_length,
                        offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const u8,
                        ) as u32),
                    );
                // `expand_class_def` consumes (and internally frees) the
                // `ocd` it's handed and returns a brand-new `*mut ClassDef`
                // -- kept as a plain local raw pointer through that
                // consuming call, then adopted into `(*subtable).first`
                // only once settled, so no Rust-`Box`-into-a-`free()`-only
                // API mismatch is ever created.
                let mut first_raw: *mut ClassDef = read_class_def(
                    data as *const u8,
                    table_length,
                    offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ) as u32),
                );
                first_raw = expand_class_def(
                    cov_0,
                    first_raw,
                );
                otl_coverage_free(cov_0);
                cov_0 = ::core::ptr::null_mut::<Coverage>();
                (*subtable).first = classdef_from_raw(first_raw);
                (*subtable).second = classdef_from_raw(read_class_def(
                    data as *const u8,
                    table_length,
                    offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(10 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ) as u32),
                ));
                if !((*subtable).first.is_none() || (*subtable).second.is_none()) {
                    let first_cd: *mut ClassDef = (*subtable).first.as_deref_mut().unwrap();
                    let second_cd: *mut ClassDef = (*subtable).second.as_deref_mut().unwrap();
                    let mut class1_count: GlyphClass = read_16u(
                        data.offset(offset as isize)
                            .offset(12 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ) as GlyphClass;
                    let mut class2_count_0: GlyphClass = read_16u(
                        data.offset(offset as isize)
                            .offset(14 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ) as GlyphClass;
                    if !((*first_cd).maxclass as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        != class1_count as ::core::ffi::c_int)
                    {
                        if !((*second_cd).maxclass as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            != class2_count_0 as ::core::ffi::c_int)
                        {
                            if !(table_length
                                < offset.wrapping_add(16 as u32).wrapping_add(
                                    (class1_count as ::core::ffi::c_int
                                        * class2_count_0 as ::core::ffi::c_int
                                        * (len1_0 as ::core::ffi::c_int
                                            + len2_0 as ::core::ffi::c_int))
                                        as u32,
                                ))
                            {
                                // Format 2 fills every cell exhaustively and
                                // in order, so (unlike Format 1's `cid`
                                // -indexed overwrite pass) no pre-sized
                                // placeholder grid is needed -- each row is
                                // just pushed as it's read.
                                let mut first_values: Vec<Vec<PositionValue>> =
                                    Vec::with_capacity(class1_count as usize);
                                let mut second_values: Vec<Vec<PositionValue>> =
                                    Vec::with_capacity(class1_count as usize);
                                let mut j_4: GlyphClass = 0 as GlyphClass;
                                while (j_4 as ::core::ffi::c_int)
                                    < class1_count as ::core::ffi::c_int
                                {
                                    let mut row1: Vec<PositionValue> =
                                        Vec::with_capacity(class2_count_0 as usize);
                                    let mut row2: Vec<PositionValue> =
                                        Vec::with_capacity(class2_count_0 as usize);
                                    let mut k_2: GlyphClass = 0 as GlyphClass;
                                    while (k_2 as ::core::ffi::c_int)
                                        < class2_count_0 as ::core::ffi::c_int
                                    {
                                        row1.push(read_gpos_value(
                                            data,
                                            table_length,
                                            offset.wrapping_add(16 as u32).wrapping_add(
                                                ((j_4 as ::core::ffi::c_int
                                                    * class2_count_0 as ::core::ffi::c_int
                                                    + k_2 as ::core::ffi::c_int)
                                                    * (len1_0 as ::core::ffi::c_int
                                                        + len2_0 as ::core::ffi::c_int))
                                                    as u32,
                                            ),
                                            format1_0,
                                        ));
                                        row2.push(read_gpos_value(
                                            data,
                                            table_length,
                                            offset
                                                .wrapping_add(16 as u32)
                                                .wrapping_add(
                                                    ((j_4 as ::core::ffi::c_int
                                                        * class2_count_0 as ::core::ffi::c_int
                                                        + k_2 as ::core::ffi::c_int)
                                                        * (len1_0 as ::core::ffi::c_int
                                                            + len2_0 as ::core::ffi::c_int))
                                                        as u32,
                                                )
                                                .wrapping_add(len1_0 as u32),
                                            format2_0,
                                        ));
                                        k_2 = k_2.wrapping_add(1);
                                    }
                                    first_values.push(row1);
                                    second_values.push(row2);
                                    j_4 = j_4.wrapping_add(1);
                                }
                                (*subtable).first_values = first_values;
                                (*subtable).second_values = second_values;
                                return subtable_from_raw(subtable, Subtable::GposPair);
                            }
                        }
                    }
                }
            }
        }
    }
    I_SUBTABLE_GPOS_PAIR.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gpos_dump_pair(mut _subtable: *const Subtable) -> *mut JsonValue {
    let Subtable::GposPair(mut_subtable) = &*_subtable else { unreachable!() };
    let subtable: *const GposPairSubtable = mut_subtable;
    let first_cd: *const ClassDef = (*subtable).first.as_deref().unwrap();
    let second_cd: *const ClassDef = (*subtable).second.as_deref().unwrap();
    let mut st: *mut JsonValue = json_object_new(3 as usize);
    json_object_push(
        st,
        b"first\0" as *const u8 as *const ::core::ffi::c_char,
        OTL_I_CLASS_DEF.dump.expect("non-null function pointer")(first_cd),
    );
    json_object_push(
        st,
        b"second\0" as *const u8 as *const ::core::ffi::c_char,
        OTL_I_CLASS_DEF.dump.expect("non-null function pointer")(second_cd),
    );
    let mut mat: *mut JsonValue = json_array_new(
        ((*first_cd).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
    );
    let mut j: GlyphClass = 0 as GlyphClass;
    while j as ::core::ffi::c_int <= (*first_cd).maxclass as ::core::ffi::c_int {
        let mut row: *mut JsonValue = json_array_new(
            ((*second_cd).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as usize,
        );
        let mut k: GlyphClass = 0 as GlyphClass;
        while k as ::core::ffi::c_int <= (*second_cd).maxclass as ::core::ffi::c_int {
            let mut f1: u8 = required_position_format(
                (&(*subtable).first_values)[j as usize][k as usize],
            );
            let mut f2: u8 = required_position_format(
                (&(*subtable).second_values)[j as usize][k as usize],
            );
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
                    let mut pair: *mut JsonValue = json_object_new(2 as usize);
                    if f1 != 0 {
                        json_object_push(
                            pair,
                            b"first\0" as *const u8 as *const ::core::ffi::c_char,
                            gpos_dump_value(
                                (&(*subtable).first_values)[j as usize][k as usize],
                            ),
                        );
                    }
                    if f2 != 0 {
                        json_object_push(
                            pair,
                            b"second\0" as *const u8 as *const ::core::ffi::c_char,
                            gpos_dump_value(
                                (&(*subtable).second_values)[j as usize][k as usize],
                            ),
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
pub unsafe extern "C" fn otl_gpos_parse_pair(
    mut _subtable: *const ParsedValue,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut class1_count: GlyphClass = 0;
    let mut class2_count: GlyphClass = 0;
    let mut subtable: *mut GposPairSubtable =
        (
            I_SUBTABLE_GPOS_PAIR
                .create
                .expect("non-null function pointer"))();
    let mut _mat: *const ParsedValue = json_obj_get_type(
        _subtable,
        b"matrix\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    (*subtable).first = classdef_from_raw(OTL_I_CLASS_DEF.parse.expect("non-null function pointer")(json_obj_get_type(
        _subtable,
        b"first\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    )));
    (*subtable).second =
        classdef_from_raw(OTL_I_CLASS_DEF.parse.expect("non-null function pointer")(json_obj_get_type(
            _subtable,
            b"second\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Object,
        )));
    if _mat.is_null() || (*subtable).first.is_none() || (*subtable).second.is_none() {
        I_SUBTABLE_GPOS_PAIR.free.expect("non-null function pointer")(subtable);
        return ::core::ptr::null_mut::<Subtable>();
    } else {
        let first_cd: *mut ClassDef = (*subtable).first.as_deref_mut().unwrap();
        let second_cd: *mut ClassDef = (*subtable).second.as_deref_mut().unwrap();
        class1_count = ((*first_cd).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as GlyphClass;
        class2_count = ((*second_cd).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as GlyphClass;
        let mut first_values: Vec<Vec<PositionValue>> =
            vec![vec![position_zero(); class2_count as usize]; class1_count as usize];
        let mut second_values: Vec<Vec<PositionValue>> =
            vec![vec![position_zero(); class2_count as usize]; class1_count as usize];
        let mut j_0: GlyphClass = 0 as GlyphClass;
        while (j_0 as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int
            && (j_0 as ::core::ffi::c_uint) < json_arr_len(_mat)
        {
            let mut _row: *const ParsedValue = json_arr_at(_mat, j_0 as u32);
            if !(_row.is_null()
                || json_type_of(_row) != JsonType::Array)
            {
                let mut k_0: GlyphClass = 0 as GlyphClass;
                while (k_0 as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int
                    && (k_0 as ::core::ffi::c_uint) < json_arr_len(_row)
                {
                    let mut _item: *const ParsedValue = json_arr_at(_row, k_0 as u32);
                    if json_type_of(_item) == JsonType::Integer
                    {
                        first_values[j_0 as usize][k_0 as usize].d_width =
                            json_int_val(_item) as Pos;
                    } else if json_type_of(_item) == JsonType::Double
                    {
                        first_values[j_0 as usize][k_0 as usize].d_width =
                            json_dbl_val(_item) as Pos;
                    } else if json_type_of(_item) == JsonType::Object
                    {
                        first_values[j_0 as usize][k_0 as usize] =
                            gpos_parse_value(json_obj_get(
                                _item,
                                b"first\0" as *const u8 as *const ::core::ffi::c_char,
                            ));
                        second_values[j_0 as usize][k_0 as usize] =
                            gpos_parse_value(json_obj_get(
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
unsafe extern "C" fn cov_from_cd(mut cd: *const ClassDef) -> *mut Coverage {
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
unsafe extern "C" fn by_pair_second_glyph(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut IndividualGposPair)).gid as ::core::ffi::c_int
        - (*(b as *mut IndividualGposPair)).gid as ::core::ffi::c_int;
}
pub unsafe extern "C" fn otfcc_build_gpos_pair_individual(
    mut _subtable: *const Subtable,
) -> *mut BkBlock {
    let Subtable::GposPair(mut_subtable) = &*_subtable else { unreachable!() };
    let subtable: *const GposPairSubtable = mut_subtable;
    let first_cd: *const ClassDef = (*subtable).first.as_deref().unwrap();
    let second_cd: *const ClassDef = (*subtable).second.as_deref().unwrap();
    let mut format1: u16 = 0 as u16;
    let mut format2: u16 = 0 as u16;
    let mut class1_count: GlyphClass = ((*first_cd).maxclass as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as GlyphClass;
    let mut class2_count: GlyphClass = ((*second_cd).maxclass as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as GlyphClass;
    let mut j: GlyphClass = 0 as GlyphClass;
    while (j as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int {
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int {
            format1 = (format1 as ::core::ffi::c_int
                | required_position_format(
                    (&(*subtable).first_values)[j as usize][k as usize],
                ) as ::core::ffi::c_int) as u16;
            format2 = (format2 as ::core::ffi::c_int
                | required_position_format(
                    (&(*subtable).second_values)[j as usize][k as usize],
                ) as ::core::ffi::c_int) as u16;
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    let mut pair_counts: *mut GlyphId = ::core::ptr::null_mut::<GlyphId>();
    pair_counts = __caryll_allocate_clean(
        (::core::mem::size_of::<GlyphId>() as usize)
            .wrapping_mul((*first_cd).glyphs.len()),
        290 as ::core::ffi::c_ulong,
    ) as *mut GlyphId;
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*first_cd).glyphs.len() {
        *pair_counts.offset(j_0 as isize) = 0 as GlyphId;
        let mut k_0: GlyphId = 0 as GlyphId;
        while (k_0 as usize) < (*second_cd).glyphs.len() {
            let mut c1: GlyphClass = (&(*first_cd).classes)[j_0 as usize];
            let mut c2: GlyphClass = (&(*second_cd).classes)[k_0 as usize];
            if required_position_format(
                (&(*subtable).first_values)[c1 as usize][c2 as usize],
            ) as ::core::ffi::c_int
                | required_position_format(
                    (&(*subtable).second_values)[c1 as usize][c2 as usize],
                ) as ::core::ffi::c_int
                != 0
            {
                let ref mut fresh10 = *pair_counts.offset(j_0 as isize);
                *fresh10 = (*fresh10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
            }
            k_0 = k_0.wrapping_add(1);
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut cov: *mut Coverage = cov_from_cd(first_cd);
    shrink_coverage(cov, true);
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(cov))), bk_int(BkCellType::B16, (format1 as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (format2 as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*first_cd).glyphs.len() as ::core::ffi::c_int) as u32)]);
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
                current_pair_count = *pair_counts.offset(k_1 as isize) as TableId;
            }
            k_1 = k_1.wrapping_add(1);
        }
        let mut pair_set: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (current_pair_count as ::core::ffi::c_int) as u32)]);
        let mut pairs: *mut IndividualGposPair = ::core::ptr::null_mut::<IndividualGposPair>();
        pairs = __caryll_allocate_clean(
            (::core::mem::size_of::<IndividualGposPair>() as usize)
                .wrapping_mul(current_pair_count as usize),
            324 as ::core::ffi::c_ulong,
        ) as *mut IndividualGposPair;
        let mut n: usize = 0 as usize;
        let mut k_2: GlyphId = 0 as GlyphId;
        while (k_2 as usize) < (*second_cd).glyphs.len() {
            let mut c2_0: GlyphClass = (&(*second_cd).classes)[k_2 as usize];
            if required_position_format(
                (&(*subtable).first_values)[c1_0 as usize][c2_0 as usize],
            ) as ::core::ffi::c_int
                | required_position_format(
                    (&(*subtable).second_values)[c1_0 as usize][c2_0 as usize],
                ) as ::core::ffi::c_int
                != 0
            {
                (*pairs.offset(n as isize)).gid =
                    (&(*second_cd).glyphs)[k_2 as usize].index;
                (*pairs.offset(n as isize)).fv =
                    (&(*subtable).first_values)[c1_0 as usize][c2_0 as usize];
                (*pairs.offset(n as isize)).sv =
                    (&(*subtable).second_values)[c1_0 as usize][c2_0 as usize];
                n = n.wrapping_add(1);
            }
            k_2 = k_2.wrapping_add(1);
        }
        qsort(
            pairs as *mut ::core::ffi::c_void,
            current_pair_count as usize,
            ::core::mem::size_of::<IndividualGposPair>() as usize,
            Some(
                by_pair_second_glyph
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        let mut n_0: usize = 0 as usize;
        while n_0 < current_pair_count as usize {
            bk_push(pair_set, &[bk_int(BkCellType::B16, ((*pairs.offset(n_0 as isize)).gid as ::core::ffi::c_int) as u32), bk_ptr(BkCellType::Embed, bk_gpos_value((*pairs.offset(n_0 as isize)).fv, format1)), bk_ptr(BkCellType::Embed, bk_gpos_value((*pairs.offset(n_0 as isize)).sv, format2))]);
            n_0 = n_0.wrapping_add(1);
        }
        free(pairs as *mut ::core::ffi::c_void);
        pairs = ::core::ptr::null_mut::<IndividualGposPair>();
        bk_push(root, &[bk_ptr(BkCellType::P16, pair_set)]);
        j_1 = j_1.wrapping_add(1);
    }
    otl_coverage_free(cov);
    cov = ::core::ptr::null_mut::<Coverage>();
    free(pair_counts as *mut ::core::ffi::c_void);
    pair_counts = ::core::ptr::null_mut::<GlyphId>();
    return root;
}
pub unsafe extern "C" fn otfcc_build_gpos_pair_classes(
    mut _subtable: *const Subtable,
) -> *mut BkBlock {
    let Subtable::GposPair(mut_subtable) = &*_subtable else { unreachable!() };
    let subtable: *const GposPairSubtable = mut_subtable;
    let first_cd: *const ClassDef = (*subtable).first.as_deref().unwrap();
    let second_cd: *const ClassDef = (*subtable).second.as_deref().unwrap();
    let mut format1: u16 = 0 as u16;
    let mut format2: u16 = 0 as u16;
    let mut class1_count: GlyphClass = ((*first_cd).maxclass as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as GlyphClass;
    let mut class2_count: GlyphClass = ((*second_cd).maxclass as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as GlyphClass;
    let mut j: GlyphClass = 0 as GlyphClass;
    while (j as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int {
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int {
            format1 = (format1 as ::core::ffi::c_int
                | required_position_format(
                    (&(*subtable).first_values)[j as usize][k as usize],
                ) as ::core::ffi::c_int) as u16;
            format2 = (format2 as ::core::ffi::c_int
                | required_position_format(
                    (&(*subtable).second_values)[j as usize][k as usize],
                ) as ::core::ffi::c_int) as u16;
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    let mut cov: *mut Coverage = cov_from_cd(first_cd);
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 2 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(cov))), bk_int(BkCellType::B16, (format1 as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (format2 as ::core::ffi::c_int) as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_CLASS_DEF.build.expect("non-null function pointer")(
            first_cd,
        ))), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_CLASS_DEF.build.expect("non-null function pointer")(
            second_cd,
        ))), bk_int(BkCellType::B16, (class1_count as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (class2_count as ::core::ffi::c_int) as u32)]);
    let mut j_0: GlyphClass = 0 as GlyphClass;
    while (j_0 as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int {
        let mut k_0: GlyphClass = 0 as GlyphClass;
        while (k_0 as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int {
            bk_push(root, &[bk_ptr(BkCellType::Embed, bk_gpos_value(
                    (&(*subtable).first_values)[j_0 as usize][k_0 as usize],
                    format1,
                )), bk_ptr(BkCellType::Embed, bk_gpos_value(
                    (&(*subtable).second_values)[j_0 as usize][k_0 as usize],
                    format2,
                ))]);
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
