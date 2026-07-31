#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{calloc, free};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxRecord {
    pub y_pel_height: u16,
    pub y_max: i16,
    pub y_min: i16,
}
#[derive(Clone)]
pub struct VdmxRatioRange {
    pub b_charset: u8,
    pub x_ratio: u8,
    pub y_start_ratio: u8,
    pub y_end_ratio: u8,
    pub records: Vec<VdmxRecord>,
}
#[derive(Clone)]
pub struct VdmxTable {
    pub version: u16,
    pub ratios: Vec<VdmxRatioRange>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VdmxTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VdmxTable, *const VdmxTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VdmxTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VdmxTable>,
    pub free: Option<unsafe extern "C" fn(*mut VdmxTable) -> ()>,
}
unsafe extern "C" fn init_vdmx(mut t: *mut VdmxTable) {
    (*t).ratios = Vec::new();
}
unsafe extern "C" fn dispose_vdmx(mut t: *mut VdmxTable) {
    (*t).ratios = Vec::new();
}
#[inline]
unsafe extern "C" fn table_vdmx_free(mut x: *mut VdmxTable) {
    if x.is_null() {
        return;
    }
    table_vdmx_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_vdmx_copy(mut dst: *mut VdmxTable, mut src: *const VdmxTable) {
    (*dst).version = (*src).version;
    (*dst).ratios = (*src).ratios.clone();
}
pub static TABLE_I_VDMX: VdmxTableElementInterface = {
    VdmxTableElementInterface {
        init: Some(table_vdmx_init as unsafe extern "C" fn(*mut VdmxTable) -> ()),
        copy: Some(
            table_vdmx_copy as unsafe extern "C" fn(*mut VdmxTable, *const VdmxTable) -> (),
        ),
        dispose: Some(table_vdmx_dispose as unsafe extern "C" fn(*mut VdmxTable) -> ()),
        create: Some(table_vdmx_create),
        free: Some(table_vdmx_free as unsafe extern "C" fn(*mut VdmxTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_vdmx_init(mut x: *mut VdmxTable) {
    init_vdmx(x);
}
#[inline]
unsafe extern "C" fn table_vdmx_dispose(mut x: *mut VdmxTable) {
    dispose_vdmx(x);
}
#[inline]
unsafe extern "C" fn table_vdmx_create() -> *mut VdmxTable {
    // `calloc`, not `malloc`: `init_vdmx` assigns straight into `(*t).ratios`
    // (`= Vec::new()`), which drops whatever was already there first. See
    // rust/README.md's `GaspTable` note -- same bug, applied here up front.
    let mut x: *mut VdmxTable =
        calloc(1, ::core::mem::size_of::<VdmxTable>() as usize) as *mut VdmxTable;
    table_vdmx_init(x);
    return x;
}
