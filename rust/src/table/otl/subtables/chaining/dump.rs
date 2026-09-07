use crate::support::built_json::BuiltValue;
use crate::support::primitives::TableId;
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
    // `chaining_rule_const` itself is a safe fn, but still returns a raw
    // `*const ChainingRule` (chaining/common.rs's own `Canonical`-variant
    // extraction hasn't been widened to a lifetime-bound reference) --
    // dereference it once, narrowly, the same bridge pattern `vf/vq.rs`'s
    // `vqs_compare` established for a single remaining call into an
    // unconverted shell.
    let rule: &ChainingRule = unsafe { &*chaining_rule_const(subtable) };
    let mut _st = BuiltValue::new_object(4);
    let mut _match = BuiltValue::new_array(rule.match_count as usize);
    let mut j: TableId = 0 as TableId;
    while (j as i32) < rule.match_count as i32 {
        _match.push_item(dump_coverage(&rule.match_0[j as usize]));
        j = j.wrapping_add(1);
    }
    _st.push_field(b"match", _match);
    let mut _apply = BuiltValue::new_array(rule.apply.len());
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < rule.apply.len() {
        let mut _application = BuiltValue::new_object(2);
        _application.push_field(b"at", BuiltValue::Int(rule.apply[j_0 as usize].index as i64));
        _application.push_field(
            b"lookup",
            BuiltValue::str_truncated_at_nul(&rule.apply[j_0 as usize].lookup.name),
        );
        _apply.push_item(_application);
        j_0 = j_0.wrapping_add(1);
    }
    _st.push_field(b"apply", _apply.preserialize());
    _st.push_field(b"inputBegins", BuiltValue::Int(rule.input_begins as i64));
    _st.push_field(b"inputEnds", BuiltValue::Int(rule.input_ends as i64));
    _st
}
