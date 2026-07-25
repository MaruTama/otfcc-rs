#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memcpy, memset, qsort, strcmp};
unsafe extern "C" {
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
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
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}


use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage, otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage};
use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_dispose, otfcc_Handle, otfcc_GlyphHandle, HANDLE_STATE_EMPTY};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphid_t};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, sds, sdshdr16, sdshdr32, sdshdr64, sdshdr8};
use crate::vendor::json::{json_array, json_object, json_pre_serialized, json_string, json_type, json_value};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{b16, bk_Block, bk_int, bk_new_Block, bk_ptr, bk_push, p16};
use crate::support::{NULL, __compar_fn_t};
use crate::table::otl::{__caryll_vectorinterface_subtable_gsub_ligature, otl_GsubLigatureEntry, otl_Subtable, subtable_gsub_ligature};
use crate::table::otl::subtables::{otl_BuildHeuristics};
use crate::vendor::json_builder::{json_serialize_mode_packed, json_serialize_opts};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UT_hash_bucket, UT_hash_handle, UT_hash_table};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_GsubLigatureEntry {
    pub init: Option<unsafe extern "C" fn(*mut otl_GsubLigatureEntry) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut otl_GsubLigatureEntry, *const otl_GsubLigatureEntry) -> (),
    >,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_GsubLigatureEntry, *mut otl_GsubLigatureEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_GsubLigatureEntry) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut otl_GsubLigatureEntry, otl_GsubLigatureEntry) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_GsubLigatureEntry, otl_GsubLigatureEntry) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ligature_aggerator {
    pub gid: ::core::ffi::c_int,
    pub hh: UT_hash_handle,
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
unsafe extern "C" fn deleteGsubLigatureEntry(mut entry: *mut otl_GsubLigatureEntry) {
    otfcc_Handle_dispose(&raw mut (*entry).to);
    otl_Coverage_free((*entry).from);
    (*entry).from = ::core::ptr::null_mut::<otl_Coverage>();
}
static gss_typeinfo: __caryll_elementinterface_otl_GsubLigatureEntry = {
    __caryll_elementinterface_otl_GsubLigatureEntry {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(
            deleteGsubLigatureEntry as unsafe extern "C" fn(*mut otl_GsubLigatureEntry) -> (),
        ),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_move(dst: *mut subtable_gsub_ligature, src: *mut subtable_gsub_ligature) {
    cvec_move(as_cvec(dst), as_cvec(src));
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_growTo(arr: *mut subtable_gsub_ligature, target: usize) {
    cvec_grow_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_free(mut x: *mut subtable_gsub_ligature) {
    if x.is_null() {
        return;
    }
    subtable_gsub_ligature_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_pop(arr: *mut subtable_gsub_ligature) -> otl_GsubLigatureEntry {
    cvec_pop(as_cvec(arr))
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_copyReplace(
    mut dst: *mut subtable_gsub_ligature,
    src: subtable_gsub_ligature,
) {
    subtable_gsub_ligature_dispose(dst);
    subtable_gsub_ligature_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_copy(
    mut dst: *mut subtable_gsub_ligature,
    mut src: *const subtable_gsub_ligature,
) {
    subtable_gsub_ligature_init(dst);
    subtable_gsub_ligature_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if gss_typeinfo.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            gss_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_GsubLigatureEntry,
                (*src).items.offset(j as isize) as *mut otl_GsubLigatureEntry
                    as *const otl_GsubLigatureEntry,
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
unsafe extern "C" fn subtable_gsub_ligature_dispose(mut arr: *mut subtable_gsub_ligature) {
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
                (*arr).items.offset(j as isize) as *mut otl_GsubLigatureEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_GsubLigatureEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_replace(
    mut dst: *mut subtable_gsub_ligature,
    src: subtable_gsub_ligature,
) {
    subtable_gsub_ligature_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gsub_ligature>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_initCapN(
    mut arr: *mut subtable_gsub_ligature,
    mut n: usize,
) {
    subtable_gsub_ligature_init(arr);
    subtable_gsub_ligature_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_growToN(arr: *mut subtable_gsub_ligature, target: usize) {
    cvec_grow_to_n(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_initN(
    mut arr: *mut subtable_gsub_ligature,
    mut n: usize,
) {
    subtable_gsub_ligature_init(arr);
    subtable_gsub_ligature_growToN(arr, n);
    subtable_gsub_ligature_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_create() -> *mut subtable_gsub_ligature {
    let mut x: *mut subtable_gsub_ligature =
        malloc(::core::mem::size_of::<subtable_gsub_ligature>() as usize)
            as *mut subtable_gsub_ligature;
    subtable_gsub_ligature_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_createN(mut n: usize) -> *mut subtable_gsub_ligature {
    let mut t: *mut subtable_gsub_ligature =
        malloc(::core::mem::size_of::<subtable_gsub_ligature>() as usize)
            as *mut subtable_gsub_ligature;
    subtable_gsub_ligature_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_filterEnv(
    mut arr: *mut subtable_gsub_ligature,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_GsubLigatureEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_GsubLigatureEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if gss_typeinfo.dispose.is_some() {
                gss_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_GsubLigatureEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn as_cvec(arr: *mut subtable_gsub_ligature) -> *mut CVecRaw<otl_GsubLigatureEntry> {
    arr as *mut CVecRaw<otl_GsubLigatureEntry>
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_init(arr: *mut subtable_gsub_ligature) {
    cvec_init(as_cvec(arr));
}
#[unsafe(no_mangle)]
pub static iSubtable_gsub_ligature: __caryll_vectorinterface_subtable_gsub_ligature = {
    __caryll_vectorinterface_subtable_gsub_ligature {
        init: Some(
            subtable_gsub_ligature_init as unsafe extern "C" fn(*mut subtable_gsub_ligature) -> (),
        ),
        copy: Some(
            subtable_gsub_ligature_copy
                as unsafe extern "C" fn(
                    *mut subtable_gsub_ligature,
                    *const subtable_gsub_ligature,
                ) -> (),
        ),
        move_0: Some(
            subtable_gsub_ligature_move
                as unsafe extern "C" fn(
                    *mut subtable_gsub_ligature,
                    *mut subtable_gsub_ligature,
                ) -> (),
        ),
        dispose: Some(
            subtable_gsub_ligature_dispose
                as unsafe extern "C" fn(*mut subtable_gsub_ligature) -> (),
        ),
        replace: Some(
            subtable_gsub_ligature_replace
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, subtable_gsub_ligature) -> (),
        ),
        copyReplace: Some(
            subtable_gsub_ligature_copyReplace
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, subtable_gsub_ligature) -> (),
        ),
        create: Some(subtable_gsub_ligature_create),
        free: Some(
            subtable_gsub_ligature_free as unsafe extern "C" fn(*mut subtable_gsub_ligature) -> (),
        ),
        initN: Some(
            subtable_gsub_ligature_initN
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, usize) -> (),
        ),
        initCapN: Some(
            subtable_gsub_ligature_initCapN
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, usize) -> (),
        ),
        createN: Some(
            subtable_gsub_ligature_createN
                as unsafe extern "C" fn(usize) -> *mut subtable_gsub_ligature,
        ),
        fill: Some(
            subtable_gsub_ligature_fill
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, usize) -> (),
        ),
        clear: Some(
            subtable_gsub_ligature_dispose
                as unsafe extern "C" fn(*mut subtable_gsub_ligature) -> (),
        ),
        push: Some(
            subtable_gsub_ligature_push
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, otl_GsubLigatureEntry) -> (),
        ),
        shrinkToFit: Some(
            subtable_gsub_ligature_shrinkToFit
                as unsafe extern "C" fn(*mut subtable_gsub_ligature) -> (),
        ),
        pop: Some(
            subtable_gsub_ligature_pop
                as unsafe extern "C" fn(*mut subtable_gsub_ligature) -> otl_GsubLigatureEntry,
        ),
        disposeItem: Some(
            subtable_gsub_ligature_disposeItem
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, usize) -> (),
        ),
        filterEnv: Some(
            subtable_gsub_ligature_filterEnv
                as unsafe extern "C" fn(
                    *mut subtable_gsub_ligature,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GsubLigatureEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gsub_ligature_sort
                as unsafe extern "C" fn(
                    *mut subtable_gsub_ligature,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GsubLigatureEntry,
                            *const otl_GsubLigatureEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_resizeTo(arr: *mut subtable_gsub_ligature, target: usize) {
    cvec_resize_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_shrinkToFit(mut arr: *mut subtable_gsub_ligature) {
    subtable_gsub_ligature_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_disposeItem(
    mut arr: *mut subtable_gsub_ligature,
    mut n: usize,
) {
    if gss_typeinfo.dispose.is_some() {
        gss_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_GsubLigatureEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_sort(
    mut arr: *mut subtable_gsub_ligature,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otl_GsubLigatureEntry,
            *const otl_GsubLigatureEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_GsubLigatureEntry>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubLigatureEntry,
                    *const otl_GsubLigatureEntry,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_fill(
    mut arr: *mut subtable_gsub_ligature,
    mut n: usize,
) {
    while (*arr).length < n {
        let mut x: otl_GsubLigatureEntry = otl_GsubLigatureEntry {
            from: ::core::ptr::null_mut::<otl_Coverage>(),
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
                ::core::mem::size_of::<otl_GsubLigatureEntry>() as usize,
            );
        }
        subtable_gsub_ligature_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_push(arr: *mut subtable_gsub_ligature, elem: otl_GsubLigatureEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_grow(arr: *mut subtable_gsub_ligature) {
    cvec_grow(as_cvec(arr));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otl_read_gsub_ligature(
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut startCoverage: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    let mut setCount: glyphid_t = 0;
    let mut ligatureCount: u32 = 0;
    let mut current_block: u64;
    let mut subtable: *mut subtable_gsub_ligature =
        (
            iSubtable_gsub_ligature
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
            ) as glyphid_t;
            if !(setCount as ::core::ffi::c_int != (*startCoverage).numGlyphs as ::core::ffi::c_int)
            {
                if !(tableLength
                    < offset.wrapping_add(6 as u32).wrapping_add(
                        (setCount as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                    ))
                {
                    ligatureCount = 0 as u32;
                    let mut j: glyphid_t = 0 as glyphid_t;
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
                            let mut j_0: glyphid_t = 0 as glyphid_t;
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
                                let mut lc: glyphid_t =
                                    read_16u(data.offset(setOffset_0 as isize) as *const u8)
                                        as glyphid_t;
                                let mut k: glyphid_t = 0 as glyphid_t;
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
                                    let mut ligComponents: glyphid_t = read_16u(
                                        data.offset(ligOffset as isize)
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            as *const u8,
                                    )
                                        as glyphid_t;
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
                                    let mut cov: *mut otl_Coverage =
                                        otl_Coverage_create();
                                    pushToCoverage(
                                        cov,
                                        handle_fromIndex(
                                            (*(*startCoverage).glyphs.offset(j_0 as isize)).index,
                                        )
                                            as otfcc_GlyphHandle,
                                    );
                                    let mut m: glyphid_t = 1 as glyphid_t;
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
                                                    as glyphid_t,
                                            )
                                                as otfcc_GlyphHandle,
                                        );
                                        m = m.wrapping_add(1);
                                    }
                                    iSubtable_gsub_ligature
                                        .push
                                        .expect("non-null function pointer")(
                                        subtable,
                                        otl_GsubLigatureEntry {
                                            from: cov,
                                            to: handle_fromIndex(
                                                read_16u(data.offset(ligOffset as isize)
                                                    as *const u8)
                                                    as glyphid_t,
                                            )
                                                as otfcc_GlyphHandle,
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
                                    return subtable as *mut otl_Subtable;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    iSubtable_gsub_ligature
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otl_gsub_dump_ligature(
    mut _subtable: *const otl_Subtable,
) -> *mut json_value {
    let mut subtable: *const subtable_gsub_ligature = &raw const (*_subtable).gsub_ligature;
    let mut st: *mut json_value = json_array_new((*subtable).length);
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*subtable).length {
        let mut entry: *mut json_value = json_object_new(2 as usize);
        json_object_push(
            entry,
            b"from\0" as *const u8 as *const ::core::ffi::c_char,
            otl_iCoverage.dump.expect("non-null function pointer")(
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
    let mut ret: *mut json_value = json_object_new(1 as usize);
    json_object_push(
        ret,
        b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
        st,
    );
    return ret;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otl_gsub_parse_ligature(
    mut _subtable: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    if !json_obj_get_type(
        _subtable,
        b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    )
    .is_null()
    {
        _subtable = json_obj_get_type(
            _subtable,
            b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
            json_array,
        );
        let mut st: *mut subtable_gsub_ligature =
            (
                iSubtable_gsub_ligature
                    .create
                    .expect("non-null function pointer"))();
        let mut n: glyphid_t = (*_subtable).u.array.length as glyphid_t;
        let mut k: glyphid_t = 0 as glyphid_t;
        while (k as ::core::ffi::c_int) < n as ::core::ffi::c_int {
            let mut entry: *mut json_value =
                *(*_subtable).u.array.values.offset(k as isize) as *mut json_value;
            let mut _from: *mut json_value = json_obj_get_type(
                entry,
                b"from\0" as *const u8 as *const ::core::ffi::c_char,
                json_array,
            );
            let mut _to: *mut json_value = json_obj_get_type(
                entry,
                b"to\0" as *const u8 as *const ::core::ffi::c_char,
                json_string,
            );
            if !(_from.is_null() || _to.is_null()) {
                iSubtable_gsub_ligature
                    .push
                    .expect("non-null function pointer")(
                    st,
                    otl_GsubLigatureEntry {
                        from: otl_iCoverage.parse.expect("non-null function pointer")(_from),
                        to: handle_fromName(sdsnewlen(
                            (*_to).u.string.ptr as *const ::core::ffi::c_void,
                            (*_to).u.string.length as usize,
                        )) as otfcc_GlyphHandle,
                    },
                );
            }
            k = k.wrapping_add(1);
        }
        return st as *mut otl_Subtable;
    } else {
        let mut st_0: *mut subtable_gsub_ligature =
            (
                iSubtable_gsub_ligature
                    .create
                    .expect("non-null function pointer"))();
        let mut n_0: glyphid_t = (*_subtable).u.array.length as glyphid_t;
        let mut k_0: glyphid_t = 0 as glyphid_t;
        while (k_0 as ::core::ffi::c_int) < n_0 as ::core::ffi::c_int {
            let mut _from_0: *mut json_value =
                (*(*_subtable).u.object.values.offset(k_0 as isize)).value as *mut json_value;
            if !(_from_0.is_null()
                || (*_from_0).type_0 as ::core::ffi::c_uint
                    != json_array as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                iSubtable_gsub_ligature
                    .push
                    .expect("non-null function pointer")(
                    st_0,
                    otl_GsubLigatureEntry {
                        from: otl_iCoverage.parse.expect("non-null function pointer")(_from_0),
                        to: handle_fromName(sdsnewlen(
                            (*(*_subtable).u.object.values.offset(k_0 as isize)).name
                                as *const ::core::ffi::c_void,
                            (*(*_subtable).u.object.values.offset(k_0 as isize)).name_length
                                as usize,
                        )) as otfcc_GlyphHandle,
                    },
                );
            }
            k_0 = k_0.wrapping_add(1);
        }
        return st_0 as *mut otl_Subtable;
    };
}
unsafe extern "C" fn by_gid(
    mut a: *mut ligature_aggerator,
    mut b: *mut ligature_aggerator,
) -> ::core::ffi::c_int {
    return (*a).gid - (*b).gid;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_build_gsub_ligature_subtable(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gsub_ligature = &raw const (*_subtable).gsub_ligature;
    let mut h: *mut ligature_aggerator = ::core::ptr::null_mut::<ligature_aggerator>();
    let mut s: *mut ligature_aggerator = ::core::ptr::null_mut::<ligature_aggerator>();
    let mut tmp: *mut ligature_aggerator = ::core::ptr::null_mut::<ligature_aggerator>();
    let mut nLigatures: glyphid_t = (*subtable).length as glyphid_t;
    let mut j: glyphid_t = 0 as glyphid_t;
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
        s = ::core::ptr::null_mut::<ligature_aggerator>();
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
                        as *mut ligature_aggerator
                        as *mut ligature_aggerator;
                } else {
                    s = ::core::ptr::null_mut::<ligature_aggerator>();
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
                            as *mut ligature_aggerator
                            as *mut ligature_aggerator;
                    } else {
                        s = ::core::ptr::null_mut::<ligature_aggerator>();
                    }
                }
            }
        }
        if s.is_null() {
            s = __caryll_allocate_clean(
                ::core::mem::size_of::<ligature_aggerator>() as usize,
                132 as ::core::ffi::c_ulong,
            ) as *mut ligature_aggerator;
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
                (*s).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                    as *mut UT_hash_table as *mut UT_hash_table;
                if (*s).hh.tbl.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*s).hh.tbl as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<UT_hash_table>() as usize,
                    );
                    (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                    (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                    (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                    (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                        .offset_from(s as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        as isize;
                    (*(*s).hh.tbl).buckets = malloc(
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    ) as *mut UT_hash_bucket;
                    (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                    if (*(*s).hh.tbl).buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
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
                (*(*h).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
            }
            let mut _ha_bkt: ::core::ffi::c_uint = 0;
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt = _ha_hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head: *mut UT_hash_bucket =
                (*(*h).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
            (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
            (*s).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
            (*s).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
            if !(*_ha_head).hh_head.is_null() {
                (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hh as *mut UT_hash_handle;
            }
            (*_ha_head).hh_head = &raw mut (*s).hh as *mut UT_hash_handle;
            if (*_ha_head).count
                >= (*_ha_head)
                    .expand_mult
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                && (*(*s).hh.tbl).noexpand == 0
            {
                let mut _he_bkt: ::core::ffi::c_uint = 0;
                let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                let mut _he_thh: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_hh_nxt: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_new_buckets: *mut UT_hash_bucket =
                    ::core::ptr::null_mut::<UT_hash_bucket>();
                let mut _he_newbkt: *mut UT_hash_bucket = ::core::ptr::null_mut::<UT_hash_bucket>();
                _he_new_buckets = malloc(
                    (2 as usize)
                        .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                ) as *mut UT_hash_bucket;
                if _he_new_buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        _he_new_buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (2 as usize)
                            .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
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
                            as *mut UT_hash_handle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*s).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                            (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                            if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                (*(*s).hh.tbl).nonideal_items =
                                    (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                (*_he_newbkt).expand_mult = (*_he_newbkt)
                                    .count
                                    .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
                            }
                            (*_he_thh).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                            (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UT_hash_handle;
                            if !(*_he_newbkt).hh_head.is_null() {
                                (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                            }
                            (*_he_newbkt).hh_head = _he_thh as *mut UT_hash_handle;
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
    let mut _hs_p: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_q: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_e: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_list: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_tail: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    if !h.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*h).hh as *mut UT_hash_handle;
        while _hs_looping != 0 as ::core::ffi::c_uint {
            _hs_p = _hs_list;
            _hs_list = ::core::ptr::null_mut::<UT_hash_handle>();
            _hs_tail = ::core::ptr::null_mut::<UT_hash_handle>();
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
                            as *mut UT_hash_handle
                    } else {
                        ::core::ptr::null_mut::<UT_hash_handle>()
                    }) as *mut UT_hash_handle;
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
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    } else if _hs_qsize == 0 as ::core::ffi::c_uint || _hs_q.is_null() {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*h).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if by_gid(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ligature_aggerator,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ligature_aggerator,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*h).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(*h).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
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
                    as *mut ::core::ffi::c_void as *mut ligature_aggerator
                    as *mut ligature_aggerator;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    let mut startcov: *mut otl_Coverage = otl_Coverage_create();
    s = h;
    while !s.is_null() {
        pushToCoverage(
            startcov,
            handle_fromIndex((*s).gid as glyphid_t)
                as otfcc_GlyphHandle,
        );
        s = (*s).hh.next as *mut ligature_aggerator;
    }
    let mut root: *mut bk_Block = bk_new_Block(&[bk_int(b16, 1 as u32), bk_ptr(p16, bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
            startcov,
        ))), bk_int(b16, ((*startcov).numGlyphs as ::core::ffi::c_int) as u32)]);
    s = h;
    while !s.is_null() {
        let mut nLigsHere: glyphid_t = 0 as glyphid_t;
        let mut j_0: glyphid_t = 0 as glyphid_t;
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
        let mut ligset: *mut bk_Block = bk_new_Block(&[bk_int(b16, (nLigsHere as ::core::ffi::c_int) as u32)]);
        let mut j_1: glyphid_t = 0 as glyphid_t;
        while (j_1 as ::core::ffi::c_int) < nLigatures as ::core::ffi::c_int {
            if (*(*(*(*subtable).items.offset(j_1 as isize)).from)
                .glyphs
                .offset(0 as ::core::ffi::c_int as isize))
            .index as ::core::ffi::c_int
                == (*s).gid
            {
                let mut ligdef: *mut bk_Block = bk_new_Block(&[bk_int(b16, ((*(*subtable).items.offset(j_1 as isize)).to.index as ::core::ffi::c_int) as u32), bk_int(b16, ((*(*(*subtable).items.offset(j_1 as isize)).from).numGlyphs
                        as ::core::ffi::c_int) as u32)]);
                let mut m: glyphid_t = 1 as glyphid_t;
                while (m as ::core::ffi::c_int)
                    < (*(*(*subtable).items.offset(j_1 as isize)).from).numGlyphs
                        as ::core::ffi::c_int
                {
                    bk_push(ligdef, &[bk_int(b16, ((*(*(*(*subtable).items.offset(j_1 as isize)).from)
                            .glyphs
                            .offset(m as isize))
                        .index as ::core::ffi::c_int) as u32)]);
                    m = m.wrapping_add(1);
                }
                bk_push(ligset, &[bk_ptr(p16, ligdef)]);
            }
            j_1 = j_1.wrapping_add(1);
        }
        bk_push(root, &[bk_ptr(p16, ligset)]);
        s = (*s).hh.next as *mut ligature_aggerator;
    }
    otl_Coverage_free(startcov);
    s = h;
    tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut ligature_aggerator
        as *mut ligature_aggerator;
    while !s.is_null() {
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*h).hh.tbl as *mut ::core::ffi::c_void);
            h = ::core::ptr::null_mut::<ligature_aggerator>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*h).hh.tbl).tail {
                (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh2 = (*_hd_hh_del).next;
            } else {
                h = (*_hd_hh_del).next as *mut ligature_aggerator as *mut ligature_aggerator;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh3 = (*_hd_hh_del).prev;
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
        free(s as *mut ::core::ffi::c_void);
        s = ::core::ptr::null_mut::<ligature_aggerator>();
        s = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut ligature_aggerator
            as *mut ligature_aggerator;
    }
    return bk_build_Block(root);
}
