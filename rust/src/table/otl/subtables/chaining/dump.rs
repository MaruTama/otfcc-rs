use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_integer_new, json_null_new, json_object_new,
    json_object_push, json_string_new_from_bytes, preserialize,
};
use crate::support::primitives::TableId;
use crate::table::otl::coverage::{Coverage, dump_coverage};
use crate::table::otl::subtables::chaining::common::{chaining_is_canonical, chaining_rule_const};
use crate::table::otl::{ChainingRule, ChainingSubtable, Subtable};

pub unsafe extern "C" fn otl_dump_chaining(mut _subtable: *const Subtable) -> *mut BuiltValue {
    unsafe {
        let Subtable::Chaining(mut_subtable) = &*_subtable else {
            unreachable!()
        };
        let subtable: *const ChainingSubtable = mut_subtable;
        if !chaining_is_canonical(subtable) {
            return json_null_new();
        }
        let rule: *const ChainingRule = chaining_rule_const(subtable);
        let mut _st: *mut BuiltValue = json_object_new(4 as usize);
        let mut _match: *mut BuiltValue = json_array_new((*rule).match_count as usize);
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_int) < (*rule).match_count as ::core::ffi::c_int {
            json_array_push(
                _match,
                dump_coverage(&(&(*rule).match_0)[j as usize] as *const Coverage),
            );
            j = j.wrapping_add(1);
        }
        json_object_push(
            _st,
            b"match\0" as *const u8 as *const ::core::ffi::c_char,
            _match,
        );
        let mut _apply: *mut BuiltValue = json_array_new((*rule).apply.len());
        let mut j_0: TableId = 0 as TableId;
        while (j_0 as usize) < (*rule).apply.len() {
            let mut _application: *mut BuiltValue = json_object_new(2 as usize);
            json_object_push(
                _application,
                b"at\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((&(*rule).apply)[j_0 as usize].index as i64),
            );
            json_object_push(
                _application,
                b"lookup\0" as *const u8 as *const ::core::ffi::c_char,
                json_string_new_from_bytes(&(&(*rule).apply)[j_0 as usize].lookup.name),
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
            json_integer_new((*rule).input_begins as i64),
        );
        json_object_push(
            _st,
            b"inputEnds\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*rule).input_ends as i64),
        );
        return _st;
    }
}
