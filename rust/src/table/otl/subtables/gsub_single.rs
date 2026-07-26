#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};
unsafe extern "C" {
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new(_: *const ::core::ffi::c_char) -> *mut json_value;
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}


use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage, otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage};
use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_dispose, otfcc_Handle_dup, otfcc_Handle_empty, otfcc_Handle, otfcc_GlyphHandle, HANDLE_STATE_EMPTY};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_string, json_value};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{b16, bk_Block, bk_int, bk_new_Block, bk_ptr, bk_push, p16};

use crate::support::glyph_order::{glyph_handle};
use crate::table::otl::{__caryll_vectorinterface_subtable_gsub_single, otl_GsubSingleEntry, otl_Subtable, subtable_gsub_single};
use crate::table::otl::subtables::otl_BuildHeuristics;
use crate::vendor::uthash::{UT_hash_handle};
use crate::support::{__compar_fn_t};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_GsubSingleEntry {
    pub init: Option<unsafe extern "C" fn(*mut otl_GsubSingleEntry) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_GsubSingleEntry, *const otl_GsubSingleEntry) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_GsubSingleEntry, *mut otl_GsubSingleEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_GsubSingleEntry) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_GsubSingleEntry, otl_GsubSingleEntry) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_GsubSingleEntry, otl_GsubSingleEntry) -> ()>,
}
unsafe extern "C" fn gss_entry_ctor(mut entry: *mut otl_GsubSingleEntry) {
    (*entry).from = otfcc_Handle_empty() as otfcc_GlyphHandle;
    (*entry).to = otfcc_Handle_empty() as otfcc_GlyphHandle;
}
unsafe extern "C" fn gss_entry_copyctor(
    mut dst: *mut otl_GsubSingleEntry,
    mut src: *const otl_GsubSingleEntry,
) {
    (*dst).from = otfcc_Handle_dup((*src).from as otfcc_Handle)
        as otfcc_GlyphHandle;
    (*dst).to = otfcc_Handle_dup((*src).to as otfcc_Handle)
        as otfcc_GlyphHandle;
}
unsafe extern "C" fn gss_entry_dtor(mut entry: *mut otl_GsubSingleEntry) {
    otfcc_Handle_dispose(&raw mut (*entry).from);
    otfcc_Handle_dispose(&raw mut (*entry).to);
}
static gss_typeinfo: __caryll_elementinterface_otl_GsubSingleEntry = {
    __caryll_elementinterface_otl_GsubSingleEntry {
        init: Some(gss_entry_ctor as unsafe extern "C" fn(*mut otl_GsubSingleEntry) -> ()),
        copy: Some(
            gss_entry_copyctor
                as unsafe extern "C" fn(*mut otl_GsubSingleEntry, *const otl_GsubSingleEntry) -> (),
        ),
        move_0: None,
        dispose: Some(gss_entry_dtor as unsafe extern "C" fn(*mut otl_GsubSingleEntry) -> ()),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe fn as_cvec(arr: *mut subtable_gsub_single) -> *mut CVecRaw<otl_GsubSingleEntry> {
    arr as *mut CVecRaw<otl_GsubSingleEntry>
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_init(arr: *mut subtable_gsub_single) {
    cvec_init(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_filterEnv(
    mut arr: *mut subtable_gsub_single,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_GsubSingleEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_GsubSingleEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if gss_typeinfo.dispose.is_some() {
                gss_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_GsubSingleEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[unsafe(no_mangle)]
pub static iSubtable_gsub_single: __caryll_vectorinterface_subtable_gsub_single = {
    __caryll_vectorinterface_subtable_gsub_single {
        init: Some(
            subtable_gsub_single_init as unsafe extern "C" fn(*mut subtable_gsub_single) -> (),
        ),
        copy: Some(
            subtable_gsub_single_copy
                as unsafe extern "C" fn(
                    *mut subtable_gsub_single,
                    *const subtable_gsub_single,
                ) -> (),
        ),
        move_0: Some(
            subtable_gsub_single_move
                as unsafe extern "C" fn(*mut subtable_gsub_single, *mut subtable_gsub_single) -> (),
        ),
        dispose: Some(
            subtable_gsub_single_dispose as unsafe extern "C" fn(*mut subtable_gsub_single) -> (),
        ),
        replace: Some(
            subtable_gsub_single_replace
                as unsafe extern "C" fn(*mut subtable_gsub_single, subtable_gsub_single) -> (),
        ),
        copyReplace: Some(
            subtable_gsub_single_copyReplace
                as unsafe extern "C" fn(*mut subtable_gsub_single, subtable_gsub_single) -> (),
        ),
        create: Some(subtable_gsub_single_create),
        free: Some(
            subtable_gsub_single_free as unsafe extern "C" fn(*mut subtable_gsub_single) -> (),
        ),
        initN: Some(
            subtable_gsub_single_initN
                as unsafe extern "C" fn(*mut subtable_gsub_single, usize) -> (),
        ),
        initCapN: Some(
            subtable_gsub_single_initCapN
                as unsafe extern "C" fn(*mut subtable_gsub_single, usize) -> (),
        ),
        createN: Some(
            subtable_gsub_single_createN
                as unsafe extern "C" fn(usize) -> *mut subtable_gsub_single,
        ),
        fill: Some(
            subtable_gsub_single_fill
                as unsafe extern "C" fn(*mut subtable_gsub_single, usize) -> (),
        ),
        clear: Some(
            subtable_gsub_single_dispose as unsafe extern "C" fn(*mut subtable_gsub_single) -> (),
        ),
        push: Some(
            subtable_gsub_single_push
                as unsafe extern "C" fn(*mut subtable_gsub_single, otl_GsubSingleEntry) -> (),
        ),
        shrinkToFit: Some(
            subtable_gsub_single_shrinkToFit
                as unsafe extern "C" fn(*mut subtable_gsub_single) -> (),
        ),
        pop: Some(
            subtable_gsub_single_pop
                as unsafe extern "C" fn(*mut subtable_gsub_single) -> otl_GsubSingleEntry,
        ),
        disposeItem: Some(
            subtable_gsub_single_disposeItem
                as unsafe extern "C" fn(*mut subtable_gsub_single, usize) -> (),
        ),
        filterEnv: Some(
            subtable_gsub_single_filterEnv
                as unsafe extern "C" fn(
                    *mut subtable_gsub_single,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GsubSingleEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gsub_single_sort
                as unsafe extern "C" fn(
                    *mut subtable_gsub_single,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GsubSingleEntry,
                            *const otl_GsubSingleEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_single_shrinkToFit(mut arr: *mut subtable_gsub_single) {
    subtable_gsub_single_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_resizeTo(arr: *mut subtable_gsub_single, target: usize) {
    cvec_resize_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_disposeItem(
    mut arr: *mut subtable_gsub_single,
    mut n: usize,
) {
    if gss_typeinfo.dispose.is_some() {
        gss_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_GsubSingleEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_sort(
    mut arr: *mut subtable_gsub_single,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otl_GsubSingleEntry,
            *const otl_GsubSingleEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_GsubSingleEntry>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubSingleEntry,
                    *const otl_GsubSingleEntry,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_fill(mut arr: *mut subtable_gsub_single, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_GsubSingleEntry = otl_GsubSingleEntry {
            from: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            to: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
        };
        if gss_typeinfo.init.is_some() {
            gss_typeinfo.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_GsubSingleEntry>() as usize,
            );
        }
        subtable_gsub_single_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_push(arr: *mut subtable_gsub_single, elem: otl_GsubSingleEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_grow(arr: *mut subtable_gsub_single) {
    cvec_grow(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_growTo(arr: *mut subtable_gsub_single, target: usize) {
    cvec_grow_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_copyReplace(
    mut dst: *mut subtable_gsub_single,
    src: subtable_gsub_single,
) {
    subtable_gsub_single_dispose(dst);
    subtable_gsub_single_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_pop(arr: *mut subtable_gsub_single) -> otl_GsubSingleEntry {
    cvec_pop(as_cvec(arr))
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_copy(
    mut dst: *mut subtable_gsub_single,
    mut src: *const subtable_gsub_single,
) {
    subtable_gsub_single_init(dst);
    subtable_gsub_single_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if gss_typeinfo.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            gss_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_GsubSingleEntry,
                (*src).items.offset(j as isize) as *mut otl_GsubSingleEntry
                    as *const otl_GsubSingleEntry,
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
unsafe extern "C" fn subtable_gsub_single_dispose(mut arr: *mut subtable_gsub_single) {
    if arr.is_null() {
        return;
    }
    if gss_typeinfo.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            gss_typeinfo.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_GsubSingleEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_GsubSingleEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_replace(
    mut dst: *mut subtable_gsub_single,
    src: subtable_gsub_single,
) {
    subtable_gsub_single_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gsub_single>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_initCapN(
    mut arr: *mut subtable_gsub_single,
    mut n: usize,
) {
    subtable_gsub_single_init(arr);
    subtable_gsub_single_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_growToN(arr: *mut subtable_gsub_single, target: usize) {
    cvec_grow_to_n(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_initN(mut arr: *mut subtable_gsub_single, mut n: usize) {
    subtable_gsub_single_init(arr);
    subtable_gsub_single_growToN(arr, n);
    subtable_gsub_single_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_free(mut x: *mut subtable_gsub_single) {
    if x.is_null() {
        return;
    }
    subtable_gsub_single_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_createN(mut n: usize) -> *mut subtable_gsub_single {
    let mut t: *mut subtable_gsub_single =
        malloc(::core::mem::size_of::<subtable_gsub_single>() as usize)
            as *mut subtable_gsub_single;
    subtable_gsub_single_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_create() -> *mut subtable_gsub_single {
    let mut x: *mut subtable_gsub_single =
        malloc(::core::mem::size_of::<subtable_gsub_single>() as usize)
            as *mut subtable_gsub_single;
    subtable_gsub_single_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gsub_single_move(
    dst: *mut subtable_gsub_single,
    src: *mut subtable_gsub_single,
) {
    cvec_move(as_cvec(dst), as_cvec(src));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otl_read_gsub_single(
    data: font_file_pointer,
    mut tableLength: u32,
    mut subtableOffset: u32,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut subtableFormat: u16 = 0;
    let mut current_block: u64;
    let mut subtable: *mut subtable_gsub_single =
        (
            iSubtable_gsub_single
                .create
                .expect("non-null function pointer"))();
    let mut from: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    let mut to: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    if !(tableLength < subtableOffset.wrapping_add(6 as u32)) {
        subtableFormat = read_16u(data.offset(subtableOffset as isize) as *const u8);
        from = readCoverage(
            data as *const u8,
            tableLength,
            subtableOffset.wrapping_add(read_16u(
                data.offset(subtableOffset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !(from.is_null() || (*from).numGlyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
            if subtableFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                to = __caryll_allocate_clean(
                    ::core::mem::size_of::<otl_Coverage>() as usize,
                    36 as ::core::ffi::c_ulong,
                ) as *mut otl_Coverage;
                (*to).numGlyphs = (*from).numGlyphs;
                (*to).glyphs = __caryll_allocate_clean(
                    (::core::mem::size_of::<otfcc_GlyphHandle>() as usize)
                        .wrapping_mul((*to).numGlyphs as usize),
                    38 as ::core::ffi::c_ulong,
                ) as *mut otfcc_GlyphHandle;
                let mut delta: u16 = read_16u(
                    data.offset(subtableOffset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *const u8,
                );
                let mut j: glyphid_t = 0 as glyphid_t;
                while (j as ::core::ffi::c_int) < (*from).numGlyphs as ::core::ffi::c_int {
                    *(*to).glyphs.offset(j as isize) = handle_fromIndex(
                        ((*(*from).glyphs.offset(j as isize)).index as ::core::ffi::c_int
                            + delta as ::core::ffi::c_int) as glyphid_t,
                    ) as otfcc_GlyphHandle;
                    j = j.wrapping_add(1);
                }
                current_block = 126606456056746247;
            } else {
                let mut toglyphs: glyphid_t = read_16u(
                    data.offset(subtableOffset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *const u8,
                ) as glyphid_t;
                if tableLength
                    < subtableOffset.wrapping_add(6 as u32).wrapping_add(
                        (toglyphs as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                    )
                    || toglyphs as ::core::ffi::c_int != (*from).numGlyphs as ::core::ffi::c_int
                {
                    current_block = 2938280209257981098;
                } else {
                    to = __caryll_allocate_clean(
                        ::core::mem::size_of::<otl_Coverage>() as usize,
                        48 as ::core::ffi::c_ulong,
                    ) as *mut otl_Coverage;
                    (*to).numGlyphs = toglyphs;
                    (*to).glyphs = __caryll_allocate_clean(
                        (::core::mem::size_of::<otfcc_GlyphHandle>() as usize)
                            .wrapping_mul((*to).numGlyphs as usize),
                        50 as ::core::ffi::c_ulong,
                    ) as *mut otfcc_GlyphHandle;
                    let mut j_0: glyphid_t = 0 as glyphid_t;
                    while (j_0 as ::core::ffi::c_int) < (*to).numGlyphs as ::core::ffi::c_int {
                        *(*to).glyphs.offset(j_0 as isize) =
                            handle_fromIndex(read_16u(
                                data.offset(subtableOffset as isize)
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as glyphid_t) as otfcc_GlyphHandle;
                        j_0 = j_0.wrapping_add(1);
                    }
                    current_block = 126606456056746247;
                }
            }
            match current_block {
                2938280209257981098 => {}
                _ => {
                    let mut j_1: glyphid_t = 0 as glyphid_t;
                    while (j_1 as ::core::ffi::c_int) < (*from).numGlyphs as ::core::ffi::c_int {
                        iSubtable_gsub_single
                            .push
                            .expect("non-null function pointer")(
                            subtable,
                            otl_GsubSingleEntry {
                                from: otfcc_Handle_dup(
                                    *(*from).glyphs.offset(j_1 as isize) as otfcc_Handle,
                                ) as otfcc_GlyphHandle,
                                to: otfcc_Handle_dup(
                                    *(*to).glyphs.offset(j_1 as isize) as otfcc_Handle,
                                ) as otfcc_GlyphHandle,
                            },
                        );
                        j_1 = j_1.wrapping_add(1);
                    }
                    if !from.is_null() {
                        otl_Coverage_free(from);
                    }
                    if !to.is_null() {
                        otl_Coverage_free(to);
                    }
                    return subtable as *mut otl_Subtable;
                }
            }
        }
    }
    iSubtable_gsub_single
        .free
        .expect("non-null function pointer")(subtable);
    if !from.is_null() {
        otl_Coverage_free(from);
    }
    if !to.is_null() {
        otl_Coverage_free(to);
    }
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otl_gsub_dump_single(
    mut _subtable: *const otl_Subtable,
) -> *mut json_value {
    let mut subtable: *const subtable_gsub_single = &raw const (*_subtable).gsub_single;
    let mut st: *mut json_value = json_object_new((*subtable).length);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otl_gsub_parse_single(
    mut _subtable: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut subtable: *mut subtable_gsub_single =
        (
            iSubtable_gsub_single
                .create
                .expect("non-null function pointer"))();
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_uint) < (*_subtable).u.object.length {
        if !(*(*_subtable).u.object.values.offset(j as isize))
            .value
            .is_null()
            && (*(*(*_subtable).u.object.values.offset(j as isize)).value).type_0
                as ::core::ffi::c_uint
                == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut from: glyph_handle =
                handle_fromName(sdsnewlen(
                    (*(*_subtable).u.object.values.offset(j as isize)).name
                        as *const ::core::ffi::c_void,
                    (*(*_subtable).u.object.values.offset(j as isize)).name_length as usize,
                )) as glyph_handle;
            let mut to: glyph_handle =
                handle_fromName(sdsnewlen(
                    (*(*(*_subtable).u.object.values.offset(j as isize)).value)
                        .u
                        .string
                        .ptr as *const ::core::ffi::c_void,
                    (*(*(*_subtable).u.object.values.offset(j as isize)).value)
                        .u
                        .string
                        .length as usize,
                )) as glyph_handle;
            iSubtable_gsub_single
                .push
                .expect("non-null function pointer")(
                subtable,
                otl_GsubSingleEntry {
                    from: from as otfcc_GlyphHandle,
                    to: to as otfcc_GlyphHandle,
                },
            );
        }
        j = j.wrapping_add(1);
    }
    return subtable as *mut otl_Subtable;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_build_gsub_single_subtable(
    mut _subtable: *const otl_Subtable,
    mut heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gsub_single = &raw const (*_subtable).gsub_single;
    let mut isConstantDifference: bool = (*subtable).length > 0 as usize;
    if isConstantDifference {
        let mut difference: i32 = (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
            .to
            .index as i32
            - (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                .from
                .index as i32;
        isConstantDifference = isConstantDifference as ::core::ffi::c_int != 0
            && difference < 0x8000 as i32
            && difference > -(0x8000 as i32);
        let mut j: glyphid_t = 1 as glyphid_t;
        while (j as usize) < (*subtable).length {
            let mut diffJ: i32 = (*(*subtable).items.offset(j as isize)).to.index as i32
                - (*(*subtable).items.offset(j as isize)).from.index as i32;
            isConstantDifference = isConstantDifference as ::core::ffi::c_int != 0
                && diffJ == difference
                && diffJ < 0x8000 as i32
                && diffJ > -(0x8000 as i32);
            j = j.wrapping_add(1);
        }
    }
    let mut cov: *mut otl_Coverage = otl_Coverage_create();
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as usize) < (*subtable).length {
        pushToCoverage(
            cov,
            otfcc_Handle_dup(
                (*(*subtable).items.offset(j_0 as isize)).from as otfcc_Handle,
            ) as otfcc_GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let mut coverageBuf: *mut caryll_Buffer = otl_iCoverage
        .buildFormat
        .expect("non-null function pointer")(
        cov,
        heuristics.contains(otl_BuildHeuristics::GSUB_VERT) as u16,
    );
    if isConstantDifference as ::core::ffi::c_int != 0
        && !heuristics.contains(otl_BuildHeuristics::GSUB_VERT)
    {
        let mut b: *mut bk_Block = bk_new_Block(&[bk_int(b16, 1 as u32), bk_ptr(p16, bk_newBlockFromBuffer(coverageBuf)), bk_int(b16, ((*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                .to
                .index as ::core::ffi::c_int
                - (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                    .from
                    .index as ::core::ffi::c_int) as u32)]);
        otl_Coverage_free(cov);
        return bk_build_Block(b);
    } else {
        let mut b_0: *mut bk_Block = bk_new_Block(&[bk_int(b16, 2 as u32), bk_ptr(p16, bk_newBlockFromBuffer(coverageBuf)), bk_int(b16, ((*subtable).length) as u32)]);
        let mut k: glyphid_t = 0 as glyphid_t;
        while (k as usize) < (*subtable).length {
            bk_push(b_0, &[bk_int(b16, ((*(*subtable).items.offset(k as isize)).to.index as ::core::ffi::c_int) as u32)]);
            k = k.wrapping_add(1);
        }
        otl_Coverage_free(cov);
        return bk_build_Block(b_0);
    };
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsub_single_map_hash {
    pub fromid: ::core::ffi::c_int,
    pub fromname: sds,
    pub toid: ::core::ffi::c_int,
    pub toname: sds,
    pub hh: UT_hash_handle,
}
