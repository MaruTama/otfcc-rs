#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
unsafe extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    static iSubtable_chaining: __caryll_elementinterface_subtable_chaining;
}


use crate::support::json_funcs::{json_obj_get_type, json_obj_getnum, json_obj_getnum_fallback};
use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage};
use crate::support::handle::{handle_fromName, otfcc_Handle_empty, otfcc_LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};


use crate::support::options::{otfcc_Options};
use crate::support::primitives::{tableid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_array, json_object, json_string, json_value};

use crate::table::otl::{__caryll_elementinterface_subtable_chaining, otl_ChainLookupApplication, otl_ChainingRule, otl_Subtable, otl_chaining_canonical, subtable_chaining};
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
