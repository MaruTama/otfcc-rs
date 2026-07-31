#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};
use crate::vendor::sds::{SdsRaw};
use crate::support::cvec::{CVecRaw, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_pop, cvec_push, cvec_resize_to};
use crate::support::{ComparFn};
use crate::vendor::sds::{sdsfree};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetaEntry {
    pub tag: u32,
    pub data: SdsRaw,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetaEntryElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut MetaEntry) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut MetaEntry, *const MetaEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut MetaEntry) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetaEntries {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut MetaEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetaEntriesVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut MetaEntries) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut MetaEntries, *const MetaEntries) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut MetaEntries) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut MetaEntries>,
    pub free: Option<unsafe extern "C" fn(*mut MetaEntries) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut MetaEntries, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut MetaEntries, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut MetaEntries>,
    pub fill: Option<unsafe extern "C" fn(*mut MetaEntries, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut MetaEntries) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut MetaEntries, MetaEntry) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut MetaEntries) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut MetaEntries) -> MetaEntry>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut MetaEntries, usize) -> ()>,
    pub filter_env: Option<
        unsafe extern "C" fn(
            *mut MetaEntries,
            Option<unsafe extern "C" fn(*const MetaEntry, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut MetaEntries,
            Option<unsafe extern "C" fn(*const MetaEntry, *const MetaEntry) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetaTable {
    pub version: u32,
    pub flags: u32,
    pub entries: MetaEntries,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetaTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut MetaTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut MetaTable, *const MetaTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut MetaTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut MetaTable>,
    pub free: Option<unsafe extern "C" fn(*mut MetaTable) -> ()>,
}
unsafe extern "C" fn init_meta_entry(mut e: *mut MetaEntry) {
    (*e).tag = 1 as u32;
    (*e).data = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn dispose_meta_entry(mut e: *mut MetaEntry) {
    sdsfree((*e).data);
}
pub static META_I_ENTRY: MetaEntryElementInterface = {
    MetaEntryElementInterface {
        init: Some(meta_entry_init as unsafe extern "C" fn(*mut MetaEntry) -> ()),
        copy: Some(
            meta_entry_copy as unsafe extern "C" fn(*mut MetaEntry, *const MetaEntry) -> (),
        ),
        dispose: Some(meta_entry_dispose as unsafe extern "C" fn(*mut MetaEntry) -> ()),
    }
};
#[inline]
unsafe extern "C" fn meta_entry_init(mut x: *mut MetaEntry) {
    init_meta_entry(x);
}
#[inline]
unsafe extern "C" fn meta_entry_dispose(mut x: *mut MetaEntry) {
    dispose_meta_entry(x);
}
#[inline]
unsafe extern "C" fn meta_entry_copy(mut dst: *mut MetaEntry, mut src: *const MetaEntry) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MetaEntry>() as usize,
    );
}
#[inline]
unsafe extern "C" fn meta_entries_filter_env(
    mut arr: *mut MetaEntries,
    mut fn_0: Option<unsafe extern "C" fn(*const MetaEntry, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut MetaEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if META_I_ENTRY.dispose.is_some() {
                META_I_ENTRY.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut MetaEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn meta_entries_as_cvec(arr: *mut MetaEntries) -> *mut CVecRaw<MetaEntry> {
    arr as *mut CVecRaw<MetaEntry>
}
#[inline]
unsafe extern "C" fn meta_entries_init(arr: *mut MetaEntries) {
    cvec_init(meta_entries_as_cvec(arr));
}
pub static META_I_ENTRIES: MetaEntriesVectorInterface = {
    MetaEntriesVectorInterface {
        init: Some(meta_entries_init as unsafe extern "C" fn(*mut MetaEntries) -> ()),
        copy: Some(
            meta_entries_copy as unsafe extern "C" fn(*mut MetaEntries, *const MetaEntries) -> (),
        ),
        dispose: Some(meta_entries_dispose as unsafe extern "C" fn(*mut MetaEntries) -> ()),
        create: Some(meta_entries_create),
        free: Some(meta_entries_free as unsafe extern "C" fn(*mut MetaEntries) -> ()),
        init_n: Some(meta_entries_init_n as unsafe extern "C" fn(*mut MetaEntries, usize) -> ()),
        init_cap_n: Some(
            meta_entries_init_cap_n as unsafe extern "C" fn(*mut MetaEntries, usize) -> (),
        ),
        create_n: Some(meta_entries_create_n as unsafe extern "C" fn(usize) -> *mut MetaEntries),
        fill: Some(meta_entries_fill as unsafe extern "C" fn(*mut MetaEntries, usize) -> ()),
        clear: Some(meta_entries_dispose as unsafe extern "C" fn(*mut MetaEntries) -> ()),
        push: Some(meta_entries_push as unsafe extern "C" fn(*mut MetaEntries, MetaEntry) -> ()),
        shrink_to_fit: Some(
            meta_entries_shrink_to_fit as unsafe extern "C" fn(*mut MetaEntries) -> (),
        ),
        pop: Some(meta_entries_pop as unsafe extern "C" fn(*mut MetaEntries) -> MetaEntry),
        dispose_item: Some(
            meta_entries_dispose_item as unsafe extern "C" fn(*mut MetaEntries, usize) -> (),
        ),
        filter_env: Some(
            meta_entries_filter_env
                as unsafe extern "C" fn(
                    *mut MetaEntries,
                    Option<
                        unsafe extern "C" fn(*const MetaEntry, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            meta_entries_sort
                as unsafe extern "C" fn(
                    *mut MetaEntries,
                    Option<
                        unsafe extern "C" fn(
                            *const MetaEntry,
                            *const MetaEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn meta_entries_dispose_item(mut arr: *mut MetaEntries, mut n: usize) {
    if META_I_ENTRY.dispose.is_some() {
        META_I_ENTRY.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut MetaEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn meta_entries_sort(
    mut arr: *mut MetaEntries,
    mut fn_0: Option<
        unsafe extern "C" fn(*const MetaEntry, *const MetaEntry) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<MetaEntry>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const MetaEntry, *const MetaEntry) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn meta_entries_fill(mut arr: *mut MetaEntries, mut n: usize) {
    while (*arr).length < n {
        let mut x: MetaEntry = MetaEntry {
            tag: 0,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        if META_I_ENTRY.init.is_some() {
            META_I_ENTRY.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<MetaEntry>() as usize,
            );
        }
        meta_entries_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn meta_entries_push(arr: *mut MetaEntries, elem: MetaEntry) {
    cvec_push(meta_entries_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn meta_entries_grow_to(arr: *mut MetaEntries, target: usize) {
    cvec_grow_to(meta_entries_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn meta_entries_pop(arr: *mut MetaEntries) -> MetaEntry {
    cvec_pop(meta_entries_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn meta_entries_copy(mut dst: *mut MetaEntries, mut src: *const MetaEntries) {
    meta_entries_init(dst);
    meta_entries_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if META_I_ENTRY.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            META_I_ENTRY.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut MetaEntry,
                (*src).items.offset(j as isize) as *mut MetaEntry as *const MetaEntry,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn meta_entries_dispose(mut arr: *mut MetaEntries) {
    if arr.is_null() {
        return;
    }
    if META_I_ENTRY.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            META_I_ENTRY.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut MetaEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<MetaEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn meta_entries_init_cap_n(mut arr: *mut MetaEntries, mut n: usize) {
    meta_entries_init(arr);
    meta_entries_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn meta_entries_grow_to_n(arr: *mut MetaEntries, target: usize) {
    cvec_grow_to_n(meta_entries_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn meta_entries_init_n(mut arr: *mut MetaEntries, mut n: usize) {
    meta_entries_init(arr);
    meta_entries_grow_to_n(arr, n);
    meta_entries_fill(arr, n);
}
#[inline]
unsafe extern "C" fn meta_entries_free(mut x: *mut MetaEntries) {
    if x.is_null() {
        return;
    }
    meta_entries_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn meta_entries_create_n(mut n: usize) -> *mut MetaEntries {
    let mut t: *mut MetaEntries =
        malloc(::core::mem::size_of::<MetaEntries>() as usize) as *mut MetaEntries;
    meta_entries_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn meta_entries_create() -> *mut MetaEntries {
    let mut x: *mut MetaEntries =
        malloc(::core::mem::size_of::<MetaEntries>() as usize) as *mut MetaEntries;
    meta_entries_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn meta_entries_shrink_to_fit(mut arr: *mut MetaEntries) {
    meta_entries_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn meta_entries_resize_to(arr: *mut MetaEntries, target: usize) {
    cvec_resize_to(meta_entries_as_cvec(arr), target);
}
unsafe extern "C" fn init_meta_table(mut t: *mut MetaTable) {
    (*t).version = 1 as u32;
    (*t).flags = 0 as u32;
    META_I_ENTRIES.init.expect("non-null function pointer")(&raw mut (*t).entries);
}
unsafe extern "C" fn dispose_meta_table(mut t: *mut MetaTable) {
    META_I_ENTRIES.dispose.expect("non-null function pointer")(&raw mut (*t).entries);
}
#[inline]
unsafe extern "C" fn table_meta_free(mut x: *mut MetaTable) {
    if x.is_null() {
        return;
    }
    table_meta_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_meta_copy(mut dst: *mut MetaTable, mut src: *const MetaTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MetaTable>() as usize,
    );
}
pub static TABLE_I_META: MetaTableElementInterface = {
    MetaTableElementInterface {
        init: Some(table_meta_init as unsafe extern "C" fn(*mut MetaTable) -> ()),
        copy: Some(
            table_meta_copy as unsafe extern "C" fn(*mut MetaTable, *const MetaTable) -> (),
        ),
        dispose: Some(table_meta_dispose as unsafe extern "C" fn(*mut MetaTable) -> ()),
        create: Some(table_meta_create),
        free: Some(table_meta_free as unsafe extern "C" fn(*mut MetaTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_meta_init(mut x: *mut MetaTable) {
    init_meta_table(x);
}
#[inline]
unsafe extern "C" fn table_meta_dispose(mut x: *mut MetaTable) {
    dispose_meta_table(x);
}
#[inline]
unsafe extern "C" fn table_meta_create() -> *mut MetaTable {
    let mut x: *mut MetaTable =
        malloc(::core::mem::size_of::<MetaTable>() as usize) as *mut MetaTable;
    table_meta_init(x);
    return x;
}
