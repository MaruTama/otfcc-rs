
use crate::support::primitives::{Pos, TableId};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VV {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut Pos,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VvVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VV, *const VV) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VV, *mut VV) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VV, VV) -> ()>,
    pub copy_replace: Option<unsafe extern "C" fn(*mut VV, VV) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VV>,
    pub free: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut VV, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut VV, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut VV>,
    pub fill: Option<unsafe extern "C" fn(*mut VV, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut VV, Pos) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut VV) -> Pos>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut VV, usize) -> ()>,
    pub filter_env: Option<
        unsafe extern "C" fn(
            *mut VV,
            Option<unsafe extern "C" fn(*const Pos, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut VV,
            Option<unsafe extern "C" fn(*const Pos, *const Pos) -> ::core::ffi::c_int>,
        ) -> (),
    >,
    pub neutral: Option<unsafe extern "C" fn(TableId) -> VV>,
}
