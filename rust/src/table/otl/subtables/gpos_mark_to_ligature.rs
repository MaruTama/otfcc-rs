use libc::{free, malloc, memcmp, memcpy, memset, qsort, strcmp, strlen};
extern "C" {
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_object_push_length(
        object: *mut json_value,
        name_length: ::core::ffi::c_uint,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new_length(
        length: ::core::ffi::c_uint,
        _: *const ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
    static otl_iMarkArray: __caryll_vectorinterface_otl_MarkArray;
    fn otl_anchor_absent() -> otl_Anchor;
    fn otl_read_anchor(
        data: font_file_pointer,
        tableLength: u32,
        offset: u32,
    ) -> otl_Anchor;
    fn otl_parse_anchor(v: *mut json_value) -> otl_Anchor;
    fn bkFromAnchor(a: otl_Anchor) -> *mut bk_Block;
    fn otl_readMarkArray(
        array: *mut otl_MarkArray,
        cov: *mut otl_Coverage,
        data: font_file_pointer,
        tableLength: u32,
        offset: u32,
    );
    fn otl_parseMarkArray(
        _marks: *mut json_value,
        array: *mut otl_MarkArray,
        h: *mut *mut otl_ClassnameHash,
        options: *const otfcc_Options,
    );
}


use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage, otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage};
use crate::support::handle::{handle_fromName, otfcc_Handle_dispose, otfcc_Handle_dup, otfcc_Handle, otfcc_GlyphHandle, HANDLE_STATE_EMPTY};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphclass_t, glyphid_t};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, sds, sdshdr16, sdshdr32, sdshdr64, sdshdr8};
use crate::vendor::json::{json_array, json_object, json_pre_serialized, json_type, json_value};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{b16, bk_Block, bkover, p16};
use crate::support::{NULL};
use crate::table::otl::{__caryll_elementinterface_subtable_gpos_markToLigature, __caryll_vectorinterface_otl_LigatureArray, __caryll_vectorinterface_otl_MarkArray, otl_Anchor, otl_LigatureArray, otl_LigatureBaseRecord, otl_MarkArray, otl_Subtable, subtable_gpos_markToLigature};
use crate::table::otl::subtables::{otl_BuildHeuristics};
use crate::table::otl::subtables::gpos_common::{otl_ClassnameHash};
use crate::vendor::json_builder::{json_serialize_mode_packed, json_serialize_opts};
use crate::vendor::uthash::{UT_hash_bucket, UT_hash_handle};
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_LigatureBaseRecord {
    pub init: Option<unsafe extern "C" fn(*mut otl_LigatureBaseRecord) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut otl_LigatureBaseRecord, *const otl_LigatureBaseRecord) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(*mut otl_LigatureBaseRecord, *mut otl_LigatureBaseRecord) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LigatureBaseRecord) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut otl_LigatureBaseRecord, otl_LigatureBaseRecord) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_LigatureBaseRecord, otl_LigatureBaseRecord) -> ()>,
}
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<json_value>();
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        }
        _k = _k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_get_type(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    type_0: json_type,
) -> *mut json_value {
    let mut v: *mut json_value = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return v;
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn preserialize(mut x: *mut json_value) -> *mut json_value {
    let mut opts: json_serialize_opts = json_serialize_opts {
        mode: json_serialize_mode_packed,
        opts: 0,
        indent_size: 0,
    };
    let mut preserialize_len: usize = json_measure_ex(x, opts);
    let mut buf: *mut ::core::ffi::c_char = malloc(preserialize_len) as *mut ::core::ffi::c_char;
    json_serialize_ex(buf, x, opts);
    json_builder_free(x);
    let mut xx: *mut json_value = json_string_new_nocopy(
        preserialize_len.wrapping_sub(1 as usize) as ::core::ffi::c_uint,
        buf,
    );
    (*xx).type_0 = json_pre_serialized;
    return xx;
}
unsafe extern "C" fn deleteLigArrayItem(mut entry: *mut otl_LigatureBaseRecord) {
    otfcc_Handle_dispose(&raw mut (*entry).glyph);
    if !(*entry).anchors.is_null() {
        let mut k: glyphid_t = 0 as glyphid_t;
        while (k as ::core::ffi::c_int) < (*entry).componentCount as ::core::ffi::c_int {
            free(*(*entry).anchors.offset(k as isize) as *mut ::core::ffi::c_void);
            let ref mut fresh0 = *(*entry).anchors.offset(k as isize);
            *fresh0 = ::core::ptr::null_mut::<otl_Anchor>();
            k = k.wrapping_add(1);
        }
        free((*entry).anchors as *mut ::core::ffi::c_void);
        (*entry).anchors = ::core::ptr::null_mut::<*mut otl_Anchor>();
    }
}
static mut la_typeinfo: __caryll_elementinterface_otl_LigatureBaseRecord = {
    __caryll_elementinterface_otl_LigatureBaseRecord {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(
            deleteLigArrayItem as unsafe extern "C" fn(*mut otl_LigatureBaseRecord) -> (),
        ),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn otl_LigatureArray_growToN(arr: *mut otl_LigatureArray, target: usize) {
    cvec_grow_to_n(otl_LigatureArray_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_push(arr: *mut otl_LigatureArray, elem: otl_LigatureBaseRecord) {
    cvec_push(otl_LigatureArray_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_grow(arr: *mut otl_LigatureArray) {
    cvec_grow(otl_LigatureArray_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_growTo(arr: *mut otl_LigatureArray, target: usize) {
    cvec_grow_to(otl_LigatureArray_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_pop(arr: *mut otl_LigatureArray) -> otl_LigatureBaseRecord {
    cvec_pop(otl_LigatureArray_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_copyReplace(
    mut dst: *mut otl_LigatureArray,
    src: otl_LigatureArray,
) {
    otl_LigatureArray_dispose(dst);
    otl_LigatureArray_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_copy(
    mut dst: *mut otl_LigatureArray,
    mut src: *const otl_LigatureArray,
) {
    otl_LigatureArray_init(dst);
    otl_LigatureArray_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if la_typeinfo.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            la_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_LigatureBaseRecord,
                (*src).items.offset(j as isize) as *mut otl_LigatureBaseRecord
                    as *const otl_LigatureBaseRecord,
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
unsafe extern "C" fn otl_LigatureArray_dispose(mut arr: *mut otl_LigatureArray) {
    if arr.is_null() {
        return;
    }
    if la_typeinfo.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh2 = j;
            j = j.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            la_typeinfo.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_LigatureBaseRecord,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_LigatureBaseRecord>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_replace(
    mut dst: *mut otl_LigatureArray,
    src: otl_LigatureArray,
) {
    otl_LigatureArray_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_LigatureArray>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_initCapN(mut arr: *mut otl_LigatureArray, mut n: usize) {
    otl_LigatureArray_init(arr);
    otl_LigatureArray_growToN(arr, n);
}
#[inline]
unsafe fn otl_LigatureArray_as_cvec(arr: *mut otl_LigatureArray) -> *mut CVecRaw<otl_LigatureBaseRecord> {
    arr as *mut CVecRaw<otl_LigatureBaseRecord>
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_init(arr: *mut otl_LigatureArray) {
    cvec_init(otl_LigatureArray_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_initN(mut arr: *mut otl_LigatureArray, mut n: usize) {
    otl_LigatureArray_init(arr);
    otl_LigatureArray_growToN(arr, n);
    otl_LigatureArray_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_free(mut x: *mut otl_LigatureArray) {
    if x.is_null() {
        return;
    }
    otl_LigatureArray_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_createN(mut n: usize) -> *mut otl_LigatureArray {
    let mut t: *mut otl_LigatureArray =
        malloc(::core::mem::size_of::<otl_LigatureArray>() as usize) as *mut otl_LigatureArray;
    otl_LigatureArray_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_create() -> *mut otl_LigatureArray {
    let mut x: *mut otl_LigatureArray =
        malloc(::core::mem::size_of::<otl_LigatureArray>() as usize) as *mut otl_LigatureArray;
    otl_LigatureArray_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_filterEnv(
    mut arr: *mut otl_LigatureArray,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_LigatureBaseRecord, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_LigatureBaseRecord,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if la_typeinfo.dispose.is_some() {
                la_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_LigatureBaseRecord,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[no_mangle]
pub static mut otl_iLigatureArray: __caryll_vectorinterface_otl_LigatureArray = {
    __caryll_vectorinterface_otl_LigatureArray {
        init: Some(otl_LigatureArray_init as unsafe extern "C" fn(*mut otl_LigatureArray) -> ()),
        copy: Some(
            otl_LigatureArray_copy
                as unsafe extern "C" fn(*mut otl_LigatureArray, *const otl_LigatureArray) -> (),
        ),
        move_0: Some(
            otl_LigatureArray_move
                as unsafe extern "C" fn(*mut otl_LigatureArray, *mut otl_LigatureArray) -> (),
        ),
        dispose: Some(
            otl_LigatureArray_dispose as unsafe extern "C" fn(*mut otl_LigatureArray) -> (),
        ),
        replace: Some(
            otl_LigatureArray_replace
                as unsafe extern "C" fn(*mut otl_LigatureArray, otl_LigatureArray) -> (),
        ),
        copyReplace: Some(
            otl_LigatureArray_copyReplace
                as unsafe extern "C" fn(*mut otl_LigatureArray, otl_LigatureArray) -> (),
        ),
        create: Some(otl_LigatureArray_create),
        free: Some(otl_LigatureArray_free as unsafe extern "C" fn(*mut otl_LigatureArray) -> ()),
        initN: Some(
            otl_LigatureArray_initN as unsafe extern "C" fn(*mut otl_LigatureArray, usize) -> (),
        ),
        initCapN: Some(
            otl_LigatureArray_initCapN
                as unsafe extern "C" fn(*mut otl_LigatureArray, usize) -> (),
        ),
        createN: Some(
            otl_LigatureArray_createN as unsafe extern "C" fn(usize) -> *mut otl_LigatureArray,
        ),
        fill: Some(
            otl_LigatureArray_fill as unsafe extern "C" fn(*mut otl_LigatureArray, usize) -> (),
        ),
        clear: Some(
            otl_LigatureArray_dispose as unsafe extern "C" fn(*mut otl_LigatureArray) -> (),
        ),
        push: Some(
            otl_LigatureArray_push
                as unsafe extern "C" fn(*mut otl_LigatureArray, otl_LigatureBaseRecord) -> (),
        ),
        shrinkToFit: Some(
            otl_LigatureArray_shrinkToFit as unsafe extern "C" fn(*mut otl_LigatureArray) -> (),
        ),
        pop: Some(
            otl_LigatureArray_pop
                as unsafe extern "C" fn(*mut otl_LigatureArray) -> otl_LigatureBaseRecord,
        ),
        disposeItem: Some(
            otl_LigatureArray_disposeItem
                as unsafe extern "C" fn(*mut otl_LigatureArray, usize) -> (),
        ),
        filterEnv: Some(
            otl_LigatureArray_filterEnv
                as unsafe extern "C" fn(
                    *mut otl_LigatureArray,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_LigatureBaseRecord,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_LigatureArray_sort
                as unsafe extern "C" fn(
                    *mut otl_LigatureArray,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_LigatureBaseRecord,
                            *const otl_LigatureBaseRecord,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LigatureArray_shrinkToFit(mut arr: *mut otl_LigatureArray) {
    otl_LigatureArray_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_resizeTo(arr: *mut otl_LigatureArray, target: usize) {
    cvec_resize_to(otl_LigatureArray_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_move(dst: *mut otl_LigatureArray, src: *mut otl_LigatureArray) {
    cvec_move(otl_LigatureArray_as_cvec(dst), otl_LigatureArray_as_cvec(src));
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_disposeItem(mut arr: *mut otl_LigatureArray, mut n: usize) {
    if la_typeinfo.dispose.is_some() {
        la_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_LigatureBaseRecord
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_sort(
    mut arr: *mut otl_LigatureArray,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otl_LigatureBaseRecord,
            *const otl_LigatureBaseRecord,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_LigatureBaseRecord>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_LigatureBaseRecord,
                    *const otl_LigatureBaseRecord,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_LigatureArray_fill(mut arr: *mut otl_LigatureArray, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_LigatureBaseRecord = otl_LigatureBaseRecord {
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            componentCount: 0,
            anchors: ::core::ptr::null_mut::<*mut otl_Anchor>(),
        };
        if la_typeinfo.init.is_some() {
            la_typeinfo.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_LigatureBaseRecord>() as usize,
            );
        }
        otl_LigatureArray_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn initMarkToLigature(mut subtable: *mut subtable_gpos_markToLigature) {
    otl_iMarkArray.init.expect("non-null function pointer")(&raw mut (*subtable).markArray);
    otl_iLigatureArray.init.expect("non-null function pointer")(&raw mut (*subtable).ligArray);
}
#[inline]
unsafe extern "C" fn disposeMarkToLigature(mut subtable: *mut subtable_gpos_markToLigature) {
    otl_iMarkArray.dispose.expect("non-null function pointer")(&raw mut (*subtable).markArray);
    otl_iLigatureArray
        .dispose
        .expect("non-null function pointer")(&raw mut (*subtable).ligArray);
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToLigature_init(mut x: *mut subtable_gpos_markToLigature) {
    initMarkToLigature(x);
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToLigature_copyReplace(
    mut dst: *mut subtable_gpos_markToLigature,
    src: subtable_gpos_markToLigature,
) {
    subtable_gpos_markToLigature_dispose(dst);
    subtable_gpos_markToLigature_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToLigature_copy(
    mut dst: *mut subtable_gpos_markToLigature,
    mut src: *const subtable_gpos_markToLigature,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_markToLigature>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToLigature_replace(
    mut dst: *mut subtable_gpos_markToLigature,
    src: subtable_gpos_markToLigature,
) {
    subtable_gpos_markToLigature_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_markToLigature>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToLigature_move(
    mut dst: *mut subtable_gpos_markToLigature,
    mut src: *mut subtable_gpos_markToLigature,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_markToLigature>() as usize,
    );
    subtable_gpos_markToLigature_init(src);
}
#[no_mangle]
pub static mut iSubtable_gpos_markToLigature:
    __caryll_elementinterface_subtable_gpos_markToLigature = {
    __caryll_elementinterface_subtable_gpos_markToLigature {
        init: Some(
            subtable_gpos_markToLigature_init
                as unsafe extern "C" fn(*mut subtable_gpos_markToLigature) -> (),
        ),
        copy: Some(
            subtable_gpos_markToLigature_copy
                as unsafe extern "C" fn(
                    *mut subtable_gpos_markToLigature,
                    *const subtable_gpos_markToLigature,
                ) -> (),
        ),
        move_0: Some(
            subtable_gpos_markToLigature_move
                as unsafe extern "C" fn(
                    *mut subtable_gpos_markToLigature,
                    *mut subtable_gpos_markToLigature,
                ) -> (),
        ),
        dispose: Some(
            subtable_gpos_markToLigature_dispose
                as unsafe extern "C" fn(*mut subtable_gpos_markToLigature) -> (),
        ),
        replace: Some(
            subtable_gpos_markToLigature_replace
                as unsafe extern "C" fn(
                    *mut subtable_gpos_markToLigature,
                    subtable_gpos_markToLigature,
                ) -> (),
        ),
        copyReplace: Some(
            subtable_gpos_markToLigature_copyReplace
                as unsafe extern "C" fn(
                    *mut subtable_gpos_markToLigature,
                    subtable_gpos_markToLigature,
                ) -> (),
        ),
        create: Some(subtable_gpos_markToLigature_create),
        free: Some(
            subtable_gpos_markToLigature_free
                as unsafe extern "C" fn(*mut subtable_gpos_markToLigature) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_markToLigature_create() -> *mut subtable_gpos_markToLigature {
    let mut x: *mut subtable_gpos_markToLigature =
        malloc(::core::mem::size_of::<subtable_gpos_markToLigature>() as usize)
            as *mut subtable_gpos_markToLigature;
    subtable_gpos_markToLigature_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToLigature_free(mut x: *mut subtable_gpos_markToLigature) {
    if x.is_null() {
        return;
    }
    subtable_gpos_markToLigature_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToLigature_dispose(
    mut x: *mut subtable_gpos_markToLigature,
) {
    disposeMarkToLigature(x);
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_gpos_markToLigature(
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut markArrayOffset: u32 = 0;
    let mut ligArrayOffset: u32 = 0;
    let mut current_block: u64;
    let mut subtable: *mut subtable_gpos_markToLigature =
        (
            iSubtable_gpos_markToLigature
                .create
                .expect("non-null function pointer"))();
    let mut marks: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    let mut bases: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    if !(tableLength < offset.wrapping_add(12 as u32)) {
        marks = readCoverage(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        bases = readCoverage(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !(marks.is_null()
            || (*marks).numGlyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || bases.is_null()
            || (*bases).numGlyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            (*subtable).classCount = read_16u(
                data.offset(offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize) as *const u8,
            ) as glyphclass_t;
            markArrayOffset = offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32);
            otl_readMarkArray(
                &raw mut (*subtable).markArray,
                marks,
                data,
                tableLength,
                markArrayOffset,
            );
            ligArrayOffset = offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(10 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32);
            if !(tableLength
                < ligArrayOffset.wrapping_add(2 as u32).wrapping_add(
                    (2 as ::core::ffi::c_int * (*bases).numGlyphs as ::core::ffi::c_int)
                        as u32,
                ))
            {
                if !(read_16u(data.offset(ligArrayOffset as isize) as *const u8)
                    as ::core::ffi::c_int
                    != (*bases).numGlyphs as ::core::ffi::c_int)
                {
                    let mut j: glyphid_t = 0 as glyphid_t;
                    loop {
                        if !((j as ::core::ffi::c_int) < (*bases).numGlyphs as ::core::ffi::c_int) {
                            current_block = 17788412896529399552;
                            break;
                        }
                        let mut lig: otl_LigatureBaseRecord = otl_LigatureBaseRecord {
                            glyph: otfcc_Handle {
                                state: HANDLE_STATE_EMPTY,
                                index: 0,
                                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                            componentCount: 0,
                            anchors: ::core::ptr::null_mut::<*mut otl_Anchor>(),
                        };
                        lig.glyph = otfcc_Handle_dup(
                            *(*bases).glyphs.offset(j as isize) as otfcc_Handle,
                        ) as otfcc_GlyphHandle;
                        let mut ligAttachOffset: u32 = ligArrayOffset.wrapping_add(read_16u(
                            data.offset(ligArrayOffset as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(
                                    (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        )
                            as u32);
                        if tableLength < ligAttachOffset.wrapping_add(2 as u32) {
                            current_block = 14470250473917821325;
                            break;
                        }
                        lig.componentCount =
                            read_16u(data.offset(ligAttachOffset as isize) as *const u8)
                                as glyphid_t;
                        if tableLength
                            < ligAttachOffset.wrapping_add(2 as u32).wrapping_add(
                                (2 as ::core::ffi::c_int
                                    * lig.componentCount as ::core::ffi::c_int
                                    * (*subtable).classCount as ::core::ffi::c_int)
                                    as u32,
                            )
                        {
                            current_block = 14470250473917821325;
                            break;
                        }
                        lig.anchors = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut otl_Anchor>() as usize)
                                .wrapping_mul(lig.componentCount as usize),
                            58 as ::core::ffi::c_ulong,
                        ) as *mut *mut otl_Anchor;
                        let mut _offset: u32 = ligAttachOffset.wrapping_add(2 as u32);
                        let mut k: glyphid_t = 0 as glyphid_t;
                        while (k as ::core::ffi::c_int) < lig.componentCount as ::core::ffi::c_int {
                            let ref mut fresh3 = *lig.anchors.offset(k as isize);
                            *fresh3 = __caryll_allocate_clean(
                                (::core::mem::size_of::<otl_Anchor>() as usize)
                                    .wrapping_mul((*subtable).classCount as usize),
                                62 as ::core::ffi::c_ulong,
                            ) as *mut otl_Anchor;
                            let mut m: glyphclass_t = 0 as glyphclass_t;
                            while (m as ::core::ffi::c_int)
                                < (*subtable).classCount as ::core::ffi::c_int
                            {
                                let mut anchorOffset: u32 =
                                    read_16u(data.offset(_offset as isize) as *const u8)
                                        as u32;
                                if anchorOffset != 0 {
                                    *(*lig.anchors.offset(k as isize)).offset(m as isize) =
                                        otl_read_anchor(
                                            data,
                                            tableLength,
                                            ligAttachOffset.wrapping_add(anchorOffset),
                                        );
                                } else {
                                    *(*lig.anchors.offset(k as isize)).offset(m as isize) =
                                        otl_anchor_absent();
                                }
                                _offset = _offset.wrapping_add(2 as u32);
                                m = m.wrapping_add(1);
                            }
                            k = k.wrapping_add(1);
                        }
                        otl_iLigatureArray.push.expect("non-null function pointer")(
                            &raw mut (*subtable).ligArray,
                            lig,
                        );
                        j = j.wrapping_add(1);
                    }
                    match current_block {
                        14470250473917821325 => {}
                        _ => {
                            if !marks.is_null() {
                                otl_Coverage_free(marks);
                            }
                            if !bases.is_null() {
                                otl_Coverage_free(bases);
                            }
                            return subtable as *mut otl_Subtable;
                        }
                    }
                }
            }
        }
    }
    if !marks.is_null() {
        otl_Coverage_free(marks);
    }
    if !bases.is_null() {
        otl_Coverage_free(bases);
    }
    iSubtable_gpos_markToLigature
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_dump_markToLigature(
    mut st: *const otl_Subtable,
) -> *mut json_value {
    let mut subtable: *const subtable_gpos_markToLigature = &raw const (*st).gpos_markToLigature;
    let mut _subtable: *mut json_value = json_object_new(3 as usize);
    let mut _marks: *mut json_value = json_object_new((*subtable).markArray.length);
    let mut _bases: *mut json_value = json_object_new((*subtable).ligArray.length);
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*subtable).markArray.length {
        let mut _mark: *mut json_value = json_object_new(3 as usize);
        let mut markClassName: sds = sdscatfmt(
            sdsempty(),
            b"ac_%i\0" as *const u8 as *const ::core::ffi::c_char,
            (*(*subtable).markArray.items.offset(j as isize)).markClass as ::core::ffi::c_int,
        );
        json_object_push(
            _mark,
            b"class\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new_length(
                sdslen(markClassName) as ::core::ffi::c_uint,
                markClassName as *const ::core::ffi::c_char,
            ),
        );
        sdsfree(markClassName);
        json_object_push(
            _mark,
            b"x\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*(*subtable).markArray.items.offset(j as isize)).anchor.x as i64),
        );
        json_object_push(
            _mark,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*(*subtable).markArray.items.offset(j as isize)).anchor.y as i64),
        );
        json_object_push(
            _marks,
            (*(*subtable).markArray.items.offset(j as isize)).glyph.name
                as *const ::core::ffi::c_char,
            preserialize(_mark),
        );
        j = j.wrapping_add(1);
    }
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as usize) < (*subtable).ligArray.length {
        let mut base: *mut otl_LigatureBaseRecord =
            (*subtable).ligArray.items.offset(j_0 as isize) as *mut otl_LigatureBaseRecord;
        let mut _base: *mut json_value = json_array_new((*base).componentCount as usize);
        let mut k: glyphid_t = 0 as glyphid_t;
        while (k as ::core::ffi::c_int) < (*base).componentCount as ::core::ffi::c_int {
            let mut _bk: *mut json_value = json_object_new((*subtable).classCount as usize);
            let mut m: glyphclass_t = 0 as glyphclass_t;
            while (m as ::core::ffi::c_int) < (*subtable).classCount as ::core::ffi::c_int {
                if (*(*(*base).anchors.offset(k as isize)).offset(m as isize)).present {
                    let mut _anchor: *mut json_value = json_object_new(2 as usize);
                    json_object_push(
                        _anchor,
                        b"x\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer_new(
                            (*(*(*base).anchors.offset(k as isize)).offset(m as isize)).x
                                as i64,
                        ),
                    );
                    json_object_push(
                        _anchor,
                        b"y\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer_new(
                            (*(*(*base).anchors.offset(k as isize)).offset(m as isize)).y
                                as i64,
                        ),
                    );
                    let mut markClassName_0: sds = sdscatfmt(
                        sdsempty(),
                        b"ac_%i\0" as *const u8 as *const ::core::ffi::c_char,
                        m as ::core::ffi::c_int,
                    );
                    json_object_push_length(
                        _bk,
                        sdslen(markClassName_0) as ::core::ffi::c_uint,
                        markClassName_0 as *const ::core::ffi::c_char,
                        _anchor,
                    );
                    sdsfree(markClassName_0);
                }
                m = m.wrapping_add(1);
            }
            json_array_push(_base, _bk);
            k = k.wrapping_add(1);
        }
        json_object_push(
            _bases,
            (*base).glyph.name as *const ::core::ffi::c_char,
            preserialize(_base),
        );
        j_0 = j_0.wrapping_add(1);
    }
    json_object_push(
        _subtable,
        b"classCount\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*subtable).classCount as i64),
    );
    json_object_push(
        _subtable,
        b"marks\0" as *const u8 as *const ::core::ffi::c_char,
        _marks,
    );
    json_object_push(
        _subtable,
        b"bases\0" as *const u8 as *const ::core::ffi::c_char,
        _bases,
    );
    return _subtable;
}
unsafe extern "C" fn parseBases(
    mut _bases: *mut json_value,
    mut subtable: *mut subtable_gpos_markToLigature,
    mut h: *mut *mut otl_ClassnameHash,
    mut options: *const otfcc_Options,
) {
    let mut classCount: glyphclass_t = (if !(*h).is_null() {
        (*(**h).hh.tbl).num_items
    } else {
        0 as ::core::ffi::c_uint
    }) as glyphclass_t;
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_uint) < (*_bases).u.object.length {
        let mut gname: *mut ::core::ffi::c_char =
            (*(*_bases).u.object.values.offset(j as isize)).name;
        let mut lig: otl_LigatureBaseRecord = otl_LigatureBaseRecord {
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            componentCount: 0,
            anchors: ::core::ptr::null_mut::<*mut otl_Anchor>(),
        };
        lig.componentCount = 0 as glyphid_t;
        lig.anchors = ::core::ptr::null_mut::<*mut otl_Anchor>();
        lig.glyph = handle_fromName(sdsnewlen(
            (*(*_bases).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
            (*(*_bases).u.object.values.offset(j as isize)).name_length as usize,
        )) as otfcc_GlyphHandle;
        let mut baseRecord: *mut json_value =
            (*(*_bases).u.object.values.offset(j as isize)).value as *mut json_value;
        if baseRecord.is_null()
            || (*baseRecord).type_0 as ::core::ffi::c_uint
                != json_array as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            otl_iLigatureArray.push.expect("non-null function pointer")(
                &raw mut (*subtable).ligArray,
                lig,
            );
        } else {
            lig.componentCount = (*baseRecord).u.array.length as glyphid_t;
            lig.anchors = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut otl_Anchor>() as usize)
                    .wrapping_mul(lig.componentCount as usize),
                146 as ::core::ffi::c_ulong,
            ) as *mut *mut otl_Anchor;
            let mut k: glyphid_t = 0 as glyphid_t;
            while (k as ::core::ffi::c_int) < lig.componentCount as ::core::ffi::c_int {
                let mut _componentRecord: *mut json_value =
                    *(*baseRecord).u.array.values.offset(k as isize) as *mut json_value;
                let ref mut fresh6 = *lig.anchors.offset(k as isize);
                *fresh6 = __caryll_allocate_clean(
                    (::core::mem::size_of::<otl_Anchor>() as usize)
                        .wrapping_mul(classCount as usize),
                    150 as ::core::ffi::c_ulong,
                ) as *mut otl_Anchor;
                let mut m: glyphclass_t = 0 as glyphclass_t;
                while (m as ::core::ffi::c_int) < classCount as ::core::ffi::c_int {
                    *(*lig.anchors.offset(k as isize)).offset(m as isize) = otl_anchor_absent();
                    m = m.wrapping_add(1);
                }
                if !(_componentRecord.is_null()
                    || (*_componentRecord).type_0 as ::core::ffi::c_uint
                        != json_object as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    let mut m_0: glyphclass_t = 0 as glyphclass_t;
                    while (m_0 as ::core::ffi::c_uint) < (*_componentRecord).u.object.length {
                        let mut className: sds = sdsnewlen(
                            (*(*_componentRecord).u.object.values.offset(m_0 as isize)).name
                                as *const ::core::ffi::c_void,
                            (*(*_componentRecord).u.object.values.offset(m_0 as isize)).name_length
                                as usize,
                        );
                        let mut s: *mut otl_ClassnameHash =
                            ::core::ptr::null_mut::<otl_ClassnameHash>();
                        let mut _hf_hashv: ::core::ffi::c_uint = 0;
                        let mut _hj_i: ::core::ffi::c_uint = 0;
                        let mut _hj_j: ::core::ffi::c_uint = 0;
                        let mut _hj_k: ::core::ffi::c_uint = 0;
                        let mut _hj_key: *const ::core::ffi::c_uchar =
                            className as *const ::core::ffi::c_uchar;
                        _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
                        _hj_i = _hj_j;
                        _hj_k =
                            strlen(className as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
                        while _hj_k >= 12 as ::core::ffi::c_uint {
                            _hj_i = _hj_i.wrapping_add(
                                (*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 8 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(
                                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 16 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(
                                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int,
                                    ),
                            );
                            _hj_j = _hj_j.wrapping_add(
                                (*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 8 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(
                                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 16 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(
                                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int,
                                    ),
                            );
                            _hf_hashv = _hf_hashv.wrapping_add(
                                (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 8 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(
                                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 16 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(
                                        (*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
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
                            .wrapping_add(strlen(className as *const ::core::ffi::c_char)
                                as ::core::ffi::c_uint);
                        let mut current_block_60: u64;
                        match _hj_k {
                            11 => {
                                _hf_hashv = _hf_hashv.wrapping_add(
                                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_60 = 9227261782747844496;
                            }
                            10 => {
                                current_block_60 = 9227261782747844496;
                            }
                            9 => {
                                current_block_60 = 18202155370509119360;
                            }
                            8 => {
                                current_block_60 = 5681848287071205093;
                            }
                            7 => {
                                current_block_60 = 4599947766850985381;
                            }
                            6 => {
                                current_block_60 = 1884041102650695646;
                            }
                            5 => {
                                current_block_60 = 4244705422846740112;
                            }
                            4 => {
                                current_block_60 = 12409020096634314305;
                            }
                            3 => {
                                current_block_60 = 12224275105439652028;
                            }
                            2 => {
                                current_block_60 = 16847718851714741986;
                            }
                            1 => {
                                current_block_60 = 17727222704389703247;
                            }
                            _ => {
                                current_block_60 = 2116367355679836638;
                            }
                        }
                        match current_block_60 {
                            9227261782747844496 => {
                                _hf_hashv = _hf_hashv.wrapping_add(
                                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_60 = 18202155370509119360;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            18202155370509119360 => {
                                _hf_hashv = _hf_hashv.wrapping_add(
                                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_60 = 5681848287071205093;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            5681848287071205093 => {
                                _hj_j = _hj_j.wrapping_add(
                                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_60 = 4599947766850985381;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            4599947766850985381 => {
                                _hj_j = _hj_j.wrapping_add(
                                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_60 = 1884041102650695646;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            1884041102650695646 => {
                                _hj_j = _hj_j.wrapping_add(
                                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_60 = 4244705422846740112;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            4244705422846740112 => {
                                _hj_j = _hj_j.wrapping_add(
                                    *_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint,
                                );
                                current_block_60 = 12409020096634314305;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            12409020096634314305 => {
                                _hj_i = _hj_i.wrapping_add(
                                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_60 = 12224275105439652028;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            12224275105439652028 => {
                                _hj_i = _hj_i.wrapping_add(
                                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_60 = 16847718851714741986;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            16847718851714741986 => {
                                _hj_i = _hj_i.wrapping_add(
                                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_60 = 17727222704389703247;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            17727222704389703247 => {
                                _hj_i = _hj_i.wrapping_add(
                                    *_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint,
                                );
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
                        s = ::core::ptr::null_mut::<otl_ClassnameHash>();
                        if !(*h).is_null() {
                            let mut _hf_bkt: ::core::ffi::c_uint = 0;
                            _hf_bkt = _hf_hashv
                                & (*(**h).hh.tbl)
                                    .num_buckets
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                if !(*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize))
                                    .hh_head
                                    .is_null()
                                {
                                    s = ((*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                                        as *mut ::core::ffi::c_char)
                                        .offset(-(*(**h).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut otl_ClassnameHash
                                        as *mut otl_ClassnameHash;
                                } else {
                                    s = ::core::ptr::null_mut::<otl_ClassnameHash>();
                                }
                                while !s.is_null() {
                                    if (*s).hh.hashv == _hf_hashv
                                        && (*s).hh.keylen
                                            == strlen(className as *const ::core::ffi::c_char)
                                                as ::core::ffi::c_uint
                                    {
                                        if memcmp(
                                            (*s).hh.key,
                                            className as *const ::core::ffi::c_void,
                                            strlen(className as *const ::core::ffi::c_char)
                                                as ::core::ffi::c_uint
                                                as usize,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            break;
                                        }
                                    }
                                    if !(*s).hh.hh_next.is_null() {
                                        s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                            .offset(-(*(**h).hh.tbl).hho)
                                            as *mut ::core::ffi::c_void
                                            as *mut otl_ClassnameHash
                                            as *mut otl_ClassnameHash;
                                    } else {
                                        s = ::core::ptr::null_mut::<otl_ClassnameHash>();
                                    }
                                }
                            }
                        }
                        if s.is_null() {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[OTFCC-fea] Invalid anchor class name <%s> for /%s. This base anchor is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    className,
                                    gname,
                                ),
                            );
                        } else {
                            *(*lig.anchors.offset(k as isize)).offset((*s).classID as isize) =
                                otl_parse_anchor(
                                    (*(*_componentRecord).u.object.values.offset(m_0 as isize))
                                        .value
                                        as *mut json_value,
                                );
                        }
                        sdsfree(className);
                        m_0 = m_0.wrapping_add(1);
                    }
                }
                k = k.wrapping_add(1);
            }
            otl_iLigatureArray.push.expect("non-null function pointer")(
                &raw mut (*subtable).ligArray,
                lig,
            );
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_parse_markToLigature(
    mut _subtable: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut _marks: *mut json_value = json_obj_get_type(
        _subtable,
        b"marks\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    let mut _bases: *mut json_value = json_obj_get_type(
        _subtable,
        b"bases\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if _marks.is_null() || _bases.is_null() {
        return ::core::ptr::null_mut::<otl_Subtable>();
    }
    let mut st: *mut subtable_gpos_markToLigature =
        (
            iSubtable_gpos_markToLigature
                .create
                .expect("non-null function pointer"))();
    let mut h: *mut otl_ClassnameHash = ::core::ptr::null_mut::<otl_ClassnameHash>();
    otl_parseMarkArray(_marks, &raw mut (*st).markArray, &raw mut h, options);
    (*st).classCount = (if !h.is_null() {
        (*(*h).hh.tbl).num_items
    } else {
        0 as ::core::ffi::c_uint
    }) as glyphclass_t;
    parseBases(_bases, st, &raw mut h, options);
    let mut s: *mut otl_ClassnameHash = ::core::ptr::null_mut::<otl_ClassnameHash>();
    let mut tmp: *mut otl_ClassnameHash = ::core::ptr::null_mut::<otl_ClassnameHash>();
    s = h;
    tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut otl_ClassnameHash
        as *mut otl_ClassnameHash;
    while !s.is_null() {
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*h).hh.tbl as *mut ::core::ffi::c_void);
            h = ::core::ptr::null_mut::<otl_ClassnameHash>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*h).hh.tbl).tail {
                (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh4 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh4 = (*_hd_hh_del).next;
            } else {
                h = (*_hd_hh_del).next as *mut otl_ClassnameHash as *mut otl_ClassnameHash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh5 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh5 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(*h).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UT_hash_bucket;
            (*_hd_head).count = (*_hd_head).count.wrapping_sub(1);
            if (*_hd_head).hh_head == _hd_hh_del {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_sub(1);
        }
        sdsfree((*s).className);
        free(s as *mut ::core::ffi::c_void);
        s = ::core::ptr::null_mut::<otl_ClassnameHash>();
        s = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut otl_ClassnameHash
            as *mut otl_ClassnameHash;
    }
    return st as *mut otl_Subtable;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gpos_markToLigature(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gpos_markToLigature =
        &raw const (*_subtable).gpos_markToLigature;
    let mut marks: *mut otl_Coverage = otl_Coverage_create();
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*subtable).markArray.length {
        pushToCoverage(
            marks,
            otfcc_Handle_dup(
                (*(*subtable).markArray.items.offset(j as isize)).glyph as otfcc_Handle,
            ) as otfcc_GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let mut bases: *mut otl_Coverage = otl_Coverage_create();
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as usize) < (*subtable).ligArray.length {
        pushToCoverage(
            bases,
            otfcc_Handle_dup(
                (*(*subtable).ligArray.items.offset(j_0 as isize)).glyph as otfcc_Handle,
            ) as otfcc_GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
            marks,
        )),
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
            bases,
        )),
        b16 as ::core::ffi::c_int,
        (*subtable).classCount as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut markArray: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        (*subtable).markArray.length,
        bkover as ::core::ffi::c_int,
    );
    let mut j_1: glyphid_t = 0 as glyphid_t;
    while (j_1 as usize) < (*subtable).markArray.length {
        bk_push(
            markArray,
            b16 as ::core::ffi::c_int,
            (*(*subtable).markArray.items.offset(j_1 as isize)).markClass as ::core::ffi::c_int,
            p16 as ::core::ffi::c_int,
            bkFromAnchor((*(*subtable).markArray.items.offset(j_1 as isize)).anchor),
            bkover as ::core::ffi::c_int,
        );
        j_1 = j_1.wrapping_add(1);
    }
    let mut ligatureArray: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        (*subtable).ligArray.length,
        bkover as ::core::ffi::c_int,
    );
    let mut j_2: glyphid_t = 0 as glyphid_t;
    while (j_2 as usize) < (*subtable).ligArray.length {
        let mut attach: *mut bk_Block = bk_new_Block(
            b16 as ::core::ffi::c_int,
            (*(*subtable).ligArray.items.offset(j_2 as isize)).componentCount as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
        let mut k: glyphid_t = 0 as glyphid_t;
        while (k as ::core::ffi::c_int)
            < (*(*subtable).ligArray.items.offset(j_2 as isize)).componentCount
                as ::core::ffi::c_int
        {
            let mut m: glyphclass_t = 0 as glyphclass_t;
            while (m as ::core::ffi::c_int) < (*subtable).classCount as ::core::ffi::c_int {
                bk_push(
                    attach,
                    p16 as ::core::ffi::c_int,
                    bkFromAnchor(
                        *(*(*(*subtable).ligArray.items.offset(j_2 as isize))
                            .anchors
                            .offset(k as isize))
                        .offset(m as isize),
                    ),
                    bkover as ::core::ffi::c_int,
                );
                m = m.wrapping_add(1);
            }
            k = k.wrapping_add(1);
        }
        bk_push(
            ligatureArray,
            p16 as ::core::ffi::c_int,
            attach,
            bkover as ::core::ffi::c_int,
        );
        j_2 = j_2.wrapping_add(1);
    }
    bk_push(
        root,
        p16 as ::core::ffi::c_int,
        markArray,
        p16 as ::core::ffi::c_int,
        ligatureArray,
        bkover as ::core::ffi::c_int,
    );
    otl_Coverage_free(marks);
    otl_Coverage_free(bases);
    return bk_build_Block(root);
}
