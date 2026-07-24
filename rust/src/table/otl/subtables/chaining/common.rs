extern "C" {
    fn malloc(__size: usize) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: usize,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: usize,
    ) -> *mut ::core::ffi::c_void;
    static otl_iCoverage: __otfcc_ICoverage;
    static otl_iClassDef: __otfcc_IClassDef;
}
use crate::table::otl::classdef::{otl_ClassDef_free, otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage_free, otl_Coverage};
use crate::support::handle::{otfcc_Handle_dispose, otfcc_GlyphHandle, otfcc_LookupHandle};
pub type json_type = ::core::ffi::c_uint;
pub const json_pre_serialized: json_type = 8;
pub const json_null: json_type = 7;
pub const json_boolean: json_type = 6;
pub const json_string: json_type = 5;
pub const json_double: json_type = 4;
pub const json_integer: json_type = 3;
pub const json_array: json_type = 2;
pub const json_object: json_type = 1;
pub const json_none: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_value {
    pub parent: *mut _json_value,
    pub type_0: json_type,
    pub u: C2RustUnnamed_0,
    pub _reserved: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub next_alloc: *mut _json_value,
    pub object_mem: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub boolean: ::core::ffi::c_int,
    pub integer: i64,
    pub dbl: ::core::ffi::c_double,
    pub string: C2RustUnnamed_3,
    pub object: C2RustUnnamed_2,
    pub array: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut json_object_entry,
}
pub type json_object_entry = _json_object_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_object_entry {
    pub name: *mut ::core::ffi::c_char,
    pub name_length: ::core::ffi::c_uint,
    pub value: *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub length: ::core::ffi::c_uint,
    pub ptr: *mut ::core::ffi::c_char,
}
pub type json_value = _json_value;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: usize,
    pub size: usize,
    pub free: usize,
    pub data: *mut u8,
}
pub type glyphid_t = u16;
pub type glyphclass_t = u16;
pub type tableid_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __otfcc_ICoverage {
    pub init: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_Coverage, *const otl_Coverage) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_Coverage, *mut otl_Coverage) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_Coverage>,
    pub free: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_Coverage, u32) -> ()>,
    pub read: Option<unsafe extern "C" fn(*const u8, u32, u32) -> *mut otl_Coverage>,
    pub dump: Option<unsafe extern "C" fn(*const otl_Coverage) -> *mut json_value>,
    pub parse: Option<unsafe extern "C" fn(*const json_value) -> *mut otl_Coverage>,
    pub build: Option<unsafe extern "C" fn(*const otl_Coverage) -> *mut caryll_Buffer>,
    pub buildFormat:
        Option<unsafe extern "C" fn(*const otl_Coverage, u16) -> *mut caryll_Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut otl_Coverage, bool) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_Coverage, otfcc_GlyphHandle) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __otfcc_IClassDef {
    pub init: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_ClassDef, *const otl_ClassDef) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_ClassDef, *mut otl_ClassDef) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_ClassDef, otl_ClassDef) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_ClassDef, otl_ClassDef) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_ClassDef>,
    pub free: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub push:
        Option<unsafe extern "C" fn(*mut otl_ClassDef, otfcc_GlyphHandle, glyphclass_t) -> ()>,
    pub read: Option<unsafe extern "C" fn(*const u8, u32, u32) -> *mut otl_ClassDef>,
    pub expand:
        Option<unsafe extern "C" fn(*mut otl_Coverage, *mut otl_ClassDef) -> *mut otl_ClassDef>,
    pub dump: Option<unsafe extern "C" fn(*const otl_ClassDef) -> *mut json_value>,
    pub parse: Option<unsafe extern "C" fn(*const json_value) -> *mut otl_ClassDef>,
    pub build: Option<unsafe extern "C" fn(*const otl_ClassDef) -> *mut caryll_Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_chaining {
    pub type_0: otl_chaining_type,
    pub c2rust_unnamed: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub rule: otl_ChainingRule,
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub rulesCount: tableid_t,
    pub rules: *mut *mut otl_ChainingRule,
    pub bc: *mut otl_ClassDef,
    pub ic: *mut otl_ClassDef,
    pub fc: *mut otl_ClassDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_ChainingRule {
    pub matchCount: tableid_t,
    pub inputBegins: tableid_t,
    pub inputEnds: tableid_t,
    pub match_0: *mut *mut otl_Coverage,
    pub applyCount: tableid_t,
    pub apply: *mut otl_ChainLookupApplication,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_ChainLookupApplication {
    pub index: tableid_t,
    pub lookup: otfcc_LookupHandle,
}
pub type otl_chaining_type = ::core::ffi::c_uint;
pub const otl_chaining_classified: otl_chaining_type = 2;
pub const otl_chaining_poly: otl_chaining_type = 1;
pub const otl_chaining_canonical: otl_chaining_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_subtable_chaining {
    pub init: Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut subtable_chaining, *const subtable_chaining) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut subtable_chaining, *mut subtable_chaining) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut subtable_chaining, subtable_chaining) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut subtable_chaining, subtable_chaining) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_chaining>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn otl_init_chaining(mut subtable: *mut subtable_chaining) {
    memset(
        subtable as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<subtable_chaining>() as usize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn otl_dispose_chaining(mut subtable: *mut subtable_chaining) {
    if (*subtable).type_0 as u64 != 0 {
        if !(*subtable).c2rust_unnamed.c2rust_unnamed.rules.is_null() {
            let mut j: tableid_t = 0 as tableid_t;
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
                ::core::ptr::null_mut::<*mut otl_ChainingRule>();
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
#[no_mangle]
pub static mut iSubtable_chaining: __caryll_elementinterface_subtable_chaining = {
    __caryll_elementinterface_subtable_chaining {
        init: Some(subtable_chaining_init as unsafe extern "C" fn(*mut subtable_chaining) -> ()),
        copy: Some(
            subtable_chaining_copy
                as unsafe extern "C" fn(*mut subtable_chaining, *const subtable_chaining) -> (),
        ),
        move_0: Some(
            subtable_chaining_move
                as unsafe extern "C" fn(*mut subtable_chaining, *mut subtable_chaining) -> (),
        ),
        dispose: Some(
            subtable_chaining_dispose as unsafe extern "C" fn(*mut subtable_chaining) -> (),
        ),
        replace: Some(
            subtable_chaining_replace
                as unsafe extern "C" fn(*mut subtable_chaining, subtable_chaining) -> (),
        ),
        copyReplace: Some(
            subtable_chaining_copyReplace
                as unsafe extern "C" fn(*mut subtable_chaining, subtable_chaining) -> (),
        ),
        create: Some(subtable_chaining_create),
        free: Some(subtable_chaining_free as unsafe extern "C" fn(*mut subtable_chaining) -> ()),
    }
};
#[inline]
unsafe extern "C" fn subtable_chaining_free(mut x: *mut subtable_chaining) {
    if x.is_null() {
        return;
    }
    subtable_chaining_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_chaining_dispose(mut x: *mut subtable_chaining) {
    otl_dispose_chaining(x);
}
#[inline]
unsafe extern "C" fn subtable_chaining_init(mut x: *mut subtable_chaining) {
    otl_init_chaining(x);
}
#[inline]
unsafe extern "C" fn subtable_chaining_create() -> *mut subtable_chaining {
    let mut x: *mut subtable_chaining =
        malloc(::core::mem::size_of::<subtable_chaining>() as usize) as *mut subtable_chaining;
    subtable_chaining_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_chaining_copyReplace(
    mut dst: *mut subtable_chaining,
    src: subtable_chaining,
) {
    subtable_chaining_dispose(dst);
    subtable_chaining_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_chaining_replace(
    mut dst: *mut subtable_chaining,
    src: subtable_chaining,
) {
    subtable_chaining_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_chaining>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_chaining_copy(
    mut dst: *mut subtable_chaining,
    mut src: *const subtable_chaining,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_chaining>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_chaining_move(
    mut dst: *mut subtable_chaining,
    mut src: *mut subtable_chaining,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_chaining>() as usize,
    );
    subtable_chaining_init(src);
}
#[inline]
unsafe extern "C" fn closeRule(mut rule: *mut otl_ChainingRule) {
    if !rule.is_null()
        && !(*rule).match_0.is_null()
        && (*rule).matchCount as ::core::ffi::c_int != 0
    {
        let mut k: tableid_t = 0 as tableid_t;
        while (k as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
            otl_Coverage_free(
                *(*rule).match_0.offset(k as isize),
            );
            k = k.wrapping_add(1);
        }
        free((*rule).match_0 as *mut ::core::ffi::c_void);
        (*rule).match_0 = ::core::ptr::null_mut::<*mut otl_Coverage>();
    }
    if !rule.is_null() && !(*rule).apply.is_null() {
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*rule).applyCount as ::core::ffi::c_int {
            otfcc_Handle_dispose(
                &raw mut (*(*rule).apply.offset(j as isize)).lookup,
            );
            j = j.wrapping_add(1);
        }
        free((*rule).apply as *mut ::core::ffi::c_void);
        (*rule).apply = ::core::ptr::null_mut::<otl_ChainLookupApplication>();
    }
}
#[inline]
unsafe extern "C" fn deleteRule(mut rule: *mut otl_ChainingRule) {
    if rule.is_null() {
        return;
    }
    closeRule(rule);
    free(rule as *mut ::core::ffi::c_void);
    rule = ::core::ptr::null_mut::<otl_ChainingRule>();
}
