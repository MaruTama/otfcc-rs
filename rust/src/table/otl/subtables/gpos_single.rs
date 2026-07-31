#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memset, qsort};

use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_name, otfcc_handle_dispose, otfcc_handle_dup, Handle, GlyphHandle, HandleState};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::table::otl::{GposSingleSubtableVectorInterface, GposSingleEntry, PositionValue, Subtable, GposSingleSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::support::{ComparFn};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::table::otl::subtables::gpos_common::{bk_gpos_value, gpos_dump_value, gpos_parse_value, position_format_length, read_gpos_value, required_position_format};
use crate::vendor::json_builder::{json_object_new, json_object_push};
use crate::vendor::sds::{sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposSingleEntryElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GposSingleEntry) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut GposSingleEntry, *const GposSingleEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GposSingleEntry) -> ()>,
}
unsafe extern "C" fn delete_gpos_single_entry(mut entry: *mut GposSingleEntry) {
    otfcc_handle_dispose(&raw mut (*entry).target);
}
static GSS_TYPEINFO: GposSingleEntryElementInterface = {
    GposSingleEntryElementInterface {
        init: None,
        copy: None,
        dispose: Some(
            delete_gpos_single_entry as unsafe extern "C" fn(*mut GposSingleEntry) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_single_resize_to(arr: *mut GposSingleSubtable, target: usize) {
    cvec_resize_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_filter_env(
    mut arr: *mut GposSingleSubtable,
    mut fn_0: Option<
        unsafe extern "C" fn(*const GposSingleEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut GposSingleEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if GSS_TYPEINFO.dispose.is_some() {
                GSS_TYPEINFO.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut GposSingleEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn as_cvec(arr: *mut GposSingleSubtable) -> *mut CVecRaw<GposSingleEntry> {
    arr as *mut CVecRaw<GposSingleEntry>
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_init(arr: *mut GposSingleSubtable) {
    cvec_init(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_dispose_item(
    mut arr: *mut GposSingleSubtable,
    mut n: usize,
) {
    if GSS_TYPEINFO.dispose.is_some() {
        GSS_TYPEINFO.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut GposSingleEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_sort(
    mut arr: *mut GposSingleSubtable,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const GposSingleEntry,
            *const GposSingleEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<GposSingleEntry>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const GposSingleEntry,
                    *const GposSingleEntry,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_fill(mut arr: *mut GposSingleSubtable, mut n: usize) {
    while (*arr).length < n {
        let mut x: GposSingleEntry = GposSingleEntry {
            target: Handle {
                state: HandleState::Empty,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            value: PositionValue {
                dx: 0.,
                dy: 0.,
                d_width: 0.,
                d_height: 0.,
            },
        };
        if GSS_TYPEINFO.init.is_some() {
            GSS_TYPEINFO.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<GposSingleEntry>() as usize,
            );
        }
        subtable_gpos_single_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_push(arr: *mut GposSingleSubtable, elem: GposSingleEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_grow_to(arr: *mut GposSingleSubtable, target: usize) {
    cvec_grow_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_pop(arr: *mut GposSingleSubtable) -> GposSingleEntry {
    cvec_pop(as_cvec(arr))
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_copy(
    mut dst: *mut GposSingleSubtable,
    mut src: *const GposSingleSubtable,
) {
    subtable_gpos_single_init(dst);
    subtable_gpos_single_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if GSS_TYPEINFO.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            GSS_TYPEINFO.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut GposSingleEntry,
                (*src).items.offset(j as isize) as *mut GposSingleEntry
                    as *const GposSingleEntry,
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
unsafe extern "C" fn subtable_gpos_single_dispose(mut arr: *mut GposSingleSubtable) {
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
                (*arr).items.offset(j as isize) as *mut GposSingleEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<GposSingleEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_init_cap_n(
    mut arr: *mut GposSingleSubtable,
    mut n: usize,
) {
    subtable_gpos_single_init(arr);
    subtable_gpos_single_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_grow_to_n(arr: *mut GposSingleSubtable, target: usize) {
    cvec_grow_to_n(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_init_n(mut arr: *mut GposSingleSubtable, mut n: usize) {
    subtable_gpos_single_init(arr);
    subtable_gpos_single_grow_to_n(arr, n);
    subtable_gpos_single_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_free(mut x: *mut GposSingleSubtable) {
    if x.is_null() {
        return;
    }
    subtable_gpos_single_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_create_n(mut n: usize) -> *mut GposSingleSubtable {
    let mut t: *mut GposSingleSubtable =
        malloc(::core::mem::size_of::<GposSingleSubtable>() as usize)
            as *mut GposSingleSubtable;
    subtable_gpos_single_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_create() -> *mut GposSingleSubtable {
    let mut x: *mut GposSingleSubtable =
        malloc(::core::mem::size_of::<GposSingleSubtable>() as usize)
            as *mut GposSingleSubtable;
    subtable_gpos_single_init(x);
    return x;
}
pub static I_SUBTABLE_GPOS_SINGLE: GposSingleSubtableVectorInterface = {
    GposSingleSubtableVectorInterface {
        init: Some(
            subtable_gpos_single_init as unsafe extern "C" fn(*mut GposSingleSubtable) -> (),
        ),
        copy: Some(
            subtable_gpos_single_copy
                as unsafe extern "C" fn(
                    *mut GposSingleSubtable,
                    *const GposSingleSubtable,
                ) -> (),
        ),
        dispose: Some(
            subtable_gpos_single_dispose as unsafe extern "C" fn(*mut GposSingleSubtable) -> (),
        ),
        create: Some(subtable_gpos_single_create),
        free: Some(
            subtable_gpos_single_free as unsafe extern "C" fn(*mut GposSingleSubtable) -> (),
        ),
        init_n: Some(
            subtable_gpos_single_init_n
                as unsafe extern "C" fn(*mut GposSingleSubtable, usize) -> (),
        ),
        init_cap_n: Some(
            subtable_gpos_single_init_cap_n
                as unsafe extern "C" fn(*mut GposSingleSubtable, usize) -> (),
        ),
        create_n: Some(
            subtable_gpos_single_create_n
                as unsafe extern "C" fn(usize) -> *mut GposSingleSubtable,
        ),
        fill: Some(
            subtable_gpos_single_fill
                as unsafe extern "C" fn(*mut GposSingleSubtable, usize) -> (),
        ),
        clear: Some(
            subtable_gpos_single_dispose as unsafe extern "C" fn(*mut GposSingleSubtable) -> (),
        ),
        push: Some(
            subtable_gpos_single_push
                as unsafe extern "C" fn(*mut GposSingleSubtable, GposSingleEntry) -> (),
        ),
        shrink_to_fit: Some(
            subtable_gpos_single_shrink_to_fit
                as unsafe extern "C" fn(*mut GposSingleSubtable) -> (),
        ),
        pop: Some(
            subtable_gpos_single_pop
                as unsafe extern "C" fn(*mut GposSingleSubtable) -> GposSingleEntry,
        ),
        dispose_item: Some(
            subtable_gpos_single_dispose_item
                as unsafe extern "C" fn(*mut GposSingleSubtable, usize) -> (),
        ),
        filter_env: Some(
            subtable_gpos_single_filter_env
                as unsafe extern "C" fn(
                    *mut GposSingleSubtable,
                    Option<
                        unsafe extern "C" fn(
                            *const GposSingleEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gpos_single_sort
                as unsafe extern "C" fn(
                    *mut GposSingleSubtable,
                    Option<
                        unsafe extern "C" fn(
                            *const GposSingleEntry,
                            *const GposSingleEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_single_shrink_to_fit(mut arr: *mut GposSingleSubtable) {
    subtable_gpos_single_resize_to(arr, (*arr).length);
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
    let mut subtable: *mut GposSingleSubtable =
        (
            I_SUBTABLE_GPOS_SINGLE
                .create
                .expect("non-null function pointer"))();
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
                    I_SUBTABLE_GPOS_SINGLE
                        .push
                        .expect("non-null function pointer")(
                        subtable,
                        GposSingleEntry {
                            target: otfcc_handle_dup(
                                *(*targets).glyphs.offset(j as isize) as Handle,
                            ) as GlyphHandle,
                            value: v,
                        },
                    );
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
                        I_SUBTABLE_GPOS_SINGLE
                            .push
                            .expect("non-null function pointer")(
                            subtable,
                            GposSingleEntry {
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
                            },
                        );
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
    I_SUBTABLE_GPOS_SINGLE
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gpos_dump_single(
    mut _subtable: *const Subtable,
) -> *mut JsonValue {
    let mut subtable: *const GposSingleSubtable = &raw const (*_subtable).gpos_single;
    let mut st: *mut JsonValue = json_object_new((*subtable).length);
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).length {
        json_object_push(
            st,
            (*(*subtable).items.offset(j as isize)).target.name as *const ::core::ffi::c_char,
            gpos_dump_value((*(*subtable).items.offset(j as isize)).value),
        );
        j = j.wrapping_add(1);
    }
    return st;
}
pub unsafe extern "C" fn otl_gpos_parse_single(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut subtable: *mut GposSingleSubtable =
        (
            I_SUBTABLE_GPOS_SINGLE
                .create
                .expect("non-null function pointer"))();
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
            I_SUBTABLE_GPOS_SINGLE
                .push
                .expect("non-null function pointer")(
                subtable,
                GposSingleEntry {
                    target: handle_from_name(gname)
                        as GlyphHandle,
                    value: gpos_parse_value(
                        (*(*_subtable).u.object.values.offset(j as isize)).value as *mut JsonValue,
                    ),
                },
            );
        }
        j = j.wrapping_add(1);
    }
    return subtable as *mut Subtable;
}
pub unsafe extern "C" fn otfcc_build_gpos_single(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let mut subtable: *const GposSingleSubtable = &raw const (*_subtable).gpos_single;
    let mut is_const: bool = (*subtable).length > 0 as usize;
    let mut format: u16 = 0 as u16;
    if (*subtable).length > 0 as usize {
        let mut j: GlyphId = 0 as GlyphId;
        while (j as usize) < (*subtable).length {
            is_const = is_const as ::core::ffi::c_int != 0
                && (*(*subtable).items.offset(j as isize)).value.dx
                    == (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                        .value
                        .dx
                && (*(*subtable).items.offset(j as isize)).value.dy
                    == (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                        .value
                        .dy
                && (*(*subtable).items.offset(j as isize)).value.d_width
                    == (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                        .value
                        .d_width
                && (*(*subtable).items.offset(j as isize)).value.d_height
                    == (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                        .value
                        .d_height;
            format = (format as ::core::ffi::c_int
                | required_position_format((*(*subtable).items.offset(j as isize)).value)
                    as ::core::ffi::c_int) as u16;
            j = j.wrapping_add(1);
        }
    }
    let mut cov: *mut Coverage = otl_coverage_create();
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).length {
        push_to_coverage(
            cov,
            otfcc_handle_dup(
                (*(*subtable).items.offset(j_0 as isize)).target as Handle,
            ) as GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let mut coverage_buf: *mut Buffer =
        OTL_I_COVERAGE.build.expect("non-null function pointer")(cov);
    if is_const {
        let mut b: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(coverage_buf)), bk_int(BkCellType::B16, (format as ::core::ffi::c_int) as u32), bk_ptr(BkCellType::Embed, bk_gpos_value(
                (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize)).value,
                format,
            ))]);
        otl_coverage_free(cov);
        return bk_build_block(b);
    } else {
        let mut b_0: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 2 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(coverage_buf)), bk_int(BkCellType::B16, (format as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*subtable).length) as u32)]);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as usize) < (*subtable).length {
            bk_push(b_0, &[bk_ptr(BkCellType::Embed, bk_gpos_value((*(*subtable).items.offset(k as isize)).value, format))]);
            k = k.wrapping_add(1);
        }
        otl_coverage_free(cov);
        return bk_build_block(b_0);
    };
}
