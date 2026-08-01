#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc};


use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dispose, otfcc_handle_dup, Handle, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::table::otl::{GsubSingleEntry, Subtable, GsubSingleSubtable};
use crate::table::otl::subtables::BuildHeuristics;
use crate::vendor::uthash::{UtHashHandle};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::vendor::json_builder::{json_object_new, json_object_push, json_string_new};
use crate::vendor::sds::{sdsnewlen};
unsafe extern "C" fn gss_entry_dtor(mut entry: *mut GsubSingleEntry) {
    otfcc_handle_dispose(&raw mut (*entry).from);
    otfcc_handle_dispose(&raw mut (*entry).to);
}
pub(crate) unsafe fn dispose_gsub_single_subtable(arr: *mut GsubSingleSubtable) {
    for e in (*arr).iter_mut() {
        gss_entry_dtor(e);
    }
    *arr = Vec::new();
}
pub(crate) unsafe extern "C" fn subtable_gsub_single_free(x: *mut GsubSingleSubtable) {
    if x.is_null() {
        return;
    }
    dispose_gsub_single_subtable(x);
    free(x as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn subtable_gsub_single_create() -> *mut GsubSingleSubtable {
    let x: *mut GsubSingleSubtable =
        malloc(::core::mem::size_of::<GsubSingleSubtable>() as usize)
            as *mut GsubSingleSubtable;
    x.write(Vec::new());
    x
}
pub unsafe extern "C" fn otl_read_gsub_single(
    data: FontFilePointer,
    mut table_length: u32,
    mut subtable_offset: u32,
    _max_glyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut subtable_format: u16 = 0;
    let mut current_block: u64;
    let subtable: *mut GsubSingleSubtable = subtable_gsub_single_create();
    let mut from: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut to: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    if !(table_length < subtable_offset.wrapping_add(6 as u32)) {
        subtable_format = read_16u(data.offset(subtable_offset as isize) as *const u8);
        from = read_coverage(
            data as *const u8,
            table_length,
            subtable_offset.wrapping_add(read_16u(
                data.offset(subtable_offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !(from.is_null() || (*from).num_glyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
            if subtable_format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                to = __caryll_allocate_clean(
                    ::core::mem::size_of::<Coverage>() as usize,
                    36 as ::core::ffi::c_ulong,
                ) as *mut Coverage;
                (*to).num_glyphs = (*from).num_glyphs;
                (*to).glyphs = __caryll_allocate_clean(
                    (::core::mem::size_of::<GlyphHandle>() as usize)
                        .wrapping_mul((*to).num_glyphs as usize),
                    38 as ::core::ffi::c_ulong,
                ) as *mut GlyphHandle;
                let mut delta: u16 = read_16u(
                    data.offset(subtable_offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *const u8,
                );
                let mut j: GlyphId = 0 as GlyphId;
                while (j as ::core::ffi::c_int) < (*from).num_glyphs as ::core::ffi::c_int {
                    *(*to).glyphs.offset(j as isize) = handle_from_index(
                        ((*(*from).glyphs.offset(j as isize)).index as ::core::ffi::c_int
                            + delta as ::core::ffi::c_int) as GlyphId,
                    ) as GlyphHandle;
                    j = j.wrapping_add(1);
                }
                current_block = 126606456056746247;
            } else {
                let mut toglyphs: GlyphId = read_16u(
                    data.offset(subtable_offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *const u8,
                ) as GlyphId;
                if table_length
                    < subtable_offset.wrapping_add(6 as u32).wrapping_add(
                        (toglyphs as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                    )
                    || toglyphs as ::core::ffi::c_int != (*from).num_glyphs as ::core::ffi::c_int
                {
                    current_block = 2938280209257981098;
                } else {
                    to = __caryll_allocate_clean(
                        ::core::mem::size_of::<Coverage>() as usize,
                        48 as ::core::ffi::c_ulong,
                    ) as *mut Coverage;
                    (*to).num_glyphs = toglyphs;
                    (*to).glyphs = __caryll_allocate_clean(
                        (::core::mem::size_of::<GlyphHandle>() as usize)
                            .wrapping_mul((*to).num_glyphs as usize),
                        50 as ::core::ffi::c_ulong,
                    ) as *mut GlyphHandle;
                    let mut j_0: GlyphId = 0 as GlyphId;
                    while (j_0 as ::core::ffi::c_int) < (*to).num_glyphs as ::core::ffi::c_int {
                        *(*to).glyphs.offset(j_0 as isize) =
                            handle_from_index(read_16u(
                                data.offset(subtable_offset as isize)
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as GlyphId) as GlyphHandle;
                        j_0 = j_0.wrapping_add(1);
                    }
                    current_block = 126606456056746247;
                }
            }
            match current_block {
                2938280209257981098 => {}
                _ => {
                    let mut j_1: GlyphId = 0 as GlyphId;
                    while (j_1 as ::core::ffi::c_int) < (*from).num_glyphs as ::core::ffi::c_int {
                        (*subtable).push(GsubSingleEntry {
                            from: otfcc_handle_dup(
                                *(*from).glyphs.offset(j_1 as isize) as Handle,
                            ) as GlyphHandle,
                            to: otfcc_handle_dup(
                                *(*to).glyphs.offset(j_1 as isize) as Handle,
                            ) as GlyphHandle,
                        });
                        j_1 = j_1.wrapping_add(1);
                    }
                    if !from.is_null() {
                        otl_coverage_free(from);
                    }
                    if !to.is_null() {
                        otl_coverage_free(to);
                    }
                    return subtable as *mut Subtable;
                }
            }
        }
    }
    subtable_gsub_single_free(subtable);
    if !from.is_null() {
        otl_coverage_free(from);
    }
    if !to.is_null() {
        otl_coverage_free(to);
    }
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gsub_dump_single(
    mut _subtable: *const Subtable,
) -> *mut JsonValue {
    let subtable: *const GsubSingleSubtable = &raw const (*_subtable).gsub_single as *const GsubSingleSubtable;
    let mut st: *mut JsonValue = json_object_new((*subtable).len());
    let mut j: usize = 0 as usize;
    while j < (*subtable).len() {
        json_object_push(
            st,
            (&(*subtable))[j].from.name as *const ::core::ffi::c_char,
            json_string_new(
                (&(*subtable))[j].to.name as *const ::core::ffi::c_char,
            ),
        );
        j = j.wrapping_add(1);
    }
    return st;
}
pub unsafe extern "C" fn otl_gsub_parse_single(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    let subtable: *mut GsubSingleSubtable = subtable_gsub_single_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < (*_subtable).u.object.length {
        if !(*(*_subtable).u.object.values.offset(j as isize))
            .value
            .is_null()
            && (*(*(*_subtable).u.object.values.offset(j as isize)).value).type_0
                as ::core::ffi::c_uint
                == JsonType::String as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut from: GlyphHandle =
                handle_from_name(sdsnewlen(
                    (*(*_subtable).u.object.values.offset(j as isize)).name
                        as *const ::core::ffi::c_void,
                    (*(*_subtable).u.object.values.offset(j as isize)).name_length as usize,
                )) as GlyphHandle;
            let mut to: GlyphHandle =
                handle_from_name(sdsnewlen(
                    (*(*(*_subtable).u.object.values.offset(j as isize)).value)
                        .u
                        .string
                        .ptr as *const ::core::ffi::c_void,
                    (*(*(*_subtable).u.object.values.offset(j as isize)).value)
                        .u
                        .string
                        .length as usize,
                )) as GlyphHandle;
            (*subtable).push(GsubSingleEntry {
                from: from as GlyphHandle,
                to: to as GlyphHandle,
            });
        }
        j = j.wrapping_add(1);
    }
    return subtable as *mut Subtable;
}
pub unsafe extern "C" fn otfcc_build_gsub_single_subtable(
    mut _subtable: *const Subtable,
    mut heuristics: BuildHeuristics,
) -> *mut Buffer {
    let subtable: *const GsubSingleSubtable = &raw const (*_subtable).gsub_single as *const GsubSingleSubtable;
    let mut is_constant_difference: bool = (*subtable).len() > 0 as usize;
    if is_constant_difference {
        let mut difference: i32 = (&(*subtable))[0].to.index as i32
            - (&(*subtable))[0].from.index as i32;
        is_constant_difference = is_constant_difference as ::core::ffi::c_int != 0
            && difference < 0x8000 as i32
            && difference > -(0x8000 as i32);
        let mut j: GlyphId = 1 as GlyphId;
        while (j as usize) < (*subtable).len() {
            let mut diff_j: i32 = (&(*subtable))[j as usize].to.index as i32
                - (&(*subtable))[j as usize].from.index as i32;
            is_constant_difference = is_constant_difference as ::core::ffi::c_int != 0
                && diff_j == difference
                && diff_j < 0x8000 as i32
                && diff_j > -(0x8000 as i32);
            j = j.wrapping_add(1);
        }
    }
    let mut cov: *mut Coverage = otl_coverage_create();
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).len() {
        push_to_coverage(
            cov,
            otfcc_handle_dup(
                (&(*subtable))[j_0 as usize].from as Handle,
            ) as GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let mut coverage_buf: *mut Buffer = OTL_I_COVERAGE
        .build_format
        .expect("non-null function pointer")(
        cov,
        heuristics.contains(BuildHeuristics::GSUB_VERT) as u16,
    );
    if is_constant_difference as ::core::ffi::c_int != 0
        && !heuristics.contains(BuildHeuristics::GSUB_VERT)
    {
        let mut b: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(coverage_buf)), bk_int(BkCellType::B16, ((&(*subtable))[0]
                .to
                .index as ::core::ffi::c_int
                - (&(*subtable))[0]
                    .from
                    .index as ::core::ffi::c_int) as u32)]);
        otl_coverage_free(cov);
        return bk_build_block(b);
    } else {
        let mut b_0: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 2 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(coverage_buf)), bk_int(BkCellType::B16, ((*subtable).len()) as u32)]);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as usize) < (*subtable).len() {
            bk_push(b_0, &[bk_int(BkCellType::B16, ((&(*subtable))[k as usize].to.index as ::core::ffi::c_int) as u32)]);
            k = k.wrapping_add(1);
        }
        otl_coverage_free(cov);
        return bk_build_block(b_0);
    };
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubSingleMapHash {
    pub fromid: ::core::ffi::c_int,
    pub fromname: SdsRaw,
    pub toid: ::core::ffi::c_int,
    pub toname: SdsRaw,
    pub hh: UtHashHandle,
}
