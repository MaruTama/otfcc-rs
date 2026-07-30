#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};


use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dispose, otfcc_handle_dup, Handle, GlyphHandle, HandleState};

use crate::support::alloc::__caryll_reallocate;
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::table::otl::{GsubMultiSubtableVectorInterface, GsubMultiEntry, Subtable, GsubMultiSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::support::{ComparFn};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::vendor::json_builder::{json_object_new, json_object_push};
use crate::vendor::sds::{sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubMultiEntryElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GsubMultiEntry) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut GsubMultiEntry, *const GsubMultiEntry) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut GsubMultiEntry, *mut GsubMultiEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GsubMultiEntry) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut GsubMultiEntry, GsubMultiEntry) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut GsubMultiEntry, GsubMultiEntry) -> ()>,
}
unsafe extern "C" fn delete_gsub_multi_entry(mut entry: *mut GsubMultiEntry) {
    otfcc_handle_dispose(&raw mut (*entry).from);
    otl_coverage_free((*entry).to);
    (*entry).to = ::core::ptr::null_mut::<Coverage>();
}
static GSM_TYPEINFO: GsubMultiEntryElementInterface = {
    GsubMultiEntryElementInterface {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(delete_gsub_multi_entry as unsafe extern "C" fn(*mut GsubMultiEntry) -> ()),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe fn as_cvec(arr: *mut GsubMultiSubtable) -> *mut CVecRaw<GsubMultiEntry> {
    arr as *mut CVecRaw<GsubMultiEntry>
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_grow_to(arr: *mut GsubMultiSubtable, target: usize) {
    cvec_grow_to(as_cvec(arr), target);
}
pub static I_SUBTABLE_GSUB_MULTI: GsubMultiSubtableVectorInterface = {
    GsubMultiSubtableVectorInterface {
        init: Some(
            subtable_gsub_multi_init as unsafe extern "C" fn(*mut GsubMultiSubtable) -> (),
        ),
        copy: Some(
            subtable_gsub_multi_copy
                as unsafe extern "C" fn(*mut GsubMultiSubtable, *const GsubMultiSubtable) -> (),
        ),
        move_0: Some(
            subtable_gsub_multi_move
                as unsafe extern "C" fn(*mut GsubMultiSubtable, *mut GsubMultiSubtable) -> (),
        ),
        dispose: Some(
            subtable_gsub_multi_dispose as unsafe extern "C" fn(*mut GsubMultiSubtable) -> (),
        ),
        replace: Some(
            subtable_gsub_multi_replace
                as unsafe extern "C" fn(*mut GsubMultiSubtable, GsubMultiSubtable) -> (),
        ),
        copyReplace: Some(
            subtable_gsub_multi_copy_replace
                as unsafe extern "C" fn(*mut GsubMultiSubtable, GsubMultiSubtable) -> (),
        ),
        create: Some(subtable_gsub_multi_create),
        free: Some(
            subtable_gsub_multi_free as unsafe extern "C" fn(*mut GsubMultiSubtable) -> (),
        ),
        initN: Some(
            subtable_gsub_multi_init_n
                as unsafe extern "C" fn(*mut GsubMultiSubtable, usize) -> (),
        ),
        initCapN: Some(
            subtable_gsub_multi_init_cap_n
                as unsafe extern "C" fn(*mut GsubMultiSubtable, usize) -> (),
        ),
        createN: Some(
            subtable_gsub_multi_create_n as unsafe extern "C" fn(usize) -> *mut GsubMultiSubtable,
        ),
        fill: Some(
            subtable_gsub_multi_fill
                as unsafe extern "C" fn(*mut GsubMultiSubtable, usize) -> (),
        ),
        clear: Some(
            subtable_gsub_multi_dispose as unsafe extern "C" fn(*mut GsubMultiSubtable) -> (),
        ),
        push: Some(
            subtable_gsub_multi_push
                as unsafe extern "C" fn(*mut GsubMultiSubtable, GsubMultiEntry) -> (),
        ),
        shrinkToFit: Some(
            subtable_gsub_multi_shrink_to_fit as unsafe extern "C" fn(*mut GsubMultiSubtable) -> (),
        ),
        pop: Some(
            subtable_gsub_multi_pop
                as unsafe extern "C" fn(*mut GsubMultiSubtable) -> GsubMultiEntry,
        ),
        disposeItem: Some(
            subtable_gsub_multi_dispose_item
                as unsafe extern "C" fn(*mut GsubMultiSubtable, usize) -> (),
        ),
        filterEnv: Some(
            subtable_gsub_multi_filter_env
                as unsafe extern "C" fn(
                    *mut GsubMultiSubtable,
                    Option<
                        unsafe extern "C" fn(
                            *const GsubMultiEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gsub_multi_sort
                as unsafe extern "C" fn(
                    *mut GsubMultiSubtable,
                    Option<
                        unsafe extern "C" fn(
                            *const GsubMultiEntry,
                            *const GsubMultiEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_multi_shrink_to_fit(mut arr: *mut GsubMultiSubtable) {
    subtable_gsub_multi_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_resize_to(arr: *mut GsubMultiSubtable, target: usize) {
    cvec_resize_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_move(
    dst: *mut GsubMultiSubtable,
    src: *mut GsubMultiSubtable,
) {
    cvec_move(as_cvec(dst), as_cvec(src));
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_dispose_item(
    mut arr: *mut GsubMultiSubtable,
    mut n: usize,
) {
    if GSM_TYPEINFO.dispose.is_some() {
        GSM_TYPEINFO.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut GsubMultiEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_sort(
    mut arr: *mut GsubMultiSubtable,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const GsubMultiEntry,
            *const GsubMultiEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<GsubMultiEntry>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const GsubMultiEntry,
                    *const GsubMultiEntry,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_fill(mut arr: *mut GsubMultiSubtable, mut n: usize) {
    while (*arr).length < n {
        let mut x: GsubMultiEntry = GsubMultiEntry {
            from: Handle {
                state: HandleState::Empty,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            to: ::core::ptr::null_mut::<Coverage>(),
        };
        if GSM_TYPEINFO.init.is_some() {
            GSM_TYPEINFO.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<GsubMultiEntry>() as usize,
            );
        }
        subtable_gsub_multi_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_push(arr: *mut GsubMultiSubtable, elem: GsubMultiEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_grow(arr: *mut GsubMultiSubtable) {
    cvec_grow(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_init(arr: *mut GsubMultiSubtable) {
    cvec_init(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_copy_replace(
    mut dst: *mut GsubMultiSubtable,
    src: GsubMultiSubtable,
) {
    subtable_gsub_multi_dispose(dst);
    subtable_gsub_multi_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_pop(arr: *mut GsubMultiSubtable) -> GsubMultiEntry {
    cvec_pop(as_cvec(arr))
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_copy(
    mut dst: *mut GsubMultiSubtable,
    mut src: *const GsubMultiSubtable,
) {
    subtable_gsub_multi_init(dst);
    subtable_gsub_multi_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if GSM_TYPEINFO.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            GSM_TYPEINFO.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut GsubMultiEntry,
                (*src).items.offset(j as isize) as *mut GsubMultiEntry
                    as *const GsubMultiEntry,
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
unsafe extern "C" fn subtable_gsub_multi_dispose(mut arr: *mut GsubMultiSubtable) {
    if arr.is_null() {
        return;
    }
    if GSM_TYPEINFO.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            GSM_TYPEINFO.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut GsubMultiEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<GsubMultiEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_replace(
    mut dst: *mut GsubMultiSubtable,
    src: GsubMultiSubtable,
) {
    subtable_gsub_multi_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GsubMultiSubtable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_init_cap_n(
    mut arr: *mut GsubMultiSubtable,
    mut n: usize,
) {
    subtable_gsub_multi_init(arr);
    subtable_gsub_multi_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_grow_to_n(arr: *mut GsubMultiSubtable, target: usize) {
    cvec_grow_to_n(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_init_n(mut arr: *mut GsubMultiSubtable, mut n: usize) {
    subtable_gsub_multi_init(arr);
    subtable_gsub_multi_grow_to_n(arr, n);
    subtable_gsub_multi_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_free(mut x: *mut GsubMultiSubtable) {
    if x.is_null() {
        return;
    }
    subtable_gsub_multi_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_create_n(mut n: usize) -> *mut GsubMultiSubtable {
    let mut t: *mut GsubMultiSubtable =
        malloc(::core::mem::size_of::<GsubMultiSubtable>() as usize) as *mut GsubMultiSubtable;
    subtable_gsub_multi_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_create() -> *mut GsubMultiSubtable {
    let mut x: *mut GsubMultiSubtable =
        malloc(::core::mem::size_of::<GsubMultiSubtable>() as usize) as *mut GsubMultiSubtable;
    subtable_gsub_multi_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_filter_env(
    mut arr: *mut GsubMultiSubtable,
    mut fn_0: Option<
        unsafe extern "C" fn(*const GsubMultiEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut GsubMultiEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if GSM_TYPEINFO.dispose.is_some() {
                GSM_TYPEINFO.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut GsubMultiEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
pub unsafe extern "C" fn otl_read_gsub_multi(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    _max_glyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut seq_count: GlyphId = 0;
    let subtable: *mut GsubMultiSubtable =
        (
            I_SUBTABLE_GSUB_MULTI
                .create
                .expect("non-null function pointer"))();
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
        if seq_count as ::core::ffi::c_int == (*from).numGlyphs as ::core::ffi::c_int {
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
                    I_SUBTABLE_GSUB_MULTI
                        .push
                        .expect("non-null function pointer")(
                        subtable,
                        GsubMultiEntry {
                            from: otfcc_handle_dup(
                                *(*from).glyphs.offset(j as isize) as Handle,
                            ) as GlyphHandle,
                            to: cov,
                        },
                    );
                }
                otl_coverage_free(from);
                return subtable as *mut Subtable;
            }
        }
    }
    if !from.is_null() {
        otl_coverage_free(from);
    }
    I_SUBTABLE_GSUB_MULTI
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gsub_dump_multi(
    mut _subtable: *const Subtable,
) -> *mut JsonValue {
    let subtable: *const GsubMultiSubtable = &raw const (*_subtable).gsub_multi;
    let st: *mut JsonValue = json_object_new((*subtable).length);
    for j in 0..(*subtable).length as GlyphId {
        let entry = (*subtable).items.offset(j as isize);
        json_object_push(
            st,
            (*entry).from.name as *const ::core::ffi::c_char,
            OTL_I_COVERAGE.dump.expect("non-null function pointer")((*entry).to),
        );
    }
    return st;
}
pub unsafe extern "C" fn otl_gsub_parse_multi(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    let st: *mut GsubMultiSubtable =
        (
            I_SUBTABLE_GSUB_MULTI
                .create
                .expect("non-null function pointer"))();
    for k in 0..(*_subtable).u.object.length as GlyphId {
        let entry = (*_subtable).u.object.values.offset(k as isize);
        let _to: *mut JsonValue = (*entry).value as *mut JsonValue;
        if !_to.is_null() && (*_to).type_0 == JsonType::Array {
            I_SUBTABLE_GSUB_MULTI
                .push
                .expect("non-null function pointer")(
                st,
                GsubMultiEntry {
                    from: handle_from_name(sdsnewlen(
                        (*entry).name as *const ::core::ffi::c_void,
                        (*entry).name_length as usize,
                    )) as GlyphHandle,
                    to: OTL_I_COVERAGE.parse.expect("non-null function pointer")(_to),
                },
            );
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
                (*(*subtable).items.offset(j as isize)).from as Handle,
            ) as GlyphHandle,
        );
    }
    let root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(cov))), bk_int(BkCellType::B16, (end as ::core::ffi::c_int - start as ::core::ffi::c_int) as u32)]);
    for j_0 in start..end {
        let to = (*(*subtable).items.offset(j_0 as isize)).to;
        let b: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*to).numGlyphs as ::core::ffi::c_int) as u32)]);
        for k in 0..(*to).numGlyphs {
            bk_push(b, &[bk_int(BkCellType::B16, ((*(*to).glyphs.offset(k as isize)).index as ::core::ffi::c_int) as u32)]);
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
    let mut subtable: *const GsubMultiSubtable = &raw const (*_subtable).gsub_multi;
    let mut parts: *mut *mut Buffer = ::core::ptr::null_mut::<*mut Buffer>();
    let mut n_parts: TableId = 0 as TableId;
    let mut start: GlyphId = 0 as GlyphId;
    while (start as usize) < (*subtable).length {
        let mut size: usize = (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize;
        let mut end: GlyphId = start;
        while (end as usize) < (*subtable).length {
            let mut entry_size: usize = ((2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize)
                .wrapping_add(
                    ((*(*(*subtable).items.offset(end as isize)).to).numGlyphs as usize)
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
    let mut subtable: *const GsubMultiSubtable = &raw const (*_subtable).gsub_multi;
    return build_gsub_multi_subtable_range(subtable, 0 as GlyphId, (*subtable).length as GlyphId);
}
