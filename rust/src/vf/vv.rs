
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
    pub dispose: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VV>,
    pub free: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut VV, usize) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut VV, Pos) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub neutral: Option<unsafe extern "C" fn(TableId) -> VV>,
}
