#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::libcff::CffCharstringOperator;
use crate::support::buffer::{bufninit, bufnwrite8, Buffer};
use crate::support::buffer::{bufwrite8};
unsafe extern "C" {
    fn modf(
        __x: ::core::ffi::c_double,
        __iptr: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub unsafe fn cff_build_header() -> *mut Buffer {
    return bufninit(&[1 as u8, 0 as u8, 4 as u8, 4 as u8]);
}
pub unsafe fn cff_merge_cs2_operator(
    mut blob: *mut Buffer,
    mut val: CffCharstringOperator,
) {
    let val = val.0;
    if val >= 0x100 as i32 {
        bufnwrite8(blob, &[(val >> 8 as ::core::ffi::c_int) as u8, (val & 0xff as i32) as u8]);
    } else {
        bufnwrite8(blob, &[(val & 0xff as i32) as u8]);
    };
}
pub unsafe fn cff_merge_cs2_int(mut blob: *mut Buffer, mut val: i32) {
    if val >= -(1131 as i32) && val <= -(108 as i32) {
        bufnwrite8(blob, &[(((-(108 as i32) - val) / 256 as i32 + 251 as i32) as u8
                as ::core::ffi::c_int) as u8, (((-(108 as i32) - val) % 256 as i32) as u8 as ::core::ffi::c_int) as u8]);
    } else if val >= -(107 as i32) && val <= 107 as i32 {
        bufnwrite8(blob, &[((val + 139 as i32) as u8 as ::core::ffi::c_int) as u8]);
    } else if val >= 108 as i32 && val <= 1131 as i32 {
        bufnwrite8(blob, &[(((val - 108 as i32) / 256 as i32 + 247 as i32) as u8
                as ::core::ffi::c_int) as u8, (((val - 108 as i32) % 256 as i32) as u8 as ::core::ffi::c_int) as u8]);
    } else if val >= -(32768 as i32) && val <= 32767 as i32 {
        bufnwrite8(blob, &[28 as u8, ((val >> 8 as ::core::ffi::c_int) as u8 as ::core::ffi::c_int) as u8, ((val << 8 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as u8
                as ::core::ffi::c_int) as u8]);
    } else {
        cff_merge_cs2_int(blob, 0 as i32);
    };
}
unsafe fn merge_cs2_real(mut blob: *mut Buffer, mut val: ::core::ffi::c_double) {
    let mut integer_part: i16 = floor(val) as i16;
    let mut fraction_part: u16 = ((val
        - integer_part as ::core::ffi::c_int as ::core::ffi::c_double)
        * 65536.0f64) as u16;
    bufnwrite8(blob, &[0xff as u8, (integer_part as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as u8, (integer_part as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as u8, (fraction_part as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as u8, (fraction_part as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as u8]);
}
pub unsafe fn cff_merge_cs2_operand(
    mut blob: *mut Buffer,
    mut val: ::core::ffi::c_double,
) {
    let mut intpart: ::core::ffi::c_double = 0.;
    if modf(val, &raw mut intpart) == 0.0f64 {
        cff_merge_cs2_int(blob, intpart as i32);
    } else {
        merge_cs2_real(blob, val);
    };
}
pub unsafe fn cff_merge_cs2_special(mut blob: *mut Buffer, mut val: u8) {
    bufwrite8(blob, val);
}
pub unsafe fn cff_build_offset(mut val: i32) -> *mut Buffer {
    return bufninit(&[29 as u8, (val >> 24 as ::core::ffi::c_int & 0xff as i32) as u8, (val >> 16 as ::core::ffi::c_int & 0xff as i32) as u8, (val >> 8 as ::core::ffi::c_int & 0xff as i32) as u8, (val & 0xff as i32) as u8]);
}
