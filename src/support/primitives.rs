#![forbid(unsafe_code)]
//! otfcc's scalar vocabulary, the Rust counterpart of
//! `c/include/otfcc/primitives.h`.
//!
//! c2rust declared each of these in every file that used one — `GlyphId` 65
//! times, `Pos` 57 — so the whole set now lives here and is imported. The
//! comments come from the C header: they are the only place the *meaning* of
//! these aliases is written down, and `u16` on its own does not tell a reader
//! whether a number is a glyph index, a class, or a table index.

/// 2.14 fixed-point, a value in [-1, 1].
pub type F2Dot14 = i16;
/// 16.16 fixed-point, used for intermediate coordinates.
///
/// Handle with care around GVAR's implicit deltas: the arithmetic helpers
/// below saturate towards ±infinity, and infinity short-circuits expressions.
pub type F16Dot16 = i32;

/// Glyph index.
pub type GlyphId = u16;
/// A collection length as the 16-bit count the font format stores.
///
/// Every count in an OpenType table is 16 bits, and the two ways a count
/// reaches the writers keep it there: the JSON reader rejects any collection
/// past 65,535 members before parsing anything (`support::json_limits`), and
/// the binary reader only ever produces collections it sized from a 16-bit
/// field. A longer collection therefore means that invariant broke upstream.
/// Fail loudly here rather than truncate with `as` and emit a table whose
/// count disagrees with its contents.
///
/// Use it where the length is a count of *that* kind. A length that can be
/// legitimately larger -- e.g. points in a contour read from a binary font,
/// which can be 65,536 -- must not go through it.
#[track_caller]
pub fn count_u16(len: usize) -> GlyphId {
    GlyphId::try_from(len).expect("count exceeds the 16-bit limit of the font format")
}
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
pub type Pos = f64;
/// Transform scaling factor.
pub type Scale = f64;
/// Length.
pub type Length = f64;

pub const F16DOT16_PRECISION: i32 = 16_i32;
pub const F16DOT16_K: i32 =
    1_i32 << (F16DOT16_PRECISION - 1_i32);
pub const F16DOT16_INFINITY: F16Dot16 = 0x7fffffff_i32 as F16Dot16;
pub const F16DOT16_NEGATIVE_INFINITY: F16Dot16 = 0x80000000_u32 as F16Dot16;
pub fn from_f2dot14(x: F2Dot14) -> f64 {
    return x as i32 as f64 / 16384.0f64;
}
// `f64::round`, not libm's `round` through an `extern "C"` block: the two
// agree bit-for-bit (both round half away from zero, per IEEE 754 / C99),
// verified across ~16,000 inputs covering every half-way case in both
// directions, NaN, +/-inf and the exact `x * 16384.0` / `x * 65536.0`
// shapes these two functions use. Dropping the extern is what lets this
// whole module become `forbid(unsafe_code)`, and it also un-blocks Miri:
// `table/glyf/read.rs`'s IUP interpolation regression test was
// `#[cfg_attr(miri, ignore)]`d purely because reaching `to_fixed`
// meant calling libm `round`, which Miri cannot execute on macOS.
pub fn to_f2dot14(x: f64) -> i16 {
    return (x * 16384.0f64).round() as i16;
}
pub fn from_fixed(x: F16Dot16) -> f64 {
    return x as f64 / 65536.0f64;
}
pub fn to_fixed(x: f64) -> F16Dot16 {
    return (x * 65536.0f64).round() as F16Dot16;
}
#[inline]
fn clamp(value: i64) -> F16Dot16 {
    value.clamp(F16DOT16_NEGATIVE_INFINITY as i64, F16DOT16_INFINITY as i64) as F16Dot16
}
pub fn f1616_add(a: F16Dot16, b: F16Dot16) -> F16Dot16 {
    return a + b;
}
pub fn f1616_minus(a: F16Dot16, b: F16Dot16) -> F16Dot16 {
    return a - b;
}
pub fn f1616_multiply(a: F16Dot16, b: F16Dot16) -> F16Dot16 {
    let tmp: i64 = a as i64 * b as i64 + F16DOT16_K as i64;
    let product: F16Dot16 = clamp(tmp >> F16DOT16_PRECISION);
    return product;
}
#[inline]
fn divide(mut a: i64, b: i32) -> F16Dot16 {
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
pub fn f1616_muldiv(a: F16Dot16, b: F16Dot16, c: F16Dot16) -> F16Dot16 {
    let tmp: i64 = a as i64 * b as i64 + F16DOT16_K as i64;
    return divide(tmp, c);
}
pub fn f1616_divide(a: F16Dot16, b: F16Dot16) -> F16Dot16 {
    return divide((a as i64) << F16DOT16_PRECISION, b);
}

/// `bytes` up to its first NUL, or all of it if it has none: how glyph
/// names and other strings that came from C strings are read.
pub fn until_nul(bytes: &[u8]) -> &[u8] {
    let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    &bytes[..len]
}

#[cfg(test)]
mod until_nul_tests {
    use super::until_nul;

    #[test]
    fn cuts_at_the_first_nul_only() {
        assert_eq!(until_nul(b"abc"), b"abc");
        assert_eq!(until_nul(b"ab\0c\0"), b"ab");
        assert_eq!(until_nul(b"\0abc"), b"");
        assert_eq!(until_nul(b""), b"");
    }
}

#[cfg(test)]
mod count_u16_tests {
    use super::*;

    #[test]
    fn counts_up_to_the_16_bit_limit_convert_exactly() {
        assert_eq!(count_u16(0), 0);
        assert_eq!(count_u16(usize::from(GlyphId::MAX)), GlyphId::MAX);
    }

    #[test]
    #[should_panic(expected = "16-bit limit")]
    fn one_past_the_limit_panics_instead_of_wrapping_to_zero() {
        count_u16(usize::from(GlyphId::MAX) + 1);
    }
}
