#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc};

use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_name, otfcc_handle_dispose, otfcc_handle_dup, Handle, GlyphHandle};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::table::otl::{GposSingleEntry, PositionValue, Subtable, GposSingleSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::table::otl::subtables::gpos_common::{bk_gpos_value, gpos_dump_value, gpos_parse_value, position_format_length, read_gpos_value, required_position_format};
use crate::vendor::json_builder::{json_object_new, json_object_push};
use crate::vendor::sds::{sdsnewlen};
unsafe extern "C" fn delete_gpos_single_entry(mut entry: *mut GposSingleEntry) {
    otfcc_handle_dispose(&raw mut (*entry).target);
}
pub(crate) unsafe fn dispose_gpos_single_subtable(arr: *mut GposSingleSubtable) {
    for e in (*arr).iter_mut() {
        delete_gpos_single_entry(e);
    }
    *arr = Vec::new();
}
pub(crate) unsafe extern "C" fn subtable_gpos_single_free(x: *mut GposSingleSubtable) {
    if x.is_null() {
        return;
    }
    dispose_gpos_single_subtable(x);
    free(x as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn subtable_gpos_single_create() -> *mut GposSingleSubtable {
    let x: *mut GposSingleSubtable =
        malloc(::core::mem::size_of::<GposSingleSubtable>() as usize)
            as *mut GposSingleSubtable;
    x.write(Vec::new());
    x
}
pub unsafe extern "C" fn otl_read_gpos_single(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    _max_glyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut subtable_format: u16 = 0;
    let mut current_block: u64;
    let subtable: *mut GposSingleSubtable = subtable_gpos_single_create();
    let mut targets: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    if !(table_length < offset.wrapping_add(6 as u32)) {
        subtable_format = read_16u(data.offset(offset as isize) as *const u8);
        targets = read_coverage(
            data as *const u8,
            table_length,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !(targets.is_null()
            || (*targets).num_glyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            if subtable_format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                let mut v: PositionValue = read_gpos_value(
                    data,
                    table_length,
                    offset.wrapping_add(6 as u32),
                    read_16u(
                        data.offset(offset as isize)
                            .offset(4 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ),
                );
                let mut j: GlyphId = 0 as GlyphId;
                while (j as ::core::ffi::c_int) < (*targets).num_glyphs as ::core::ffi::c_int {
                    (*subtable).push(GposSingleEntry {
                        target: otfcc_handle_dup(
                            *(*targets).glyphs.offset(j as isize) as Handle,
                        ) as GlyphHandle,
                        value: v,
                    });
                    j = j.wrapping_add(1);
                }
                current_block = 6009453772311597924;
            } else {
                let mut value_format: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *const u8,
                );
                let mut value_count: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(6 as ::core::ffi::c_int as isize)
                        as *const u8,
                );
                if table_length
                    < offset.wrapping_add(8 as u32).wrapping_add(
                        (position_format_length(value_format) as ::core::ffi::c_int
                            * value_count as ::core::ffi::c_int) as u32,
                    )
                {
                    current_block = 18154618883129817269;
                } else if value_count as ::core::ffi::c_int
                    != (*targets).num_glyphs as ::core::ffi::c_int
                {
                    current_block = 18154618883129817269;
                } else {
                    let mut j_0: GlyphId = 0 as GlyphId;
                    while (j_0 as ::core::ffi::c_int) < (*targets).num_glyphs as ::core::ffi::c_int {
                        (*subtable).push(GposSingleEntry {
                            target: otfcc_handle_dup(
                                *(*targets).glyphs.offset(j_0 as isize) as Handle,
                            ) as GlyphHandle,
                            value: read_gpos_value(
                                data,
                                table_length,
                                offset.wrapping_add(8 as u32).wrapping_add(
                                    (j_0 as ::core::ffi::c_int
                                        * position_format_length(value_format)
                                            as ::core::ffi::c_int)
                                        as u32,
                                ),
                                value_format,
                            ),
                        });
                        j_0 = j_0.wrapping_add(1);
                    }
                    current_block = 6009453772311597924;
                }
            }
            match current_block {
                18154618883129817269 => {}
                _ => {
                    if !targets.is_null() {
                        otl_coverage_free(targets);
                    }
                    return subtable as *mut Subtable;
                }
            }
        }
    }
    if !targets.is_null() {
        otl_coverage_free(targets);
    }
    subtable_gpos_single_free(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gpos_dump_single(
    mut _subtable: *const Subtable,
) -> *mut JsonValue {
    let subtable: *const GposSingleSubtable = &raw const (*_subtable).gpos_single as *const GposSingleSubtable;
    let mut st: *mut JsonValue = json_object_new((*subtable).len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).len() {
        json_object_push(
            st,
            (&(*subtable))[j as usize].target.name as *const ::core::ffi::c_char,
            gpos_dump_value((&(*subtable))[j as usize].value),
        );
        j = j.wrapping_add(1);
    }
    return st;
}
pub unsafe extern "C" fn otl_gpos_parse_single(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    let subtable: *mut GposSingleSubtable = subtable_gpos_single_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < (*_subtable).u.object.length {
        if !(*(*_subtable).u.object.values.offset(j as isize))
            .value
            .is_null()
            && (*(*(*_subtable).u.object.values.offset(j as isize)).value).type_0
                as ::core::ffi::c_uint
                == JsonType::Object as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut gname: SdsRaw = sdsnewlen(
                (*(*_subtable).u.object.values.offset(j as isize)).name
                    as *const ::core::ffi::c_void,
                (*(*_subtable).u.object.values.offset(j as isize)).name_length as usize,
            );
            (*subtable).push(GposSingleEntry {
                target: handle_from_name(gname)
                    as GlyphHandle,
                value: gpos_parse_value(
                    (*(*_subtable).u.object.values.offset(j as isize)).value as *mut JsonValue,
                ),
            });
        }
        j = j.wrapping_add(1);
    }
    return subtable as *mut Subtable;
}
pub unsafe extern "C" fn otfcc_build_gpos_single(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let subtable: *const GposSingleSubtable = &raw const (*_subtable).gpos_single as *const GposSingleSubtable;
    let mut is_const: bool = (*subtable).len() > 0 as usize;
    let mut format: u16 = 0 as u16;
    if (*subtable).len() > 0 as usize {
        let mut j: GlyphId = 0 as GlyphId;
        while (j as usize) < (*subtable).len() {
            is_const = is_const as ::core::ffi::c_int != 0
                && (&(*subtable))[j as usize].value.dx
                    == (&(*subtable))[0].value.dx
                && (&(*subtable))[j as usize].value.dy
                    == (&(*subtable))[0].value.dy
                && (&(*subtable))[j as usize].value.d_width
                    == (&(*subtable))[0].value.d_width
                && (&(*subtable))[j as usize].value.d_height
                    == (&(*subtable))[0].value.d_height;
            format = (format as ::core::ffi::c_int
                | required_position_format((&(*subtable))[j as usize].value)
                    as ::core::ffi::c_int) as u16;
            j = j.wrapping_add(1);
        }
    }
    let mut cov: *mut Coverage = otl_coverage_create();
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).len() {
        push_to_coverage(
            cov,
            otfcc_handle_dup(
                (&(*subtable))[j_0 as usize].target as Handle,
            ) as GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let mut coverage_buf: *mut Buffer =
        OTL_I_COVERAGE.build.expect("non-null function pointer")(cov);
    if is_const {
        let mut b: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(coverage_buf)), bk_int(BkCellType::B16, (format as ::core::ffi::c_int) as u32), bk_ptr(BkCellType::Embed, bk_gpos_value(
                (&(*subtable))[0].value,
                format,
            ))]);
        otl_coverage_free(cov);
        return bk_build_block(b);
    } else {
        let mut b_0: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 2 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(coverage_buf)), bk_int(BkCellType::B16, (format as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*subtable).len()) as u32)]);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as usize) < (*subtable).len() {
            bk_push(b_0, &[bk_ptr(BkCellType::Embed, bk_gpos_value((&(*subtable))[k as usize].value, format))]);
            k = k.wrapping_add(1);
        }
        otl_coverage_free(cov);
        return bk_build_block(b_0);
    };
}
