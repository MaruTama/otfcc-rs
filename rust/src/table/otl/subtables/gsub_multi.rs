#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc};


use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dispose, otfcc_handle_dup, Handle, GlyphHandle};

use crate::support::alloc::__caryll_reallocate;
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::table::otl::{GsubMultiEntry, Subtable, GsubMultiSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::vendor::json_builder::{json_object_new, json_object_push_bytes_key};
use crate::vendor::sds::{sdsnewlen};
unsafe extern "C" fn delete_gsub_multi_entry(mut entry: *mut GsubMultiEntry) {
    otfcc_handle_dispose(&raw mut (*entry).from);
    otl_coverage_free((*entry).to);
    (*entry).to = ::core::ptr::null_mut::<Coverage>();
}
pub(crate) unsafe fn dispose_gsub_multi_subtable(arr: *mut GsubMultiSubtable) {
    for e in (*arr).iter_mut() {
        delete_gsub_multi_entry(e);
    }
    *arr = Vec::new();
}
pub(crate) unsafe extern "C" fn subtable_gsub_multi_free(x: *mut GsubMultiSubtable) {
    if x.is_null() {
        return;
    }
    dispose_gsub_multi_subtable(x);
    free(x as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn subtable_gsub_multi_create() -> *mut GsubMultiSubtable {
    let x: *mut GsubMultiSubtable =
        malloc(::core::mem::size_of::<GsubMultiSubtable>() as usize) as *mut GsubMultiSubtable;
    x.write(Vec::new());
    x
}
pub unsafe extern "C" fn otl_read_gsub_multi(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    _max_glyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut seq_count: GlyphId = 0;
    let subtable: *mut GsubMultiSubtable = subtable_gsub_multi_create();
    let mut from: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    if !(table_length < offset.wrapping_add(6 as u32)) {
        from = read_coverage(
            data as *const u8,
            table_length,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        seq_count = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as GlyphId;
        if seq_count as usize == (*from).len() {
            if !(table_length
                < offset.wrapping_add(6 as u32).wrapping_add(
                    (seq_count as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                ))
            {
                for j in 0..seq_count {
                    let seq_offset: u32 = offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            as *const u8,
                    )
                        as u32);
                    let cov: *mut Coverage =
                        otl_coverage_create();
                    let n: GlyphId =
                        read_16u(data.offset(seq_offset as isize) as *const u8) as GlyphId;
                    for k in 0..n {
                        push_to_coverage(
                            cov,
                            handle_from_index(read_16u(
                                data.offset(seq_offset as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as GlyphId) as GlyphHandle,
                        );
                    }
                    (*subtable).push(GsubMultiEntry {
                        from: otfcc_handle_dup((&(*from))[j as usize].clone() as Handle) as GlyphHandle,
                        to: cov,
                    });
                }
                otl_coverage_free(from);
                return subtable as *mut Subtable;
            }
        }
    }
    if !from.is_null() {
        otl_coverage_free(from);
    }
    subtable_gsub_multi_free(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gsub_dump_multi(
    mut _subtable: *const Subtable,
) -> *mut JsonValue {
    let subtable: *const GsubMultiSubtable = &raw const (*_subtable).gsub_multi as *const GsubMultiSubtable;
    let st: *mut JsonValue = json_object_new((*subtable).len());
    for j in 0..(*subtable).len() as GlyphId {
        let entry = &(&(*subtable))[j as usize];
        json_object_push_bytes_key(
            st,
            &(*entry).from.name,
            OTL_I_COVERAGE.dump.expect("non-null function pointer")((*entry).to),
        );
    }
    return st;
}
pub unsafe extern "C" fn otl_gsub_parse_multi(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    let st: *mut GsubMultiSubtable = subtable_gsub_multi_create();
    for k in 0..(*_subtable).u.object.length as GlyphId {
        let entry = (*_subtable).u.object.values.offset(k as isize);
        let _to: *mut JsonValue = (*entry).value as *mut JsonValue;
        if !_to.is_null() && (*_to).type_0 == JsonType::Array {
            (*st).push(GsubMultiEntry {
                from: handle_from_name(sdsnewlen(
                    (*entry).name as *const ::core::ffi::c_void,
                    (*entry).name_length as usize,
                )) as GlyphHandle,
                to: OTL_I_COVERAGE.parse.expect("non-null function pointer")(_to),
            });
        }
    }
    return st as *mut Subtable;
}
unsafe extern "C" fn build_gsub_multi_subtable_range(
    subtable: *const GsubMultiSubtable,
    start: GlyphId,
    end: GlyphId,
) -> *mut Buffer {
    let cov: *mut Coverage = otl_coverage_create();
    for j in start..end {
        push_to_coverage(
            cov,
            otfcc_handle_dup(
                (&(*subtable))[j as usize].from.clone() as Handle,
            ) as GlyphHandle,
        );
    }
    let root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(cov))), bk_int(BkCellType::B16, (end as ::core::ffi::c_int - start as ::core::ffi::c_int) as u32)]);
    for j_0 in start..end {
        let to = (&(*subtable))[j_0 as usize].to;
        let b: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*to).len() as ::core::ffi::c_int) as u32)]);
        for k in 0..(*to).len() {
            bk_push(b, &[bk_int(BkCellType::B16, ((&(*to))[k].index as ::core::ffi::c_int) as u32)]);
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
    mut count: *mut TableId,
) -> *mut *mut Buffer {
    let subtable: *const GsubMultiSubtable = &raw const (*_subtable).gsub_multi as *const GsubMultiSubtable;
    let mut parts: *mut *mut Buffer = ::core::ptr::null_mut::<*mut Buffer>();
    let mut n_parts: TableId = 0 as TableId;
    let mut start: GlyphId = 0 as GlyphId;
    while (start as usize) < (*subtable).len() {
        let mut size: usize = (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize;
        let mut end: GlyphId = start;
        while (end as usize) < (*subtable).len() {
            let mut entry_size: usize = ((2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize)
                .wrapping_add(
                    ((*(&(*subtable))[end as usize].to).len())
                        .wrapping_mul(2 as usize),
                );
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
pub unsafe extern "C" fn otfcc_build_gsub_multi_subtable(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let subtable: *const GsubMultiSubtable = &raw const (*_subtable).gsub_multi as *const GsubMultiSubtable;
    return build_gsub_multi_subtable_range(subtable, 0 as GlyphId, (*subtable).len() as GlyphId);
}
