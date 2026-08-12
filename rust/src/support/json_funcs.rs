#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
// c/lib/support/json/json-funcs.h. Every helper in it is `static inline`, so
// c2rust re-emitted a private copy into each translation unit that used one:
// 32 copies of json_obj_get, 30 of json_obj_get_type, 16 of preserialize, and so
// on down to 2. Across the header's 17 helpers that came to **139 definitions**;
// now there is one of each. Every name's copies were verified textually
// identical before any was deleted, and none was ever externally linked (no
// #[no_mangle] on the per-file form), so consolidating them changes no ABI.
//
// One helper is still where c2rust put it: `json_from_sds`, in table/cff.rs. It
// needs `sdslen`, itself a `static inline` from SdsRaw.h with 20 copies of its own,
// and that family is a separate pass. It only ever had one copy, so there is
// nothing to collapse -- moving it would just be moving it.

use crate::vendor::json::{JsonType, JsonValue};
use crate::vendor::sds::{SdsRaw, sdsnewlen};
use libc::strcmp;

/// The inverse of [`otfcc_dump_flags`]: read a bitfield back from JSON.
///
/// A number is taken as the raw field value; an object is read label by label.
/// Anything else -- including a missing key, which arrives here as null -- is 0.
pub unsafe fn otfcc_parse_flags(v: *const JsonValue, labels: &[&::core::ffi::CStr]) -> u32 {
    if v.is_null() {
        return 0;
    }
    match (*v).type_0 {
        JsonType::Integer => json_int_val(v) as u32,
        JsonType::Double => json_dbl_val(v) as u32,
        JsonType::Object => {
            let mut flags: u32 = 0;
            for (j, label) in labels.iter().enumerate() {
                if json_obj_getbool(v, label.as_ptr()) {
                    flags |= (1 as u32) << j;
                }
            }
            flags
        }
        _ => 0,
    }
}

/// Look up `key` in a JSON object and read it as a boolean.
///
/// False for anything that is not a boolean-valued member of an object: a null
/// or non-object `obj`, a missing key, or a key whose value has another type.
pub unsafe fn json_obj_getbool(obj: *const JsonValue, key: *const ::core::ffi::c_char) -> bool {
    let mut _k: u32 = 0 as u32;
    while _k < json_obj_len(obj) {
        let ck: *mut ::core::ffi::c_char = json_obj_key_at(obj, _k);
        let cv: *mut JsonValue = json_obj_val_at(obj, _k);
        if strcmp(ck, key) == 0 as ::core::ffi::c_int && !cv.is_null() && (*cv).type_0 == JsonType::Boolean
        {
            return json_bool_val(cv);
        }
        _k = _k.wrapping_add(1);
    }
    false
}

/// Look up `key` in a JSON object, of whatever type; NULL when there is no such
/// member (or `obj` is not an object).
///
/// The first member whose name matches wins, which matters because the parser
/// keeps duplicate keys rather than collapsing them.
pub unsafe fn json_obj_get(
    obj: *const JsonValue,
    key: *const ::core::ffi::c_char,
) -> *mut JsonValue {
    let mut _k: u32 = 0 as u32;
    while _k < json_obj_len(obj) {
        let ck: *mut ::core::ffi::c_char = json_obj_key_at(obj, _k);
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return json_obj_val_at(obj, _k);
        }
        _k = _k.wrapping_add(1);
    }
    ::core::ptr::null_mut::<JsonValue>()
}

/// [`json_obj_get`], but NULL unless the member has the type asked for.
pub unsafe fn json_obj_get_type(
    obj: *const JsonValue,
    key: *const ::core::ffi::c_char,
    type_0: JsonType,
) -> *mut JsonValue {
    let v: *mut JsonValue = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 == type_0 {
        return v;
    }
    ::core::ptr::null_mut::<JsonValue>()
}

/// A member's string value, copied into a fresh [`SdsRaw`]; NULL if it is not a
/// string. The caller owns the copy.
pub unsafe fn json_obj_getsds(obj: *const JsonValue, key: *const ::core::ffi::c_char) -> SdsRaw {
    let v: *mut JsonValue = json_obj_get_type(obj, key, JsonType::String);
    if v.is_null() {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    } else {
        sdsnewlen(
            json_str_ptr(v) as *const ::core::ffi::c_void,
            json_str_len(v) as usize,
        )
    }
}

