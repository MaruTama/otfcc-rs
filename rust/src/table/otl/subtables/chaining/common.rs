#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};

use crate::table::otl::{ChainingRule, ChainingRuleSet, ChainingSubtable};

pub unsafe fn otl_init_chaining(mut subtable: *mut ChainingSubtable) {
    // No all-zero bit pattern is a valid `ChainingSubtable` (it owns `Vec`
    // fields through every variant), so place a valid empty `Canonical`
    // value directly instead of the old `memset`.
    ::core::ptr::write(subtable, ChainingSubtable::Canonical(ChainingRule::default()));
}
pub unsafe fn otl_dispose_chaining(mut subtable: *mut ChainingSubtable) {
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
/// Frees a `*mut ChainingSubtable` allocated with `__caryll_allocate_clean`
/// (`calloc`) -- **not** one of `subtable_chaining_create()`'s own `Box`-
/// allocated results (Stage 7-2-d changed only that function's own
/// allocation strategy, not this one, precisely because this free function
/// is still needed unchanged for a genuinely different allocation origin).
/// `chaining/classifier.rs`'s `try_classify_around` builds its replacement
/// subtable with `__caryll_allocate_clean` directly (a still-malloc-shaped
/// intermediate, out of this stage's scope) and frees it here when
/// `otfcc_classified_build_chaining` swaps it back out -- that `calloc`/
/// `free` pairing is exactly what this function still does. A `Box`-derived
/// `ChainingSubtable` (from `subtable_chaining_create()`, e.g. the read-path
/// error handling in `chaining/read.rs`) must instead be reclaimed with
/// `drop(Box::from_raw(x))` directly, matching `subtable_from_raw`'s own
/// `Box::from_raw` -- mixing `Box`-allocated memory into this `free()` would
/// be exactly the allocator-mismatch hazard Stage 7-2-d elsewhere converts
/// away from.
#[inline]
pub(crate) unsafe fn subtable_chaining_free(mut x: *mut ChainingSubtable) {
    if x.is_null() {
        return;
    }
    subtable_chaining_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe fn subtable_chaining_dispose(mut x: *mut ChainingSubtable) {
    otl_dispose_chaining(x);
}
#[inline]
pub(crate) unsafe fn subtable_chaining_create() -> *mut ChainingSubtable {
    // A real Rust allocation now, not a `malloc`'d shell (Stage 7-2-d):
    // `Box::into_raw` gives back a pointer with the same shape (`*mut
    // ChainingSubtable`) every caller already expects -- `otl_read_contextual`/
    // `otl_read_chaining` immediately thread it through several raw-pointer
    // helpers (`read_contextual_format1`/`2`, `read_chaining_format1`/`2`)
    // before it ever reaches `subtable_from_raw`, so it stays `*mut T`-shaped
    // rather than becoming `Box<ChainingSubtable>` at the API boundary the
    // way the simpler, single-function subtable `_create()`s in this
    // directory now do. It must from here on only ever be reclaimed with
    // `Box::from_raw` (`subtable_from_raw`, or a direct `drop(Box::from_raw(
    // ..))` on an error path in `chaining/read.rs`), **never**
    // `subtable_chaining_free` -- see that function's doc comment for why.
    Box::into_raw(Box::new(ChainingSubtable::Canonical(ChainingRule::default())))
}
