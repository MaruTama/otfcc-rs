#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::json_funcs::{preserialize};


use crate::support::primitives::{TableId};
use crate::vendor::json::JsonValue;
use crate::table::otl::{ChainingRule, Subtable, ChainingSubtable};
use crate::table::otl::coverage::{otl_iCoverage};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_null_new, json_object_new, json_object_push, json_string_new};

pub unsafe extern "C" fn otl_dump_chaining(mut _subtable: *const Subtable) -> *mut JsonValue {
    let mut subtable: *const ChainingSubtable = &raw const (*_subtable).chaining;
    if (*subtable).type_0 as u64 != 0 {
        return json_null_new();
    }
    let mut rule: *const ChainingRule = &raw const (*subtable).c2rust_unnamed.rule;
    let mut _st: *mut JsonValue = json_object_new(4 as usize);
    let mut _match: *mut JsonValue = json_array_new((*rule).matchCount as usize);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
        json_array_push(
            _match,
            otl_iCoverage.dump.expect("non-null function pointer")(
                *(*rule).match_0.offset(j as isize),
            ),
        );
        j = j.wrapping_add(1);
    }
    json_object_push(
        _st,
        b"match\0" as *const u8 as *const ::core::ffi::c_char,
        _match,
    );
    let mut _apply: *mut JsonValue = json_array_new((*rule).applyCount as usize);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int) < (*rule).applyCount as ::core::ffi::c_int {
        let mut _application: *mut JsonValue = json_object_new(2 as usize);
        json_object_push(
            _application,
            b"at\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*(*rule).apply.offset(j_0 as isize)).index as i64),
        );
        json_object_push(
            _application,
            b"lookup\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new(
                (*(*rule).apply.offset(j_0 as isize)).lookup.name as *const ::core::ffi::c_char,
            ),
        );
        json_array_push(_apply, _application);
        j_0 = j_0.wrapping_add(1);
    }
    json_object_push(
        _st,
        b"apply\0" as *const u8 as *const ::core::ffi::c_char,
        preserialize(_apply),
    );
    json_object_push(
        _st,
        b"inputBegins\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*rule).inputBegins as i64),
    );
    json_object_push(
        _st,
        b"inputEnds\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*rule).inputEnds as i64),
    );
    return _st;
}
