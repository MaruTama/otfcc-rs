#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{LookupHandle, handle_from_name, otfcc_handle_empty};
use crate::support::parsed_json::ParsedValue;
use crate::table::otl::coverage::coverage_from_raw;

use crate::support::options::Options;
use crate::support::primitives::TableId;
use crate::vendor::json::JsonType;

use crate::table::otl::coverage::parse_coverage;
use crate::table::otl::{ChainLookupApplication, ChainingRule, ChainingSubtable, Subtable};
pub fn otl_parse_chaining(_subtable: Option<&ParsedValue>, _options: &Options) -> Option<Subtable> {
    let sv = _subtable;
    let match_val = sv.and_then(|v| v.get_typed(b"match", JsonType::Array));
    let apply_val = sv.and_then(|v| v.get_typed(b"apply", JsonType::Array));
    let (Some(match_val), Some(apply_val)) = (match_val, apply_val) else {
        return None;
    };
    let sv = sv.unwrap();
    let match_items = match_val.as_array().unwrap();
    let apply_items = apply_val.as_array().unwrap();
    let mut rule = ChainingRule {
        match_count: match_items.len() as TableId,
        match_0: Vec::with_capacity(match_items.len()),
        ..ChainingRule::default()
    };
    rule.input_begins = sv.get_num_or(b"inputBegins", 0.0) as TableId;
    rule.input_ends = sv.get_num_or(b"inputEnds", rule.match_count as f64) as TableId;
    for item in match_items {
        // `parse_coverage` is a safe fn; `coverage_from_raw` is the one
        // still-unsafe `Box::from_raw` boundary it hands off to (same
        // `vqs_compare`-style narrow bridge used throughout this
        // migration).
        rule.match_0
            .push(unsafe { coverage_from_raw(parse_coverage(Some(item))) });
    }
    rule.apply = Vec::with_capacity(apply_items.len());
    for application in apply_items {
        let mut index: TableId = 0 as TableId;
        let mut lookup: LookupHandle = otfcc_handle_empty() as LookupHandle;
        if application.as_object().is_some() {
            if let Some(ln) = application.get_typed(b"lookup", JsonType::String) {
                lookup = handle_from_name(ln.as_str_bytes().map(|b| b.to_vec())) as LookupHandle;
                index = application.get_num(b"at") as TableId;
            }
        }
        rule.apply.push(ChainLookupApplication { index, lookup });
    }
    Some(Subtable::Chaining(ChainingSubtable::Canonical(rule)))
}
