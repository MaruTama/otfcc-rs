#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{calloc, free};

// `.data` holds either a UTF-8 string tag's bytes or raw (possibly
// non-UTF-8) base64-decoded bytes, so `Vec<u8>`, not `String`. Neither
// `MetaEntry` nor `MetaTable` derives `Clone` anymore: the only call to
// `.clone()` anywhere in this module was inside `table_meta_copy`
// (below), confirmed dead (only ever called from its own static
// initializer) before removing both.
#[repr(C)]
pub struct MetaEntry {
    pub tag: u32,
    pub data: Vec<u8>,
}
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
unsafe extern "C" fn init_meta_table(mut t: *mut MetaTable) {
    (*t).version = 1 as u32;
    (*t).flags = 0 as u32;
    (*t).entries = Vec::new();
}
// Every element's `.data: Vec<u8>` now has real drop glue, so dropping
// the `Vec<MetaEntry>` disposes everything correctly on its own -- no
// manual per-element walk needed, same as `TsiTable`/`ChainingRule`'s
// containers earlier in this migration.
unsafe extern "C" fn dispose_meta_table(mut t: *mut MetaTable) {
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
pub static TABLE_I_META: MetaTableElementInterface = {
    MetaTableElementInterface {
        init: Some(table_meta_init as unsafe extern "C" fn(*mut MetaTable) -> ()),
        copy: None,
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
