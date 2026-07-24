extern "C" {
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type f2dot14 = i16;
pub type f16dot16 = i32;
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
pub unsafe extern "C" fn otfcc_to_f2dot14(x: ::core::ffi::c_double) -> i16 {
    return round(x * 16384.0f64) as i16;
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
unsafe extern "C" fn clamp(value: i64) -> f16dot16 {
    value.clamp(f16dot16_negativeIntinity as i64, f16dot16_infinity as i64) as f16dot16
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
    let mut tmp: i64 = a as i64 * b as i64 + f16dot16_k as i64;
    let mut product: f16dot16 = clamp(tmp >> f16dot16_precision);
    return product;
}
#[inline]
unsafe extern "C" fn divide(mut a: i64, b: i32) -> f16dot16 {
    if b == 0 {
        return if a < 0 {
            f16dot16_negativeIntinity
        } else {
            f16dot16_infinity
        };
    }
    if (a < 0) != (b < 0) {
        a -= (b / 2) as i64;
    } else {
        a += (b / 2) as i64;
    }
    return clamp(a / b as i64);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_f1616_muldiv(
    mut a: f16dot16,
    mut b: f16dot16,
    mut c: f16dot16,
) -> f16dot16 {
    let mut tmp: i64 = a as i64 * b as i64 + f16dot16_k as i64;
    return divide(tmp, c as i32);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_f1616_divide(mut a: f16dot16, mut b: f16dot16) -> f16dot16 {
    return divide((a as i64) << f16dot16_precision, b as i32);
}
