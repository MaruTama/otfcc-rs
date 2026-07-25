use libc::{free, malloc, memcpy, memset, qsort};
extern "C" {
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}


use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage, otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage};
use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_dispose, otfcc_Handle_dup, otfcc_Handle, otfcc_GlyphHandle, HANDLE_STATE_EMPTY};

use crate::support::alloc::__caryll_reallocate;
use crate::support::binio::{read_16u};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphid_t, tableid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_array, json_value};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{b16, bk_Block, bkover, p16};

use crate::table::otl::{__caryll_vectorinterface_subtable_gsub_multi, otl_GsubMultiEntry, otl_Subtable, subtable_gsub_multi};
use crate::table::otl::subtables::{otl_BuildHeuristics};
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_GsubMultiEntry {
    pub init: Option<unsafe extern "C" fn(*mut otl_GsubMultiEntry) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_GsubMultiEntry, *const otl_GsubMultiEntry) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_GsubMultiEntry, *mut otl_GsubMultiEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_GsubMultiEntry) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_GsubMultiEntry, otl_GsubMultiEntry) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_GsubMultiEntry, otl_GsubMultiEntry) -> ()>,
}
unsafe extern "C" fn deleteGsubMultiEntry(mut entry: *mut otl_GsubMultiEntry) {
    otfcc_Handle_dispose(&raw mut (*entry).from);
    otl_Coverage_free((*entry).to);
    (*entry).to = ::core::ptr::null_mut::<otl_Coverage>();
}
static mut gsm_typeinfo: __caryll_elementinterface_otl_GsubMultiEntry = {
    __caryll_elementinterface_otl_GsubMultiEntry {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(deleteGsubMultiEntry as unsafe extern "C" fn(*mut otl_GsubMultiEntry) -> ()),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe fn as_cvec(arr: *mut subtable_gsub_multi) -> *mut CVecRaw<otl_GsubMultiEntry> {
    arr as *mut CVecRaw<otl_GsubMultiEntry>
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_growTo(arr: *mut subtable_gsub_multi, target: usize) {
    cvec_grow_to(as_cvec(arr), target);
}
#[no_mangle]
pub static mut iSubtable_gsub_multi: __caryll_vectorinterface_subtable_gsub_multi = {
    __caryll_vectorinterface_subtable_gsub_multi {
        init: Some(
            subtable_gsub_multi_init as unsafe extern "C" fn(*mut subtable_gsub_multi) -> (),
        ),
        copy: Some(
            subtable_gsub_multi_copy
                as unsafe extern "C" fn(*mut subtable_gsub_multi, *const subtable_gsub_multi) -> (),
        ),
        move_0: Some(
            subtable_gsub_multi_move
                as unsafe extern "C" fn(*mut subtable_gsub_multi, *mut subtable_gsub_multi) -> (),
        ),
        dispose: Some(
            subtable_gsub_multi_dispose as unsafe extern "C" fn(*mut subtable_gsub_multi) -> (),
        ),
        replace: Some(
            subtable_gsub_multi_replace
                as unsafe extern "C" fn(*mut subtable_gsub_multi, subtable_gsub_multi) -> (),
        ),
        copyReplace: Some(
            subtable_gsub_multi_copyReplace
                as unsafe extern "C" fn(*mut subtable_gsub_multi, subtable_gsub_multi) -> (),
        ),
        create: Some(subtable_gsub_multi_create),
        free: Some(
            subtable_gsub_multi_free as unsafe extern "C" fn(*mut subtable_gsub_multi) -> (),
        ),
        initN: Some(
            subtable_gsub_multi_initN
                as unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> (),
        ),
        initCapN: Some(
            subtable_gsub_multi_initCapN
                as unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> (),
        ),
        createN: Some(
            subtable_gsub_multi_createN as unsafe extern "C" fn(usize) -> *mut subtable_gsub_multi,
        ),
        fill: Some(
            subtable_gsub_multi_fill
                as unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> (),
        ),
        clear: Some(
            subtable_gsub_multi_dispose as unsafe extern "C" fn(*mut subtable_gsub_multi) -> (),
        ),
        push: Some(
            subtable_gsub_multi_push
                as unsafe extern "C" fn(*mut subtable_gsub_multi, otl_GsubMultiEntry) -> (),
        ),
        shrinkToFit: Some(
            subtable_gsub_multi_shrinkToFit as unsafe extern "C" fn(*mut subtable_gsub_multi) -> (),
        ),
        pop: Some(
            subtable_gsub_multi_pop
                as unsafe extern "C" fn(*mut subtable_gsub_multi) -> otl_GsubMultiEntry,
        ),
        disposeItem: Some(
            subtable_gsub_multi_disposeItem
                as unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> (),
        ),
        filterEnv: Some(
            subtable_gsub_multi_filterEnv
                as unsafe extern "C" fn(
                    *mut subtable_gsub_multi,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GsubMultiEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gsub_multi_sort
                as unsafe extern "C" fn(
                    *mut subtable_gsub_multi,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GsubMultiEntry,
                            *const otl_GsubMultiEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_multi_shrinkToFit(mut arr: *mut subtable_gsub_multi) {
    subtable_gsub_multi_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_resizeTo(arr: *mut subtable_gsub_multi, target: usize) {
    cvec_resize_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_move(
    dst: *mut subtable_gsub_multi,
    src: *mut subtable_gsub_multi,
) {
    cvec_move(as_cvec(dst), as_cvec(src));
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_disposeItem(
    mut arr: *mut subtable_gsub_multi,
    mut n: usize,
) {
    if gsm_typeinfo.dispose.is_some() {
        gsm_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_GsubMultiEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_sort(
    mut arr: *mut subtable_gsub_multi,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otl_GsubMultiEntry,
            *const otl_GsubMultiEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_GsubMultiEntry>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubMultiEntry,
                    *const otl_GsubMultiEntry,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_fill(mut arr: *mut subtable_gsub_multi, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_GsubMultiEntry = otl_GsubMultiEntry {
            from: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            to: ::core::ptr::null_mut::<otl_Coverage>(),
        };
        if gsm_typeinfo.init.is_some() {
            gsm_typeinfo.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_GsubMultiEntry>() as usize,
            );
        }
        subtable_gsub_multi_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_push(arr: *mut subtable_gsub_multi, elem: otl_GsubMultiEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_grow(arr: *mut subtable_gsub_multi) {
    cvec_grow(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_init(arr: *mut subtable_gsub_multi) {
    cvec_init(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_copyReplace(
    mut dst: *mut subtable_gsub_multi,
    src: subtable_gsub_multi,
) {
    subtable_gsub_multi_dispose(dst);
    subtable_gsub_multi_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_pop(arr: *mut subtable_gsub_multi) -> otl_GsubMultiEntry {
    cvec_pop(as_cvec(arr))
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_copy(
    mut dst: *mut subtable_gsub_multi,
    mut src: *const subtable_gsub_multi,
) {
    subtable_gsub_multi_init(dst);
    subtable_gsub_multi_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if gsm_typeinfo.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            gsm_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_GsubMultiEntry,
                (*src).items.offset(j as isize) as *mut otl_GsubMultiEntry
                    as *const otl_GsubMultiEntry,
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
unsafe extern "C" fn subtable_gsub_multi_dispose(mut arr: *mut subtable_gsub_multi) {
    if arr.is_null() {
        return;
    }
    if gsm_typeinfo.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            gsm_typeinfo.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_GsubMultiEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_GsubMultiEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_replace(
    mut dst: *mut subtable_gsub_multi,
    src: subtable_gsub_multi,
) {
    subtable_gsub_multi_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gsub_multi>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_initCapN(
    mut arr: *mut subtable_gsub_multi,
    mut n: usize,
) {
    subtable_gsub_multi_init(arr);
    subtable_gsub_multi_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_growToN(arr: *mut subtable_gsub_multi, target: usize) {
    cvec_grow_to_n(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_initN(mut arr: *mut subtable_gsub_multi, mut n: usize) {
    subtable_gsub_multi_init(arr);
    subtable_gsub_multi_growToN(arr, n);
    subtable_gsub_multi_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_free(mut x: *mut subtable_gsub_multi) {
    if x.is_null() {
        return;
    }
    subtable_gsub_multi_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_createN(mut n: usize) -> *mut subtable_gsub_multi {
    let mut t: *mut subtable_gsub_multi =
        malloc(::core::mem::size_of::<subtable_gsub_multi>() as usize) as *mut subtable_gsub_multi;
    subtable_gsub_multi_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_create() -> *mut subtable_gsub_multi {
    let mut x: *mut subtable_gsub_multi =
        malloc(::core::mem::size_of::<subtable_gsub_multi>() as usize) as *mut subtable_gsub_multi;
    subtable_gsub_multi_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_filterEnv(
    mut arr: *mut subtable_gsub_multi,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_GsubMultiEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_GsubMultiEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if gsm_typeinfo.dispose.is_some() {
                gsm_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_GsubMultiEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_gsub_multi(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut seqCount: glyphid_t = 0;
    let subtable: *mut subtable_gsub_multi =
        (
            iSubtable_gsub_multi
                .create
                .expect("non-null function pointer"))();
    let mut from: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    if !(tableLength < offset.wrapping_add(6 as u32)) {
        from = readCoverage(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        seqCount = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as glyphid_t;
        if seqCount as ::core::ffi::c_int == (*from).numGlyphs as ::core::ffi::c_int {
            if !(tableLength
                < offset.wrapping_add(6 as u32).wrapping_add(
                    (seqCount as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                ))
            {
                for j in 0..seqCount {
                    let seqOffset: u32 = offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            as *const u8,
                    )
                        as u32);
                    let cov: *mut otl_Coverage =
                        otl_Coverage_create();
                    let n: glyphid_t =
                        read_16u(data.offset(seqOffset as isize) as *const u8) as glyphid_t;
                    for k in 0..n {
                        pushToCoverage(
                            cov,
                            handle_fromIndex(read_16u(
                                data.offset(seqOffset as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as glyphid_t) as otfcc_GlyphHandle,
                        );
                    }
                    iSubtable_gsub_multi
                        .push
                        .expect("non-null function pointer")(
                        subtable,
                        otl_GsubMultiEntry {
                            from: otfcc_Handle_dup(
                                *(*from).glyphs.offset(j as isize) as otfcc_Handle,
                            ) as otfcc_GlyphHandle,
                            to: cov,
                        },
                    );
                }
                otl_Coverage_free(from);
                return subtable as *mut otl_Subtable;
            }
        }
    }
    if !from.is_null() {
        otl_Coverage_free(from);
    }
    iSubtable_gsub_multi
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_dump_multi(
    mut _subtable: *const otl_Subtable,
) -> *mut json_value {
    let subtable: *const subtable_gsub_multi = &raw const (*_subtable).gsub_multi;
    let st: *mut json_value = json_object_new((*subtable).length);
    for j in 0..(*subtable).length as glyphid_t {
        let entry = (*subtable).items.offset(j as isize);
        json_object_push(
            st,
            (*entry).from.name as *const ::core::ffi::c_char,
            otl_iCoverage.dump.expect("non-null function pointer")((*entry).to),
        );
    }
    return st;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_parse_multi(
    mut _subtable: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let st: *mut subtable_gsub_multi =
        (
            iSubtable_gsub_multi
                .create
                .expect("non-null function pointer"))();
    for k in 0..(*_subtable).u.object.length as glyphid_t {
        let entry = (*_subtable).u.object.values.offset(k as isize);
        let _to: *mut json_value = (*entry).value as *mut json_value;
        if !_to.is_null() && (*_to).type_0 == json_array {
            iSubtable_gsub_multi
                .push
                .expect("non-null function pointer")(
                st,
                otl_GsubMultiEntry {
                    from: handle_fromName(sdsnewlen(
                        (*entry).name as *const ::core::ffi::c_void,
                        (*entry).name_length as usize,
                    )) as otfcc_GlyphHandle,
                    to: otl_iCoverage.parse.expect("non-null function pointer")(_to),
                },
            );
        }
    }
    return st as *mut otl_Subtable;
}
unsafe extern "C" fn buildGsubMultiSubtableRange(
    subtable: *const subtable_gsub_multi,
    start: glyphid_t,
    end: glyphid_t,
) -> *mut caryll_Buffer {
    let cov: *mut otl_Coverage = otl_Coverage_create();
    for j in start..end {
        pushToCoverage(
            cov,
            otfcc_Handle_dup(
                (*(*subtable).items.offset(j as isize)).from as otfcc_Handle,
            ) as otfcc_GlyphHandle,
        );
    }
    let root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(cov)),
        b16 as ::core::ffi::c_int,
        end as ::core::ffi::c_int - start as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    for j_0 in start..end {
        let to = (*(*subtable).items.offset(j_0 as isize)).to;
        let b: *mut bk_Block = bk_new_Block(
            b16 as ::core::ffi::c_int,
            (*to).numGlyphs as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
        for k in 0..(*to).numGlyphs {
            bk_push(
                b,
                b16 as ::core::ffi::c_int,
                (*(*to).glyphs.offset(k as isize)).index as ::core::ffi::c_int,
                bkover as ::core::ffi::c_int,
            );
        }
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            b,
            bkover as ::core::ffi::c_int,
        );
    }
    otl_Coverage_free(cov);
    return bk_build_Block(root);
}
pub const GSUB_MULTI_SUBTABLE_SIZE_LIMIT: ::core::ffi::c_int = 0xff00 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gsub_multi_subtable_split(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
    mut count: *mut tableid_t,
) -> *mut *mut caryll_Buffer {
    let mut subtable: *const subtable_gsub_multi = &raw const (*_subtable).gsub_multi;
    let mut parts: *mut *mut caryll_Buffer = ::core::ptr::null_mut::<*mut caryll_Buffer>();
    let mut nParts: tableid_t = 0 as tableid_t;
    let mut start: glyphid_t = 0 as glyphid_t;
    while (start as usize) < (*subtable).length {
        let mut size: usize = (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize;
        let mut end: glyphid_t = start;
        while (end as usize) < (*subtable).length {
            let mut entrySize: usize = ((2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize)
                .wrapping_add(
                    ((*(*(*subtable).items.offset(end as isize)).to).numGlyphs as usize)
                        .wrapping_mul(2 as usize),
                );
            if end as ::core::ffi::c_int > start as ::core::ffi::c_int
                && size.wrapping_add(entrySize) > GSUB_MULTI_SUBTABLE_SIZE_LIMIT as usize
            {
                break;
            }
            size = size.wrapping_add(entrySize);
            end = end.wrapping_add(1);
        }
        parts = __caryll_reallocate(
            parts as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut caryll_Buffer>() as usize)
                .wrapping_mul((nParts as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize),
            125 as ::core::ffi::c_ulong,
        ) as *mut *mut caryll_Buffer;
        let ref mut fresh2 = *parts.offset(nParts as isize);
        *fresh2 = buildGsubMultiSubtableRange(subtable, start, end);
        nParts = nParts.wrapping_add(1);
        start = end;
    }
    if nParts == 0 {
        parts = __caryll_reallocate(
            parts as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut caryll_Buffer>() as usize).wrapping_mul(1 as usize),
            132 as ::core::ffi::c_ulong,
        ) as *mut *mut caryll_Buffer;
        let ref mut fresh3 = *parts.offset(0 as ::core::ffi::c_int as isize);
        *fresh3 = buildGsubMultiSubtableRange(subtable, 0 as glyphid_t, 0 as glyphid_t);
        nParts = 1 as tableid_t;
    }
    *count = nParts;
    return parts;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gsub_multi_subtable(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gsub_multi = &raw const (*_subtable).gsub_multi;
    return buildGsubMultiSubtableRange(subtable, 0 as glyphid_t, (*subtable).length as glyphid_t);
}
