#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{strcmp};
unsafe extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    static iSubtable_chaining: __caryll_elementinterface_subtable_chaining;
}


use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage};
use crate::support::handle::{handle_fromName, otfcc_Handle_empty, otfcc_LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};


use crate::support::options::{otfcc_Options};
use crate::support::primitives::{tableid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_array, json_double, json_integer, json_object, json_string, json_type, json_value};

use crate::table::otl::{__caryll_elementinterface_subtable_chaining, otl_ChainLookupApplication, otl_ChainingRule, otl_Subtable, otl_chaining_canonical, subtable_chaining};
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 != json_object
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
unsafe extern "C" fn json_obj_getnum(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 != json_object
    {
        return 0.0f64;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 == json_integer
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 == json_double
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0.0f64;
}
#[inline]
unsafe extern "C" fn json_obj_getnum_fallback(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    mut fallback: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 != json_object
    {
        return fallback;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 == json_integer
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 == json_double
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return fallback;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otl_parse_chaining(
    mut _subtable: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut _match: *mut json_value = json_obj_get_type(
        _subtable,
        b"match\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    let mut _apply: *mut json_value = json_obj_get_type(
        _subtable,
        b"apply\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    if _match.is_null() || _apply.is_null() {
        return ::core::ptr::null_mut::<otl_Subtable>();
    }
    let mut subtable: *mut subtable_chaining =
        (
            iSubtable_chaining
                .create
                .expect("non-null function pointer"))();
    (*subtable).type_0 = otl_chaining_canonical;
    let mut rule: *mut otl_ChainingRule = &raw mut (*subtable).c2rust_unnamed.rule;
    (*rule).matchCount = (*_match).u.array.length as tableid_t;
    (*rule).match_0 = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut otl_Coverage>() as usize)
            .wrapping_mul((*rule).matchCount as usize),
        14 as ::core::ffi::c_ulong,
    ) as *mut *mut otl_Coverage;
    (*rule).applyCount = (*_apply).u.array.length as tableid_t;
    (*rule).apply = __caryll_allocate_clean(
        (::core::mem::size_of::<otl_ChainLookupApplication>() as usize)
            .wrapping_mul((*rule).applyCount as usize),
        16 as ::core::ffi::c_ulong,
    ) as *mut otl_ChainLookupApplication;
    (*rule).inputBegins = json_obj_getnum_fallback(
        _subtable,
        b"inputBegins\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as tableid_t;
    (*rule).inputEnds = json_obj_getnum_fallback(
        _subtable,
        b"inputEnds\0" as *const u8 as *const ::core::ffi::c_char,
        (*rule).matchCount as ::core::ffi::c_double,
    ) as tableid_t;
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
        let ref mut fresh0 = *(*rule).match_0.offset(j as isize);
        *fresh0 = otl_iCoverage.parse.expect("non-null function pointer")(
            *(*_match).u.array.values.offset(j as isize),
        );
        j = j.wrapping_add(1);
    }
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as ::core::ffi::c_int) < (*rule).applyCount as ::core::ffi::c_int {
        (*(*rule).apply.offset(j_0 as isize)).index = 0 as tableid_t;
        (*(*rule).apply.offset(j_0 as isize)).lookup =
            otfcc_Handle_empty() as otfcc_LookupHandle;
        let mut _application: *mut json_value =
            *(*_apply).u.array.values.offset(j_0 as isize) as *mut json_value;
        if (*_application).type_0 == json_object
        {
            let mut _ln: *mut json_value = json_obj_get_type(
                _application,
                b"lookup\0" as *const u8 as *const ::core::ffi::c_char,
                json_string,
            );
            if !_ln.is_null() {
                (*(*rule).apply.offset(j_0 as isize)).lookup =
                    handle_fromName(sdsnewlen(
                        (*_ln).u.string.ptr as *const ::core::ffi::c_void,
                        (*_ln).u.string.length as usize,
                    )) as otfcc_LookupHandle;
                (*(*rule).apply.offset(j_0 as isize)).index = json_obj_getnum(
                    _application,
                    b"at\0" as *const u8 as *const ::core::ffi::c_char,
                ) as tableid_t;
            }
        }
        j_0 = j_0.wrapping_add(1);
    }
    return subtable as *mut otl_Subtable;
}
