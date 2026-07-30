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

use crate::support::primitives::Pos;
use crate::vendor::json::{JsonType, JsonValue};
use crate::vendor::json_builder::{
    json_boolean_new, json_builder_free, json_double_new, json_integer_new, json_measure_ex,
    json_object_new, json_object_push, json_object_push_length, json_serialize_ex,
    JSON_SERIALIZE_MODE_PACKED, JsonSerializeOpts, json_string_new_nocopy,
};
use crate::vendor::sds::{SdsRaw, sdsnewlen};
use libc::{malloc, strcmp};

unsafe extern "C" {
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

/// Serialize a bitfield as a JSON object of `label: true` pairs, one per set bit.
///
/// Bit `i` is named by `labels[i]`; bits past the end of `labels` are dropped,
/// which is how C behaved too -- there the tables were NUL-terminated
/// `const char *[]` and the loop stopped at the first null entry, so the
/// terminator and the length carry the same information. Every table was dense
/// up to that terminator (checked entry by entry), so a plain slice reproduces
/// the walk exactly.
pub unsafe fn otfcc_dump_flags(
    flags: ::core::ffi::c_int,
    labels: &[&::core::ffi::CStr],
) -> *mut JsonValue {
    let v: *mut JsonValue = json_object_new(0);
    for (j, label) in labels.iter().enumerate() {
        if flags & (1 as ::core::ffi::c_int) << j != 0 {
            json_object_push(v, label.as_ptr(), json_boolean_new(1));
        }
    }
    v
}

/// The inverse of [`otfcc_dump_flags`]: read a bitfield back from JSON.
///
/// A number is taken as the raw field value; an object is read label by label.
/// Anything else -- including a missing key, which arrives here as null -- is 0.
pub unsafe fn otfcc_parse_flags(v: *const JsonValue, labels: &[&::core::ffi::CStr]) -> u32 {
    if v.is_null() {
        return 0;
    }
    match (*v).type_0 {
        JsonType::Integer => (*v).u.integer as u32,
        JsonType::Double => (*v).u.dbl as u32,
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
    if obj.is_null() || (*obj).type_0 != JsonType::Object {
        return false;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let cv: *mut JsonValue =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut JsonValue;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int && !cv.is_null() && (*cv).type_0 == JsonType::Boolean
        {
            return (*cv).u.boolean != 0;
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
    if obj.is_null() || (*obj).type_0 != JsonType::Object {
        return ::core::ptr::null_mut::<JsonValue>();
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut JsonValue;
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
            (*v).u.string.ptr as *const ::core::ffi::c_void,
            (*v).u.string.length as usize,
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
        (*v).u.string.ptr
    }
}

/// Push `b` under a four-character OpenType tag, unpacked big-endian from `tag`.
pub unsafe fn json_object_push_tag(
    a: *mut JsonValue,
    tag: u32,
    b: *mut JsonValue,
) -> *mut JsonValue {
    let mut tags: [::core::ffi::c_char; 4] = [
        ((tag & 0xff000000 as u32) >> 24 as ::core::ffi::c_int) as ::core::ffi::c_char,
        ((tag & 0xff0000 as u32) >> 16 as ::core::ffi::c_int) as ::core::ffi::c_char,
        ((tag & 0xff00 as u32) >> 8 as ::core::ffi::c_int) as ::core::ffi::c_char,
        (tag & 0xff as u32) as ::core::ffi::c_char,
    ];
    json_object_push_length(
        a,
        4 as ::core::ffi::c_uint,
        &raw mut tags as *mut ::core::ffi::c_char,
        b,
    )
}

/// A number, whether the JSON spelled it as an integer or a double; 0.0 for
/// anything else, including null.
pub unsafe fn json_numof(cv: *const JsonValue) -> ::core::ffi::c_double {
    if !cv.is_null() && (*cv).type_0 == JsonType::Integer {
        return (*cv).u.integer as ::core::ffi::c_double;
    }
    if !cv.is_null() && (*cv).type_0 == JsonType::Double {
        return (*cv).u.dbl;
    }
    0.0f64
}

/// A boolean; false for anything else, including null.
pub unsafe fn json_boolof(cv: *const JsonValue) -> bool {
    if !cv.is_null() && (*cv).type_0 == JsonType::Boolean {
        return (*cv).u.boolean != 0;
    }
    false
}

/// A coordinate, written as an integer when it is one so the JSON stays readable.
pub unsafe fn json_new_position(z: Pos) -> *mut JsonValue {
    if round(z as ::core::ffi::c_double) == z {
        json_integer_new(z as i64)
    } else {
        json_double_new(z as ::core::ffi::c_double)
    }
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
    if obj.is_null() || (*obj).type_0 != JsonType::Object {
        return fallback;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let cv: *mut JsonValue =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut JsonValue;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null() && (*cv).type_0 == JsonType::Integer {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null() && (*cv).type_0 == JsonType::Double {
                return (*cv).u.dbl;
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
    if obj.is_null() || (*obj).type_0 != JsonType::Object {
        return fallback;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let cv: *mut JsonValue =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut JsonValue;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null() && (*cv).type_0 == JsonType::Integer {
                return (*cv).u.integer as i32;
            }
            if !cv.is_null() && (*cv).type_0 == JsonType::Double {
                return (*cv).u.dbl as i32;
            }
        }
        _k = _k.wrapping_add(1);
    }
    fallback
}

/// Serialize a subtree now and keep the text, so the writer can splice it in
/// verbatim later. Consumes `x`.
///
/// The result is a `JsonType::String` retagged as [`JsonType::PreSerialized`], which the
/// serializer copies out as-is rather than descending into.
pub unsafe fn preserialize(x: *mut JsonValue) -> *mut JsonValue {
    let opts: JsonSerializeOpts = JsonSerializeOpts {
        mode: JSON_SERIALIZE_MODE_PACKED,
        opts: 0,
        indent_size: 0,
    };
    let preserialize_len: usize = json_measure_ex(x, opts);
    let buf: *mut ::core::ffi::c_char = malloc(preserialize_len) as *mut ::core::ffi::c_char;
    json_serialize_ex(buf, x, opts);
    json_builder_free(x);
    let xx: *mut JsonValue = json_string_new_nocopy(
        preserialize_len.wrapping_sub(1 as usize) as ::core::ffi::c_uint,
        buf,
    );
    (*xx).type_0 = JsonType::PreSerialized;
    xx
}
