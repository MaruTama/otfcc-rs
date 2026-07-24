extern "C" {
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type __int16_t = i16;
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type f2dot14 = int16_t;
pub type f16dot16 = int32_t;
pub const f16dot16_precision: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const f16dot16_k: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << f16dot16_precision - 1 as ::core::ffi::c_int;
pub const f16dot16_infinity: f16dot16 = 0x7fffffff as ::core::ffi::c_int as f16dot16;
pub const f16dot16_negativeIntinity: f16dot16 = 0x80000000 as ::core::ffi::c_uint as f16dot16;
#[no_mangle]
pub unsafe extern "C" fn otfcc_from_f2dot14(x: f2dot14) -> ::core::ffi::c_double {
    return x as ::core::ffi::c_int as ::core::ffi::c_double / 16384.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_to_f2dot14(x: ::core::ffi::c_double) -> int16_t {
    return round(x * 16384.0f64) as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_from_fixed(x: f16dot16) -> ::core::ffi::c_double {
    return x as ::core::ffi::c_double / 65536.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16 {
    return round(x * 65536.0f64) as f16dot16;
}
#[inline]
unsafe extern "C" fn clamp(value: int64_t) -> f16dot16 {
    value.clamp(f16dot16_negativeIntinity as int64_t, f16dot16_infinity as int64_t) as f16dot16
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_f1616_add(mut a: f16dot16, mut b: f16dot16) -> f16dot16 {
    return a + b;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_f1616_minus(mut a: f16dot16, mut b: f16dot16) -> f16dot16 {
    return a - b;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_f1616_multiply(mut a: f16dot16, mut b: f16dot16) -> f16dot16 {
    let mut tmp: int64_t = a as int64_t * b as int64_t + f16dot16_k as int64_t;
    let mut product: f16dot16 = clamp(tmp >> f16dot16_precision);
    return product;
}
#[inline]
unsafe extern "C" fn divide(mut a: int64_t, b: int32_t) -> f16dot16 {
    if b == 0 {
        return if a < 0 {
            f16dot16_negativeIntinity
        } else {
            f16dot16_infinity
        };
    }
    if (a < 0) != (b < 0) {
        a -= (b / 2) as int64_t;
    } else {
        a += (b / 2) as int64_t;
    }
    return clamp(a / b as int64_t);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_f1616_muldiv(
    mut a: f16dot16,
    mut b: f16dot16,
    mut c: f16dot16,
) -> f16dot16 {
    let mut tmp: int64_t = a as int64_t * b as int64_t + f16dot16_k as int64_t;
    return divide(tmp, c as int32_t);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_f1616_divide(mut a: f16dot16, mut b: f16dot16) -> f16dot16 {
    return divide((a as int64_t) << f16dot16_precision, b as int32_t);
}
