#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy};


use crate::support::parsed_json::{ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_getnum_fallback};
use crate::table::otl::coverage::{Coverage, coverage_from_raw, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_index, GlyphHandle};

use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};
use crate::vendor::json::{JsonType};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::table::otl::{GsubReverseSubtableElementInterface, Subtable, GsubReverseSubtable, subtable_from_raw};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::support::built_json::{BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push};

#[inline]
unsafe extern "C" fn init_gsub_reverse(mut subtable: *mut GsubReverseSubtable) {
    // `.write()`, not a field assignment: `subtable` is fresh from
    // `malloc` (uninitialized, not zeroed), so there is nothing valid to
    // drop first -- an `=` here would attempt to drop whatever garbage
    // bytes were already there.
    (&raw mut (*subtable).match_0).write(Vec::new());
    (&raw mut (*subtable).to).write(Vec::new());
}
#[inline]
pub(crate) unsafe extern "C" fn dispose_gsub_reverse(mut subtable: *mut GsubReverseSubtable) {
    // Both fields are real `Vec`s by the time this runs (never called on
    // the freshly-`malloc`'d, not-yet-`init`'d state) -- assigning a fresh
    // empty one drops the old contents correctly, no manual per-element
    // walk needed anymore.
    (*subtable).match_0 = Vec::new();
    (*subtable).to = Vec::new();
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_dispose(mut x: *mut GsubReverseSubtable) {
    dispose_gsub_reverse(x);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_free(mut x: *mut GsubReverseSubtable) {
    if x.is_null() {
        return;
    }
    subtable_gsub_reverse_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub static I_SUBTABLE_GSUB_REVERSE: GsubReverseSubtableElementInterface = {
    GsubReverseSubtableElementInterface {
        init: Some(
            subtable_gsub_reverse_init as unsafe extern "C" fn(*mut GsubReverseSubtable) -> (),
        ),
        copy: Some(
            subtable_gsub_reverse_copy
                as unsafe extern "C" fn(
                    *mut GsubReverseSubtable,
                    *const GsubReverseSubtable,
                ) -> (),
        ),
        dispose: Some(
            subtable_gsub_reverse_dispose as unsafe extern "C" fn(*mut GsubReverseSubtable) -> (),
        ),
        create: Some(subtable_gsub_reverse_create),
        free: Some(
            subtable_gsub_reverse_free as unsafe extern "C" fn(*mut GsubReverseSubtable) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_create() -> *mut GsubReverseSubtable {
    let mut x: *mut GsubReverseSubtable =
        malloc(::core::mem::size_of::<GsubReverseSubtable>() as usize)
            as *mut GsubReverseSubtable;
    subtable_gsub_reverse_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_init(mut x: *mut GsubReverseSubtable) {
    init_gsub_reverse(x);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_copy(
    mut dst: *mut GsubReverseSubtable,
    mut src: *const GsubReverseSubtable,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GsubReverseSubtable>() as usize,
    );
}
// Was a manual index-swapping loop over `start..end`, meeting in the
// middle -- exactly what `[T]::reverse` does, now that `match_0` is a real
// `Vec<Coverage>` slice instead of an array of raw pointers to swap by
// value. `input_index == 0` (nothing to reverse) falls out of slicing an
// empty range, no separate guard needed.
unsafe fn reverse_backtracks(match_0: &mut [Coverage], input_index: TableId) {
    match_0[..input_index as usize].reverse();
}
pub unsafe extern "C" fn otl_read_gsub_reverse(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    _max_glyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut n_backtrack: TableId = 0;
    let mut n_forward: TableId = 0;
    let mut n_replacement: TableId = 0;
    let mut subtable: *mut GsubReverseSubtable =
        (
            I_SUBTABLE_GSUB_REVERSE
                .create
                .expect("non-null function pointer"))();
    if !(table_length < offset.wrapping_add(6 as u32)) {
        n_backtrack = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        if !(table_length
            < offset.wrapping_add(6 as u32).wrapping_add(
                (n_backtrack as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
            ))
        {
            n_forward = read_16u(
                data.offset(offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize)
                    .offset((n_backtrack as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                    as *const u8,
            ) as TableId;
            if !(table_length
                < offset.wrapping_add(8 as u32).wrapping_add(
                    ((n_backtrack as ::core::ffi::c_int + n_forward as ::core::ffi::c_int)
                        * 2 as ::core::ffi::c_int) as u32,
                ))
            {
                n_replacement = read_16u(
                    data.offset(offset as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset(
                            ((n_backtrack as ::core::ffi::c_int + n_forward as ::core::ffi::c_int)
                                * 2 as ::core::ffi::c_int) as isize,
                        ) as *const u8,
                ) as TableId;
                if !(table_length
                    < offset.wrapping_add(10 as u32).wrapping_add(
                        ((n_backtrack as ::core::ffi::c_int
                            + n_forward as ::core::ffi::c_int
                            + n_replacement as ::core::ffi::c_int)
                            * 2 as ::core::ffi::c_int) as u32,
                    ))
                {
                    (*subtable).match_count = (n_backtrack as ::core::ffi::c_int
                        + n_forward as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as TableId;
                    // Filled out of sequential order below (backtrack slots,
                    // then the input slot at `input_index`, then forward
                    // slots) -- every one of the `match_count` slots is
                    // written exactly once by the time this subtable is
                    // returned, so pre-sizing with placeholder empty
                    // `Coverage`s and index-assigning is the direct
                    // replacement for the old `offset`-indexed writes into
                    // `__caryll_allocate_clean`'d memory.
                    (*subtable).match_0 =
                        vec![Coverage::new(); (*subtable).match_count as usize];
                    (*subtable).input_index = n_backtrack;
                    let mut j: TableId = 0 as TableId;
                    while (j as ::core::ffi::c_int) < n_backtrack as ::core::ffi::c_int {
                        let mut cov_offset: u32 = offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        )
                            as u32);
                        (&mut (*subtable).match_0)[j as usize] = coverage_from_raw(read_coverage(
                            data as *const u8,
                            table_length,
                            cov_offset,
                        ));
                        j = j.wrapping_add(1);
                    }
                    let mut cov_offset_0: u32 = offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *const u8,
                    )
                        as u32);
                    (&mut (*subtable).match_0)[(*subtable).input_index as usize] = coverage_from_raw(read_coverage(
                        data as *const u8,
                        table_length,
                        cov_offset_0,
                    ));
                    if !(n_replacement as usize
                        != (&(*subtable).match_0)[(*subtable).input_index as usize].len())
                    {
                        let mut j_0: TableId = 0 as TableId;
                        while (j_0 as ::core::ffi::c_int) < n_forward as ::core::ffi::c_int {
                            let mut cov_offset_1: u32 = offset.wrapping_add(read_16u(
                                data.offset(offset as isize)
                                    .offset(8 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (n_backtrack as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    )
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as u32);
                            let fwd_idx: usize = (n_backtrack as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                + j_0 as ::core::ffi::c_int)
                                as usize;
                            (&mut (*subtable).match_0)[fwd_idx] = coverage_from_raw(read_coverage(
                                data as *const u8,
                                table_length,
                                cov_offset_1,
                            ));
                            j_0 = j_0.wrapping_add(1);
                        }
                        (*subtable).to = Coverage::new();
                        let mut j_1: TableId = 0 as TableId;
                        while (j_1 as ::core::ffi::c_int) < n_replacement as ::core::ffi::c_int {
                            push_to_coverage(
                                &mut (*subtable).to as *mut Coverage,
                                handle_from_index(
                                    read_16u(
                                        data.offset(offset as isize)
                                            .offset(10 as ::core::ffi::c_int as isize)
                                            .offset(
                                                ((n_backtrack as ::core::ffi::c_int
                                                    + n_forward as ::core::ffi::c_int
                                                    + j_1 as ::core::ffi::c_int)
                                                    * 2 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            as *const u8,
                                    ) as GlyphId,
                                ) as GlyphHandle,
                            );
                            j_1 = j_1.wrapping_add(1);
                        }
                        reverse_backtracks(&mut (*subtable).match_0, (*subtable).input_index);
                        return subtable_from_raw(subtable, Subtable::GsubReverse);
                    }
                }
            }
        }
    }
    I_SUBTABLE_GSUB_REVERSE
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gsub_dump_reverse(
    mut _subtable: *const Subtable,
) -> *mut BuiltValue {
    let Subtable::GsubReverse(mut_subtable) = &*_subtable else { unreachable!() };
    let subtable: *const GsubReverseSubtable = mut_subtable;
    let mut _st: *mut BuiltValue = json_object_new(3 as usize);
    let mut _match: *mut BuiltValue = json_array_new((*subtable).match_count as usize);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*subtable).match_count as ::core::ffi::c_int {
        json_array_push(
            _match,
            OTL_I_COVERAGE.dump.expect("non-null function pointer")(
                &(&(*subtable).match_0)[j as usize] as *const Coverage,
            ),
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
        OTL_I_COVERAGE.dump.expect("non-null function pointer")(&(*subtable).to as *const Coverage),
    );
    json_object_push(
        _st,
        b"inputIndex\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*subtable).input_index as i64),
    );
    return _st;
}
pub unsafe extern "C" fn otl_gsub_parse_reverse(
    mut _subtable: *const ParsedValue,
    mut _options: *const Options,
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
    let mut subtable: *mut GsubReverseSubtable =
        (
            I_SUBTABLE_GSUB_REVERSE
                .create
                .expect("non-null function pointer"))();
    (*subtable).match_count = json_arr_len(_match) as TableId;
    (*subtable).match_0 = Vec::with_capacity((*subtable).match_count as usize);
    (*subtable).input_index = json_obj_getnum_fallback(
        _subtable,
        b"inputIndex\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as TableId;
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*subtable).match_count as ::core::ffi::c_int {
        (*subtable).match_0.push(coverage_from_raw(
            OTL_I_COVERAGE.parse.expect("non-null function pointer")(
                json_arr_at(_match, j as u32),
            ),
        ));
        j = j.wrapping_add(1);
    }
    (*subtable).to = coverage_from_raw(
        OTL_I_COVERAGE.parse.expect("non-null function pointer")(_to),
    );
    return subtable_from_raw(subtable, Subtable::GsubReverse);
}
pub unsafe extern "C" fn otfcc_build_gsub_reverse(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let Subtable::GsubReverse(mut_subtable) = &*_subtable else { unreachable!() };
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
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
            &(&(*subtable).match_0)[(*subtable).input_index as usize] as *const Coverage,
        )))]);
    bk_push(root, &[bk_int(BkCellType::B16, ((*subtable).input_index as ::core::ffi::c_int) as u32)]);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*subtable).input_index as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
                &(&(*subtable).match_0)[j as usize] as *const Coverage,
            )))]);
        j = j.wrapping_add(1);
    }
    bk_push(root, &[bk_int(BkCellType::B16, ((*subtable).match_count as ::core::ffi::c_int
            - (*subtable).input_index as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as u32)]);
    let mut j_0: TableId =
        ((*subtable).input_index as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
    while (j_0 as ::core::ffi::c_int) < (*subtable).match_count as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
                &(&(*subtable).match_0)[j_0 as usize] as *const Coverage,
            )))]);
        j_0 = j_0.wrapping_add(1);
    }
    bk_push(root, &[bk_int(BkCellType::B16, ((*subtable).to.len() as ::core::ffi::c_int) as u32)]);
    let mut j_1: TableId = 0 as TableId;
    while (j_1 as usize) < (*subtable).to.len() {
        bk_push(root, &[bk_int(BkCellType::B16, ((&(*subtable).to)[j_1 as usize].index as ::core::ffi::c_int) as u32)]);
        j_1 = j_1.wrapping_add(1);
    }
    return bk_build_block(root);
}
