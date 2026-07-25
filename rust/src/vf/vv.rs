
use crate::support::primitives::{pos_t, tableid_t};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VV {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut pos_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_VV {
    pub init: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VV, *const VV) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VV, *mut VV) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VV, VV) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VV, VV) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VV>,
    pub free: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut VV, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut VV, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut VV>,
    pub fill: Option<unsafe extern "C" fn(*mut VV, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut VV, pos_t) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut VV) -> pos_t>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut VV, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut VV,
            Option<unsafe extern "C" fn(*const pos_t, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut VV,
            Option<unsafe extern "C" fn(*const pos_t, *const pos_t) -> ::core::ffi::c_int>,
        ) -> (),
    >,
    pub neutral: Option<unsafe extern "C" fn(tableid_t) -> VV>,
}
