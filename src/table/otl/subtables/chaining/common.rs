use crate::table::otl::{ChainingRule, ChainingRuleSet, ChainingSubtable, Subtable};

/// Returns a mutable reference into the `Canonical` variant's payload.
/// Panics if called on a `Poly`/`Classified` subtable; every call site
/// only reaches here with a `Canonical` one.
pub(crate) fn chaining_rule_mut(subtable: &mut ChainingSubtable) -> &mut ChainingRule {
    match subtable {
        ChainingSubtable::Canonical(rule) => rule,
        _ => unreachable!("chaining_rule_mut: subtable is not Canonical"),
    }
}
/// Returns a shared reference into the `Canonical` variant's payload.
pub(crate) fn chaining_rule_const(subtable: &ChainingSubtable) -> &ChainingRule {
    match subtable {
        ChainingSubtable::Canonical(rule) => rule,
        _ => unreachable!("chaining_rule_const: subtable is not Canonical"),
    }
}
/// Returns a mutable reference into the `Poly`/`Classified` payload -- both
/// variants carry the same `ChainingRuleSet` shape, so callers that don't
/// care which one it is (most of them) can use this without matching twice.
pub(crate) fn chaining_ruleset_mut(subtable: &mut ChainingSubtable) -> &mut ChainingRuleSet {
    match subtable {
        ChainingSubtable::Poly(rs) | ChainingSubtable::Classified(rs) => rs,
        ChainingSubtable::Canonical(_) => {
            unreachable!("chaining_ruleset_mut: subtable is Canonical")
        }
    }
}
/// Shared reference counterpart of `chaining_ruleset_mut`, above.
pub(crate) fn chaining_ruleset_const(subtable: &ChainingSubtable) -> &ChainingRuleSet {
    match subtable {
        ChainingSubtable::Poly(rs) | ChainingSubtable::Classified(rs) => rs,
        ChainingSubtable::Canonical(_) => {
            unreachable!("chaining_ruleset_const: subtable is Canonical")
        }
    }
}
/// `build.rs`'s binary-format choice (class-list vs coverage-list
/// encoding) is the one place `Poly` and `Classified` need distinguishing,
/// even though they share the same `ChainingRuleSet` shape.
pub(crate) fn chaining_is_classified(subtable: &ChainingSubtable) -> bool {
    matches!(subtable, ChainingSubtable::Classified(_))
}
/// True unless the subtable is `Canonical` (one rule per subtable) --
/// i.e. it is still a ruleset.
pub(crate) fn chaining_is_canonical(subtable: &ChainingSubtable) -> bool {
    matches!(subtable, ChainingSubtable::Canonical(_))
}
/// Extract the `&ChainingSubtable` a `SubtableList` slot holds, panicking on
/// an empty slot or a non-`Chaining` variant -- every caller only ever calls
/// this on a lookup already known to be an
/// `OTL_TYPE_{GSUB,GPOS}_{CHAINING,CONTEXT}` one, so every slot's payload is
/// a `ChainingSubtable` by construction. Shared by `classifier.rs` and
/// `build.rs`'s `chaining_lookup_is_contextual_lookup`.
pub(crate) fn chaining_subtable_ref(slot: &Option<Box<Subtable>>) -> &ChainingSubtable {
    let Subtable::Chaining(subtable) = slot
        .as_deref()
        .expect("subtable slot should not be empty at this point")
    else {
        unreachable!()
    };
    subtable
}
