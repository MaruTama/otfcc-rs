use crate::support::handle::{LookupHandle, handle_empty, handle_from_name};
use otfcc_json::ParsedValue;

use crate::support::primitives::TableId;
use otfcc_json::JsonType;

use crate::table::otl::coverage::parse_coverage;
use crate::table::otl::{ChainLookupApplication, ChainingRule, ChainingSubtable, Subtable};
pub fn otl_parse_chaining(_subtable: Option<&ParsedValue>) -> Option<Subtable> {
    let sv = _subtable;
    let match_val = sv.and_then(|v| v.get_typed(b"match", JsonType::Array));
    let apply_val = sv.and_then(|v| v.get_typed(b"apply", JsonType::Array));
    let (Some(match_val), Some(apply_val)) = (match_val, apply_val) else {
        return None;
    };
    let sv = sv.expect("match_val and apply_val came from sv");
    let match_items = match_val.as_array().expect("get_typed returned an Array");
    let apply_items = apply_val.as_array().expect("get_typed returned an Array");
    let mut rule = ChainingRule {
        match_count: match_items.len() as TableId,
        sequence: Vec::with_capacity(match_items.len()),
        ..ChainingRule::default()
    };
    rule.input_begins = sv.get_num_or(b"inputBegins", 0.0) as TableId;
    rule.input_ends = sv.get_num_or(b"inputEnds", f64::from(rule.match_count)) as TableId;
    for item in match_items {
        rule.sequence.push(parse_coverage(Some(item)));
    }
    rule.apply = Vec::with_capacity(apply_items.len());
    for application in apply_items {
        let mut index: TableId = 0 as TableId;
        let mut lookup: LookupHandle = handle_empty() as LookupHandle;
        if application.as_object().is_some()
            && let Some(ln) = application.get_typed(b"lookup", JsonType::String)
        {
            lookup = handle_from_name(ln.as_str_bytes().map(|b| b.to_vec())) as LookupHandle;
            index = application.get_num(b"at") as TableId;
        }
        rule.apply.push(ChainLookupApplication { index, lookup });
    }
    Some(Subtable::Chaining(ChainingSubtable::Canonical(rule)))
}
