#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
//! otfcc's scalar vocabulary, the Rust counterpart of
//! `c/include/otfcc/primitives.h`.
//!
//! c2rust declared each of these in every file that used one — `GlyphId` 65
//! times, `Pos` 57 — so the whole set now lives here and is imported. The
//! comments come from the C header: they are the only place the *meaning* of
//! these aliases is written down, and `u16` on its own does not tell a reader
//! whether a number is a glyph index, a class, or a table index.

unsafe extern "C" {
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

/// 2.14 fixed-point, a value in [-1, 1].
pub type F2Dot14 = i16;
/// 16.16 fixed-point, used for intermediate coordinates.
///
/// Handle with care around GVAR's implicit deltas: the arithmetic helpers
/// below saturate towards ±infinity, and infinity short-circuits expressions.
pub type F16Dot16 = i32;

/// Glyph index.
pub type GlyphId = u16;
/// Glyph class.
pub type GlyphClass = u16;
/// GASP glyph size.
pub type GlyphSize = u16;
/// Table / font structure index.
pub type TableId = u16;
/// Color index.
pub type ColorId = u16;
/// Shape index.
pub type ShapeId = u16;
/// cff/CFF2 string index.
pub type CffSid = u16;
/// cff arity / stack depth.
pub type Arity = u32;
/// Unicode code point.
pub type Unicode = u32;

/// Position.
pub type Pos = ::core::ffi::c_double;
/// Transform scaling factor.
pub type Scale = ::core::ffi::c_double;
/// Length.
pub type Length = ::core::ffi::c_double;

/// A cursor into the raw bytes of a font file.
pub type FontFilePointer = *mut u8;
pub const F16DOT16_PRECISION: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const F16DOT16_K: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << F16DOT16_PRECISION - 1 as ::core::ffi::c_int;
pub const F16DOT16_INFINITY: F16Dot16 = 0x7fffffff as ::core::ffi::c_int as F16Dot16;
pub const F16DOT16_NEGATIVE_INFINITY: F16Dot16 = 0x80000000 as ::core::ffi::c_uint as F16Dot16;
pub unsafe extern "C" fn otfcc_from_f2dot14(x: F2Dot14) -> ::core::ffi::c_double {
    return x as ::core::ffi::c_int as ::core::ffi::c_double / 16384.0f64;
}
pub unsafe extern "C" fn otfcc_to_f2dot14(x: ::core::ffi::c_double) -> i16 {
    return round(x * 16384.0f64) as i16;
}
pub unsafe extern "C" fn otfcc_from_fixed(x: F16Dot16) -> ::core::ffi::c_double {
    return x as ::core::ffi::c_double / 65536.0f64;
}
pub unsafe extern "C" fn otfcc_to_fixed(x: ::core::ffi::c_double) -> F16Dot16 {
    return round(x * 65536.0f64) as F16Dot16;
}
#[inline]
unsafe extern "C" fn clamp(value: i64) -> F16Dot16 {
    value.clamp(F16DOT16_NEGATIVE_INFINITY as i64, F16DOT16_INFINITY as i64) as F16Dot16
}
pub unsafe extern "C" fn otfcc_f1616_add(mut a: F16Dot16, mut b: F16Dot16) -> F16Dot16 {
    return a + b;
}
pub unsafe extern "C" fn otfcc_f1616_minus(mut a: F16Dot16, mut b: F16Dot16) -> F16Dot16 {
    return a - b;
}
pub unsafe extern "C" fn otfcc_f1616_multiply(mut a: F16Dot16, mut b: F16Dot16) -> F16Dot16 {
    let mut tmp: i64 = a as i64 * b as i64 + F16DOT16_K as i64;
    let mut product: F16Dot16 = clamp(tmp >> F16DOT16_PRECISION);
    return product;
}
#[inline]
unsafe extern "C" fn divide(mut a: i64, b: i32) -> F16Dot16 {
    if b == 0 {
        return if a < 0 {
            F16DOT16_NEGATIVE_INFINITY
        } else {
            F16DOT16_INFINITY
        };
    }
    if (a < 0) != (b < 0) {
        a -= (b / 2) as i64;
    } else {
        a += (b / 2) as i64;
    }
    return clamp(a / b as i64);
}
pub unsafe extern "C" fn otfcc_f1616_muldiv(
    mut a: F16Dot16,
    mut b: F16Dot16,
    mut c: F16Dot16,
) -> F16Dot16 {
    let mut tmp: i64 = a as i64 * b as i64 + F16DOT16_K as i64;
    return divide(tmp, c as i32);
}
pub unsafe extern "C" fn otfcc_f1616_divide(mut a: F16Dot16, mut b: F16Dot16) -> F16Dot16 {
    return divide((a as i64) << F16DOT16_PRECISION, b as i32);
}
