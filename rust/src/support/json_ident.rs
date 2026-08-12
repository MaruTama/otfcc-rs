#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{strcmp};

use crate::vendor::json::{JsonValue};
use crate::support::json_funcs::{json_arr_at, json_arr_len, json_bool_val, json_dbl_val, json_int_val, json_obj_key_at, json_obj_len, json_obj_val_at, json_str_len, json_str_ptr};
unsafe extern "C" fn compare_json_arrays(
    mut a: *const JsonValue,
    mut b: *const JsonValue,
) -> bool {
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_uint) < json_arr_len(a) {
        if !json_ident(
            json_arr_at(a, j as u32),
            json_arr_at(b, j as u32),
        ) {
            return false;
        }
        j = j.wrapping_add(1);
    }
    return true;
}
unsafe extern "C" fn compare_json_objects(
    mut a: *const JsonValue,
    mut b: *const JsonValue,
) -> bool {
    // Builds `a`'s members into `key -> (value, checked)`, first occurrence
    // of a key wins -- matching the original uthash HASH_FIND-before-add
    // here exactly: it looked up the key before every insert and simply
    // skipped inserting when already present, so a later duplicate key in
    // `a` was always silently dropped, never overwriting or appending.
    // `entry().or_insert(...)` reproduces that "first wins, rest ignored"
    // behavior directly. Then walks `b`'s members, requiring each to find
    // a match by key and recursively compare `json_ident`-equal, and
    // finally requires every *distinct* key from `a` to have been matched.
    // A caller (`json_ident`) has already checked `a.length == b.length`
    // before this runs, but that says nothing about duplicate keys within
    // either object -- an object with duplicate keys still only ever
    // contributes one entry per distinct key here, so two objects with a
    // different multiset of duplicates (but the same length and the same
    // distinct-key values) can compare equal. That is the original's
    // exact behavior, not something this rewrite changes.
    let mut seen: std::collections::HashMap<&::core::ffi::CStr, (*mut JsonValue, bool)> =
        std::collections::HashMap::new();
    let mut j: u32 = 0 as u32;
    while j < json_obj_len(a) {
        let key: &::core::ffi::CStr = ::core::ffi::CStr::from_ptr(json_obj_key_at(a, j));
        seen.entry(key).or_insert((json_obj_val_at(a, j), false));
        j = j.wrapping_add(1);
    }
    let mut j_0: u32 = 0 as u32;
    while j_0 < json_obj_len(b) {
        let key: &::core::ffi::CStr = ::core::ffi::CStr::from_ptr(json_obj_key_at(b, j_0));
        let value = json_obj_val_at(b, j_0);
        match seen.get_mut(key) {
            None => return false,
            Some((val, checked)) => {
                if !json_ident(*val, value) {
                    return false;
                }
                *checked = true;
            }
        }
        j_0 = j_0.wrapping_add(1);
    }
    return seen.values().all(|(_, checked)| *checked);
}
pub unsafe extern "C" fn json_ident(mut a: *const JsonValue, mut b: *const JsonValue) -> bool {
    if a.is_null() && b.is_null() {
        return true;
    }
    if a.is_null() || b.is_null() {
        return false;
    }
    if (*a).type_0 as ::core::ffi::c_uint != (*b).type_0 as ::core::ffi::c_uint {
        return false;
    }
    match (*a).type_0 as ::core::ffi::c_uint {
        0 | 7 => return true,
        3 => return json_int_val(a) == json_int_val(b),
        4 => return json_dbl_val(a) == json_dbl_val(b),
        6 => return json_bool_val(a) == json_bool_val(b),
        5 => {
            return json_str_len(a) == json_str_len(b)
                && strcmp(json_str_ptr(a), json_str_ptr(b)) == 0 as ::core::ffi::c_int;
        }
        2 => {
            return json_arr_len(a) == json_arr_len(b)
                && compare_json_arrays(a, b) as ::core::ffi::c_int != 0;
        }
        1 => {
            return json_obj_len(a) == json_obj_len(b)
                && compare_json_objects(a, b) as ::core::ffi::c_int != 0;
        }
        _ => return false,
    };
}
