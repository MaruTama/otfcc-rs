use libc::{free, malloc, memcpy, memset, qsort, strcmp};
extern "C" {
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
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
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
    fn otl_anchor_absent() -> otl_Anchor;
    fn otl_read_anchor(
        data: font_file_pointer,
        tableLength: u32,
        offset: u32,
    ) -> otl_Anchor;
    fn otl_dump_anchor(a: otl_Anchor) -> *mut json_value;
    fn otl_parse_anchor(v: *mut json_value) -> otl_Anchor;
    fn bkFromAnchor(a: otl_Anchor) -> *mut bk_Block;
}

use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage, otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage};
use crate::support::handle::{handle_fromName, otfcc_Handle_dispose, otfcc_Handle_dup, otfcc_Handle, otfcc_GlyphHandle, HANDLE_STATE_EMPTY};
use crate::support::binio::{read_16u};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_object, json_pre_serialized, json_value};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{b16, bk_Block, bkover, p16};

use crate::table::otl::{__caryll_vectorinterface_subtable_gpos_cursive, otl_Anchor, otl_GposCursiveEntry, otl_Subtable, subtable_gpos_cursive};
use crate::table::otl::subtables::{otl_BuildHeuristics};
use crate::vendor::json_builder::{json_serialize_mode_packed, json_serialize_opts};
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_GposCursiveEntry {
    pub init: Option<unsafe extern "C" fn(*mut otl_GposCursiveEntry) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_GposCursiveEntry, *const otl_GposCursiveEntry) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_GposCursiveEntry, *mut otl_GposCursiveEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_GposCursiveEntry) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut otl_GposCursiveEntry, otl_GposCursiveEntry) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_GposCursiveEntry, otl_GposCursiveEntry) -> ()>,
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
unsafe extern "C" fn deleteGposCursiveEntry(mut entry: *mut otl_GposCursiveEntry) {
    otfcc_Handle_dispose(&raw mut (*entry).target);
}
static mut gss_typeinfo: __caryll_elementinterface_otl_GposCursiveEntry = {
    __caryll_elementinterface_otl_GposCursiveEntry {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(
            deleteGposCursiveEntry as unsafe extern "C" fn(*mut otl_GposCursiveEntry) -> (),
        ),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_move(dst: *mut subtable_gpos_cursive, src: *mut subtable_gpos_cursive) {
    cvec_move(as_cvec(dst), as_cvec(src));
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_resizeTo(arr: *mut subtable_gpos_cursive, target: usize) {
    cvec_resize_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_filterEnv(
    mut arr: *mut subtable_gpos_cursive,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_GposCursiveEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_GposCursiveEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if gss_typeinfo.dispose.is_some() {
                gss_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_GposCursiveEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn as_cvec(arr: *mut subtable_gpos_cursive) -> *mut CVecRaw<otl_GposCursiveEntry> {
    arr as *mut CVecRaw<otl_GposCursiveEntry>
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_init(arr: *mut subtable_gpos_cursive) {
    cvec_init(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_disposeItem(
    mut arr: *mut subtable_gpos_cursive,
    mut n: usize,
) {
    if gss_typeinfo.dispose.is_some() {
        gss_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_GposCursiveEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_sort(
    mut arr: *mut subtable_gpos_cursive,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otl_GposCursiveEntry,
            *const otl_GposCursiveEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_GposCursiveEntry>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_GposCursiveEntry,
                    *const otl_GposCursiveEntry,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_fill(
    mut arr: *mut subtable_gpos_cursive,
    mut n: usize,
) {
    while (*arr).length < n {
        let mut x: otl_GposCursiveEntry = otl_GposCursiveEntry {
            target: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            enter: otl_Anchor {
                present: false,
                x: 0.,
                y: 0.,
            },
            exit: otl_Anchor {
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
                ::core::mem::size_of::<otl_GposCursiveEntry>() as usize,
            );
        }
        subtable_gpos_cursive_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_push(arr: *mut subtable_gpos_cursive, elem: otl_GposCursiveEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_grow(arr: *mut subtable_gpos_cursive) {
    cvec_grow(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_growTo(arr: *mut subtable_gpos_cursive, target: usize) {
    cvec_grow_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_pop(arr: *mut subtable_gpos_cursive) -> otl_GposCursiveEntry {
    cvec_pop(as_cvec(arr))
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_copyReplace(
    mut dst: *mut subtable_gpos_cursive,
    src: subtable_gpos_cursive,
) {
    subtable_gpos_cursive_dispose(dst);
    subtable_gpos_cursive_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_copy(
    mut dst: *mut subtable_gpos_cursive,
    mut src: *const subtable_gpos_cursive,
) {
    subtable_gpos_cursive_init(dst);
    subtable_gpos_cursive_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if gss_typeinfo.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            gss_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_GposCursiveEntry,
                (*src).items.offset(j as isize) as *mut otl_GposCursiveEntry
                    as *const otl_GposCursiveEntry,
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
unsafe extern "C" fn subtable_gpos_cursive_dispose(mut arr: *mut subtable_gpos_cursive) {
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
                (*arr).items.offset(j as isize) as *mut otl_GposCursiveEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_GposCursiveEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_replace(
    mut dst: *mut subtable_gpos_cursive,
    src: subtable_gpos_cursive,
) {
    subtable_gpos_cursive_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_cursive>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_initCapN(
    mut arr: *mut subtable_gpos_cursive,
    mut n: usize,
) {
    subtable_gpos_cursive_init(arr);
    subtable_gpos_cursive_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_growToN(arr: *mut subtable_gpos_cursive, target: usize) {
    cvec_grow_to_n(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_initN(
    mut arr: *mut subtable_gpos_cursive,
    mut n: usize,
) {
    subtable_gpos_cursive_init(arr);
    subtable_gpos_cursive_growToN(arr, n);
    subtable_gpos_cursive_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_free(mut x: *mut subtable_gpos_cursive) {
    if x.is_null() {
        return;
    }
    subtable_gpos_cursive_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_createN(mut n: usize) -> *mut subtable_gpos_cursive {
    let mut t: *mut subtable_gpos_cursive =
        malloc(::core::mem::size_of::<subtable_gpos_cursive>() as usize)
            as *mut subtable_gpos_cursive;
    subtable_gpos_cursive_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_create() -> *mut subtable_gpos_cursive {
    let mut x: *mut subtable_gpos_cursive =
        malloc(::core::mem::size_of::<subtable_gpos_cursive>() as usize)
            as *mut subtable_gpos_cursive;
    subtable_gpos_cursive_init(x);
    return x;
}
#[no_mangle]
pub static mut iSubtable_gpos_cursive: __caryll_vectorinterface_subtable_gpos_cursive = {
    __caryll_vectorinterface_subtable_gpos_cursive {
        init: Some(
            subtable_gpos_cursive_init as unsafe extern "C" fn(*mut subtable_gpos_cursive) -> (),
        ),
        copy: Some(
            subtable_gpos_cursive_copy
                as unsafe extern "C" fn(
                    *mut subtable_gpos_cursive,
                    *const subtable_gpos_cursive,
                ) -> (),
        ),
        move_0: Some(
            subtable_gpos_cursive_move
                as unsafe extern "C" fn(
                    *mut subtable_gpos_cursive,
                    *mut subtable_gpos_cursive,
                ) -> (),
        ),
        dispose: Some(
            subtable_gpos_cursive_dispose as unsafe extern "C" fn(*mut subtable_gpos_cursive) -> (),
        ),
        replace: Some(
            subtable_gpos_cursive_replace
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, subtable_gpos_cursive) -> (),
        ),
        copyReplace: Some(
            subtable_gpos_cursive_copyReplace
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, subtable_gpos_cursive) -> (),
        ),
        create: Some(subtable_gpos_cursive_create),
        free: Some(
            subtable_gpos_cursive_free as unsafe extern "C" fn(*mut subtable_gpos_cursive) -> (),
        ),
        initN: Some(
            subtable_gpos_cursive_initN
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, usize) -> (),
        ),
        initCapN: Some(
            subtable_gpos_cursive_initCapN
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, usize) -> (),
        ),
        createN: Some(
            subtable_gpos_cursive_createN
                as unsafe extern "C" fn(usize) -> *mut subtable_gpos_cursive,
        ),
        fill: Some(
            subtable_gpos_cursive_fill
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, usize) -> (),
        ),
        clear: Some(
            subtable_gpos_cursive_dispose as unsafe extern "C" fn(*mut subtable_gpos_cursive) -> (),
        ),
        push: Some(
            subtable_gpos_cursive_push
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, otl_GposCursiveEntry) -> (),
        ),
        shrinkToFit: Some(
            subtable_gpos_cursive_shrinkToFit
                as unsafe extern "C" fn(*mut subtable_gpos_cursive) -> (),
        ),
        pop: Some(
            subtable_gpos_cursive_pop
                as unsafe extern "C" fn(*mut subtable_gpos_cursive) -> otl_GposCursiveEntry,
        ),
        disposeItem: Some(
            subtable_gpos_cursive_disposeItem
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, usize) -> (),
        ),
        filterEnv: Some(
            subtable_gpos_cursive_filterEnv
                as unsafe extern "C" fn(
                    *mut subtable_gpos_cursive,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GposCursiveEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gpos_cursive_sort
                as unsafe extern "C" fn(
                    *mut subtable_gpos_cursive,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GposCursiveEntry,
                            *const otl_GposCursiveEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_shrinkToFit(mut arr: *mut subtable_gpos_cursive) {
    subtable_gpos_cursive_resizeTo(arr, (*arr).length);
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_gpos_cursive(
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut valueCount: glyphid_t = 0;
    let mut subtable: *mut subtable_gpos_cursive =
        (
            iSubtable_gpos_cursive
                .create
                .expect("non-null function pointer"))();
    let mut targets: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
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
            ) as glyphid_t;
            if !(tableLength
                < offset.wrapping_add(6 as u32).wrapping_add(
                    (4 as ::core::ffi::c_int * valueCount as ::core::ffi::c_int) as u32,
                ))
            {
                if !(valueCount as ::core::ffi::c_int != (*targets).numGlyphs as ::core::ffi::c_int)
                {
                    let mut j: glyphid_t = 0 as glyphid_t;
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
                        let mut enter: otl_Anchor = otl_anchor_absent();
                        let mut exit: otl_Anchor = otl_anchor_absent();
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
                            otl_GposCursiveEntry {
                                target: otfcc_Handle_dup(
                                    *(*targets).glyphs.offset(j as isize) as otfcc_Handle,
                                ) as otfcc_GlyphHandle,
                                enter: enter,
                                exit: exit,
                            },
                        );
                        j = j.wrapping_add(1);
                    }
                    if !targets.is_null() {
                        otl_Coverage_free(targets);
                    }
                    return subtable as *mut otl_Subtable;
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
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_dump_cursive(
    mut _subtable: *const otl_Subtable,
) -> *mut json_value {
    let mut subtable: *const subtable_gpos_cursive = &raw const (*_subtable).gpos_cursive;
    let mut st: *mut json_value = json_object_new((*subtable).length);
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*subtable).length {
        let mut rec: *mut json_value = json_object_new(2 as usize);
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
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_parse_cursive(
    mut _subtable: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut subtable: *mut subtable_gpos_cursive =
        (
            iSubtable_gpos_cursive
                .create
                .expect("non-null function pointer"))();
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_uint) < (*_subtable).u.object.length {
        if !(*(*_subtable).u.object.values.offset(j as isize))
            .value
            .is_null()
            && (*(*(*_subtable).u.object.values.offset(j as isize)).value).type_0
                as ::core::ffi::c_uint
                == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut gname: sds = sdsnewlen(
                (*(*_subtable).u.object.values.offset(j as isize)).name
                    as *const ::core::ffi::c_void,
                (*(*_subtable).u.object.values.offset(j as isize)).name_length as usize,
            );
            iSubtable_gpos_cursive
                .push
                .expect("non-null function pointer")(
                subtable,
                otl_GposCursiveEntry {
                    target: handle_fromName(gname)
                        as otfcc_GlyphHandle,
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
    return subtable as *mut otl_Subtable;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gpos_cursive(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gpos_cursive = &raw const (*_subtable).gpos_cursive;
    let mut cov: *mut otl_Coverage = otl_Coverage_create();
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*subtable).length {
        pushToCoverage(
            cov,
            otfcc_Handle_dup(
                (*(*subtable).items.offset(j as isize)).target as otfcc_Handle,
            ) as otfcc_GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(cov)),
        b16 as ::core::ffi::c_int,
        (*subtable).length,
        bkover as ::core::ffi::c_int,
    );
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as usize) < (*subtable).length {
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            bkFromAnchor((*(*subtable).items.offset(j_0 as isize)).enter),
            p16 as ::core::ffi::c_int,
            bkFromAnchor((*(*subtable).items.offset(j_0 as isize)).exit),
            bkover as ::core::ffi::c_int,
        );
        j_0 = j_0.wrapping_add(1);
    }
    otl_Coverage_free(cov);
    return bk_build_Block(root);
}
