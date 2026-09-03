use crate::support::built_json::BuiltValue;
use crate::support::primitives::TableId;
use crate::table::otl::coverage::{Coverage, dump_coverage};
use crate::table::otl::subtables::chaining::common::{chaining_is_canonical, chaining_rule_const};
use crate::table::otl::{ChainingRule, ChainingSubtable, Subtable};

pub unsafe fn otl_dump_chaining(mut _subtable: *const Subtable) -> BuiltValue {
    unsafe {
        let Subtable::Chaining(mut_subtable) = &*_subtable else {
            unreachable!()
        };
        let subtable: *const ChainingSubtable = mut_subtable;
        if !chaining_is_canonical(subtable) {
            return BuiltValue::Null;
        }
        let rule: *const ChainingRule = chaining_rule_const(subtable);
        let mut _st = BuiltValue::new_object(4);
        let mut _match = BuiltValue::new_array((*rule).match_count as usize);
        let mut j: TableId = 0 as TableId;
        while (j as i32) < (*rule).match_count as i32 {
            _match.push_item(dump_coverage(
                &(&(*rule).match_0)[j as usize] as *const Coverage,
            ));
            j = j.wrapping_add(1);
        }
        _st.push_field(b"match", _match);
        let mut _apply = BuiltValue::new_array((*rule).apply.len());
        let mut j_0: TableId = 0 as TableId;
        while (j_0 as usize) < (*rule).apply.len() {
            let mut _application = BuiltValue::new_object(2);
            _application.push_field(b"at", BuiltValue::Int((&(*rule).apply)[j_0 as usize].index as i64));
            _application.push_field(
                b"lookup",
                BuiltValue::str_truncated_at_nul(&(&(*rule).apply)[j_0 as usize].lookup.name),
            );
            _apply.push_item(_application);
            j_0 = j_0.wrapping_add(1);
        }
        _st.push_field(b"apply", _apply.preserialize());
        _st.push_field(b"inputBegins", BuiltValue::Int((*rule).input_begins as i64));
        _st.push_field(b"inputEnds", BuiltValue::Int((*rule).input_ends as i64));
        _st
    }
}
