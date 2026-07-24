use crate::support::buffer::{caryll_Buffer};
extern "C" {
    fn modf(
        __x: ::core::ffi::c_double,
        __iptr: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn bufninit(n: u32, ...) -> *mut caryll_Buffer;
    fn bufwrite8(buf: *mut caryll_Buffer, byte: u8);
    fn bufnwrite8(buf: *mut caryll_Buffer, n: u32, ...);
}
#[no_mangle]
pub unsafe extern "C" fn cff_buildHeader() -> *mut caryll_Buffer {
    return bufninit(
        4 as u32,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cff_mergeCS2Operator(mut blob: *mut caryll_Buffer, mut val: i32) {
    if val >= 0x100 as i32 {
        bufnwrite8(
            blob,
            2 as u32,
            val >> 8 as ::core::ffi::c_int,
            val & 0xff as i32,
        );
    } else {
        bufnwrite8(blob, 1 as u32, val & 0xff as i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_mergeCS2Int(mut blob: *mut caryll_Buffer, mut val: i32) {
    if val >= -(1131 as i32) && val <= -(108 as i32) {
        bufnwrite8(
            blob,
            2 as u32,
            ((-(108 as i32) - val) / 256 as i32 + 251 as i32) as u8
                as ::core::ffi::c_int,
            ((-(108 as i32) - val) % 256 as i32) as u8 as ::core::ffi::c_int,
        );
    } else if val >= -(107 as i32) && val <= 107 as i32 {
        bufnwrite8(
            blob,
            1 as u32,
            (val + 139 as i32) as u8 as ::core::ffi::c_int,
        );
    } else if val >= 108 as i32 && val <= 1131 as i32 {
        bufnwrite8(
            blob,
            2 as u32,
            ((val - 108 as i32) / 256 as i32 + 247 as i32) as u8
                as ::core::ffi::c_int,
            ((val - 108 as i32) % 256 as i32) as u8 as ::core::ffi::c_int,
        );
    } else if val >= -(32768 as i32) && val <= 32767 as i32 {
        bufnwrite8(
            blob,
            3 as u32,
            28 as ::core::ffi::c_int,
            (val >> 8 as ::core::ffi::c_int) as u8 as ::core::ffi::c_int,
            (val << 8 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as u8
                as ::core::ffi::c_int,
        );
    } else {
        cff_mergeCS2Int(blob, 0 as i32);
    };
}
unsafe extern "C" fn mergeCS2Real(mut blob: *mut caryll_Buffer, mut val: ::core::ffi::c_double) {
    let mut integerPart: i16 = floor(val) as i16;
    let mut fractionPart: u16 = ((val
        - integerPart as ::core::ffi::c_int as ::core::ffi::c_double)
        * 65536.0f64) as u16;
    bufnwrite8(
        blob,
        5 as u32,
        0xff as ::core::ffi::c_int,
        integerPart as ::core::ffi::c_int >> 8 as ::core::ffi::c_int,
        integerPart as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
        fractionPart as ::core::ffi::c_int >> 8 as ::core::ffi::c_int,
        fractionPart as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cff_mergeCS2Operand(
    mut blob: *mut caryll_Buffer,
    mut val: ::core::ffi::c_double,
) {
    let mut intpart: ::core::ffi::c_double = 0.;
    if modf(val, &raw mut intpart) == 0.0f64 {
        cff_mergeCS2Int(blob, intpart as i32);
    } else {
        mergeCS2Real(blob, val);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_mergeCS2Special(mut blob: *mut caryll_Buffer, mut val: u8) {
    bufwrite8(blob, val);
}
#[no_mangle]
pub unsafe extern "C" fn cff_buildOffset(mut val: i32) -> *mut caryll_Buffer {
    return bufninit(
        5 as u32,
        29 as ::core::ffi::c_int,
        val >> 24 as ::core::ffi::c_int & 0xff as i32,
        val >> 16 as ::core::ffi::c_int & 0xff as i32,
        val >> 8 as ::core::ffi::c_int & 0xff as i32,
        val & 0xff as i32,
    );
}
