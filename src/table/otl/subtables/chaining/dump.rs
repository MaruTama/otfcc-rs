use crate::table::otl::coverage::dump_coverage;
use crate::table::otl::subtables::chaining::common::{chaining_is_canonical, chaining_rule_const};
use crate::table::otl::{ChainingRule, Subtable};
use otfcc_json::BuiltValue;

/// Writes a single-rule subtable as `{ match, apply, inputBegins, inputEnds }`:
/// one coverage per position, the lookups applied at positions, and where
/// the input sequence starts and ends. A subtable still holding a rule set
/// is written as `null`.
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
    // Bounded by `rule.match_count`, which need not equal `sequence.len()`.
    for cov in rule.sequence.iter().take(rule.match_count as usize) {
        _match.push_item(dump_coverage(cov));
    }
    _st.push_field(b"match", _match);
    let mut _apply = BuiltValue::new_array(rule.apply.len());
    for application in rule.apply.iter() {
        let mut _application = BuiltValue::new_object(2);
        _application.push_field(b"at", BuiltValue::Int(i64::from(application.index)));
        _application.push_field(
            b"lookup",
            BuiltValue::str_truncated_at_nul(&application.lookup.name),
        );
        _apply.push_item(_application);
    }
    _st.push_field(b"apply", _apply.preserialize());
    _st.push_field(
        b"inputBegins",
        BuiltValue::Int(i64::from(rule.input_begins)),
    );
    _st.push_field(b"inputEnds", BuiltValue::Int(i64::from(rule.input_ends)));
    _st
}
