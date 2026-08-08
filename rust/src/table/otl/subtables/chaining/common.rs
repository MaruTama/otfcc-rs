#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};
use crate::table::otl::classdef::otl_class_def_free;

use crate::table::otl::{ChainingSubtableElementInterface, ChainingRule, ChainingRuleSet, ChainingSubtable};

pub unsafe extern "C" fn otl_init_chaining(mut subtable: *mut ChainingSubtable) {
    memset(
        subtable as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ChainingSubtable>() as usize,
    );
}
pub unsafe extern "C" fn otl_dispose_chaining(mut subtable: *mut ChainingSubtable) {
    if (*subtable).type_0 as u64 != 0 {
        // Every creation site that sets `type_0` to `Poly`/`Classified`
        // placement-constructs `.rules` (a valid, possibly-empty `Vec`) in
        // the same breath -- see `read.rs`/`classifier.rs` -- so by the
        // time `type_0 != Canonical` is observable here, `.rules` is
        // always a valid `Vec`, never the raw zeroed bytes `create()`'s
        // `memset` leaves behind. No `is_null()`-style guard needed.
        let ruleset: *mut ChainingRuleSet =
            &raw mut (*subtable).c2rust_unnamed.c2rust_unnamed as *mut ChainingRuleSet;
        // Each element's `Box<ChainingRule>` (where `Some`) already has a
        // real `Drop` (see `table/otl.rs`), so dropping the `Vec` disposes
        // every rule correctly -- no manual per-element walk needed.
        (*ruleset).rules = Vec::new();
        if !(*ruleset).bc.is_null() {
            otl_class_def_free((*ruleset).bc);
        }
        if !(*ruleset).ic.is_null() {
            otl_class_def_free((*ruleset).ic);
        }
        if !(*ruleset).fc.is_null() {
            otl_class_def_free((*ruleset).fc);
        }
    } else {
        close_rule(&raw mut (*subtable).c2rust_unnamed.rule as *mut ChainingRule);
    };
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
    mut dst: *mut ChainingSubtable,
    mut src: *const ChainingSubtable,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ChainingSubtable>() as usize,
    );
}
/// The `Canonical` variant (`ChainingBody.rule: ManuallyDrop<ChainingRule>`)
/// is never `Box`-owned, so `ManuallyDrop` suppresses its automatic `Drop` --
/// this runs `ChainingRule`'s `Drop` impl explicitly through the raw
/// pointer instead, exactly like disposing any other `ManuallyDrop` field.
#[inline]
unsafe extern "C" fn close_rule(mut rule: *mut ChainingRule) {
    if !rule.is_null() {
        ::core::ptr::drop_in_place(rule);
    }
}
