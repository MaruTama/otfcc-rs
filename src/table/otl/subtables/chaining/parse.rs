use crate::logger::ByteStr;
use crate::support::handle::{LookupHandle, handle_empty, handle_from_name};
use otfcc_json::ParsedValue;

use crate::support::primitives::TableId;
use otfcc_json::JsonType;

use crate::table::otl::coverage::parse_coverage;
use crate::table::otl::{ChainLookupApplication, ChainingRule, ChainingSubtable, Subtable};
/// Reads the object written by `otl_dump_chaining` into a single-rule
/// subtable, or `None` if `match` or `apply` is missing. Without
/// `inputBegins`/`inputEnds` the whole match is the input.
///
/// A `match` position may be a name from `classes`, the lookup's
/// `classes` object, instead of a glyph list; it stands for that object's
/// list. A name `classes` does not have matches no glyph.
pub fn otl_parse_chaining(
    _subtable: Option<&ParsedValue>,
    classes: Option<&ParsedValue>,
) -> Option<Subtable> {
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
        let list = match item.as_str_bytes() {
            Some(name) => {
                let class = classes.and_then(|c| c.get(name));
                if class.is_none() {
                    tracing::warn!("Glyph class {} is not defined.", ByteStr(name));
                }
                class
            }
            None => Some(item),
        };
        rule.sequence.push(parse_coverage(list));
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

#[cfg(test)]
mod otl_parse_chaining_tests {
    use super::*;
    use otfcc_json::parse_json;

    fn first_rule(subtable: &str, classes: Option<&str>) -> ChainingRule {
        let st = parse_json(subtable.as_bytes()).expect("valid JSON");
        let classes = classes.map(|c| parse_json(c.as_bytes()).expect("valid JSON"));
        let Some(Subtable::Chaining(ChainingSubtable::Canonical(rule))) =
            otl_parse_chaining(Some(&st), classes.as_ref())
        else {
            panic!("parsed a single-rule subtable")
        };
        rule
    }

    fn names(rule: &ChainingRule) -> Vec<Vec<String>> {
        rule.sequence
            .iter()
            .map(|c| {
                c.iter()
                    .map(|h| String::from_utf8_lossy(&h.name).into_owned())
                    .collect()
            })
            .collect()
    }

    #[test]
    fn a_class_name_stands_for_its_glyph_list() {
        let rule = first_rule(
            r#"{"match": ["@c1", ["x"], "@c1"], "apply": []}"#,
            Some(r#"{"@c1": ["a", "b"]}"#),
        );
        assert_eq!(
            names(&rule),
            vec![vec!["a", "b"], vec!["x"], vec!["a", "b"]]
        );
    }

    #[test]
    fn an_undefined_class_name_matches_no_glyph() {
        let rule = first_rule(r#"{"match": ["@c9", ["x"]], "apply": []}"#, None);
        assert_eq!(names(&rule), vec![vec![], vec!["x".to_string()]]);
    }
}
