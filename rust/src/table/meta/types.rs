#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

// `.data` holds either a UTF-8 string tag's bytes or raw (possibly
// non-UTF-8) base64-decoded bytes, so `Vec<u8>`, not `String`.
#[repr(C)]
pub struct MetaEntry {
    pub tag: u32,
    pub data: Vec<u8>,
}
// Stage 6-4 "Box化": every field this struct (transitively) owns is
// already a `Vec`/scalar, so no `Drop` impl is needed -- `Box::new`
// construction plus the standard drop glue is sufficient. The entire
// `MetaTableElementInterface` vtable is deleted: grepping confirmed only
// `.create`/`.free` were ever called from outside this file.
pub struct MetaTable {
    pub version: u32,
    pub flags: u32,
    pub entries: Vec<MetaEntry>,
}
