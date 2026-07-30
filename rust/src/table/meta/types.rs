#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};
use crate::vendor::sds::{SdsRaw};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
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
    pub move_0: Option<unsafe extern "C" fn(*mut MetaEntry, *mut MetaEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut MetaEntry) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut MetaEntry, MetaEntry) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut MetaEntry, MetaEntry) -> ()>,
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
    pub move_0: Option<unsafe extern "C" fn(*mut MetaEntries, *mut MetaEntries) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut MetaEntries) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut MetaEntries, MetaEntries) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut MetaEntries, MetaEntries) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut MetaEntries>,
    pub free: Option<unsafe extern "C" fn(*mut MetaEntries) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut MetaEntries, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut MetaEntries, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut MetaEntries>,
    pub fill: Option<unsafe extern "C" fn(*mut MetaEntries, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut MetaEntries) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut MetaEntries, MetaEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut MetaEntries) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut MetaEntries) -> MetaEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut MetaEntries, usize) -> ()>,
    pub filterEnv: Option<
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
    pub move_0: Option<unsafe extern "C" fn(*mut MetaTable, *mut MetaTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut MetaTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut MetaTable, MetaTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut MetaTable, MetaTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut MetaTable>,
    pub free: Option<unsafe extern "C" fn(*mut MetaTable) -> ()>,
}
unsafe extern "C" fn initMetaEntry(mut e: *mut MetaEntry) {
    (*e).tag = 1 as u32;
    (*e).data = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn disposeMetaEntry(mut e: *mut MetaEntry) {
    sdsfree((*e).data);
}
pub static meta_iEntry: MetaEntryElementInterface = {
    MetaEntryElementInterface {
        init: Some(meta_Entry_init as unsafe extern "C" fn(*mut MetaEntry) -> ()),
        copy: Some(
            meta_Entry_copy as unsafe extern "C" fn(*mut MetaEntry, *const MetaEntry) -> (),
        ),
        move_0: Some(
            meta_Entry_move as unsafe extern "C" fn(*mut MetaEntry, *mut MetaEntry) -> (),
        ),
        dispose: Some(meta_Entry_dispose as unsafe extern "C" fn(*mut MetaEntry) -> ()),
        replace: Some(
            meta_Entry_replace as unsafe extern "C" fn(*mut MetaEntry, MetaEntry) -> (),
        ),
        copyReplace: Some(
            meta_Entry_copyReplace as unsafe extern "C" fn(*mut MetaEntry, MetaEntry) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn meta_Entry_replace(mut dst: *mut MetaEntry, src: MetaEntry) {
    meta_Entry_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MetaEntry>() as usize,
    );
}
#[inline]
unsafe extern "C" fn meta_Entry_init(mut x: *mut MetaEntry) {
    initMetaEntry(x);
}
#[inline]
unsafe extern "C" fn meta_Entry_dispose(mut x: *mut MetaEntry) {
    disposeMetaEntry(x);
}
#[inline]
unsafe extern "C" fn meta_Entry_copy(mut dst: *mut MetaEntry, mut src: *const MetaEntry) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MetaEntry>() as usize,
    );
}
#[inline]
unsafe extern "C" fn meta_Entry_move(mut dst: *mut MetaEntry, mut src: *mut MetaEntry) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MetaEntry>() as usize,
    );
    meta_Entry_init(src);
}
#[inline]
unsafe extern "C" fn meta_Entry_copyReplace(mut dst: *mut MetaEntry, src: MetaEntry) {
    meta_Entry_dispose(dst);
    meta_Entry_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn meta_Entries_filterEnv(
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
            if meta_iEntry.dispose.is_some() {
                meta_iEntry.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn meta_Entries_move(dst: *mut MetaEntries, src: *mut MetaEntries) {
    cvec_move(meta_Entries_as_cvec(dst), meta_Entries_as_cvec(src));
}
#[inline]
unsafe fn meta_Entries_as_cvec(arr: *mut MetaEntries) -> *mut CVecRaw<MetaEntry> {
    arr as *mut CVecRaw<MetaEntry>
}
#[inline]
unsafe extern "C" fn meta_Entries_init(arr: *mut MetaEntries) {
    cvec_init(meta_Entries_as_cvec(arr));
}
pub static meta_iEntries: MetaEntriesVectorInterface = {
    MetaEntriesVectorInterface {
        init: Some(meta_Entries_init as unsafe extern "C" fn(*mut MetaEntries) -> ()),
        copy: Some(
            meta_Entries_copy as unsafe extern "C" fn(*mut MetaEntries, *const MetaEntries) -> (),
        ),
        move_0: Some(
            meta_Entries_move as unsafe extern "C" fn(*mut MetaEntries, *mut MetaEntries) -> (),
        ),
        dispose: Some(meta_Entries_dispose as unsafe extern "C" fn(*mut MetaEntries) -> ()),
        replace: Some(
            meta_Entries_replace as unsafe extern "C" fn(*mut MetaEntries, MetaEntries) -> (),
        ),
        copyReplace: Some(
            meta_Entries_copyReplace as unsafe extern "C" fn(*mut MetaEntries, MetaEntries) -> (),
        ),
        create: Some(meta_Entries_create),
        free: Some(meta_Entries_free as unsafe extern "C" fn(*mut MetaEntries) -> ()),
        initN: Some(meta_Entries_initN as unsafe extern "C" fn(*mut MetaEntries, usize) -> ()),
        initCapN: Some(
            meta_Entries_initCapN as unsafe extern "C" fn(*mut MetaEntries, usize) -> (),
        ),
        createN: Some(meta_Entries_createN as unsafe extern "C" fn(usize) -> *mut MetaEntries),
        fill: Some(meta_Entries_fill as unsafe extern "C" fn(*mut MetaEntries, usize) -> ()),
        clear: Some(meta_Entries_dispose as unsafe extern "C" fn(*mut MetaEntries) -> ()),
        push: Some(meta_Entries_push as unsafe extern "C" fn(*mut MetaEntries, MetaEntry) -> ()),
        shrinkToFit: Some(
            meta_Entries_shrinkToFit as unsafe extern "C" fn(*mut MetaEntries) -> (),
        ),
        pop: Some(meta_Entries_pop as unsafe extern "C" fn(*mut MetaEntries) -> MetaEntry),
        disposeItem: Some(
            meta_Entries_disposeItem as unsafe extern "C" fn(*mut MetaEntries, usize) -> (),
        ),
        filterEnv: Some(
            meta_Entries_filterEnv
                as unsafe extern "C" fn(
                    *mut MetaEntries,
                    Option<
                        unsafe extern "C" fn(*const MetaEntry, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            meta_Entries_sort
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
unsafe extern "C" fn meta_Entries_disposeItem(mut arr: *mut MetaEntries, mut n: usize) {
    if meta_iEntry.dispose.is_some() {
        meta_iEntry.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut MetaEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn meta_Entries_sort(
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
unsafe extern "C" fn meta_Entries_fill(mut arr: *mut MetaEntries, mut n: usize) {
    while (*arr).length < n {
        let mut x: MetaEntry = MetaEntry {
            tag: 0,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        if meta_iEntry.init.is_some() {
            meta_iEntry.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<MetaEntry>() as usize,
            );
        }
        meta_Entries_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn meta_Entries_push(arr: *mut MetaEntries, elem: MetaEntry) {
    cvec_push(meta_Entries_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn meta_Entries_grow(arr: *mut MetaEntries) {
    cvec_grow(meta_Entries_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn meta_Entries_growTo(arr: *mut MetaEntries, target: usize) {
    cvec_grow_to(meta_Entries_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn meta_Entries_pop(arr: *mut MetaEntries) -> MetaEntry {
    cvec_pop(meta_Entries_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn meta_Entries_copyReplace(mut dst: *mut MetaEntries, src: MetaEntries) {
    meta_Entries_dispose(dst);
    meta_Entries_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn meta_Entries_copy(mut dst: *mut MetaEntries, mut src: *const MetaEntries) {
    meta_Entries_init(dst);
    meta_Entries_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if meta_iEntry.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            meta_iEntry.copy.expect("non-null function pointer")(
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
unsafe extern "C" fn meta_Entries_dispose(mut arr: *mut MetaEntries) {
    if arr.is_null() {
        return;
    }
    if meta_iEntry.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            meta_iEntry.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn meta_Entries_replace(mut dst: *mut MetaEntries, src: MetaEntries) {
    meta_Entries_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MetaEntries>() as usize,
    );
}
#[inline]
unsafe extern "C" fn meta_Entries_initCapN(mut arr: *mut MetaEntries, mut n: usize) {
    meta_Entries_init(arr);
    meta_Entries_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn meta_Entries_growToN(arr: *mut MetaEntries, target: usize) {
    cvec_grow_to_n(meta_Entries_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn meta_Entries_initN(mut arr: *mut MetaEntries, mut n: usize) {
    meta_Entries_init(arr);
    meta_Entries_growToN(arr, n);
    meta_Entries_fill(arr, n);
}
#[inline]
unsafe extern "C" fn meta_Entries_free(mut x: *mut MetaEntries) {
    if x.is_null() {
        return;
    }
    meta_Entries_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn meta_Entries_createN(mut n: usize) -> *mut MetaEntries {
    let mut t: *mut MetaEntries =
        malloc(::core::mem::size_of::<MetaEntries>() as usize) as *mut MetaEntries;
    meta_Entries_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn meta_Entries_create() -> *mut MetaEntries {
    let mut x: *mut MetaEntries =
        malloc(::core::mem::size_of::<MetaEntries>() as usize) as *mut MetaEntries;
    meta_Entries_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn meta_Entries_shrinkToFit(mut arr: *mut MetaEntries) {
    meta_Entries_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn meta_Entries_resizeTo(arr: *mut MetaEntries, target: usize) {
    cvec_resize_to(meta_Entries_as_cvec(arr), target);
}
unsafe extern "C" fn initMetaTable(mut t: *mut MetaTable) {
    (*t).version = 1 as u32;
    (*t).flags = 0 as u32;
    meta_iEntries.init.expect("non-null function pointer")(&raw mut (*t).entries);
}
unsafe extern "C" fn disposeMetaTable(mut t: *mut MetaTable) {
    meta_iEntries.dispose.expect("non-null function pointer")(&raw mut (*t).entries);
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
unsafe extern "C" fn table_meta_move(mut dst: *mut MetaTable, mut src: *mut MetaTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MetaTable>() as usize,
    );
    table_meta_init(src);
}
#[inline]
unsafe extern "C" fn table_meta_copy(mut dst: *mut MetaTable, mut src: *const MetaTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MetaTable>() as usize,
    );
}
pub static table_iMeta: MetaTableElementInterface = {
    MetaTableElementInterface {
        init: Some(table_meta_init as unsafe extern "C" fn(*mut MetaTable) -> ()),
        copy: Some(
            table_meta_copy as unsafe extern "C" fn(*mut MetaTable, *const MetaTable) -> (),
        ),
        move_0: Some(
            table_meta_move as unsafe extern "C" fn(*mut MetaTable, *mut MetaTable) -> (),
        ),
        dispose: Some(table_meta_dispose as unsafe extern "C" fn(*mut MetaTable) -> ()),
        replace: Some(
            table_meta_replace as unsafe extern "C" fn(*mut MetaTable, MetaTable) -> (),
        ),
        copyReplace: Some(
            table_meta_copyReplace as unsafe extern "C" fn(*mut MetaTable, MetaTable) -> (),
        ),
        create: Some(table_meta_create),
        free: Some(table_meta_free as unsafe extern "C" fn(*mut MetaTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_meta_init(mut x: *mut MetaTable) {
    initMetaTable(x);
}
#[inline]
unsafe extern "C" fn table_meta_dispose(mut x: *mut MetaTable) {
    disposeMetaTable(x);
}
#[inline]
unsafe extern "C" fn table_meta_replace(mut dst: *mut MetaTable, src: MetaTable) {
    table_meta_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MetaTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_meta_create() -> *mut MetaTable {
    let mut x: *mut MetaTable =
        malloc(::core::mem::size_of::<MetaTable>() as usize) as *mut MetaTable;
    table_meta_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_meta_copyReplace(mut dst: *mut MetaTable, src: MetaTable) {
    table_meta_dispose(dst);
    table_meta_copy(dst, &raw const src);
}
