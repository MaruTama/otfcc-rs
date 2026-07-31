#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy};
use crate::vendor::sds::{SdsRaw};
use crate::support::cvec::{CVecRaw, cvec_grow_to, cvec_init, cvec_push};
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
    pub push: Option<unsafe extern "C" fn(*mut MetaEntries, MetaEntry) -> ()>,
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
        push: Some(meta_entries_push as unsafe extern "C" fn(*mut MetaEntries, MetaEntry) -> ()),
    }
};
#[inline]
unsafe extern "C" fn meta_entries_push(arr: *mut MetaEntries, elem: MetaEntry) {
    cvec_push(meta_entries_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn meta_entries_grow_to(arr: *mut MetaEntries, target: usize) {
    cvec_grow_to(meta_entries_as_cvec(arr), target);
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
unsafe extern "C" fn meta_entries_free(mut x: *mut MetaEntries) {
    if x.is_null() {
        return;
    }
    meta_entries_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn meta_entries_create() -> *mut MetaEntries {
    let mut x: *mut MetaEntries =
        malloc(::core::mem::size_of::<MetaEntries>() as usize) as *mut MetaEntries;
    meta_entries_init(x);
    return x;
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
