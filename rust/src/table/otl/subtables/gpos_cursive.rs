#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};

use crate::support::json_funcs::{json_obj_get, preserialize};
use crate::table::otl::coverage::{Coverage, otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage};
use crate::support::handle::{handle_fromName, otfcc_Handle_dispose, otfcc_Handle_dup, Handle, GlyphHandle, HandleState};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_Block, bk_ptr, bk_push};

use crate::table::otl::{GposCursiveSubtableVectorInterface, Anchor, GposCursiveEntry, Subtable, GposCursiveSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::support::{ComparFn};
use crate::bk::bkblock::{bk_newBlockFromBuffer};
use crate::bk::bkgraph::{bk_build_Block};
use crate::table::otl::coverage::{otl_iCoverage};
use crate::table::otl::subtables::gpos_common::{bkFromAnchor, otl_anchor_absent, otl_dump_anchor, otl_parse_anchor, otl_read_anchor};
use crate::vendor::json_builder::{json_object_new, json_object_push};
use crate::vendor::sds::{sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposCursiveEntryElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GposCursiveEntry) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut GposCursiveEntry, *const GposCursiveEntry) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut GposCursiveEntry, *mut GposCursiveEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GposCursiveEntry) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut GposCursiveEntry, GposCursiveEntry) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut GposCursiveEntry, GposCursiveEntry) -> ()>,
}
unsafe extern "C" fn deleteGposCursiveEntry(mut entry: *mut GposCursiveEntry) {
    otfcc_Handle_dispose(&raw mut (*entry).target);
}
static gss_typeinfo: GposCursiveEntryElementInterface = {
    GposCursiveEntryElementInterface {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(
            deleteGposCursiveEntry as unsafe extern "C" fn(*mut GposCursiveEntry) -> (),
        ),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_move(dst: *mut GposCursiveSubtable, src: *mut GposCursiveSubtable) {
    cvec_move(as_cvec(dst), as_cvec(src));
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_resizeTo(arr: *mut GposCursiveSubtable, target: usize) {
    cvec_resize_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_filterEnv(
    mut arr: *mut GposCursiveSubtable,
    mut fn_0: Option<
        unsafe extern "C" fn(*const GposCursiveEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut GposCursiveEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if gss_typeinfo.dispose.is_some() {
                gss_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut GposCursiveEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn as_cvec(arr: *mut GposCursiveSubtable) -> *mut CVecRaw<GposCursiveEntry> {
    arr as *mut CVecRaw<GposCursiveEntry>
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_init(arr: *mut GposCursiveSubtable) {
    cvec_init(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_disposeItem(
    mut arr: *mut GposCursiveSubtable,
    mut n: usize,
) {
    if gss_typeinfo.dispose.is_some() {
        gss_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut GposCursiveEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_sort(
    mut arr: *mut GposCursiveSubtable,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const GposCursiveEntry,
            *const GposCursiveEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<GposCursiveEntry>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const GposCursiveEntry,
                    *const GposCursiveEntry,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_fill(
    mut arr: *mut GposCursiveSubtable,
    mut n: usize,
) {
    while (*arr).length < n {
        let mut x: GposCursiveEntry = GposCursiveEntry {
            target: Handle {
                state: HandleState::Empty,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            enter: Anchor {
                present: false,
                x: 0.,
                y: 0.,
            },
            exit: Anchor {
                present: false,
                x: 0.,
                y: 0.,
            },
        };
        if gss_typeinfo.init.is_some() {
            gss_typeinfo.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<GposCursiveEntry>() as usize,
            );
        }
        subtable_gpos_cursive_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_push(arr: *mut GposCursiveSubtable, elem: GposCursiveEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_grow(arr: *mut GposCursiveSubtable) {
    cvec_grow(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_growTo(arr: *mut GposCursiveSubtable, target: usize) {
    cvec_grow_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_pop(arr: *mut GposCursiveSubtable) -> GposCursiveEntry {
    cvec_pop(as_cvec(arr))
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_copyReplace(
    mut dst: *mut GposCursiveSubtable,
    src: GposCursiveSubtable,
) {
    subtable_gpos_cursive_dispose(dst);
    subtable_gpos_cursive_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_copy(
    mut dst: *mut GposCursiveSubtable,
    mut src: *const GposCursiveSubtable,
) {
    subtable_gpos_cursive_init(dst);
    subtable_gpos_cursive_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if gss_typeinfo.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            gss_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut GposCursiveEntry,
                (*src).items.offset(j as isize) as *mut GposCursiveEntry
                    as *const GposCursiveEntry,
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
unsafe extern "C" fn subtable_gpos_cursive_dispose(mut arr: *mut GposCursiveSubtable) {
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
                (*arr).items.offset(j as isize) as *mut GposCursiveEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<GposCursiveEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_replace(
    mut dst: *mut GposCursiveSubtable,
    src: GposCursiveSubtable,
) {
    subtable_gpos_cursive_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GposCursiveSubtable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_initCapN(
    mut arr: *mut GposCursiveSubtable,
    mut n: usize,
) {
    subtable_gpos_cursive_init(arr);
    subtable_gpos_cursive_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_growToN(arr: *mut GposCursiveSubtable, target: usize) {
    cvec_grow_to_n(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_initN(
    mut arr: *mut GposCursiveSubtable,
    mut n: usize,
) {
    subtable_gpos_cursive_init(arr);
    subtable_gpos_cursive_growToN(arr, n);
    subtable_gpos_cursive_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_free(mut x: *mut GposCursiveSubtable) {
    if x.is_null() {
        return;
    }
    subtable_gpos_cursive_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_createN(mut n: usize) -> *mut GposCursiveSubtable {
    let mut t: *mut GposCursiveSubtable =
        malloc(::core::mem::size_of::<GposCursiveSubtable>() as usize)
            as *mut GposCursiveSubtable;
    subtable_gpos_cursive_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_create() -> *mut GposCursiveSubtable {
    let mut x: *mut GposCursiveSubtable =
        malloc(::core::mem::size_of::<GposCursiveSubtable>() as usize)
            as *mut GposCursiveSubtable;
    subtable_gpos_cursive_init(x);
    return x;
}
pub static iSubtable_gpos_cursive: GposCursiveSubtableVectorInterface = {
    GposCursiveSubtableVectorInterface {
        init: Some(
            subtable_gpos_cursive_init as unsafe extern "C" fn(*mut GposCursiveSubtable) -> (),
        ),
        copy: Some(
            subtable_gpos_cursive_copy
                as unsafe extern "C" fn(
                    *mut GposCursiveSubtable,
                    *const GposCursiveSubtable,
                ) -> (),
        ),
        move_0: Some(
            subtable_gpos_cursive_move
                as unsafe extern "C" fn(
                    *mut GposCursiveSubtable,
                    *mut GposCursiveSubtable,
                ) -> (),
        ),
        dispose: Some(
            subtable_gpos_cursive_dispose as unsafe extern "C" fn(*mut GposCursiveSubtable) -> (),
        ),
        replace: Some(
            subtable_gpos_cursive_replace
                as unsafe extern "C" fn(*mut GposCursiveSubtable, GposCursiveSubtable) -> (),
        ),
        copyReplace: Some(
            subtable_gpos_cursive_copyReplace
                as unsafe extern "C" fn(*mut GposCursiveSubtable, GposCursiveSubtable) -> (),
        ),
        create: Some(subtable_gpos_cursive_create),
        free: Some(
            subtable_gpos_cursive_free as unsafe extern "C" fn(*mut GposCursiveSubtable) -> (),
        ),
        initN: Some(
            subtable_gpos_cursive_initN
                as unsafe extern "C" fn(*mut GposCursiveSubtable, usize) -> (),
        ),
        initCapN: Some(
            subtable_gpos_cursive_initCapN
                as unsafe extern "C" fn(*mut GposCursiveSubtable, usize) -> (),
        ),
        createN: Some(
            subtable_gpos_cursive_createN
                as unsafe extern "C" fn(usize) -> *mut GposCursiveSubtable,
        ),
        fill: Some(
            subtable_gpos_cursive_fill
                as unsafe extern "C" fn(*mut GposCursiveSubtable, usize) -> (),
        ),
        clear: Some(
            subtable_gpos_cursive_dispose as unsafe extern "C" fn(*mut GposCursiveSubtable) -> (),
        ),
        push: Some(
            subtable_gpos_cursive_push
                as unsafe extern "C" fn(*mut GposCursiveSubtable, GposCursiveEntry) -> (),
        ),
        shrinkToFit: Some(
            subtable_gpos_cursive_shrinkToFit
                as unsafe extern "C" fn(*mut GposCursiveSubtable) -> (),
        ),
        pop: Some(
            subtable_gpos_cursive_pop
                as unsafe extern "C" fn(*mut GposCursiveSubtable) -> GposCursiveEntry,
        ),
        disposeItem: Some(
            subtable_gpos_cursive_disposeItem
                as unsafe extern "C" fn(*mut GposCursiveSubtable, usize) -> (),
        ),
        filterEnv: Some(
            subtable_gpos_cursive_filterEnv
                as unsafe extern "C" fn(
                    *mut GposCursiveSubtable,
                    Option<
                        unsafe extern "C" fn(
                            *const GposCursiveEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gpos_cursive_sort
                as unsafe extern "C" fn(
                    *mut GposCursiveSubtable,
                    Option<
                        unsafe extern "C" fn(
                            *const GposCursiveEntry,
                            *const GposCursiveEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_shrinkToFit(mut arr: *mut GposCursiveSubtable) {
    subtable_gpos_cursive_resizeTo(arr, (*arr).length);
}
pub unsafe extern "C" fn otl_read_gpos_cursive(
    data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
    _maxGlyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut valueCount: GlyphId = 0;
    let mut subtable: *mut GposCursiveSubtable =
        (
            iSubtable_gpos_cursive
                .create
                .expect("non-null function pointer"))();
    let mut targets: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    if !(tableLength < offset.wrapping_add(6 as u32)) {
        targets = readCoverage(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !(targets.is_null()
            || (*targets).numGlyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            valueCount = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as GlyphId;
            if !(tableLength
                < offset.wrapping_add(6 as u32).wrapping_add(
                    (4 as ::core::ffi::c_int * valueCount as ::core::ffi::c_int) as u32,
                ))
            {
                if !(valueCount as ::core::ffi::c_int != (*targets).numGlyphs as ::core::ffi::c_int)
                {
                    let mut j: GlyphId = 0 as GlyphId;
                    while (j as ::core::ffi::c_int) < valueCount as ::core::ffi::c_int {
                        let mut enterOffset: u16 = read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (4 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        );
                        let mut exitOffset: u16 = read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (4 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                                )
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const u8,
                        );
                        let mut enter: Anchor = otl_anchor_absent();
                        let mut exit: Anchor = otl_anchor_absent();
                        if enterOffset != 0 {
                            enter = otl_read_anchor(
                                data,
                                tableLength,
                                offset.wrapping_add(enterOffset as u32),
                            );
                        }
                        if exitOffset != 0 {
                            exit = otl_read_anchor(
                                data,
                                tableLength,
                                offset.wrapping_add(exitOffset as u32),
                            );
                        }
                        iSubtable_gpos_cursive
                            .push
                            .expect("non-null function pointer")(
                            subtable,
                            GposCursiveEntry {
                                target: otfcc_Handle_dup(
                                    *(*targets).glyphs.offset(j as isize) as Handle,
                                ) as GlyphHandle,
                                enter: enter,
                                exit: exit,
                            },
                        );
                        j = j.wrapping_add(1);
                    }
                    if !targets.is_null() {
                        otl_Coverage_free(targets);
                    }
                    return subtable as *mut Subtable;
                }
            }
        }
    }
    if !targets.is_null() {
        otl_Coverage_free(targets);
    }
    iSubtable_gpos_cursive
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gpos_dump_cursive(
    mut _subtable: *const Subtable,
) -> *mut JsonValue {
    let mut subtable: *const GposCursiveSubtable = &raw const (*_subtable).gpos_cursive;
    let mut st: *mut JsonValue = json_object_new((*subtable).length);
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).length {
        let mut rec: *mut JsonValue = json_object_new(2 as usize);
        json_object_push(
            rec,
            b"enter\0" as *const u8 as *const ::core::ffi::c_char,
            otl_dump_anchor((*(*subtable).items.offset(j as isize)).enter),
        );
        json_object_push(
            rec,
            b"exit\0" as *const u8 as *const ::core::ffi::c_char,
            otl_dump_anchor((*(*subtable).items.offset(j as isize)).exit),
        );
        json_object_push(
            st,
            (*(*subtable).items.offset(j as isize)).target.name as *const ::core::ffi::c_char,
            preserialize(rec),
        );
        j = j.wrapping_add(1);
    }
    return st;
}
pub unsafe extern "C" fn otl_gpos_parse_cursive(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut subtable: *mut GposCursiveSubtable =
        (
            iSubtable_gpos_cursive
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
            iSubtable_gpos_cursive
                .push
                .expect("non-null function pointer")(
                subtable,
                GposCursiveEntry {
                    target: handle_fromName(gname)
                        as GlyphHandle,
                    enter: otl_parse_anchor(json_obj_get(
                        (*(*_subtable).u.object.values.offset(j as isize)).value,
                        b"enter\0" as *const u8 as *const ::core::ffi::c_char,
                    )),
                    exit: otl_parse_anchor(json_obj_get(
                        (*(*_subtable).u.object.values.offset(j as isize)).value,
                        b"exit\0" as *const u8 as *const ::core::ffi::c_char,
                    )),
                },
            );
        }
        j = j.wrapping_add(1);
    }
    return subtable as *mut Subtable;
}
pub unsafe extern "C" fn otfcc_build_gpos_cursive(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let mut subtable: *const GposCursiveSubtable = &raw const (*_subtable).gpos_cursive;
    let mut cov: *mut Coverage = otl_Coverage_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).length {
        pushToCoverage(
            cov,
            otfcc_Handle_dup(
                (*(*subtable).items.offset(j as isize)).target as Handle,
            ) as GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let mut root: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(cov))), bk_int(BkCellType::B16, ((*subtable).length) as u32)]);
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).length {
        bk_push(root, &[bk_ptr(BkCellType::P16, bkFromAnchor((*(*subtable).items.offset(j_0 as isize)).enter)), bk_ptr(BkCellType::P16, bkFromAnchor((*(*subtable).items.offset(j_0 as isize)).exit))]);
        j_0 = j_0.wrapping_add(1);
    }
    otl_Coverage_free(cov);
    return bk_build_Block(root);
}
