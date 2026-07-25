extern "C" {
    fn modf(
        __x: ::core::ffi::c_double,
        __iptr: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn bufninit(n: uint32_t, ...) -> *mut caryll_Buffer;
    fn bufwrite8(buf: *mut caryll_Buffer, byte: uint8_t);
    fn bufnwrite8(buf: *mut caryll_Buffer, n: uint32_t, ...);
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
#[no_mangle]
pub unsafe extern "C" fn cff_buildHeader() -> *mut caryll_Buffer {
    return bufninit(
        4 as uint32_t,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cff_mergeCS2Operator(mut blob: *mut caryll_Buffer, mut val: int32_t) {
    if val >= 0x100 as int32_t {
        bufnwrite8(
            blob,
            2 as uint32_t,
            val >> 8 as ::core::ffi::c_int,
            val & 0xff as int32_t,
        );
    } else {
        bufnwrite8(blob, 1 as uint32_t, val & 0xff as int32_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_mergeCS2Int(mut blob: *mut caryll_Buffer, mut val: int32_t) {
    if val >= -(1131 as int32_t) && val <= -(108 as int32_t) {
        bufnwrite8(
            blob,
            2 as uint32_t,
            ((-(108 as int32_t) - val) / 256 as int32_t + 251 as int32_t) as uint8_t
                as ::core::ffi::c_int,
            ((-(108 as int32_t) - val) % 256 as int32_t) as uint8_t as ::core::ffi::c_int,
        );
    } else if val >= -(107 as int32_t) && val <= 107 as int32_t {
        bufnwrite8(
            blob,
            1 as uint32_t,
            (val + 139 as int32_t) as uint8_t as ::core::ffi::c_int,
        );
    } else if val >= 108 as int32_t && val <= 1131 as int32_t {
        bufnwrite8(
            blob,
            2 as uint32_t,
            ((val - 108 as int32_t) / 256 as int32_t + 247 as int32_t) as uint8_t
                as ::core::ffi::c_int,
            ((val - 108 as int32_t) % 256 as int32_t) as uint8_t as ::core::ffi::c_int,
        );
    } else if val >= -(32768 as int32_t) && val <= 32767 as int32_t {
        bufnwrite8(
            blob,
            3 as uint32_t,
            28 as ::core::ffi::c_int,
            (val >> 8 as ::core::ffi::c_int) as uint8_t as ::core::ffi::c_int,
            (val << 8 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint8_t
                as ::core::ffi::c_int,
        );
    } else {
        cff_mergeCS2Int(blob, 0 as int32_t);
    };
}
unsafe extern "C" fn mergeCS2Real(mut blob: *mut caryll_Buffer, mut val: ::core::ffi::c_double) {
    let mut integerPart: int16_t = floor(val) as int16_t;
    let mut fractionPart: uint16_t = ((val
        - integerPart as ::core::ffi::c_int as ::core::ffi::c_double)
        * 65536.0f64) as uint16_t;
    bufnwrite8(
        blob,
        5 as uint32_t,
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
        cff_mergeCS2Int(blob, intpart as int32_t);
    } else {
        mergeCS2Real(blob, val);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_mergeCS2Special(mut blob: *mut caryll_Buffer, mut val: uint8_t) {
    bufwrite8(blob, val);
}
#[no_mangle]
pub unsafe extern "C" fn cff_buildOffset(mut val: int32_t) -> *mut caryll_Buffer {
    return bufninit(
        5 as uint32_t,
        29 as ::core::ffi::c_int,
        val >> 24 as ::core::ffi::c_int & 0xff as int32_t,
        val >> 16 as ::core::ffi::c_int & 0xff as int32_t,
        val >> 8 as ::core::ffi::c_int & 0xff as int32_t,
        val & 0xff as int32_t,
    );
}
