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
pub const F16DOT16_NEGATIVE_INFINITY: F16Dot16 = 0x80000000 as ::core::ffi::c_uint as F16Dot16;
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
/// C's *implicit* `Pos` (f64) -> `uint16_t` narrowing, as it happens at
/// `bufwrite16b()` call sites whose C source has no explicit intermediate
/// cast — `bufwrite16b(buf, hmtx->metrics[j].lsb)` and friends.
///
/// **`x as u16` is not the same thing and must never be substituted here.**
/// Rust's float-to-unsigned conversion *saturates*, so every negative value
/// would become 0, silently zeroing negative `hmtx.lsb`, `vmtx.tsb` and
/// `vorg.default_vertical_origin` in the built font. C converts through a
/// signed integer and reinterprets the bits, so `-41.0` has to come out as
/// `0xffd7` (which the reader decodes back to -41). Going through `i16` is
/// what reproduces that.
///
/// c2rust got this wrong originally; this doc comment (and the regression
/// test below) is the full diagnosis, since fixed.
#[inline]
pub(crate) fn pos_to_u16(x: f64) -> u16 {
    x as i16 as u16
}
/// The integer at the start of `s`, read the way C's `strtol(s, NULL,
/// base)` reads it and truncated to `i32`. `base` must be 2..=16.
///
/// Deliberately as lenient as `strtol`: optional leading ASCII whitespace,
/// an optional `+`/`-`, then the longest run of digits valid in `base`;
/// `0` when there are none, and anything after them is ignored (`"65abc"`
/// reads as 65). It reads cmap keys in otfcc's JSON (`U+0041` or `65`) and
/// the CLI's `--ttc-index` and `-O` arguments, and being stricter would
/// change what otfcc accepts there. No `0x` prefix is recognised.
///
/// Overflow wraps rather than saturating at `LONG_MAX` the way C does.
/// That is what the release build of the hand-rolled predecessors already
/// did (`i64` accumulate, then `as i32`); spelling it `wrapping_*` makes
/// debug and Miri builds agree with release instead of panicking on a
/// 19-digit JSON object key, which is reachable from fuzzed input.
pub fn parse_int_prefix(s: &[u8], base: u32) -> i32 {
    debug_assert!((2..=16).contains(&base));
    let mut it = s.iter().skip_while(|b| b.is_ascii_whitespace()).peekable();
    let negative = match it.peek() {
        Some(&&b'-') => {
            it.next();
            true
        }
        Some(&&b'+') => {
            it.next();
            false
        }
        _ => false,
    };
    let val: i64 = it
        .map_while(|&b| (b as char).to_digit(base))
        .fold(0i64, |acc, d| acc.wrapping_mul(base as i64).wrapping_add(d as i64));
    if negative { (val as i32).wrapping_neg() } else { val as i32 }
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

#[cfg(test)]
mod pos_to_u16_tests {
    use super::*;

    // The regression this guards is invisible in a plain read of the code:
    // `x as u16` compiles, looks equivalent, and quietly turns every negative
    // side bearing into 0. Real values from the payload fonts.
    #[test]
    fn pos_to_u16_wraps_negatives_instead_of_saturating() {
        assert_eq!(pos_to_u16(-41.0), 0xffd7);
        assert_eq!(pos_to_u16(-1.0), 0xffff);
        assert_eq!(pos_to_u16(-32768.0), 0x8000);
        // ...and a direct cast would not:
        assert_eq!(-41.0f64 as u16, 0);
    }

    #[test]
    fn pos_to_u16_truncates_toward_zero_like_c() {
        assert_eq!(pos_to_u16(41.9), 41);
        assert_eq!(pos_to_u16(-41.9), 0xffd7);
        assert_eq!(pos_to_u16(0.0), 0);
        // Out of range for i16, so Rust's float->int saturation applies. C
        // leaves this case undefined, and a font with a >32767 side bearing
        // is malformed anyway; recorded to pin down what we actually do.
        assert_eq!(pos_to_u16(65535.0), 0x7fff);
    }
}

#[cfg(test)]
mod parse_int_prefix_tests {
    use super::parse_int_prefix as strtol;

    #[test]
    fn strtol_matches_libc_on_the_shapes_this_crate_feeds_it() {
        assert_eq!(strtol(b"41", 10), 41);
        assert_eq!(strtol(b"0041", 16), 0x41);
        assert_eq!(strtol(b"10FFFF", 16), 0x10FFFF);
        assert_eq!(strtol(b"2", 10), 2);
        // Leading whitespace and an explicit `+` are both accepted, as by
        // `strtol` itself.
        assert_eq!(strtol(b"  \t-7", 10), -7);
        assert_eq!(strtol(b"+7", 10), 7);
        // Parsing stops at the first byte that is not a digit in `base`,
        // and "no digits at all" is 0 -- not an error, matching `strtol`'s
        // "no conversion performed" return.
        assert_eq!(strtol(b"12abc", 10), 12);
        assert_eq!(strtol(b"abc", 10), 0);
        assert_eq!(strtol(b"", 10), 0);
        assert_eq!(strtol(b"-", 10), 0);
        // Base 16 does NOT skip an `0x` prefix here (C's `strtol` does);
        // no caller in this crate passes one -- `parse_unicode` has already
        // stripped `U+`, and the CLI callers are base 10.
        assert_eq!(strtol(b"0x1F", 16), 0);
    }

    #[test]
    fn a_digit_run_too_long_for_i64_wraps_instead_of_panicking() {
        // 25 digits: the `i64` accumulator overflows several times over.
        // The pre-consolidation code panicked here under `debug_assertions`
        // (so: Miri, and `cargo fuzz`'s default profile) while release
        // wrapped silently.
        assert_eq!(strtol(b"9999999999999999999999999", 10), 1_241_513_983);
    }
}
