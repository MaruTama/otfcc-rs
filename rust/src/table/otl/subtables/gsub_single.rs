#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc};


use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dispose, otfcc_handle_dup, otfcc_handle_empty, Handle, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow_to, cvec_init, cvec_push};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::table::otl::{GsubSingleSubtableVectorInterface, GsubSingleEntry, Subtable, GsubSingleSubtable};
use crate::table::otl::subtables::BuildHeuristics;
use crate::vendor::uthash::{UtHashHandle};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::vendor::json_builder::{json_object_new, json_object_push, json_string_new};
use crate::vendor::sds::{sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubSingleEntryElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GsubSingleEntry) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut GsubSingleEntry, *const GsubSingleEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GsubSingleEntry) -> ()>,
}
unsafe extern "C" fn gss_entry_ctor(mut entry: *mut GsubSingleEntry) {
    (*entry).from = otfcc_handle_empty() as GlyphHandle;
    (*entry).to = otfcc_handle_empty() as GlyphHandle;
}
unsafe extern "C" fn gss_entry_copyctor(
    mut dst: *mut GsubSingleEntry,
    mut src: *const GsubSingleEntry,
) {
    (*dst).from = otfcc_handle_dup((*src).from as Handle)
        as GlyphHandle;
    (*dst).to = otfcc_handle_dup((*src).to as Handle)
        as GlyphHandle;
}
unsafe extern "C" fn gss_entry_dtor(mut entry: *mut GsubSingleEntry) {
    otfcc_handle_dispose(&raw mut (*entry).from);
    otfcc_handle_dispose(&raw mut (*entry).to);
}
static GSS_TYPEINFO: GsubSingleEntryElementInterface = {
    GsubSingleEntryElementInterface {
        init: Some(gss_entry_ctor as unsafe extern "C" fn(*mut GsubSingleEntry) -> ()),
        copy: Some(
            gss_entry_copyctor
                as unsafe extern "C" fn(*mut GsubSingleEntry, *const GsubSingleEntry) -> (),
        ),
        dispose: Some(gss_entry_dtor as unsafe extern "C" fn(*mut GsubSingleEntry) -> ()),
    }
};
#[inline]
unsafe fn as_cvec(arr: *mut GsubSingleSubtable) -> *mut CVecRaw<GsubSingleEntry> {
    arr as *mut CVecRaw<GsubSingleEntry>
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_init(arr: *mut GsubSingleSubtable) {
    cvec_init(as_cvec(arr));
}
pub static I_SUBTABLE_GSUB_SINGLE: GsubSingleSubtableVectorInterface = {
    GsubSingleSubtableVectorInterface {
        init: Some(
            subtable_gsub_single_init as unsafe extern "C" fn(*mut GsubSingleSubtable) -> (),
        ),
        copy: Some(
            subtable_gsub_single_copy
                as unsafe extern "C" fn(
                    *mut GsubSingleSubtable,
                    *const GsubSingleSubtable,
                ) -> (),
        ),
        dispose: Some(
            subtable_gsub_single_dispose as unsafe extern "C" fn(*mut GsubSingleSubtable) -> (),
        ),
        create: Some(subtable_gsub_single_create),
        free: Some(
            subtable_gsub_single_free as unsafe extern "C" fn(*mut GsubSingleSubtable) -> (),
        ),
        clear: Some(
            subtable_gsub_single_dispose as unsafe extern "C" fn(*mut GsubSingleSubtable) -> (),
        ),
        push: Some(
            subtable_gsub_single_push
                as unsafe extern "C" fn(*mut GsubSingleSubtable, GsubSingleEntry) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_single_push(arr: *mut GsubSingleSubtable, elem: GsubSingleEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_grow_to(arr: *mut GsubSingleSubtable, target: usize) {
    cvec_grow_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_copy(
    mut dst: *mut GsubSingleSubtable,
    mut src: *const GsubSingleSubtable,
) {
    subtable_gsub_single_init(dst);
    subtable_gsub_single_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if GSS_TYPEINFO.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            GSS_TYPEINFO.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut GsubSingleEntry,
                (*src).items.offset(j as isize) as *mut GsubSingleEntry
                    as *const GsubSingleEntry,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_dispose(mut arr: *mut GsubSingleSubtable) {
    if arr.is_null() {
        return;
    }
    if GSS_TYPEINFO.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            GSS_TYPEINFO.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut GsubSingleEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<GsubSingleEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_free(mut x: *mut GsubSingleSubtable) {
    if x.is_null() {
        return;
    }
    subtable_gsub_single_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_create() -> *mut GsubSingleSubtable {
    let mut x: *mut GsubSingleSubtable =
        malloc(::core::mem::size_of::<GsubSingleSubtable>() as usize)
            as *mut GsubSingleSubtable;
    subtable_gsub_single_init(x);
    return x;
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
    let mut subtable: *mut GsubSingleSubtable =
        (
            I_SUBTABLE_GSUB_SINGLE
                .create
                .expect("non-null function pointer"))();
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
                        I_SUBTABLE_GSUB_SINGLE
                            .push
                            .expect("non-null function pointer")(
                            subtable,
                            GsubSingleEntry {
                                from: otfcc_handle_dup(
                                    *(*from).glyphs.offset(j_1 as isize) as Handle,
                                ) as GlyphHandle,
                                to: otfcc_handle_dup(
                                    *(*to).glyphs.offset(j_1 as isize) as Handle,
                                ) as GlyphHandle,
                            },
                        );
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
    I_SUBTABLE_GSUB_SINGLE
        .free
        .expect("non-null function pointer")(subtable);
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
    let mut subtable: *const GsubSingleSubtable = &raw const (*_subtable).gsub_single;
    let mut st: *mut JsonValue = json_object_new((*subtable).length);
    let mut j: usize = 0 as usize;
    while j < (*subtable).length {
        json_object_push(
            st,
            (*(*subtable).items.offset(j as isize)).from.name as *const ::core::ffi::c_char,
            json_string_new(
                (*(*subtable).items.offset(j as isize)).to.name as *const ::core::ffi::c_char,
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
    let mut subtable: *mut GsubSingleSubtable =
        (
            I_SUBTABLE_GSUB_SINGLE
                .create
                .expect("non-null function pointer"))();
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
            I_SUBTABLE_GSUB_SINGLE
                .push
                .expect("non-null function pointer")(
                subtable,
                GsubSingleEntry {
                    from: from as GlyphHandle,
                    to: to as GlyphHandle,
                },
            );
        }
        j = j.wrapping_add(1);
    }
    return subtable as *mut Subtable;
}
pub unsafe extern "C" fn otfcc_build_gsub_single_subtable(
    mut _subtable: *const Subtable,
    mut heuristics: BuildHeuristics,
) -> *mut Buffer {
    let mut subtable: *const GsubSingleSubtable = &raw const (*_subtable).gsub_single;
    let mut is_constant_difference: bool = (*subtable).length > 0 as usize;
    if is_constant_difference {
        let mut difference: i32 = (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
            .to
            .index as i32
            - (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                .from
                .index as i32;
        is_constant_difference = is_constant_difference as ::core::ffi::c_int != 0
            && difference < 0x8000 as i32
            && difference > -(0x8000 as i32);
        let mut j: GlyphId = 1 as GlyphId;
        while (j as usize) < (*subtable).length {
            let mut diff_j: i32 = (*(*subtable).items.offset(j as isize)).to.index as i32
                - (*(*subtable).items.offset(j as isize)).from.index as i32;
            is_constant_difference = is_constant_difference as ::core::ffi::c_int != 0
                && diff_j == difference
                && diff_j < 0x8000 as i32
                && diff_j > -(0x8000 as i32);
            j = j.wrapping_add(1);
        }
    }
    let mut cov: *mut Coverage = otl_coverage_create();
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).length {
        push_to_coverage(
            cov,
            otfcc_handle_dup(
                (*(*subtable).items.offset(j_0 as isize)).from as Handle,
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
        let mut b: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(coverage_buf)), bk_int(BkCellType::B16, ((*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                .to
                .index as ::core::ffi::c_int
                - (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                    .from
                    .index as ::core::ffi::c_int) as u32)]);
        otl_coverage_free(cov);
        return bk_build_block(b);
    } else {
        let mut b_0: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 2 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(coverage_buf)), bk_int(BkCellType::B16, ((*subtable).length) as u32)]);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as usize) < (*subtable).length {
            bk_push(b_0, &[bk_int(BkCellType::B16, ((*(*subtable).items.offset(k as isize)).to.index as ::core::ffi::c_int) as u32)]);
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
