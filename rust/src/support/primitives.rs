#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
//! otfcc's scalar vocabulary, the Rust counterpart of
//! `c/include/otfcc/primitives.h`.
//!
//! c2rust declared each of these in every file that used one — `glyphid_t` 65
//! times, `pos_t` 57 — so the whole set now lives here and is imported. The
//! comments come from the C header: they are the only place the *meaning* of
//! these aliases is written down, and `u16` on its own does not tell a reader
//! whether a number is a glyph index, a class, or a table index.

unsafe extern "C" {
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

/// 2.14 fixed-point, a value in [-1, 1].
pub type f2dot14 = i16;
/// 16.16 fixed-point, used for intermediate coordinates.
///
/// Handle with care around GVAR's implicit deltas: the arithmetic helpers
/// below saturate towards ±infinity, and infinity short-circuits expressions.
pub type f16dot16 = i32;

/// Glyph index.
pub type glyphid_t = u16;
/// Glyph class.
pub type glyphclass_t = u16;
/// GASP glyph size.
pub type glyphsize_t = u16;
/// Table / font structure index.
pub type tableid_t = u16;
/// Color index.
pub type colorid_t = u16;
/// Shape index.
pub type shapeid_t = u16;
/// CFF/CFF2 string index.
pub type cffsid_t = u16;
/// CFF arity / stack depth.
pub type arity_t = u32;
/// Unicode code point.
pub type unicode_t = u32;

/// Position.
pub type pos_t = ::core::ffi::c_double;
/// Transform scaling factor.
pub type scale_t = ::core::ffi::c_double;
/// Length.
pub type length_t = ::core::ffi::c_double;

/// A cursor into the raw bytes of a font file.
pub type font_file_pointer = *mut u8;
pub const f16dot16_precision: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const f16dot16_k: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << f16dot16_precision - 1 as ::core::ffi::c_int;
pub const f16dot16_infinity: f16dot16 = 0x7fffffff as ::core::ffi::c_int as f16dot16;
pub const f16dot16_negativeIntinity: f16dot16 = 0x80000000 as ::core::ffi::c_uint as f16dot16;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_from_f2dot14(x: f2dot14) -> ::core::ffi::c_double {
    return x as ::core::ffi::c_int as ::core::ffi::c_double / 16384.0f64;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_to_f2dot14(x: ::core::ffi::c_double) -> i16 {
    return round(x * 16384.0f64) as i16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_from_fixed(x: f16dot16) -> ::core::ffi::c_double {
    return x as ::core::ffi::c_double / 65536.0f64;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16 {
    return round(x * 65536.0f64) as f16dot16;
}
#[inline]
unsafe extern "C" fn clamp(value: i64) -> f16dot16 {
    value.clamp(f16dot16_negativeIntinity as i64, f16dot16_infinity as i64) as f16dot16
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_f1616_add(mut a: f16dot16, mut b: f16dot16) -> f16dot16 {
    return a + b;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_f1616_minus(mut a: f16dot16, mut b: f16dot16) -> f16dot16 {
    return a - b;
}
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_f1616_muldiv(
    mut a: f16dot16,
    mut b: f16dot16,
    mut c: f16dot16,
) -> f16dot16 {
    let mut tmp: i64 = a as i64 * b as i64 + f16dot16_k as i64;
    return divide(tmp, c as i32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_f1616_divide(mut a: f16dot16, mut b: f16dot16) -> f16dot16 {
    return divide((a as i64) << f16dot16_precision, b as i32);
}
