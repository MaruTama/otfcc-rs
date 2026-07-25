use libc::{malloc};
extern "C" {
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new(_: *const ::core::ffi::c_char) -> *mut json_value;
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_null_new() -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    static otl_iCoverage: __otfcc_ICoverage;
}

use crate::table::otl::coverage::{__otfcc_ICoverage};


use crate::support::primitives::{tableid_t};
use crate::vendor::json::{json_pre_serialized, json_value};
use crate::table::otl::{otl_ChainingRule, otl_Subtable, subtable_chaining};
use crate::vendor::json_builder::{json_serialize_mode_packed, json_serialize_opts};

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
#[no_mangle]
pub unsafe extern "C" fn otl_dump_chaining(mut _subtable: *const otl_Subtable) -> *mut json_value {
    let mut subtable: *const subtable_chaining = &raw const (*_subtable).chaining;
    if (*subtable).type_0 as u64 != 0 {
        return json_null_new();
    }
    let mut rule: *const otl_ChainingRule = &raw const (*subtable).c2rust_unnamed.rule;
    let mut _st: *mut json_value = json_object_new(4 as usize);
    let mut _match: *mut json_value = json_array_new((*rule).matchCount as usize);
    let mut j: tableid_t = 0 as tableid_t;
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
    let mut _apply: *mut json_value = json_array_new((*rule).applyCount as usize);
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as ::core::ffi::c_int) < (*rule).applyCount as ::core::ffi::c_int {
        let mut _application: *mut json_value = json_object_new(2 as usize);
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
