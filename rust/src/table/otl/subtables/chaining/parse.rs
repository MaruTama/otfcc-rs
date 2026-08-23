#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{LookupHandle, handle_from_name, otfcc_handle_empty};
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_getnum,
    json_obj_getnum_fallback, json_str_bytes, json_type_of,
};
use crate::table::otl::coverage::coverage_from_raw;

use crate::support::options::Options;
use crate::support::primitives::TableId;
use crate::vendor::json::JsonType;

use crate::table::otl::coverage::parse_coverage;
use crate::table::otl::subtables::chaining::common::{chaining_rule_mut, subtable_chaining_create};
use crate::table::otl::{
    ChainLookupApplication, ChainingRule, ChainingSubtable, Subtable, subtable_from_raw,
};
pub unsafe extern "C" fn otl_parse_chaining(
    mut _subtable: *const ParsedValue,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut _match: *const ParsedValue = json_obj_get_type(
        _subtable,
        b"match\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    let mut _apply: *const ParsedValue = json_obj_get_type(
        _subtable,
        b"apply\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _match.is_null() || _apply.is_null() {
        return ::core::ptr::null_mut::<Subtable>();
    }
    let mut subtable: *mut ChainingSubtable = (subtable_chaining_create)();
    // `create()` already hands back a valid `Canonical(ChainingRule::
    // default())` -- no separate tag assignment or placement-construct
    // needed, unlike the pre-enum version.
    let mut rule: *mut ChainingRule = chaining_rule_mut(subtable);
    (*rule).match_count = json_arr_len(_match) as TableId;
    (*rule).match_0 = Vec::with_capacity((*rule).match_count as usize);
    (*rule).apply = Vec::with_capacity(json_arr_len(_apply) as usize);
    (*rule).input_begins = json_obj_getnum_fallback(
        _subtable,
        b"inputBegins\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as TableId;
    (*rule).input_ends = json_obj_getnum_fallback(
        _subtable,
        b"inputEnds\0" as *const u8 as *const ::core::ffi::c_char,
        (*rule).match_count as ::core::ffi::c_double,
    ) as TableId;
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*rule).match_count as ::core::ffi::c_int {
        (*rule)
            .match_0
            .push(coverage_from_raw(parse_coverage(json_arr_at(
                _match, j as u32,
            ))));
        j = j.wrapping_add(1);
    }
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int) < json_arr_len(_apply) as ::core::ffi::c_int {
        let mut index: TableId = 0 as TableId;
        let mut lookup: LookupHandle = otfcc_handle_empty() as LookupHandle;
        let mut _application: *const ParsedValue = json_arr_at(_apply, j_0 as u32);
        if json_type_of(_application) == JsonType::Object {
            let mut _ln: *const ParsedValue = json_obj_get_type(
                _application,
                b"lookup\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            if !_ln.is_null() {
                lookup = handle_from_name(Some(json_str_bytes(_ln))) as LookupHandle;
                index = json_obj_getnum(
                    _application,
                    b"at\0" as *const u8 as *const ::core::ffi::c_char,
                ) as TableId;
            }
        }
        (*rule).apply.push(ChainLookupApplication { index, lookup });
        j_0 = j_0.wrapping_add(1);
    }
    return subtable_from_raw(subtable, Subtable::Chaining);
}
