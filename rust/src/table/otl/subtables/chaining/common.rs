#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};
use crate::table::otl::classdef::otl_ClassDef_free;
use crate::table::otl::coverage::{Coverage, otl_Coverage_free};
use crate::support::handle::{otfcc_Handle_dispose};

use crate::support::primitives::{TableId};


use crate::table::otl::{ChainingSubtableElementInterface, ChainLookupApplication, ChainingRule, ChainingSubtable};

pub unsafe extern "C" fn otl_init_chaining(mut subtable: *mut ChainingSubtable) {
    memset(
        subtable as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ChainingSubtable>() as usize,
    );
}
pub unsafe extern "C" fn otl_dispose_chaining(mut subtable: *mut ChainingSubtable) {
    if (*subtable).type_0 as u64 != 0 {
        if !(*subtable).c2rust_unnamed.c2rust_unnamed.rules.is_null() {
            let mut j: TableId = 0 as TableId;
            while (j as ::core::ffi::c_int)
                < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
            {
                deleteRule(
                    *(*subtable)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .rules
                        .offset(j as isize),
                );
                j = j.wrapping_add(1);
            }
            free((*subtable).c2rust_unnamed.c2rust_unnamed.rules as *mut ::core::ffi::c_void);
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules =
                ::core::ptr::null_mut::<*mut ChainingRule>();
        }
        if !(*subtable).c2rust_unnamed.c2rust_unnamed.bc.is_null() {
            otl_ClassDef_free(
                (*subtable).c2rust_unnamed.c2rust_unnamed.bc,
            );
        }
        if !(*subtable).c2rust_unnamed.c2rust_unnamed.ic.is_null() {
            otl_ClassDef_free(
                (*subtable).c2rust_unnamed.c2rust_unnamed.ic,
            );
        }
        if !(*subtable).c2rust_unnamed.c2rust_unnamed.fc.is_null() {
            otl_ClassDef_free(
                (*subtable).c2rust_unnamed.c2rust_unnamed.fc,
            );
        }
    } else {
        closeRule(&raw mut (*subtable).c2rust_unnamed.rule);
    };
}
pub static I_SUBTABLE_CHAINING: ChainingSubtableElementInterface = {
    ChainingSubtableElementInterface {
        init: Some(subtable_chaining_init as unsafe extern "C" fn(*mut ChainingSubtable) -> ()),
        copy: Some(
            subtable_chaining_copy
                as unsafe extern "C" fn(*mut ChainingSubtable, *const ChainingSubtable) -> (),
        ),
        move_0: Some(
            subtable_chaining_move
                as unsafe extern "C" fn(*mut ChainingSubtable, *mut ChainingSubtable) -> (),
        ),
        dispose: Some(
            subtable_chaining_dispose as unsafe extern "C" fn(*mut ChainingSubtable) -> (),
        ),
        replace: Some(
            subtable_chaining_replace
                as unsafe extern "C" fn(*mut ChainingSubtable, ChainingSubtable) -> (),
        ),
        copyReplace: Some(
            subtable_chaining_copyReplace
                as unsafe extern "C" fn(*mut ChainingSubtable, ChainingSubtable) -> (),
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
unsafe extern "C" fn subtable_chaining_copyReplace(
    mut dst: *mut ChainingSubtable,
    src: ChainingSubtable,
) {
    subtable_chaining_dispose(dst);
    subtable_chaining_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_chaining_replace(
    mut dst: *mut ChainingSubtable,
    src: ChainingSubtable,
) {
    subtable_chaining_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ChainingSubtable>() as usize,
    );
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
#[inline]
unsafe extern "C" fn subtable_chaining_move(
    mut dst: *mut ChainingSubtable,
    mut src: *mut ChainingSubtable,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ChainingSubtable>() as usize,
    );
    subtable_chaining_init(src);
}
#[inline]
unsafe extern "C" fn closeRule(mut rule: *mut ChainingRule) {
    if !rule.is_null()
        && !(*rule).match_0.is_null()
        && (*rule).matchCount as ::core::ffi::c_int != 0
    {
        let mut k: TableId = 0 as TableId;
        while (k as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
            otl_Coverage_free(
                *(*rule).match_0.offset(k as isize),
            );
            k = k.wrapping_add(1);
        }
        free((*rule).match_0 as *mut ::core::ffi::c_void);
        (*rule).match_0 = ::core::ptr::null_mut::<*mut Coverage>();
    }
    if !rule.is_null() && !(*rule).apply.is_null() {
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_int) < (*rule).applyCount as ::core::ffi::c_int {
            otfcc_Handle_dispose(
                &raw mut (*(*rule).apply.offset(j as isize)).lookup,
            );
            j = j.wrapping_add(1);
        }
        free((*rule).apply as *mut ::core::ffi::c_void);
        (*rule).apply = ::core::ptr::null_mut::<ChainLookupApplication>();
    }
}
#[inline]
unsafe extern "C" fn deleteRule(mut rule: *mut ChainingRule) {
    if rule.is_null() {
        return;
    }
    closeRule(rule);
    free(rule as *mut ::core::ffi::c_void);
    rule = ::core::ptr::null_mut::<ChainingRule>();
}
