#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{LookupHandle, handle_from_name, otfcc_handle_empty};
use crate::support::parsed_json::ParsedValue;
use crate::table::otl::coverage::coverage_from_raw;

use crate::support::options::Options;
use crate::support::primitives::TableId;
use crate::vendor::json::JsonType;

use crate::table::otl::coverage::parse_coverage;
use crate::table::otl::subtables::chaining::common::{chaining_rule_mut, subtable_chaining_create};
use crate::table::otl::{
    ChainLookupApplication, ChainingRule, ChainingSubtable, Subtable, subtable_from_raw,
};
pub unsafe fn otl_parse_chaining(
    mut _subtable: *const ParsedValue,
    mut _options: &Options,
) -> *mut Subtable {
    let sv = unsafe { _subtable.as_ref() };
    let match_val = sv.and_then(|v| v.get_typed(b"match", JsonType::Array));
    let apply_val = sv.and_then(|v| v.get_typed(b"apply", JsonType::Array));
    let (Some(match_val), Some(apply_val)) = (match_val, apply_val) else {
        return ::core::ptr::null_mut::<Subtable>();
    };
    let sv = sv.unwrap();
    let subtable: *mut ChainingSubtable = (subtable_chaining_create)();
    // `create()` already hands back a valid `Canonical(ChainingRule::
    // default())` -- no separate tag assignment or placement-construct
    // needed, unlike the pre-enum version.
    let rule: *mut ChainingRule = chaining_rule_mut(subtable);
    let match_items = match_val.as_array().unwrap();
    let apply_items = apply_val.as_array().unwrap();
    (*rule).match_count = match_items.len() as TableId;
    (*rule).match_0 = Vec::with_capacity((*rule).match_count as usize);
    (*rule).apply = Vec::with_capacity(apply_items.len());
    (*rule).input_begins = sv.get_num_or(b"inputBegins", 0.0) as TableId;
    (*rule).input_ends = sv.get_num_or(b"inputEnds", (*rule).match_count as f64) as TableId;
    for item in match_items {
        (*rule)
            .match_0
            .push(coverage_from_raw(parse_coverage(item as *const ParsedValue)));
    }
    for application in apply_items {
        let mut index: TableId = 0 as TableId;
        let mut lookup: LookupHandle = otfcc_handle_empty() as LookupHandle;
        if application.as_object().is_some() {
            if let Some(ln) = application.get_typed(b"lookup", JsonType::String) {
                lookup = handle_from_name(ln.as_str_bytes().map(|b| b.to_vec())) as LookupHandle;
                index = application.get_num(b"at") as TableId;
            }
        }
        (*rule).apply.push(ChainLookupApplication { index, lookup });
    }
    return subtable_from_raw(subtable, Subtable::Chaining);
}
