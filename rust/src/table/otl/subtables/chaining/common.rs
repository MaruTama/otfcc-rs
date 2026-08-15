#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc};

use crate::table::otl::{ChainingSubtableElementInterface, ChainingRule, ChainingRuleSet, ChainingSubtable};

pub unsafe extern "C" fn otl_init_chaining(mut subtable: *mut ChainingSubtable) {
    // No all-zero bit pattern is a valid `ChainingSubtable` (it owns `Vec`
    // fields through every variant), so place a valid empty `Canonical`
    // value directly instead of the old `memset`.
    ::core::ptr::write(subtable, ChainingSubtable::Canonical(ChainingRule::default()));
}
pub unsafe extern "C" fn otl_dispose_chaining(mut subtable: *mut ChainingSubtable) {
    // `ChainingRule`/`ChainingRuleSet` fully self-drop now (see
    // `table/otl.rs`), so running the enum's own `Drop` here does exactly
    // what the old tag-gated free logic did, for whichever variant is live,
    // with no need to inspect the tag first.
    ::core::ptr::drop_in_place(subtable);
}
/// Returns a mutable pointer into the `Canonical` variant's payload.
/// Panics (rather than reading union garbage, the old failure mode) if
/// called on a `Poly`/`Classified` subtable -- every call site already
/// assumed `Canonical` at that point, matching the original C code's own
/// (unchecked) assumption.
pub(crate) unsafe fn chaining_rule_mut(subtable: *mut ChainingSubtable) -> *mut ChainingRule {
    match &mut *subtable {
        ChainingSubtable::Canonical(rule) => rule as *mut ChainingRule,
        _ => unreachable!("chaining_rule_mut: subtable is not Canonical"),
    }
}
pub(crate) unsafe fn chaining_rule_const(subtable: *const ChainingSubtable) -> *const ChainingRule {
    match &*subtable {
        ChainingSubtable::Canonical(rule) => rule as *const ChainingRule,
        _ => unreachable!("chaining_rule_const: subtable is not Canonical"),
    }
}
/// Same idea as `chaining_rule_const`, but returns `*mut`: several call
/// sites in `build.rs` reach a `ChainingRule` through a `*const
/// ChainingSubtable` and then mutate it in place (e.g. `reverse_backtracks`)
/// -- the same const-to-mut pointer cast the original C-shaped code already
/// did, preserved verbatim rather than "fixed" here.
pub(crate) unsafe fn chaining_rule_mut_from_const(
    subtable: *const ChainingSubtable,
) -> *mut ChainingRule {
    chaining_rule_const(subtable) as *mut ChainingRule
}
/// Returns a mutable pointer into the `Poly`/`Classified` payload -- both
/// variants carry the same `ChainingRuleSet` shape, so callers that don't
/// care which one it is (most of them) can use this without matching twice.
pub(crate) unsafe fn chaining_ruleset_mut(subtable: *mut ChainingSubtable) -> *mut ChainingRuleSet {
    match &mut *subtable {
        ChainingSubtable::Poly(rs) | ChainingSubtable::Classified(rs) => rs as *mut ChainingRuleSet,
        ChainingSubtable::Canonical(_) => unreachable!("chaining_ruleset_mut: subtable is Canonical"),
    }
}
pub(crate) unsafe fn chaining_ruleset_const(
    subtable: *const ChainingSubtable,
) -> *const ChainingRuleSet {
    match &*subtable {
        ChainingSubtable::Poly(rs) | ChainingSubtable::Classified(rs) => rs as *const ChainingRuleSet,
        ChainingSubtable::Canonical(_) => {
            unreachable!("chaining_ruleset_const: subtable is Canonical")
        }
    }
}
/// Replaces the old `(*subtable).type_0 == ChainingType::Classified` reads
/// -- `build.rs`'s binary-format choice (class-list vs coverage-list
/// encoding) is the one place `Poly` and `Classified` still need
/// distinguishing, even though they share the same `ChainingRuleSet` shape.
pub(crate) unsafe fn chaining_is_classified(subtable: *const ChainingSubtable) -> bool {
    matches!(&*subtable, ChainingSubtable::Classified(_))
}
/// Replaces the old `(*subtable).type_0 as u64 != 0` ("not Canonical")
/// reads -- `dump.rs`/`classifier.rs` use this to mean "still a ruleset,
/// not yet reduced to one rule per subtable".
pub(crate) unsafe fn chaining_is_canonical(subtable: *const ChainingSubtable) -> bool {
    matches!(&*subtable, ChainingSubtable::Canonical(_))
}
pub static I_SUBTABLE_CHAINING: ChainingSubtableElementInterface = {
    ChainingSubtableElementInterface {
        init: Some(subtable_chaining_init as unsafe extern "C" fn(*mut ChainingSubtable) -> ()),
        copy: Some(
            subtable_chaining_copy
                as unsafe extern "C" fn(*mut ChainingSubtable, *const ChainingSubtable) -> (),
        ),
        dispose: Some(
            subtable_chaining_dispose as unsafe extern "C" fn(*mut ChainingSubtable) -> (),
        ),
        create: Some(subtable_chaining_create),
        free: Some(subtable_chaining_free as unsafe extern "C" fn(*mut ChainingSubtable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn subtable_chaining_free(mut x: *mut ChainingSubtable) {
    if x.is_null() {
        return;
    }
    subtable_chaining_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_chaining_dispose(mut x: *mut ChainingSubtable) {
    otl_dispose_chaining(x);
}
#[inline]
unsafe extern "C" fn subtable_chaining_init(mut x: *mut ChainingSubtable) {
    otl_init_chaining(x);
}
#[inline]
unsafe extern "C" fn subtable_chaining_create() -> *mut ChainingSubtable {
    let mut x: *mut ChainingSubtable =
        malloc(::core::mem::size_of::<ChainingSubtable>() as usize) as *mut ChainingSubtable;
    subtable_chaining_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_chaining_copy(
    mut _dst: *mut ChainingSubtable,
    mut _src: *const ChainingSubtable,
) {
    // Confirmed dead: never called outside this vtable's own static
    // initializer (see `table/otl.rs`'s doc comment on `ChainingSubtable`).
    // The old `memcpy`-based body was only safe by accident, back when the
    // union's `ManuallyDrop` fields were bitwise-copyable; now that
    // `ChainingRuleSet.bc`/`.ic`/`.fc` are `Option<Box<ClassDef>>`, a raw
    // `memcpy` would alias and eventually double-free. Kept as a loud
    // failure instead of silently reintroducing that risk if this ever
    // gets wired up.
    unreachable!("ChainingSubtable::copy is dead code and unsound for owned Vec/Box data")
}