/// [`json_obj_getsds`] without the copy: the pointer belongs to the JSON tree
/// and dies with it.
pub unsafe fn json_obj_getstr_share(
    obj: *const JsonValue,
    key: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let v: *mut JsonValue = json_obj_get_type(obj, key, JsonType::String);
    if v.is_null() {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    } else {
        json_str_ptr(v)
    }
}

/// A number, whether the JSON spelled it as an integer or a double; 0.0 for
/// anything else, including null.
pub unsafe fn json_numof(cv: *const JsonValue) -> ::core::ffi::c_double {
    if !cv.is_null() && (*cv).type_0 == JsonType::Integer {
        return json_int_val(cv) as ::core::ffi::c_double;
    }
    if !cv.is_null() && (*cv).type_0 == JsonType::Double {
        return json_dbl_val(cv);
    }
    0.0f64
}

/// A boolean; false for anything else, including null.
pub unsafe fn json_boolof(cv: *const JsonValue) -> bool {
    json_bool_val(cv)
}

// The numeric lookups below walk the object themselves instead of going through
// `json_obj_get`, exactly as C wrote them, and that is not redundant: on a name
// match whose value has the wrong type they *keep looking*.
// `json_numof(json_obj_get(obj, key))` would stop at the first name match and
// return 0 instead. The two differ whenever a key appears more than once with
// different value types -- which the parser permits, since it keeps duplicate
// members. What C did duplicate is the fallback-less pair, whose bodies are the
// `_fallback` ones with the fallback spelled 0; those just delegate here.

/// A member's numeric value; 0.0 when absent or non-numeric.
pub unsafe fn json_obj_getnum(
    obj: *const JsonValue,
    key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    json_obj_getnum_fallback(obj, key, 0.0f64)
}

/// A member's numeric value, truncated to an `i32`; 0 when absent or non-numeric.
pub unsafe fn json_obj_getint(obj: *const JsonValue, key: *const ::core::ffi::c_char) -> i32 {
    json_obj_getint_fallback(obj, key, 0 as i32)
}

/// A member's numeric value, or `fallback` when absent or non-numeric.
pub unsafe fn json_obj_getnum_fallback(
    obj: *const JsonValue,
    key: *const ::core::ffi::c_char,
    fallback: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut _k: u32 = 0 as u32;
    while _k < json_obj_len(obj) {
        let ck: *mut ::core::ffi::c_char = json_obj_key_at(obj, _k);
        let cv: *mut JsonValue = json_obj_val_at(obj, _k);
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null() && (*cv).type_0 == JsonType::Integer {
                return json_int_val(cv) as ::core::ffi::c_double;
            }
            if !cv.is_null() && (*cv).type_0 == JsonType::Double {
                return json_dbl_val(cv);
            }
        }
        _k = _k.wrapping_add(1);
    }
    fallback
}

/// A member's numeric value truncated to an `i32`, or `fallback` when absent or
/// non-numeric.
pub unsafe fn json_obj_getint_fallback(
    obj: *const JsonValue,
    key: *const ::core::ffi::c_char,
    fallback: i32,
) -> i32 {
    let mut _k: u32 = 0 as u32;
    while _k < json_obj_len(obj) {
        let ck: *mut ::core::ffi::c_char = json_obj_key_at(obj, _k);
        let cv: *mut JsonValue = json_obj_val_at(obj, _k);
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null() && (*cv).type_0 == JsonType::Integer {
                return json_int_val(cv) as i32;
            }
            if !cv.is_null() && (*cv).type_0 == JsonType::Double {
                return json_dbl_val(cv) as i32;
            }
        }
        _k = _k.wrapping_add(1);
    }
    fallback
}

// The accessors below (`json_obj_len` through `json_bool_val`) are Stage
// 6-2.5's C-1: a widened accessor layer that eliminates every direct
// `.u.<field>` read outside `vendor/json.rs`/`vendor/json_builder.rs`
// themselves, without changing `JsonValue`'s representation at all -- each
// one is a thin, self-guarding wrapper around exactly the union read it
// replaces, so every call site becomes a 1:1 textual substitution and the
// output is unchanged by construction. This is deliberately *not* an
// iterator-based API (the natural next step once the representation itself
// is safe, in C-2): centralizing the union reads here first, behind the same
// index-based shape every call site already used, is what lets that later
// representation change touch only this file and `vendor/json.rs` instead of
// every consumer.
//
// Every accessor here self-guards on `.type_0` (and null) the same way
// `json_obj_getbool`/`json_numof` above already did, even at call sites that
// had already checked the type themselves -- consistent with this module's
// existing style, and free once inlined.

