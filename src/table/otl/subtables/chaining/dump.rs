use crate::support::built_json::BuiltValue;
use crate::table::otl::coverage::dump_coverage;
use crate::table::otl::subtables::chaining::common::{chaining_is_canonical, chaining_rule_const};
use crate::table::otl::{ChainingRule, Subtable};

pub fn otl_dump_chaining(_subtable: &Subtable) -> BuiltValue {
    let Subtable::Chaining(subtable) = _subtable else {
        unreachable!()
    };
    if !chaining_is_canonical(subtable) {
        return BuiltValue::Null;
    }
    let rule: &ChainingRule = chaining_rule_const(subtable);
    let mut _st = BuiltValue::new_object(4);
    let mut _match = BuiltValue::new_array(rule.match_count as usize);
    // Bounded by `rule.match_count`, not assumed equal to `match_0.len()`
    // (same count-vs-length caution established in PR #422/#423/#426-428).
    for cov in rule.match_0.iter().take(rule.match_count as usize) {
        _match.push_item(dump_coverage(cov));
    }
    _st.push_field(b"match", _match);
    let mut _apply = BuiltValue::new_array(rule.apply.len());
    for application in rule.apply.iter() {
        let mut _application = BuiltValue::new_object(2);
        _application.push_field(b"at", BuiltValue::Int(application.index as i64));
        _application.push_field(
            b"lookup",
            BuiltValue::str_truncated_at_nul(&application.lookup.name),
        );
        _apply.push_item(_application);
    }
    _st.push_field(b"apply", _apply.preserialize());
    _st.push_field(b"inputBegins", BuiltValue::Int(rule.input_begins as i64));
    _st.push_field(b"inputEnds", BuiltValue::Int(rule.input_ends as i64));
    _st
}
