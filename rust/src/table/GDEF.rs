#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};
use crate::support::json_funcs::{json_obj_get, json_obj_get_type, json_obj_getint, json_obj_getnum, preserialize};
use crate::table::otl::classdef::{ClassDef, otl_ClassDef_free, readClassDef};
use crate::table::otl::coverage::{Coverage, otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage};
use crate::support::handle::{handle_fromName, otfcc_Handle_dispose, otfcc_Handle_dup, otfcc_Handle_empty, Handle, GlyphHandle, HANDLE_STATE_EMPTY};
use crate::support::binio::{read_16u};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, Pos, ShapeId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{json_array, json_integer, json_object, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{b16, b32, BkBlock, bk_int, bk_new_Block, bk_ptr, bk_push, p16};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::support::{ComparFn};
use crate::bk::bkblock::{bk_newBlockFromBuffer};
use crate::bk::bkgraph::{bk_build_Block};
use crate::table::otl::classdef::{otl_iClassDef};
use crate::table::otl::coverage::{otl_iCoverage};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsempty, sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaretValue {
    pub format: i8,
    pub coordiante: Pos,
    pub pointIndex: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaretValueElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut CaretValue) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut CaretValue, *const CaretValue) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut CaretValue, *mut CaretValue) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CaretValue) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut CaretValue, CaretValue) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut CaretValue, CaretValue) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaretValueList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut CaretValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaretValueListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut CaretValueList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut CaretValueList, *const CaretValueList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut CaretValueList, *mut CaretValueList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CaretValueList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut CaretValueList, CaretValueList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut CaretValueList, CaretValueList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut CaretValueList>,
    pub free: Option<unsafe extern "C" fn(*mut CaretValueList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut CaretValueList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut CaretValueList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut CaretValueList>,
    pub fill: Option<unsafe extern "C" fn(*mut CaretValueList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut CaretValueList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut CaretValueList, CaretValue) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut CaretValueList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut CaretValueList) -> CaretValue>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut CaretValueList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut CaretValueList,
            Option<unsafe extern "C" fn(*const CaretValue, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut CaretValueList,
            Option<
                unsafe extern "C" fn(
                    *const CaretValue,
                    *const CaretValue,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaretValueRecord {
    pub glyph: GlyphHandle,
    pub carets: CaretValueList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaretValueRecordElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut CaretValueRecord) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut CaretValueRecord, *const CaretValueRecord) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut CaretValueRecord, *mut CaretValueRecord) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CaretValueRecord) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut CaretValueRecord, CaretValueRecord) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut CaretValueRecord, CaretValueRecord) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LigCaretTable {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut CaretValueRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LigCaretTableVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut LigCaretTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut LigCaretTable, *const LigCaretTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut LigCaretTable, *mut LigCaretTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut LigCaretTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut LigCaretTable, LigCaretTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut LigCaretTable, LigCaretTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut LigCaretTable>,
    pub free: Option<unsafe extern "C" fn(*mut LigCaretTable) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut LigCaretTable, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut LigCaretTable, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut LigCaretTable>,
    pub fill: Option<unsafe extern "C" fn(*mut LigCaretTable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut LigCaretTable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut LigCaretTable, CaretValueRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut LigCaretTable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut LigCaretTable) -> CaretValueRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut LigCaretTable, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut LigCaretTable,
            Option<
                unsafe extern "C" fn(*const CaretValueRecord, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut LigCaretTable,
            Option<
                unsafe extern "C" fn(
                    *const CaretValueRecord,
                    *const CaretValueRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GdefTable {
    pub glyphClassDef: *mut ClassDef,
    pub markAttachClassDef: *mut ClassDef,
    pub ligCarets: LigCaretTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GdefTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GdefTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut GdefTable, *const GdefTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut GdefTable, *mut GdefTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GdefTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut GdefTable, GdefTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut GdefTable, GdefTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GdefTable>,
    pub free: Option<unsafe extern "C" fn(*mut GdefTable) -> ()>,
}
pub static otl_iCaretValue: CaretValueElementInterface =
    CaretValueElementInterface {
        init: None,
        copy: None,
        move_0: None,
        dispose: None,
        replace: None,
        copyReplace: None,
    };
#[inline]
unsafe extern "C" fn otl_CaretValueList_grow(arr: *mut CaretValueList) {
    cvec_grow(otl_CaretValueList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_growTo(arr: *mut CaretValueList, target: usize) {
    cvec_grow_to(otl_CaretValueList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_pop(arr: *mut CaretValueList) -> CaretValue {
    cvec_pop(otl_CaretValueList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_copyReplace(
    mut dst: *mut CaretValueList,
    src: CaretValueList,
) {
    otl_CaretValueList_dispose(dst);
    otl_CaretValueList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_copy(
    mut dst: *mut CaretValueList,
    mut src: *const CaretValueList,
) {
    otl_CaretValueList_init(dst);
    otl_CaretValueList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iCaretValue.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iCaretValue.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut CaretValue,
                (*src).items.offset(j as isize) as *mut CaretValue as *const CaretValue,
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
unsafe extern "C" fn otl_CaretValueList_dispose(mut arr: *mut CaretValueList) {
    if arr.is_null() {
        return;
    }
    if otl_iCaretValue.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            otl_iCaretValue.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut CaretValue,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<CaretValue>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_replace(
    mut dst: *mut CaretValueList,
    src: CaretValueList,
) {
    otl_CaretValueList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CaretValueList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_initCapN(mut arr: *mut CaretValueList, mut n: usize) {
    otl_CaretValueList_init(arr);
    otl_CaretValueList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_growToN(arr: *mut CaretValueList, target: usize) {
    cvec_grow_to_n(otl_CaretValueList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_initN(mut arr: *mut CaretValueList, mut n: usize) {
    otl_CaretValueList_init(arr);
    otl_CaretValueList_growToN(arr, n);
    otl_CaretValueList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_free(mut x: *mut CaretValueList) {
    if x.is_null() {
        return;
    }
    otl_CaretValueList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_createN(mut n: usize) -> *mut CaretValueList {
    let mut t: *mut CaretValueList =
        malloc(::core::mem::size_of::<CaretValueList>() as usize) as *mut CaretValueList;
    otl_CaretValueList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_create() -> *mut CaretValueList {
    let mut x: *mut CaretValueList =
        malloc(::core::mem::size_of::<CaretValueList>() as usize) as *mut CaretValueList;
    otl_CaretValueList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_shrinkToFit(mut arr: *mut CaretValueList) {
    otl_CaretValueList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_resizeTo(arr: *mut CaretValueList, target: usize) {
    cvec_resize_to(otl_CaretValueList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_move(dst: *mut CaretValueList, src: *mut CaretValueList) {
    cvec_move(otl_CaretValueList_as_cvec(dst), otl_CaretValueList_as_cvec(src));
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_filterEnv(
    mut arr: *mut CaretValueList,
    mut fn_0: Option<unsafe extern "C" fn(*const CaretValue, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut CaretValue,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if otl_iCaretValue.dispose.is_some() {
                otl_iCaretValue.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut CaretValue,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn otl_CaretValueList_as_cvec(arr: *mut CaretValueList) -> *mut CVecRaw<CaretValue> {
    arr as *mut CVecRaw<CaretValue>
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_init(arr: *mut CaretValueList) {
    cvec_init(otl_CaretValueList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_push(arr: *mut CaretValueList, elem: CaretValue) {
    cvec_push(otl_CaretValueList_as_cvec(arr), elem);
}
pub static otl_iCaretValueList: CaretValueListVectorInterface = {
    CaretValueListVectorInterface {
        init: Some(otl_CaretValueList_init as unsafe extern "C" fn(*mut CaretValueList) -> ()),
        copy: Some(
            otl_CaretValueList_copy
                as unsafe extern "C" fn(*mut CaretValueList, *const CaretValueList) -> (),
        ),
        move_0: Some(
            otl_CaretValueList_move
                as unsafe extern "C" fn(*mut CaretValueList, *mut CaretValueList) -> (),
        ),
        dispose: Some(
            otl_CaretValueList_dispose as unsafe extern "C" fn(*mut CaretValueList) -> (),
        ),
        replace: Some(
            otl_CaretValueList_replace
                as unsafe extern "C" fn(*mut CaretValueList, CaretValueList) -> (),
        ),
        copyReplace: Some(
            otl_CaretValueList_copyReplace
                as unsafe extern "C" fn(*mut CaretValueList, CaretValueList) -> (),
        ),
        create: Some(otl_CaretValueList_create),
        free: Some(otl_CaretValueList_free as unsafe extern "C" fn(*mut CaretValueList) -> ()),
        initN: Some(
            otl_CaretValueList_initN as unsafe extern "C" fn(*mut CaretValueList, usize) -> (),
        ),
        initCapN: Some(
            otl_CaretValueList_initCapN
                as unsafe extern "C" fn(*mut CaretValueList, usize) -> (),
        ),
        createN: Some(
            otl_CaretValueList_createN as unsafe extern "C" fn(usize) -> *mut CaretValueList,
        ),
        fill: Some(
            otl_CaretValueList_fill as unsafe extern "C" fn(*mut CaretValueList, usize) -> (),
        ),
        clear: Some(
            otl_CaretValueList_dispose as unsafe extern "C" fn(*mut CaretValueList) -> (),
        ),
        push: Some(
            otl_CaretValueList_push
                as unsafe extern "C" fn(*mut CaretValueList, CaretValue) -> (),
        ),
        shrinkToFit: Some(
            otl_CaretValueList_shrinkToFit as unsafe extern "C" fn(*mut CaretValueList) -> (),
        ),
        pop: Some(
            otl_CaretValueList_pop
                as unsafe extern "C" fn(*mut CaretValueList) -> CaretValue,
        ),
        disposeItem: Some(
            otl_CaretValueList_disposeItem
                as unsafe extern "C" fn(*mut CaretValueList, usize) -> (),
        ),
        filterEnv: Some(
            otl_CaretValueList_filterEnv
                as unsafe extern "C" fn(
                    *mut CaretValueList,
                    Option<
                        unsafe extern "C" fn(
                            *const CaretValue,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_CaretValueList_sort
                as unsafe extern "C" fn(
                    *mut CaretValueList,
                    Option<
                        unsafe extern "C" fn(
                            *const CaretValue,
                            *const CaretValue,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_CaretValueList_sort(
    mut arr: *mut CaretValueList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const CaretValue, *const CaretValue) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<CaretValue>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const CaretValue,
                    *const CaretValue,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_disposeItem(
    mut arr: *mut CaretValueList,
    mut n: usize,
) {
    if otl_iCaretValue.dispose.is_some() {
        otl_iCaretValue.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut CaretValue,
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_fill(mut arr: *mut CaretValueList, mut n: usize) {
    while (*arr).length < n {
        let mut x: CaretValue = CaretValue {
            format: 0,
            coordiante: 0.,
            pointIndex: 0,
        };
        if otl_iCaretValue.init.is_some() {
            otl_iCaretValue.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<CaretValue>() as usize,
            );
        }
        otl_CaretValueList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn initGdefLigCaretRec(mut v: *mut CaretValueRecord) {
    (*v).glyph = otfcc_Handle_empty() as GlyphHandle;
    otl_iCaretValueList.init.expect("non-null function pointer")(&raw mut (*v).carets);
}
unsafe extern "C" fn deleteGdefLigCaretRec(mut v: *mut CaretValueRecord) {
    otfcc_Handle_dispose(&raw mut (*v).glyph);
    otl_iCaretValueList
        .dispose
        .expect("non-null function pointer")(&raw mut (*v).carets);
}
pub static otl_iCaretValueRecord: CaretValueRecordElementInterface = {
    CaretValueRecordElementInterface {
        init: Some(initGdefLigCaretRec as unsafe extern "C" fn(*mut CaretValueRecord) -> ()),
        copy: None,
        move_0: None,
        dispose: Some(
            deleteGdefLigCaretRec as unsafe extern "C" fn(*mut CaretValueRecord) -> (),
        ),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe fn otl_LigCaretTable_as_cvec(arr: *mut LigCaretTable) -> *mut CVecRaw<CaretValueRecord> {
    arr as *mut CVecRaw<CaretValueRecord>
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_init(arr: *mut LigCaretTable) {
    cvec_init(otl_LigCaretTable_as_cvec(arr));
}
pub static otl_iLigCaretTable: LigCaretTableVectorInterface = {
    LigCaretTableVectorInterface {
        init: Some(otl_LigCaretTable_init as unsafe extern "C" fn(*mut LigCaretTable) -> ()),
        copy: Some(
            otl_LigCaretTable_copy
                as unsafe extern "C" fn(*mut LigCaretTable, *const LigCaretTable) -> (),
        ),
        move_0: Some(
            otl_LigCaretTable_move
                as unsafe extern "C" fn(*mut LigCaretTable, *mut LigCaretTable) -> (),
        ),
        dispose: Some(
            otl_LigCaretTable_dispose as unsafe extern "C" fn(*mut LigCaretTable) -> (),
        ),
        replace: Some(
            otl_LigCaretTable_replace
                as unsafe extern "C" fn(*mut LigCaretTable, LigCaretTable) -> (),
        ),
        copyReplace: Some(
            otl_LigCaretTable_copyReplace
                as unsafe extern "C" fn(*mut LigCaretTable, LigCaretTable) -> (),
        ),
        create: Some(otl_LigCaretTable_create),
        free: Some(otl_LigCaretTable_free as unsafe extern "C" fn(*mut LigCaretTable) -> ()),
        initN: Some(
            otl_LigCaretTable_initN as unsafe extern "C" fn(*mut LigCaretTable, usize) -> (),
        ),
        initCapN: Some(
            otl_LigCaretTable_initCapN
                as unsafe extern "C" fn(*mut LigCaretTable, usize) -> (),
        ),
        createN: Some(
            otl_LigCaretTable_createN as unsafe extern "C" fn(usize) -> *mut LigCaretTable,
        ),
        fill: Some(
            otl_LigCaretTable_fill as unsafe extern "C" fn(*mut LigCaretTable, usize) -> (),
        ),
        clear: Some(
            otl_LigCaretTable_dispose as unsafe extern "C" fn(*mut LigCaretTable) -> (),
        ),
        push: Some(
            otl_LigCaretTable_push
                as unsafe extern "C" fn(*mut LigCaretTable, CaretValueRecord) -> (),
        ),
        shrinkToFit: Some(
            otl_LigCaretTable_shrinkToFit as unsafe extern "C" fn(*mut LigCaretTable) -> (),
        ),
        pop: Some(
            otl_LigCaretTable_pop
                as unsafe extern "C" fn(*mut LigCaretTable) -> CaretValueRecord,
        ),
        disposeItem: Some(
            otl_LigCaretTable_disposeItem
                as unsafe extern "C" fn(*mut LigCaretTable, usize) -> (),
        ),
        filterEnv: Some(
            otl_LigCaretTable_filterEnv
                as unsafe extern "C" fn(
                    *mut LigCaretTable,
                    Option<
                        unsafe extern "C" fn(
                            *const CaretValueRecord,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_LigCaretTable_sort
                as unsafe extern "C" fn(
                    *mut LigCaretTable,
                    Option<
                        unsafe extern "C" fn(
                            *const CaretValueRecord,
                            *const CaretValueRecord,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LigCaretTable_shrinkToFit(mut arr: *mut LigCaretTable) {
    otl_LigCaretTable_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_resizeTo(arr: *mut LigCaretTable, target: usize) {
    cvec_resize_to(otl_LigCaretTable_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_move(dst: *mut LigCaretTable, src: *mut LigCaretTable) {
    cvec_move(otl_LigCaretTable_as_cvec(dst), otl_LigCaretTable_as_cvec(src));
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_filterEnv(
    mut arr: *mut LigCaretTable,
    mut fn_0: Option<
        unsafe extern "C" fn(*const CaretValueRecord, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut CaretValueRecord,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if otl_iCaretValueRecord.dispose.is_some() {
                otl_iCaretValueRecord
                    .dispose
                    .expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut CaretValueRecord,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_disposeItem(mut arr: *mut LigCaretTable, mut n: usize) {
    if otl_iCaretValueRecord.dispose.is_some() {
        otl_iCaretValueRecord
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut CaretValueRecord
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_sort(
    mut arr: *mut LigCaretTable,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const CaretValueRecord,
            *const CaretValueRecord,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<CaretValueRecord>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const CaretValueRecord,
                    *const CaretValueRecord,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_fill(mut arr: *mut LigCaretTable, mut n: usize) {
    while (*arr).length < n {
        let mut x: CaretValueRecord = CaretValueRecord {
            glyph: Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            carets: CaretValueList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<CaretValue>(),
            },
        };
        if otl_iCaretValueRecord.init.is_some() {
            otl_iCaretValueRecord
                .init
                .expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<CaretValueRecord>() as usize,
            );
        }
        otl_LigCaretTable_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_push(arr: *mut LigCaretTable, elem: CaretValueRecord) {
    cvec_push(otl_LigCaretTable_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_grow(arr: *mut LigCaretTable) {
    cvec_grow(otl_LigCaretTable_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_growTo(arr: *mut LigCaretTable, target: usize) {
    cvec_grow_to(otl_LigCaretTable_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_pop(arr: *mut LigCaretTable) -> CaretValueRecord {
    cvec_pop(otl_LigCaretTable_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_copyReplace(
    mut dst: *mut LigCaretTable,
    src: LigCaretTable,
) {
    otl_LigCaretTable_dispose(dst);
    otl_LigCaretTable_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_copy(
    mut dst: *mut LigCaretTable,
    mut src: *const LigCaretTable,
) {
    otl_LigCaretTable_init(dst);
    otl_LigCaretTable_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iCaretValueRecord.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iCaretValueRecord
                .copy
                .expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut CaretValueRecord,
                (*src).items.offset(j as isize) as *mut CaretValueRecord
                    as *const CaretValueRecord,
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
unsafe extern "C" fn otl_LigCaretTable_dispose(mut arr: *mut LigCaretTable) {
    if arr.is_null() {
        return;
    }
    if otl_iCaretValueRecord.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh3 = j;
            j = j.wrapping_sub(1);
            if !(fresh3 != 0) {
                break;
            }
            otl_iCaretValueRecord
                .dispose
                .expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut CaretValueRecord
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<CaretValueRecord>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_replace(
    mut dst: *mut LigCaretTable,
    src: LigCaretTable,
) {
    otl_LigCaretTable_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LigCaretTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_initCapN(mut arr: *mut LigCaretTable, mut n: usize) {
    otl_LigCaretTable_init(arr);
    otl_LigCaretTable_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_growToN(arr: *mut LigCaretTable, target: usize) {
    cvec_grow_to_n(otl_LigCaretTable_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_initN(mut arr: *mut LigCaretTable, mut n: usize) {
    otl_LigCaretTable_init(arr);
    otl_LigCaretTable_growToN(arr, n);
    otl_LigCaretTable_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_free(mut x: *mut LigCaretTable) {
    if x.is_null() {
        return;
    }
    otl_LigCaretTable_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_createN(mut n: usize) -> *mut LigCaretTable {
    let mut t: *mut LigCaretTable =
        malloc(::core::mem::size_of::<LigCaretTable>() as usize) as *mut LigCaretTable;
    otl_LigCaretTable_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_create() -> *mut LigCaretTable {
    let mut x: *mut LigCaretTable =
        malloc(::core::mem::size_of::<LigCaretTable>() as usize) as *mut LigCaretTable;
    otl_LigCaretTable_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn initGDEF(mut gdef: *mut GdefTable) {
    (*gdef).glyphClassDef = ::core::ptr::null_mut::<ClassDef>();
    (*gdef).markAttachClassDef = ::core::ptr::null_mut::<ClassDef>();
    otl_iLigCaretTable.init.expect("non-null function pointer")(&raw mut (*gdef).ligCarets);
}
#[inline]
unsafe extern "C" fn disposeGDEF(mut gdef: *mut GdefTable) {
    if gdef.is_null() {
        return;
    }
    if !(*gdef).glyphClassDef.is_null() {
        otl_ClassDef_free((*gdef).glyphClassDef);
    }
    if !(*gdef).markAttachClassDef.is_null() {
        otl_ClassDef_free((*gdef).markAttachClassDef);
    }
    otl_iLigCaretTable
        .dispose
        .expect("non-null function pointer")(&raw mut (*gdef).ligCarets);
}
#[inline]
unsafe extern "C" fn table_GDEF_init(mut x: *mut GdefTable) {
    initGDEF(x);
}
#[inline]
unsafe extern "C" fn table_GDEF_dispose(mut x: *mut GdefTable) {
    disposeGDEF(x);
}
pub static table_iGDEF: GdefTableElementInterface = {
    GdefTableElementInterface {
        init: Some(table_GDEF_init as unsafe extern "C" fn(*mut GdefTable) -> ()),
        copy: Some(
            table_GDEF_copy as unsafe extern "C" fn(*mut GdefTable, *const GdefTable) -> (),
        ),
        move_0: Some(
            table_GDEF_move as unsafe extern "C" fn(*mut GdefTable, *mut GdefTable) -> (),
        ),
        dispose: Some(table_GDEF_dispose as unsafe extern "C" fn(*mut GdefTable) -> ()),
        replace: Some(
            table_GDEF_replace as unsafe extern "C" fn(*mut GdefTable, GdefTable) -> (),
        ),
        copyReplace: Some(
            table_GDEF_copyReplace as unsafe extern "C" fn(*mut GdefTable, GdefTable) -> (),
        ),
        create: Some(table_GDEF_create),
        free: Some(table_GDEF_free as unsafe extern "C" fn(*mut GdefTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_GDEF_create() -> *mut GdefTable {
    let mut x: *mut GdefTable =
        malloc(::core::mem::size_of::<GdefTable>() as usize) as *mut GdefTable;
    table_GDEF_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_GDEF_move(mut dst: *mut GdefTable, mut src: *mut GdefTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GdefTable>() as usize,
    );
    table_GDEF_init(src);
}
#[inline]
unsafe extern "C" fn table_GDEF_free(mut x: *mut GdefTable) {
    if x.is_null() {
        return;
    }
    table_GDEF_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_GDEF_copyReplace(mut dst: *mut GdefTable, src: GdefTable) {
    table_GDEF_dispose(dst);
    table_GDEF_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_GDEF_copy(mut dst: *mut GdefTable, mut src: *const GdefTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GdefTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_GDEF_replace(mut dst: *mut GdefTable, src: GdefTable) {
    table_GDEF_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GdefTable>() as usize,
    );
}
unsafe extern "C" fn readCaretValue(
    data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
) -> CaretValue {
    let mut v: CaretValue = CaretValue {
        format: 0,
        coordiante: 0.,
        pointIndex: 0,
    };
    v.format = 0 as i8;
    v.coordiante = 0 as ::core::ffi::c_int as Pos;
    v.pointIndex = 0xffff as ::core::ffi::c_int as i16;
    if !(tableLength < offset.wrapping_add(4 as u32)) {
        v.format = read_16u(data.offset(offset as isize) as *const u8) as i8;
        if v.format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            v.pointIndex = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as i16;
        } else {
            v.coordiante = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as Pos;
        }
    }
    return v;
}
unsafe extern "C" fn readLigCaretRecord(
    data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
) -> CaretValueRecord {
    let mut caretCount: ShapeId = 0;
    let mut g: CaretValueRecord = CaretValueRecord {
        glyph: Handle {
            state: HANDLE_STATE_EMPTY,
            index: 0,
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        carets: CaretValueList {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<CaretValue>(),
        },
    };
    otl_iCaretValueRecord
        .init
        .expect("non-null function pointer")(&raw mut g);
    if !(tableLength < offset.wrapping_add(2 as u32)) {
        caretCount = read_16u(data.offset(offset as isize) as *const u8) as ShapeId;
        if !(tableLength
            < offset.wrapping_add(2 as u32).wrapping_add(
                (caretCount as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
            ))
        {
            let mut j: GlyphId = 0 as GlyphId;
            while (j as ::core::ffi::c_int) < caretCount as ::core::ffi::c_int {
                otl_iCaretValueList.push.expect("non-null function pointer")(
                    &raw mut g.carets,
                    readCaretValue(
                        data,
                        tableLength,
                        offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(
                                    (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        ) as u32),
                    ),
                );
                j = j.wrapping_add(1);
            }
        }
    }
    return g;
}
pub unsafe extern "C" fn otfcc_readGDEF(
    packet: Packet,
    mut _options: *const Options,
) -> *mut GdefTable {
    let mut classdefOffset: u16 = 0;
    let mut ligCaretOffset: u16 = 0;
    let mut markAttachDefOffset: u16 = 0;
    let mut current_block: u64;
    let mut gdef: *mut GdefTable = ::core::ptr::null_mut::<GdefTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1195656518i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut tableLength: u32 = table.length;
                    if !(tableLength < 12 as u32) {
                        gdef = (
                            table_iGDEF.create.expect("non-null function pointer"))();
                        classdefOffset = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if classdefOffset != 0 {
                            (*gdef).glyphClassDef =
                                readClassDef(
                                    data as *const u8,
                                    tableLength,
                                    classdefOffset as u32,
                                );
                        }
                        ligCaretOffset = read_16u(
                            data.offset(8 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if ligCaretOffset != 0 {
                            if tableLength
                                < (ligCaretOffset as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                    as u32
                            {
                                current_block = 10802812094495641425;
                            } else {
                                let mut cov: *mut Coverage =
                                    readCoverage(
                                        data as *const u8,
                                        tableLength,
                                        (ligCaretOffset as ::core::ffi::c_int
                                            + read_16u(data.offset(
                                                ligCaretOffset as ::core::ffi::c_int as isize,
                                            )
                                                as *const u8)
                                                as ::core::ffi::c_int)
                                            as u32,
                                    );
                                if cov.is_null()
                                    || (*cov).numGlyphs as ::core::ffi::c_int
                                        != read_16u(
                                            data.offset(
                                                ligCaretOffset as ::core::ffi::c_int as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize)
                                                as *const u8,
                                        )
                                            as ::core::ffi::c_int
                                {
                                    current_block = 10802812094495641425;
                                } else if tableLength
                                    < (ligCaretOffset as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int
                                        + (*cov).numGlyphs as ::core::ffi::c_int
                                            * 2 as ::core::ffi::c_int)
                                        as u32
                                {
                                    current_block = 10802812094495641425;
                                } else {
                                    let mut j: GlyphId = 0 as GlyphId;
                                    while (j as ::core::ffi::c_int)
                                        < (*cov).numGlyphs as ::core::ffi::c_int
                                    {
                                        let mut v: CaretValueRecord = readLigCaretRecord(
                                            data,
                                            tableLength,
                                            (ligCaretOffset as ::core::ffi::c_int
                                                + read_16u(
                                                    data.offset(
                                                        ligCaretOffset as ::core::ffi::c_int
                                                            as isize,
                                                    )
                                                    .offset(4 as ::core::ffi::c_int as isize)
                                                    .offset(
                                                        (j as ::core::ffi::c_int
                                                            * 2 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                        as *const u8,
                                                )
                                                    as ::core::ffi::c_int)
                                                as u32,
                                        );
                                        v.glyph =
                                            otfcc_Handle_dup(
                                                *(*cov).glyphs.offset(j as isize) as Handle,
                                            )
                                                as GlyphHandle;
                                        otl_iLigCaretTable.push.expect("non-null function pointer")(
                                            &raw mut (*gdef).ligCarets,
                                            v,
                                        );
                                        j = j.wrapping_add(1);
                                    }
                                    otl_Coverage_free(cov);
                                    current_block = 11307063007268554308;
                                }
                            }
                        } else {
                            current_block = 11307063007268554308;
                        }
                        match current_block {
                            10802812094495641425 => {}
                            _ => {
                                markAttachDefOffset =
                                    read_16u(data.offset(10 as ::core::ffi::c_int as isize)
                                        as *const u8);
                                if markAttachDefOffset != 0 {
                                    (*gdef).markAttachClassDef =
                                        readClassDef(
                                            data as *const u8,
                                            tableLength,
                                            markAttachDefOffset as u32,
                                        );
                                }
                                return gdef;
                            }
                        }
                    }
                    table_iGDEF.free.expect("non-null function pointer")(gdef);
                    gdef = ::core::ptr::null_mut::<GdefTable>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return gdef;
}
unsafe extern "C" fn dumpGDEFLigCarets(mut gdef: *const GdefTable) -> *mut JsonValue {
    let mut _carets: *mut JsonValue = json_object_new((*gdef).ligCarets.length);
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*gdef).ligCarets.length {
        let mut name: SdsRaw = (*(*gdef).ligCarets.items.offset(j as isize)).glyph.name;
        let mut _record: *mut JsonValue =
            json_array_new((*(*gdef).ligCarets.items.offset(j as isize)).carets.length);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as usize) < (*(*gdef).ligCarets.items.offset(j as isize)).carets.length {
            let mut _cv: *mut JsonValue = json_object_new(1 as usize);
            if (*(*(*gdef).ligCarets.items.offset(j as isize))
                .carets
                .items
                .offset(k as isize))
            .format as ::core::ffi::c_int
                == 2 as ::core::ffi::c_int
            {
                json_object_push(
                    _cv,
                    b"atPoint\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(
                        (*(*(*gdef).ligCarets.items.offset(j as isize))
                            .carets
                            .items
                            .offset(k as isize))
                        .pointIndex as i64,
                    ),
                );
            } else {
                json_object_push(
                    _cv,
                    b"at\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(
                        (*(*(*gdef).ligCarets.items.offset(j as isize))
                            .carets
                            .items
                            .offset(k as isize))
                        .coordiante as i64,
                    ),
                );
            }
            json_array_push(_record, _cv);
            k = k.wrapping_add(1);
        }
        json_object_push(
            _carets,
            name as *const ::core::ffi::c_char,
            preserialize(_record),
        );
        j = j.wrapping_add(1);
    }
    return _carets;
}
pub unsafe extern "C" fn otfcc_dumpGDEF(
    mut gdef: *const GdefTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if gdef.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"GDEF"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _gdef: *mut JsonValue = json_object_new(4 as usize);
        if !(*gdef).glyphClassDef.is_null() {
            json_object_push(
                _gdef,
                b"glyphClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                otl_iClassDef.dump.expect("non-null function pointer")((*gdef).glyphClassDef),
            );
        }
        if !(*gdef).markAttachClassDef.is_null() {
            json_object_push(
                _gdef,
                b"markAttachClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                otl_iClassDef.dump.expect("non-null function pointer")((*gdef).markAttachClassDef),
            );
        }
        if (*gdef).ligCarets.length != 0 {
            json_object_push(
                _gdef,
                b"ligCarets\0" as *const u8 as *const ::core::ffi::c_char,
                dumpGDEFLigCarets(gdef),
            );
        }
        json_object_push(
            root,
            b"GDEF\0" as *const u8 as *const ::core::ffi::c_char,
            _gdef,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
unsafe extern "C" fn ligCaretFromJson(
    mut _carets: *const JsonValue,
    mut lc: *mut LigCaretTable,
) {
    if _carets.is_null()
        || (*_carets).type_0 != json_object
    {
        return;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < (*_carets).u.object.length {
        let mut a: *mut JsonValue =
            (*(*_carets).u.object.values.offset(j as isize)).value as *mut JsonValue;
        if !(a.is_null()
            || (*a).type_0 != json_array)
        {
            let mut v: CaretValueRecord = CaretValueRecord {
                glyph: Handle {
                    state: HANDLE_STATE_EMPTY,
                    index: 0,
                    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                },
                carets: CaretValueList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<CaretValue>(),
                },
            };
            otl_iCaretValueRecord
                .init
                .expect("non-null function pointer")(&raw mut v);
            v.glyph = handle_fromName(sdsnewlen(
                (*(*_carets).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
                (*(*_carets).u.object.values.offset(j as isize)).name_length as usize,
            )) as GlyphHandle;
            let mut caretCount: ShapeId = (*a).u.array.length as ShapeId;
            let mut k: GlyphId = 0 as GlyphId;
            while (k as ::core::ffi::c_int) < caretCount as ::core::ffi::c_int {
                let mut caret: CaretValue = CaretValue {
                    format: 0,
                    coordiante: 0.,
                    pointIndex: 0,
                };
                caret.format = 1 as i8;
                caret.coordiante = 0 as ::core::ffi::c_int as Pos;
                caret.pointIndex = 0xffff as ::core::ffi::c_int as i16;
                let mut _caret: *mut JsonValue =
                    *(*a).u.array.values.offset(k as isize) as *mut JsonValue;
                if !_caret.is_null()
                    && (*_caret).type_0 == json_object
                {
                    if !json_obj_get_type(
                        _caret,
                        b"atPoint\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer,
                    )
                    .is_null()
                    {
                        caret.format = 2 as i8;
                        caret.pointIndex = json_obj_getint(
                            _caret,
                            b"atPoint\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as i16;
                    } else {
                        caret.coordiante = json_obj_getnum(
                            _caret,
                            b"at\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as Pos;
                    }
                }
                otl_iCaretValueList.push.expect("non-null function pointer")(
                    &raw mut v.carets,
                    caret,
                );
                k = k.wrapping_add(1);
            }
            otl_iLigCaretTable.push.expect("non-null function pointer")(lc, v);
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe extern "C" fn otfcc_parseGDEF(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut GdefTable {
    let mut gdef: *mut GdefTable = ::core::ptr::null_mut::<GdefTable>();
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"GDEF\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"GDEF"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            gdef = (
                table_iGDEF.create.expect("non-null function pointer"))();
            (*gdef).glyphClassDef =
                otl_iClassDef.parse.expect("non-null function pointer")(json_obj_get(
                    table,
                    b"glyphClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            (*gdef).markAttachClassDef =
                otl_iClassDef.parse.expect("non-null function pointer")(json_obj_get(
                    table,
                    b"markAttachClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            ligCaretFromJson(
                json_obj_get(
                    table,
                    b"ligCarets\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &raw mut (*gdef).ligCarets,
            );
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return gdef;
}
unsafe extern "C" fn writeLigCaretRec(mut cr: *mut CaretValueRecord) -> *mut BkBlock {
    let mut bcr: *mut BkBlock = bk_new_Block(&[bk_int(b16, ((*cr).carets.length) as u32)]);
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*cr).carets.length {
        bk_push(bcr, &[bk_ptr(p16, bk_new_Block(&[bk_int(b16, ((*(*cr).carets.items.offset(j as isize)).format as ::core::ffi::c_int) as u32), bk_int(b16, (if (*(*cr).carets.items.offset(j as isize)).format as ::core::ffi::c_int
                    == 2 as ::core::ffi::c_int
                {
                    (*(*cr).carets.items.offset(j as isize)).pointIndex as ::core::ffi::c_int
                } else {
                    (*(*cr).carets.items.offset(j as isize)).coordiante as i16
                        as ::core::ffi::c_int
                }) as u32)]))]);
        j = j.wrapping_add(1);
    }
    return bcr;
}
unsafe extern "C" fn writeLigCarets(mut lc: *const LigCaretTable) -> *mut BkBlock {
    let mut cov: *mut Coverage = otl_Coverage_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*lc).length {
        pushToCoverage(
            cov,
            otfcc_Handle_dup(
                (*(*lc).items.offset(j as isize)).glyph as Handle,
            ) as GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let mut lct: *mut BkBlock = bk_new_Block(&[bk_ptr(p16, bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(cov))), bk_int(b16, ((*lc).length) as u32)]);
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*lc).length {
        bk_push(lct, &[bk_ptr(p16, writeLigCaretRec((*lc).items.offset(j_0 as isize) as *mut CaretValueRecord))]);
        j_0 = j_0.wrapping_add(1);
    }
    otl_Coverage_free(cov);
    return lct;
}
pub unsafe extern "C" fn otfcc_buildGDEF(
    mut gdef: *const GdefTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if gdef.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut bGlyphClassDef: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    let mut bAttachList: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    let mut bLigCaretList: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    let mut bMarkAttachClassDef: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    if !(*gdef).glyphClassDef.is_null() {
        bGlyphClassDef =
            bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
                (*gdef).glyphClassDef,
            ));
    }
    if (*gdef).ligCarets.length != 0 {
        bLigCaretList = writeLigCarets(&raw const (*gdef).ligCarets);
    }
    if !(*gdef).markAttachClassDef.is_null() {
        bMarkAttachClassDef =
            bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
                (*gdef).markAttachClassDef,
            ));
    }
    let mut root: *mut BkBlock = bk_new_Block(&[bk_int(b32, 0x10000 as u32), bk_ptr(p16, bGlyphClassDef), bk_ptr(p16, bAttachList), bk_ptr(p16, bLigCaretList), bk_ptr(p16, bMarkAttachClassDef)]);
    return bk_build_Block(root);
}
