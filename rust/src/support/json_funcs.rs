#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
// The label-driven flag helpers from c/lib/support/json/json-funcs.h, plus the
// one other helper from that header they depend on. They are `static inline` in
// C, so c2rust re-emitted a private copy into every translation unit that read
// or wrote a flag field: 3 copies each of otfcc_dump_flags/otfcc_parse_flags and
// 7 of json_obj_getbool, all textually identical. Never externally linked (no
// #[no_mangle]) in their per-file form, so consolidating them changes no ABI.
//
// The rest of json-funcs.h (json_obj_get with 32 copies, json_obj_getnum,
// json_obj_getint, json_obj_getnum_fallback, ...) is still duplicated per file;
// only the flag helpers are here, because the label tables they walk are what
// this module exists to serve.

use crate::vendor::json::{json_boolean, json_double, json_integer, json_object, json_value};
use crate::vendor::json_builder::{json_boolean_new, json_object_new, json_object_push};
use libc::strcmp;

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
) -> *mut json_value {
    let v: *mut json_value = json_object_new(0);
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
pub unsafe fn otfcc_parse_flags(v: *const json_value, labels: &[&::core::ffi::CStr]) -> u32 {
    if v.is_null() {
        return 0;
    }
    match (*v).type_0 {
        json_integer => (*v).u.integer as u32,
        json_double => (*v).u.dbl as u32,
        json_object => {
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
pub unsafe fn json_obj_getbool(obj: *const json_value, key: *const ::core::ffi::c_char) -> bool {
    if obj.is_null() || (*obj).type_0 != json_object {
        return false;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int && !cv.is_null() && (*cv).type_0 == json_boolean
        {
            return (*cv).u.boolean != 0;
        }
        _k = _k.wrapping_add(1);
    }
    false
}
