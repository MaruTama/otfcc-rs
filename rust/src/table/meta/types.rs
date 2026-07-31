#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{calloc, free};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::sds::{sdsfree};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetaEntry {
    pub tag: u32,
    pub data: SdsRaw,
}
#[derive(Clone)]
pub struct MetaTable {
    pub version: u32,
    pub flags: u32,
    pub entries: Vec<MetaEntry>,
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
unsafe extern "C" fn dispose_meta_entry(mut e: *mut MetaEntry) {
    sdsfree((*e).data);
}
unsafe extern "C" fn init_meta_table(mut t: *mut MetaTable) {
    (*t).version = 1 as u32;
    (*t).flags = 0 as u32;
    (*t).entries = Vec::new();
}
unsafe extern "C" fn dispose_meta_table(mut t: *mut MetaTable) {
    let entries: &mut Vec<MetaEntry> = &mut (*t).entries;
    for e in entries.iter_mut() {
        dispose_meta_entry(e as *mut MetaEntry);
    }
    (*t).entries = Vec::new();
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
    (*dst).version = (*src).version;
    (*dst).flags = (*src).flags;
    (*dst).entries = (*src).entries.clone();
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
    // `calloc`, not `malloc`: `init_meta_table` assigns straight into
    // `(*t).entries` (`= Vec::new()`), which drops whatever was already
    // there first. Zeroed memory makes that a no-op; uninitialized memory
    // makes it read a garbage capacity and attempt to deallocate through a
    // garbage pointer (see rust/README.md's `GaspTable` note -- same bug).
    let mut x: *mut MetaTable =
        calloc(1, ::core::mem::size_of::<MetaTable>() as usize) as *mut MetaTable;
    table_meta_init(x);
    return x;
}
