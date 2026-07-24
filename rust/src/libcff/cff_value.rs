pub type __int32_t = i32;
pub type int32_t = __int32_t;
pub type cff_Value_Type = ::core::ffi::c_uint;
pub const CS2_FRACTION: cff_Value_Type = 3;
pub const cff_DOUBLE: cff_Value_Type = 3;
pub const CS2_OPERAND: cff_Value_Type = 2;
pub const cff_INTEGER: cff_Value_Type = 2;
pub const CS2_OPERATOR: cff_Value_Type = 1;
pub const cff_OPERATOR: cff_Value_Type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Value {
    pub t: cff_Value_Type,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i: int32_t,
    pub d: ::core::ffi::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn cffnum(mut val: cff_Value) -> ::core::ffi::c_double {
    if val.t as ::core::ffi::c_uint == cff_INTEGER as ::core::ffi::c_int as ::core::ffi::c_uint {
        return val.c2rust_unnamed.i as ::core::ffi::c_double;
    }
    if val.t as ::core::ffi::c_uint == cff_DOUBLE as ::core::ffi::c_int as ::core::ffi::c_uint {
        return val.c2rust_unnamed.d;
    }
    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
}
