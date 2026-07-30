#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memcpy, memset, qsort};


use crate::support::json_funcs::{json_obj_get_type, preserialize};
use crate::table::otl::coverage::{Coverage, otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage};
use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_dispose, Handle, GlyphHandle, HandleState};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_Block, bk_ptr, bk_push};
use crate::support::{NULL, ComparFn};
use crate::table::otl::{GsubLigatureSubtableVectorInterface, GsubLigatureEntry, Subtable, GsubLigatureSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::bk::bkblock::{bk_newBlockFromBuffer};
use crate::bk::bkgraph::{bk_build_Block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_object_new, json_object_push, json_string_new_length};
use crate::vendor::sds::{sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubLigatureEntryElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GsubLigatureEntry) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut GsubLigatureEntry, *const GsubLigatureEntry) -> (),
    >,
    pub move_0:
        Option<unsafe extern "C" fn(*mut GsubLigatureEntry, *mut GsubLigatureEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GsubLigatureEntry) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut GsubLigatureEntry, GsubLigatureEntry) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut GsubLigatureEntry, GsubLigatureEntry) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LigatureAggregator {
    pub gid: ::core::ffi::c_int,
    pub hh: UtHashHandle,
}
#[inline]
unsafe extern "C" fn sdslen(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
unsafe extern "C" fn deleteGsubLigatureEntry(mut entry: *mut GsubLigatureEntry) {
    otfcc_Handle_dispose(&raw mut (*entry).to);
    otl_Coverage_free((*entry).from);
    (*entry).from = ::core::ptr::null_mut::<Coverage>();
}
static GSS_TYPEINFO: GsubLigatureEntryElementInterface = {
    GsubLigatureEntryElementInterface {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(
            deleteGsubLigatureEntry as unsafe extern "C" fn(*mut GsubLigatureEntry) -> (),
        ),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_move(dst: *mut GsubLigatureSubtable, src: *mut GsubLigatureSubtable) {
    cvec_move(as_cvec(dst), as_cvec(src));
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_growTo(arr: *mut GsubLigatureSubtable, target: usize) {
    cvec_grow_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_free(mut x: *mut GsubLigatureSubtable) {
    if x.is_null() {
        return;
    }
    subtable_gsub_ligature_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_pop(arr: *mut GsubLigatureSubtable) -> GsubLigatureEntry {
    cvec_pop(as_cvec(arr))
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_copyReplace(
    mut dst: *mut GsubLigatureSubtable,
    src: GsubLigatureSubtable,
) {
    subtable_gsub_ligature_dispose(dst);
    subtable_gsub_ligature_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_copy(
    mut dst: *mut GsubLigatureSubtable,
    mut src: *const GsubLigatureSubtable,
) {
    subtable_gsub_ligature_init(dst);
    subtable_gsub_ligature_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if GSS_TYPEINFO.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            GSS_TYPEINFO.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut GsubLigatureEntry,
                (*src).items.offset(j as isize) as *mut GsubLigatureEntry
                    as *const GsubLigatureEntry,
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
unsafe extern "C" fn subtable_gsub_ligature_dispose(mut arr: *mut GsubLigatureSubtable) {
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
                (*arr).items.offset(j as isize) as *mut GsubLigatureEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<GsubLigatureEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_replace(
    mut dst: *mut GsubLigatureSubtable,
    src: GsubLigatureSubtable,
) {
    subtable_gsub_ligature_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GsubLigatureSubtable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_initCapN(
    mut arr: *mut GsubLigatureSubtable,
    mut n: usize,
) {
    subtable_gsub_ligature_init(arr);
    subtable_gsub_ligature_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_growToN(arr: *mut GsubLigatureSubtable, target: usize) {
    cvec_grow_to_n(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_initN(
    mut arr: *mut GsubLigatureSubtable,
    mut n: usize,
) {
    subtable_gsub_ligature_init(arr);
    subtable_gsub_ligature_growToN(arr, n);
    subtable_gsub_ligature_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_create() -> *mut GsubLigatureSubtable {
    let mut x: *mut GsubLigatureSubtable =
        malloc(::core::mem::size_of::<GsubLigatureSubtable>() as usize)
            as *mut GsubLigatureSubtable;
    subtable_gsub_ligature_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_createN(mut n: usize) -> *mut GsubLigatureSubtable {
    let mut t: *mut GsubLigatureSubtable =
        malloc(::core::mem::size_of::<GsubLigatureSubtable>() as usize)
            as *mut GsubLigatureSubtable;
    subtable_gsub_ligature_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_filterEnv(
    mut arr: *mut GsubLigatureSubtable,
    mut fn_0: Option<
        unsafe extern "C" fn(*const GsubLigatureEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut GsubLigatureEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if GSS_TYPEINFO.dispose.is_some() {
                GSS_TYPEINFO.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut GsubLigatureEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn as_cvec(arr: *mut GsubLigatureSubtable) -> *mut CVecRaw<GsubLigatureEntry> {
    arr as *mut CVecRaw<GsubLigatureEntry>
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_init(arr: *mut GsubLigatureSubtable) {
    cvec_init(as_cvec(arr));
}
pub static I_SUBTABLE_GSUB_LIGATURE: GsubLigatureSubtableVectorInterface = {
    GsubLigatureSubtableVectorInterface {
        init: Some(
            subtable_gsub_ligature_init as unsafe extern "C" fn(*mut GsubLigatureSubtable) -> (),
        ),
        copy: Some(
            subtable_gsub_ligature_copy
                as unsafe extern "C" fn(
                    *mut GsubLigatureSubtable,
                    *const GsubLigatureSubtable,
                ) -> (),
        ),
        move_0: Some(
            subtable_gsub_ligature_move
                as unsafe extern "C" fn(
                    *mut GsubLigatureSubtable,
                    *mut GsubLigatureSubtable,
                ) -> (),
        ),
        dispose: Some(
            subtable_gsub_ligature_dispose
                as unsafe extern "C" fn(*mut GsubLigatureSubtable) -> (),
        ),
        replace: Some(
            subtable_gsub_ligature_replace
                as unsafe extern "C" fn(*mut GsubLigatureSubtable, GsubLigatureSubtable) -> (),
        ),
        copyReplace: Some(
            subtable_gsub_ligature_copyReplace
                as unsafe extern "C" fn(*mut GsubLigatureSubtable, GsubLigatureSubtable) -> (),
        ),
        create: Some(subtable_gsub_ligature_create),
        free: Some(
            subtable_gsub_ligature_free as unsafe extern "C" fn(*mut GsubLigatureSubtable) -> (),
        ),
        initN: Some(
            subtable_gsub_ligature_initN
                as unsafe extern "C" fn(*mut GsubLigatureSubtable, usize) -> (),
        ),
        initCapN: Some(
            subtable_gsub_ligature_initCapN
                as unsafe extern "C" fn(*mut GsubLigatureSubtable, usize) -> (),
        ),
        createN: Some(
            subtable_gsub_ligature_createN
                as unsafe extern "C" fn(usize) -> *mut GsubLigatureSubtable,
        ),
        fill: Some(
            subtable_gsub_ligature_fill
                as unsafe extern "C" fn(*mut GsubLigatureSubtable, usize) -> (),
        ),
        clear: Some(
            subtable_gsub_ligature_dispose
                as unsafe extern "C" fn(*mut GsubLigatureSubtable) -> (),
        ),
        push: Some(
            subtable_gsub_ligature_push
                as unsafe extern "C" fn(*mut GsubLigatureSubtable, GsubLigatureEntry) -> (),
        ),
        shrinkToFit: Some(
            subtable_gsub_ligature_shrinkToFit
                as unsafe extern "C" fn(*mut GsubLigatureSubtable) -> (),
        ),
        pop: Some(
            subtable_gsub_ligature_pop
                as unsafe extern "C" fn(*mut GsubLigatureSubtable) -> GsubLigatureEntry,
        ),
        disposeItem: Some(
            subtable_gsub_ligature_disposeItem
                as unsafe extern "C" fn(*mut GsubLigatureSubtable, usize) -> (),
        ),
        filterEnv: Some(
            subtable_gsub_ligature_filterEnv
                as unsafe extern "C" fn(
                    *mut GsubLigatureSubtable,
                    Option<
                        unsafe extern "C" fn(
                            *const GsubLigatureEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gsub_ligature_sort
                as unsafe extern "C" fn(
                    *mut GsubLigatureSubtable,
                    Option<
                        unsafe extern "C" fn(
                            *const GsubLigatureEntry,
                            *const GsubLigatureEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_resizeTo(arr: *mut GsubLigatureSubtable, target: usize) {
    cvec_resize_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_shrinkToFit(mut arr: *mut GsubLigatureSubtable) {
    subtable_gsub_ligature_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_disposeItem(
    mut arr: *mut GsubLigatureSubtable,
    mut n: usize,
) {
    if GSS_TYPEINFO.dispose.is_some() {
        GSS_TYPEINFO.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut GsubLigatureEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_sort(
    mut arr: *mut GsubLigatureSubtable,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const GsubLigatureEntry,
            *const GsubLigatureEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<GsubLigatureEntry>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const GsubLigatureEntry,
                    *const GsubLigatureEntry,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_fill(
    mut arr: *mut GsubLigatureSubtable,
    mut n: usize,
) {
    while (*arr).length < n {
        let mut x: GsubLigatureEntry = GsubLigatureEntry {
            from: ::core::ptr::null_mut::<Coverage>(),
            to: Handle {
                state: HandleState::Empty,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
        };
        if GSS_TYPEINFO.init.is_some() {
            GSS_TYPEINFO.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<GsubLigatureEntry>() as usize,
            );
        }
        subtable_gsub_ligature_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_push(arr: *mut GsubLigatureSubtable, elem: GsubLigatureEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_grow(arr: *mut GsubLigatureSubtable) {
    cvec_grow(as_cvec(arr));
}
pub unsafe extern "C" fn otl_read_gsub_ligature(
    data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
    _maxGlyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut startCoverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut setCount: GlyphId = 0;
    let mut ligatureCount: u32 = 0;
    let mut current_block: u64;
    let mut subtable: *mut GsubLigatureSubtable =
        (
            I_SUBTABLE_GSUB_LIGATURE
                .create
                .expect("non-null function pointer"))();
    if !(tableLength < offset.wrapping_add(6 as u32)) {
        startCoverage = readCoverage(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !startCoverage.is_null() {
            setCount = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as GlyphId;
            if !(setCount as ::core::ffi::c_int != (*startCoverage).numGlyphs as ::core::ffi::c_int)
            {
                if !(tableLength
                    < offset.wrapping_add(6 as u32).wrapping_add(
                        (setCount as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                    ))
                {
                    ligatureCount = 0 as u32;
                    let mut j: GlyphId = 0 as GlyphId;
                    loop {
                        if !((j as ::core::ffi::c_int) < setCount as ::core::ffi::c_int) {
                            current_block = 17860125682698302841;
                            break;
                        }
                        let mut setOffset: u32 = offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        )
                            as u32);
                        if tableLength < setOffset.wrapping_add(2 as u32) {
                            current_block = 3443835632518673764;
                            break;
                        }
                        ligatureCount = ligatureCount.wrapping_add(read_16u(
                            data.offset(setOffset as isize) as *const u8,
                        )
                            as u32);
                        if tableLength
                            < setOffset.wrapping_add(2 as u32).wrapping_add(
                                (read_16u(data.offset(setOffset as isize) as *const u8)
                                    as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int)
                                    as u32,
                            )
                        {
                            current_block = 3443835632518673764;
                            break;
                        }
                        j = j.wrapping_add(1);
                    }
                    match current_block {
                        3443835632518673764 => {}
                        _ => {
                            let mut j_0: GlyphId = 0 as GlyphId;
                            's_77: loop {
                                if !((j_0 as ::core::ffi::c_int) < setCount as ::core::ffi::c_int) {
                                    current_block = 11932355480408055363;
                                    break;
                                }
                                let mut setOffset_0: u32 = offset.wrapping_add(read_16u(
                                    data.offset(offset as isize)
                                        .offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let mut lc: GlyphId =
                                    read_16u(data.offset(setOffset_0 as isize) as *const u8)
                                        as GlyphId;
                                let mut k: GlyphId = 0 as GlyphId;
                                while (k as ::core::ffi::c_int) < lc as ::core::ffi::c_int {
                                    let mut ligOffset: u32 = setOffset_0.wrapping_add(
                                        read_16u(
                                            data.offset(setOffset_0 as isize)
                                                .offset(2 as ::core::ffi::c_int as isize)
                                                .offset(
                                                    (k as ::core::ffi::c_int
                                                        * 2 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                as *const u8,
                                        ) as u32,
                                    );
                                    if tableLength < ligOffset.wrapping_add(4 as u32) {
                                        current_block = 3443835632518673764;
                                        break 's_77;
                                    }
                                    let mut ligComponents: GlyphId = read_16u(
                                        data.offset(ligOffset as isize)
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            as *const u8,
                                    )
                                        as GlyphId;
                                    if tableLength
                                        < ligOffset.wrapping_add(2 as u32).wrapping_add(
                                            (ligComponents as ::core::ffi::c_int
                                                * 2 as ::core::ffi::c_int)
                                                as u32,
                                        )
                                    {
                                        current_block = 3443835632518673764;
                                        break 's_77;
                                    }
                                    let mut cov: *mut Coverage =
                                        otl_Coverage_create();
                                    pushToCoverage(
                                        cov,
                                        handle_fromIndex(
                                            (*(*startCoverage).glyphs.offset(j_0 as isize)).index,
                                        )
                                            as GlyphHandle,
                                    );
                                    let mut m: GlyphId = 1 as GlyphId;
                                    while (m as ::core::ffi::c_int)
                                        < ligComponents as ::core::ffi::c_int
                                    {
                                        pushToCoverage(
                                            cov,
                                            handle_fromIndex(
                                                read_16u(
                                                    data.offset(ligOffset as isize)
                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                        .offset(
                                                            (m as ::core::ffi::c_int
                                                                * 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        as *const u8,
                                                )
                                                    as GlyphId,
                                            )
                                                as GlyphHandle,
                                        );
                                        m = m.wrapping_add(1);
                                    }
                                    I_SUBTABLE_GSUB_LIGATURE
                                        .push
                                        .expect("non-null function pointer")(
                                        subtable,
                                        GsubLigatureEntry {
                                            from: cov,
                                            to: handle_fromIndex(
                                                read_16u(data.offset(ligOffset as isize)
                                                    as *const u8)
                                                    as GlyphId,
                                            )
                                                as GlyphHandle,
                                        },
                                    );
                                    k = k.wrapping_add(1);
                                }
                                j_0 = j_0.wrapping_add(1);
                            }
                            match current_block {
                                3443835632518673764 => {}
                                _ => {
                                    otl_Coverage_free(
                                        startCoverage,
                                    );
                                    return subtable as *mut Subtable;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    I_SUBTABLE_GSUB_LIGATURE
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gsub_dump_ligature(
    mut _subtable: *const Subtable,
) -> *mut JsonValue {
    let mut subtable: *const GsubLigatureSubtable = &raw const (*_subtable).gsub_ligature;
    let mut st: *mut JsonValue = json_array_new((*subtable).length);
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).length {
        let mut entry: *mut JsonValue = json_object_new(2 as usize);
        json_object_push(
            entry,
            b"from\0" as *const u8 as *const ::core::ffi::c_char,
            OTL_I_COVERAGE.dump.expect("non-null function pointer")(
                (*(*subtable).items.offset(j as isize)).from,
            ),
        );
        json_object_push(
            entry,
            b"to\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new_length(
                sdslen((*(*subtable).items.offset(j as isize)).to.name) as ::core::ffi::c_uint,
                (*(*subtable).items.offset(j as isize)).to.name as *const ::core::ffi::c_char,
            ),
        );
        json_array_push(st, preserialize(entry));
        j = j.wrapping_add(1);
    }
    let mut ret: *mut JsonValue = json_object_new(1 as usize);
    json_object_push(
        ret,
        b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
        st,
    );
    return ret;
}
pub unsafe extern "C" fn otl_gsub_parse_ligature(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    if !json_obj_get_type(
        _subtable,
        b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    )
    .is_null()
    {
        _subtable = json_obj_get_type(
            _subtable,
            b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Array,
        );
        let mut st: *mut GsubLigatureSubtable =
            (
                I_SUBTABLE_GSUB_LIGATURE
                    .create
                    .expect("non-null function pointer"))();
        let mut n: GlyphId = (*_subtable).u.array.length as GlyphId;
        let mut k: GlyphId = 0 as GlyphId;
        while (k as ::core::ffi::c_int) < n as ::core::ffi::c_int {
            let mut entry: *mut JsonValue =
                *(*_subtable).u.array.values.offset(k as isize) as *mut JsonValue;
            let mut _from: *mut JsonValue = json_obj_get_type(
                entry,
                b"from\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            );
            let mut _to: *mut JsonValue = json_obj_get_type(
                entry,
                b"to\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            if !(_from.is_null() || _to.is_null()) {
                I_SUBTABLE_GSUB_LIGATURE
                    .push
                    .expect("non-null function pointer")(
                    st,
                    GsubLigatureEntry {
                        from: OTL_I_COVERAGE.parse.expect("non-null function pointer")(_from),
                        to: handle_fromName(sdsnewlen(
                            (*_to).u.string.ptr as *const ::core::ffi::c_void,
                            (*_to).u.string.length as usize,
                        )) as GlyphHandle,
                    },
                );
            }
            k = k.wrapping_add(1);
        }
        return st as *mut Subtable;
    } else {
        let mut st_0: *mut GsubLigatureSubtable =
            (
                I_SUBTABLE_GSUB_LIGATURE
                    .create
                    .expect("non-null function pointer"))();
        let mut n_0: GlyphId = (*_subtable).u.array.length as GlyphId;
        let mut k_0: GlyphId = 0 as GlyphId;
        while (k_0 as ::core::ffi::c_int) < n_0 as ::core::ffi::c_int {
            let mut _from_0: *mut JsonValue =
                (*(*_subtable).u.object.values.offset(k_0 as isize)).value as *mut JsonValue;
            if !(_from_0.is_null()
                || (*_from_0).type_0 != JsonType::Array)
            {
                I_SUBTABLE_GSUB_LIGATURE
                    .push
                    .expect("non-null function pointer")(
                    st_0,
                    GsubLigatureEntry {
                        from: OTL_I_COVERAGE.parse.expect("non-null function pointer")(_from_0),
                        to: handle_fromName(sdsnewlen(
                            (*(*_subtable).u.object.values.offset(k_0 as isize)).name
                                as *const ::core::ffi::c_void,
                            (*(*_subtable).u.object.values.offset(k_0 as isize)).name_length
                                as usize,
                        )) as GlyphHandle,
                    },
                );
            }
            k_0 = k_0.wrapping_add(1);
        }
        return st_0 as *mut Subtable;
    };
}
unsafe extern "C" fn by_gid(
    mut a: *mut LigatureAggregator,
    mut b: *mut LigatureAggregator,
) -> ::core::ffi::c_int {
    return (*a).gid - (*b).gid;
}
pub unsafe extern "C" fn otfcc_build_gsub_ligature_subtable(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let mut subtable: *const GsubLigatureSubtable = &raw const (*_subtable).gsub_ligature;
    let mut h: *mut LigatureAggregator = ::core::ptr::null_mut::<LigatureAggregator>();
    let mut s: *mut LigatureAggregator = ::core::ptr::null_mut::<LigatureAggregator>();
    let mut tmp: *mut LigatureAggregator = ::core::ptr::null_mut::<LigatureAggregator>();
    let mut nLigatures: GlyphId = (*subtable).length as GlyphId;
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_int) < nLigatures as ::core::ffi::c_int {
        let mut sgid: ::core::ffi::c_int = (*(*(*(*subtable).items.offset(j as isize)).from)
            .glyphs
            .offset(0 as ::core::ffi::c_int as isize))
        .index as ::core::ffi::c_int;
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut sgid as *const ::core::ffi::c_uchar;
        _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
        while _hj_k >= 12 as ::core::ffi::c_uint {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
            _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _hf_hashv = _hf_hashv
            .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
        let mut current_block_50: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 483194043190260627;
            }
            10 => {
                current_block_50 = 483194043190260627;
            }
            9 => {
                current_block_50 = 9392992341002218192;
            }
            8 => {
                current_block_50 = 14840068175916424037;
            }
            7 => {
                current_block_50 = 2003362535987825465;
            }
            6 => {
                current_block_50 = 7629293359809983242;
            }
            5 => {
                current_block_50 = 11376947495104746217;
            }
            4 => {
                current_block_50 = 16637993436199044631;
            }
            3 => {
                current_block_50 = 6546859112865444725;
            }
            2 => {
                current_block_50 = 10505030521387687196;
            }
            1 => {
                current_block_50 = 5259327757700886538;
            }
            _ => {
                current_block_50 = 1356832168064818221;
            }
        }
        match current_block_50 {
            483194043190260627 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 9392992341002218192;
            }
            _ => {}
        }
        match current_block_50 {
            9392992341002218192 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 14840068175916424037;
            }
            _ => {}
        }
        match current_block_50 {
            14840068175916424037 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 2003362535987825465;
            }
            _ => {}
        }
        match current_block_50 {
            2003362535987825465 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 7629293359809983242;
            }
            _ => {}
        }
        match current_block_50 {
            7629293359809983242 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 11376947495104746217;
            }
            _ => {}
        }
        match current_block_50 {
            11376947495104746217 => {
                _hj_j =
                    _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_50 = 16637993436199044631;
            }
            _ => {}
        }
        match current_block_50 {
            16637993436199044631 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 6546859112865444725;
            }
            _ => {}
        }
        match current_block_50 {
            6546859112865444725 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 10505030521387687196;
            }
            _ => {}
        }
        match current_block_50 {
            10505030521387687196 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 5259327757700886538;
            }
            _ => {}
        }
        match current_block_50 {
            5259327757700886538 => {
                _hj_i =
                    _hj_i
                        .wrapping_add(*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        s = ::core::ptr::null_mut::<LigatureAggregator>();
        if !h.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if !(*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize))
                    .hh_head
                    .is_null()
                {
                    s = ((*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                        as *mut ::core::ffi::c_char)
                        .offset(-(*(*h).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut LigatureAggregator
                        as *mut LigatureAggregator;
                } else {
                    s = ::core::ptr::null_mut::<LigatureAggregator>();
                }
                while !s.is_null() {
                    if (*s).hh.hashv == _hf_hashv
                        && (*s).hh.keylen as usize
                            == ::core::mem::size_of::<::core::ffi::c_int>()
                    {
                        if memcmp(
                            (*s).hh.key,
                            &raw mut sgid as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(*s).hh.hh_next.is_null() {
                        s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut LigatureAggregator
                            as *mut LigatureAggregator;
                    } else {
                        s = ::core::ptr::null_mut::<LigatureAggregator>();
                    }
                }
            }
        }
        if s.is_null() {
            s = __caryll_allocate_clean(
                ::core::mem::size_of::<LigatureAggregator>() as usize,
                132 as ::core::ffi::c_ulong,
            ) as *mut LigatureAggregator;
            (*s).gid = sgid;
            let mut _ha_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i_0: ::core::ffi::c_uint = 0;
            let mut _hj_j_0: ::core::ffi::c_uint = 0;
            let mut _hj_k_0: ::core::ffi::c_uint = 0;
            let mut _hj_key_0: *const ::core::ffi::c_uchar =
                &raw mut (*s).gid as *const ::core::ffi::c_uchar;
            _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_0 = _hj_j_0;
            _hj_k_0 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            while _hj_k_0 >= 12 as ::core::ffi::c_uint {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
                _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
                _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
            }
            _ha_hashv = _ha_hashv
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_166: u64;
            match _hj_k_0 {
                11 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_166 = 3105948935974009916;
                }
                10 => {
                    current_block_166 = 3105948935974009916;
                }
                9 => {
                    current_block_166 = 16488506295619998735;
                }
                8 => {
                    current_block_166 = 2165477741955893522;
                }
                7 => {
                    current_block_166 = 16420434121503669123;
                }
                6 => {
                    current_block_166 = 4773154127383362184;
                }
                5 => {
                    current_block_166 = 11477443392506837243;
                }
                4 => {
                    current_block_166 = 4049670823543782160;
                }
                3 => {
                    current_block_166 = 8032402168972998897;
                }
                2 => {
                    current_block_166 = 13476765613733092207;
                }
                1 => {
                    current_block_166 = 7622613029762834038;
                }
                _ => {
                    current_block_166 = 7157669805658135323;
                }
            }
            match current_block_166 {
                3105948935974009916 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_166 = 16488506295619998735;
                }
                _ => {}
            }
            match current_block_166 {
                16488506295619998735 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_166 = 2165477741955893522;
                }
                _ => {}
            }
            match current_block_166 {
                2165477741955893522 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_166 = 16420434121503669123;
                }
                _ => {}
            }
            match current_block_166 {
                16420434121503669123 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_166 = 4773154127383362184;
                }
                _ => {}
            }
            match current_block_166 {
                4773154127383362184 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_166 = 11477443392506837243;
                }
                _ => {}
            }
            match current_block_166 {
                11477443392506837243 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_166 = 4049670823543782160;
                }
                _ => {}
            }
            match current_block_166 {
                4049670823543782160 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_166 = 8032402168972998897;
                }
                _ => {}
            }
            match current_block_166 {
                8032402168972998897 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_166 = 13476765613733092207;
                }
                _ => {}
            }
            match current_block_166 {
                13476765613733092207 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_166 = 7622613029762834038;
                }
                _ => {}
            }
            match current_block_166 {
                7622613029762834038 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                }
                _ => {}
            }
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
            (*s).hh.hashv = _ha_hashv;
            (*s).hh.key = &raw mut (*s).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
            (*s).hh.keylen = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            if h.is_null() {
                (*s).hh.next = NULL;
                (*s).hh.prev = NULL;
                (*s).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                    as *mut UtHashTable as *mut UtHashTable;
                if (*s).hh.tbl.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*s).hh.tbl as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<UtHashTable>() as usize,
                    );
                    (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
                    (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                    (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                    (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                        .offset_from(s as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        as isize;
                    (*(*s).hh.tbl).buckets = malloc(
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    ) as *mut UtHashBucket;
                    (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                    if (*(*s).hh.tbl).buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                        );
                    }
                }
                h = s;
            } else {
                (*s).hh.tbl = (*h).hh.tbl;
                (*s).hh.next = NULL;
                (*s).hh.prev = ((*(*h).hh.tbl).tail as *mut ::core::ffi::c_char)
                    .offset(-(*(*h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void;
                (*(*(*h).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                (*(*h).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
            }
            let mut _ha_bkt: ::core::ffi::c_uint = 0;
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt = _ha_hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head: *mut UtHashBucket =
                (*(*h).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
            (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
            (*s).hh.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
            (*s).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
            if !(*_ha_head).hh_head.is_null() {
                (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hh as *mut UtHashHandle;
            }
            (*_ha_head).hh_head = &raw mut (*s).hh as *mut UtHashHandle;
            if (*_ha_head).count
                >= (*_ha_head)
                    .expand_mult
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                && (*(*s).hh.tbl).noexpand == 0
            {
                let mut _he_bkt: ::core::ffi::c_uint = 0;
                let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                let mut _he_thh: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _he_hh_nxt: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _he_new_buckets: *mut UtHashBucket =
                    ::core::ptr::null_mut::<UtHashBucket>();
                let mut _he_newbkt: *mut UtHashBucket = ::core::ptr::null_mut::<UtHashBucket>();
                _he_new_buckets = malloc(
                    (2 as usize)
                        .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                if _he_new_buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        _he_new_buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (2 as usize)
                            .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    );
                    (*(*s).hh.tbl).ideal_chain_maxlen = ((*(*s).hh.tbl).num_items
                        >> (*(*s).hh.tbl)
                            .log2_num_buckets
                            .wrapping_add(1 as ::core::ffi::c_uint))
                    .wrapping_add(
                        if (*(*s).hh.tbl).num_items
                            & (*(*s).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                            != 0 as ::core::ffi::c_uint
                        {
                            1 as ::core::ffi::c_uint
                        } else {
                            0 as ::core::ffi::c_uint
                        },
                    );
                    (*(*s).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                    _he_bkt_i = 0 as ::core::ffi::c_uint;
                    while _he_bkt_i < (*(*s).hh.tbl).num_buckets {
                        _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                            as *mut UtHashHandle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*s).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                            (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                            if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                (*(*s).hh.tbl).nonideal_items =
                                    (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                (*_he_newbkt).expand_mult = (*_he_newbkt)
                                    .count
                                    .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
                            }
                            (*_he_thh).hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                            (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UtHashHandle;
                            if !(*_he_newbkt).hh_head.is_null() {
                                (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                            }
                            (*_he_newbkt).hh_head = _he_thh as *mut UtHashHandle;
                            _he_thh = _he_hh_nxt;
                        }
                        _he_bkt_i = _he_bkt_i.wrapping_add(1);
                    }
                    free((*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    (*(*s).hh.tbl).num_buckets = (*(*s).hh.tbl)
                        .num_buckets
                        .wrapping_mul(2 as ::core::ffi::c_uint);
                    (*(*s).hh.tbl).log2_num_buckets =
                        (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
                    (*(*s).hh.tbl).buckets = _he_new_buckets;
                    (*(*s).hh.tbl).ineff_expands = if (*(*s).hh.tbl).nonideal_items
                        > (*(*s).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                    {
                        (*(*s).hh.tbl)
                            .ineff_expands
                            .wrapping_add(1 as ::core::ffi::c_uint)
                    } else {
                        0 as ::core::ffi::c_uint
                    };
                    if (*(*s).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                        (*(*s).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                    }
                }
            }
        }
        j = j.wrapping_add(1);
    }
    let mut _hs_i: ::core::ffi::c_uint = 0;
    let mut _hs_looping: ::core::ffi::c_uint = 0;
    let mut _hs_nmerges: ::core::ffi::c_uint = 0;
    let mut _hs_insize: ::core::ffi::c_uint = 0;
    let mut _hs_psize: ::core::ffi::c_uint = 0;
    let mut _hs_qsize: ::core::ffi::c_uint = 0;
    let mut _hs_p: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_q: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_e: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_list: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_tail: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    if !h.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*h).hh as *mut UtHashHandle;
        while _hs_looping != 0 as ::core::ffi::c_uint {
            _hs_p = _hs_list;
            _hs_list = ::core::ptr::null_mut::<UtHashHandle>();
            _hs_tail = ::core::ptr::null_mut::<UtHashHandle>();
            _hs_nmerges = 0 as ::core::ffi::c_uint;
            while !_hs_p.is_null() {
                _hs_nmerges = _hs_nmerges.wrapping_add(1);
                _hs_q = _hs_p;
                _hs_psize = 0 as ::core::ffi::c_uint;
                _hs_i = 0 as ::core::ffi::c_uint;
                while _hs_i < _hs_insize {
                    _hs_psize = _hs_psize.wrapping_add(1);
                    _hs_q = (if !(*_hs_q).next.is_null() {
                        ((*_hs_q).next as *mut ::core::ffi::c_char)
                            .offset((*(*h).hh.tbl).hho)
                            as *mut UtHashHandle
                    } else {
                        ::core::ptr::null_mut::<UtHashHandle>()
                    }) as *mut UtHashHandle;
                    if _hs_q.is_null() {
                        break;
                    }
                    _hs_i = _hs_i.wrapping_add(1);
                }
                _hs_qsize = _hs_insize;
                while _hs_psize != 0 as ::core::ffi::c_uint
                    || _hs_qsize != 0 as ::core::ffi::c_uint && !_hs_q.is_null()
                {
                    if _hs_psize == 0 as ::core::ffi::c_uint {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(*h).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    } else if _hs_qsize == 0 as ::core::ffi::c_uint || _hs_q.is_null() {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*h).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if by_gid(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut LigatureAggregator,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut LigatureAggregator,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*h).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(*h).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    } else {
                        _hs_list = _hs_e;
                    }
                    if !_hs_e.is_null() {
                        (*_hs_e).prev = if !_hs_tail.is_null() {
                            (_hs_tail as *mut ::core::ffi::c_char)
                                .offset(-(*(*h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    }
                    _hs_tail = _hs_e;
                }
                _hs_p = _hs_q;
            }
            if !_hs_tail.is_null() {
                (*_hs_tail).next = NULL;
            }
            if _hs_nmerges <= 1 as ::core::ffi::c_uint {
                _hs_looping = 0 as ::core::ffi::c_uint;
                (*(*h).hh.tbl).tail = _hs_tail;
                h = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut LigatureAggregator
                    as *mut LigatureAggregator;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    let mut startcov: *mut Coverage = otl_Coverage_create();
    s = h;
    while !s.is_null() {
        pushToCoverage(
            startcov,
            handle_fromIndex((*s).gid as GlyphId)
                as GlyphHandle,
        );
        s = (*s).hh.next as *mut LigatureAggregator;
    }
    let mut root: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
            startcov,
        ))), bk_int(BkCellType::B16, ((*startcov).numGlyphs as ::core::ffi::c_int) as u32)]);
    s = h;
    while !s.is_null() {
        let mut nLigsHere: GlyphId = 0 as GlyphId;
        let mut j_0: GlyphId = 0 as GlyphId;
        while (j_0 as ::core::ffi::c_int) < nLigatures as ::core::ffi::c_int {
            if (*(*(*(*subtable).items.offset(j_0 as isize)).from)
                .glyphs
                .offset(0 as ::core::ffi::c_int as isize))
            .index as ::core::ffi::c_int
                == (*s).gid
            {
                nLigsHere = nLigsHere.wrapping_add(1);
            }
            j_0 = j_0.wrapping_add(1);
        }
        let mut ligset: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, (nLigsHere as ::core::ffi::c_int) as u32)]);
        let mut j_1: GlyphId = 0 as GlyphId;
        while (j_1 as ::core::ffi::c_int) < nLigatures as ::core::ffi::c_int {
            if (*(*(*(*subtable).items.offset(j_1 as isize)).from)
                .glyphs
                .offset(0 as ::core::ffi::c_int as isize))
            .index as ::core::ffi::c_int
                == (*s).gid
            {
                let mut ligdef: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, ((*(*subtable).items.offset(j_1 as isize)).to.index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*(*(*subtable).items.offset(j_1 as isize)).from).numGlyphs
                        as ::core::ffi::c_int) as u32)]);
                let mut m: GlyphId = 1 as GlyphId;
                while (m as ::core::ffi::c_int)
                    < (*(*(*subtable).items.offset(j_1 as isize)).from).numGlyphs
                        as ::core::ffi::c_int
                {
                    bk_push(ligdef, &[bk_int(BkCellType::B16, ((*(*(*(*subtable).items.offset(j_1 as isize)).from)
                            .glyphs
                            .offset(m as isize))
                        .index as ::core::ffi::c_int) as u32)]);
                    m = m.wrapping_add(1);
                }
                bk_push(ligset, &[bk_ptr(BkCellType::P16, ligdef)]);
            }
            j_1 = j_1.wrapping_add(1);
        }
        bk_push(root, &[bk_ptr(BkCellType::P16, ligset)]);
        s = (*s).hh.next as *mut LigatureAggregator;
    }
    otl_Coverage_free(startcov);
    s = h;
    tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut LigatureAggregator
        as *mut LigatureAggregator;
    while !s.is_null() {
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*h).hh.tbl as *mut ::core::ffi::c_void);
            h = ::core::ptr::null_mut::<LigatureAggregator>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*h).hh.tbl).tail {
                (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle
                    as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh2 = (*_hd_hh_del).next;
            } else {
                h = (*_hd_hh_del).next as *mut LigatureAggregator as *mut LigatureAggregator;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh3 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket =
                (*(*h).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
            (*_hd_head).count = (*_hd_head).count.wrapping_sub(1);
            if (*_hd_head).hh_head == _hd_hh_del {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s as *mut ::core::ffi::c_void);
        s = ::core::ptr::null_mut::<LigatureAggregator>();
        s = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut LigatureAggregator
            as *mut LigatureAggregator;
    }
    return bk_build_Block(root);
}
