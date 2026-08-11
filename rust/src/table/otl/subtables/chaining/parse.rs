#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md


use crate::support::json_funcs::{json_obj_get_type, json_obj_getnum, json_obj_getnum_fallback};
use crate::table::otl::coverage::Coverage;
use crate::support::handle::{handle_from_name, otfcc_handle_empty, LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};


use crate::support::options::{Options};
use crate::support::primitives::{TableId};
use crate::vendor::json::{JsonType, JsonValue};

use crate::table::otl::{ChainLookupApplication, ChainingRule, Subtable, ChainingType, ChainingSubtable, subtable_from_raw};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::table::otl::subtables::chaining::common::{I_SUBTABLE_CHAINING};
use crate::vendor::sds::{sdsnewlen};
pub unsafe extern "C" fn otl_parse_chaining(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut _match: *mut JsonValue = json_obj_get_type(
        _subtable,
        b"match\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    let mut _apply: *mut JsonValue = json_obj_get_type(
        _subtable,
        b"apply\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _match.is_null() || _apply.is_null() {
        return ::core::ptr::null_mut::<Subtable>();
    }
    let mut subtable: *mut ChainingSubtable =
        (
            I_SUBTABLE_CHAINING
                .create
                .expect("non-null function pointer"))();
    (*subtable).type_0 = ChainingType::Canonical;
    let mut rule: *mut ChainingRule =
        &raw mut (*subtable).c2rust_unnamed.rule as *mut ChainingRule;
    (*rule).match_count = (*_match).u.array.length as TableId;
    (*rule).match_0 = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut Coverage>() as usize)
            .wrapping_mul((*rule).match_count as usize),
        14 as ::core::ffi::c_ulong,
    ) as *mut *mut Coverage;
    // Placement-construct: `rule` sits inside the zeroed memory `create()`
    // hands back (via `otl_init_chaining`'s `memset`), not a valid `Vec`
    // bit pattern, so there is nothing to drop first.
    ::core::ptr::write(
        &raw mut (*rule).apply,
        Vec::with_capacity((*_apply).u.array.length as usize),
    );
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
        let ref mut fresh0 = *(*rule).match_0.offset(j as isize);
        *fresh0 = OTL_I_COVERAGE.parse.expect("non-null function pointer")(
            *(*_match).u.array.values.offset(j as isize),
        );
        j = j.wrapping_add(1);
    }
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int) < (*_apply).u.array.length as ::core::ffi::c_int {
        let mut index: TableId = 0 as TableId;
        let mut lookup: LookupHandle = otfcc_handle_empty() as LookupHandle;
        let mut _application: *mut JsonValue =
            *(*_apply).u.array.values.offset(j_0 as isize) as *mut JsonValue;
        if (*_application).type_0 == JsonType::Object
        {
            let mut _ln: *mut JsonValue = json_obj_get_type(
                _application,
                b"lookup\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            if !_ln.is_null() {
                lookup = handle_from_name(sdsnewlen(
                    (*_ln).u.string.ptr as *const ::core::ffi::c_void,
                    (*_ln).u.string.length as usize,
                )) as LookupHandle;
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