/// Object member count; 0 for anything but an object (including null).
pub unsafe fn json_obj_len(obj: *const JsonValue) -> u32 {
    if obj.is_null() || (*obj).type_0 != JsonType::Object {
        return 0;
    }
    (*obj).u.object.length as u32
}

/// The `i`th member's raw (NUL-terminated) name pointer; null if `obj` is not
/// an object or `i` is out of range.
pub unsafe fn json_obj_key_at(obj: *const JsonValue, i: u32) -> *mut ::core::ffi::c_char {
    if i >= json_obj_len(obj) {
        return ::core::ptr::null_mut();
    }
    (*(*obj).u.object.values.offset(i as isize)).name
}

/// The `i`th member's name length in bytes (excluding the NUL); 0 if `obj` is
/// not an object or `i` is out of range.
pub unsafe fn json_obj_key_len_at(obj: *const JsonValue, i: u32) -> u32 {
    if i >= json_obj_len(obj) {
        return 0;
    }
    (*(*obj).u.object.values.offset(i as isize)).name_length as u32
}

/// The `i`th member's value; null if `obj` is not an object or `i` is out of
/// range.
pub unsafe fn json_obj_val_at(obj: *const JsonValue, i: u32) -> *mut JsonValue {
    if i >= json_obj_len(obj) {
        return ::core::ptr::null_mut();
    }
    (*(*obj).u.object.values.offset(i as isize)).value as *mut JsonValue
}

/// Array element count; 0 for anything but an array (including null).
pub unsafe fn json_arr_len(arr: *const JsonValue) -> u32 {
    if arr.is_null() || (*arr).type_0 != JsonType::Array {
        return 0;
    }
    (*arr).u.array.length as u32
}

/// The `i`th array element; null if `arr` is not an array or `i` is out of
/// range.
pub unsafe fn json_arr_at(arr: *const JsonValue, i: u32) -> *mut JsonValue {
    if i >= json_arr_len(arr) {
        return ::core::ptr::null_mut();
    }
    *(*arr).u.array.values.offset(i as isize)
}

/// A string value's raw (NUL-terminated) pointer; null for anything but a
/// string (`PreSerialized` included -- it retags a string's payload without
/// changing it, see `preserialize`).
pub unsafe fn json_str_ptr(v: *const JsonValue) -> *mut ::core::ffi::c_char {
    if v.is_null() || !matches!((*v).type_0, JsonType::String | JsonType::PreSerialized) {
        return ::core::ptr::null_mut();
    }
    (*v).u.string.ptr
}

/// A string value's length in bytes; 0 for anything but a string
/// (`PreSerialized` included, see [`json_str_ptr`]).
pub unsafe fn json_str_len(v: *const JsonValue) -> u32 {
    if v.is_null() || !matches!((*v).type_0, JsonType::String | JsonType::PreSerialized) {
        return 0;
    }
    (*v).u.string.length as u32
}

/// An integer value's raw `i64`; 0 for anything but `JsonType::Integer`.
pub unsafe fn json_int_val(v: *const JsonValue) -> i64 {
    if v.is_null() || (*v).type_0 != JsonType::Integer {
        return 0;
    }
    (*v).u.integer
}

/// A double value's raw `f64`; 0.0 for anything but `JsonType::Double`.
pub unsafe fn json_dbl_val(v: *const JsonValue) -> ::core::ffi::c_double {
    if v.is_null() || (*v).type_0 != JsonType::Double {
        return 0.0;
    }
    (*v).u.dbl
}

/// A boolean value's raw `bool`; false for anything but `JsonType::Boolean`.
pub unsafe fn json_bool_val(v: *const JsonValue) -> bool {
    if v.is_null() || (*v).type_0 != JsonType::Boolean {
        return false;
    }
    (*v).u.boolean != 0
}

