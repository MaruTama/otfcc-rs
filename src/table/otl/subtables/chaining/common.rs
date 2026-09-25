use crate::table::otl::{ChainingRule, ChainingRuleSet, ChainingSubtable, Subtable};

/// Returns a mutable reference into the `Canonical` variant's payload.
/// Panics (rather than reading union garbage, the old failure mode) if
/// called on a `Poly`/`Classified` subtable -- every call site already
/// assumed `Canonical` at that point, matching the original C code's own
/// (unchecked) assumption. Safe `&mut ChainingRule` (not a raw pointer):
/// its one call site (`consolidate/otl/chaining.rs`'s
/// `consolidate_chaining`) already holds a real `&mut Subtable` all the
/// way down to this call, so there is no `*mut` boundary left to preserve
/// here -- same reasoning as `chaining_ruleset_mut`, below.
pub(crate) fn chaining_rule_mut(subtable: &mut ChainingSubtable) -> &mut ChainingRule {
    match subtable {
        ChainingSubtable::Canonical(rule) => rule,
        _ => unreachable!("chaining_rule_mut: subtable is not Canonical"),
    }
}
/// Returns a shared reference into the `Canonical` variant's payload.
/// Safe `&ChainingRule` (not a raw pointer): every call site (`build.rs`'s
/// `otfcc_chaining_lookup_is_contextual_lookup`, `dump.rs`, and -- since
/// Stage L-6 -- `classifier.rs`'s `try_classify_around`) only ever reads
/// through it, so there is no `*const` boundary left to preserve here.
pub(crate) fn chaining_rule_const(subtable: &ChainingSubtable) -> &ChainingRule {
    match subtable {
        ChainingSubtable::Canonical(rule) => rule,
        _ => unreachable!("chaining_rule_const: subtable is not Canonical"),
    }
}
/// Returns a mutable reference into the `Poly`/`Classified` payload -- both
/// variants carry the same `ChainingRuleSet` shape, so callers that don't
/// care which one it is (most of them) can use this without matching twice.
/// Returns a safe `&mut` (not a raw pointer): every call site either already
/// holds a real `&mut ChainingSubtable`/`unsafe`-derived reborrow of one
/// (`classifier.rs`), or -- since Stage L-5 -- an owned `Box<ChainingSubtable>`
/// (`chaining/read.rs`), so there is no `*const`/`*mut` boundary left to
/// preserve here; a raw-pointer-returning sibling would only reintroduce one.
pub(crate) fn chaining_ruleset_mut(subtable: &mut ChainingSubtable) -> &mut ChainingRuleSet {
    match subtable {
        ChainingSubtable::Poly(rs) | ChainingSubtable::Classified(rs) => rs,
        ChainingSubtable::Canonical(_) => {
            unreachable!("chaining_ruleset_mut: subtable is Canonical")
        }
    }
}
/// Shared reference counterpart of `chaining_ruleset_mut`, above -- same
/// "safe reference, not a raw pointer" reasoning: `build.rs`'s
/// `otfcc_chaining_lookup_is_contextual_lookup` only ever reads through it.
pub(crate) fn chaining_ruleset_const(subtable: &ChainingSubtable) -> &ChainingRuleSet {
    match subtable {
        ChainingSubtable::Poly(rs) | ChainingSubtable::Classified(rs) => rs,
        ChainingSubtable::Canonical(_) => {
            unreachable!("chaining_ruleset_const: subtable is Canonical")
        }
    }
}
/// Replaces the old `(*subtable).type_0 == ChainingType::Classified` reads
/// -- `build.rs`'s binary-format choice (class-list vs coverage-list
/// encoding) is the one place `Poly` and `Classified` still need
/// distinguishing, even though they share the same `ChainingRuleSet` shape.
pub(crate) fn chaining_is_classified(subtable: &ChainingSubtable) -> bool {
    matches!(subtable, ChainingSubtable::Classified(_))
}
/// Replaces the old `(*subtable).type_0 as u64 != 0` ("not Canonical")
/// reads -- `dump.rs`/`classifier.rs` use this to mean "still a ruleset,
/// not yet reduced to one rule per subtable".
pub(crate) fn chaining_is_canonical(subtable: &ChainingSubtable) -> bool {
    matches!(subtable, ChainingSubtable::Canonical(_))
}
/// Extract the `&ChainingSubtable` a `SubtableList` slot holds, panicking on
/// an empty slot (same wording `table/otl.rs`'s `subtable_at` uses for the
/// raw-pointer equivalent) or a non-`Chaining` variant -- every caller only
/// ever calls this on a lookup already known to be an
/// `OTL_TYPE_{GSUB,GPOS}_{CHAINING,CONTEXT}` one, so every slot's payload is
/// a `ChainingSubtable` by construction. Shared by `classifier.rs` and
/// `build.rs`'s `otfcc_chaining_lookup_is_contextual_lookup` -- both used to
/// reach the same slot via `subtable_at` (a raw `*mut Subtable`) followed by
/// their own `let Subtable::Chaining(..) = &*ptr else { unreachable!() }`;
/// this is that same match, but starting from (and staying) a safe
/// reference all the way through.
pub(crate) fn chaining_subtable_ref(slot: &Option<Box<Subtable>>) -> &ChainingSubtable {
    let Subtable::Chaining(subtable) = slot
        .as_deref()
        .expect("subtable slot should not be empty at this point")
    else {
        unreachable!()
    };
    subtable
}
