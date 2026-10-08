//! Byte-oriented `printf`-shaped formatting for names and log/JSON text that
//! must survive non-UTF-8 bytes unchanged: [`BytePart`], which decides how
//! one typed piece is appended to a growing `Vec<u8>`, and [`bytesbuild!`],
//! which chains pieces together. Glyph names, lookup names and cmap keys are
//! built this way, and log messages render their byte strings through the
//! same trait (`logger::ByteStr`).
//!
//! `std::fmt` is not used for the whole string because most pieces are raw
//! bytes (glyph names need not be UTF-8); only the numeric pieces, which
//! are always ASCII, are formatted with it.

use std::io::Write;

use crate::support::primitives::until_nul;

/// One piece of a [`bytesbuild!`] call: knows how to append itself to a
/// growing `Vec<u8>`.
///
/// Text may not always go through this as UTF-8 -- a glyph name that isn't
/// valid UTF-8 has to survive unchanged in the JSON output, so `format!`
/// (which would replace invalid bytes with U+FFFD) is never used on
/// caller-controlled bytes here, only on values this crate itself
/// generates as ASCII digits/hex.
pub trait BytePart {
    fn append_to_vec(self, v: &mut Vec<u8>);
}

impl BytePart for &[u8] {
    fn append_to_vec(self, v: &mut Vec<u8>) {
        v.extend_from_slice(self);
    }
}

impl<const N: usize> BytePart for &[u8; N] {
    fn append_to_vec(self, v: &mut Vec<u8>) {
        (&self[..]).append_to_vec(v);
    }
}

/// A plain Rust string literal/slice -- for static labels (operator names,
/// etc.) that are always valid UTF-8, as an alternative to a `&[u8]`/`b"..."`
/// byte-string literal when the text is genuinely text. Caller-controlled
/// bytes that might not be valid UTF-8 still go through `&[u8]`/`&Vec<u8>`
/// directly, per this module's own doc comment above.
impl BytePart for &str {
    fn append_to_vec(self, v: &mut Vec<u8>) {
        self.as_bytes().append_to_vec(v);
    }
}

/// A `Handle`'s `name` (a `Vec<u8>`): appends up to the first embedded NUL,
/// matching a C `%s`/`strlen` conversion's truncation, since every existing
/// call site passing a `Handle.name` into [`bytesbuild!`] relied on that.
/// An empty `Vec` (an unset `Handle`) appends nothing.
impl BytePart for &Vec<u8> {
    fn append_to_vec(self, v: &mut Vec<u8>) {
        until_nul(self).append_to_vec(v);
    }
}

/// A single byte (`%c`).
///
/// C converts the argument to `unsigned char`, so this is one raw byte and
/// *not* a `char`: formatting a `char` would UTF-8 encode anything above
/// 0x7f into two bytes. `otl/read.rs` builds lookup names out of the four
/// bytes of an OpenType tag this way, and those names reach the JSON
/// output.
#[derive(Debug, Clone, Copy)]
pub struct Byte(pub u8);

impl BytePart for Byte {
    fn append_to_vec(self, v: &mut Vec<u8>) {
        v.push(self.0);
    }
}

/// `%04x`
#[derive(Debug, Clone, Copy)]
pub struct Hex4(pub u32);
/// `%04X`
#[derive(Debug, Clone, Copy)]
pub struct Hex4Upper(pub u32);
/// `%02x`
#[derive(Debug, Clone, Copy)]
pub struct Hex2(pub u32);
/// `%02X`
#[derive(Debug, Clone, Copy)]
pub struct Hex2Upper(pub u32);
/// `%05d`
#[derive(Debug, Clone, Copy)]
pub struct Dec5(pub i32);

impl BytePart for i32 {
    fn append_to_vec(self, v: &mut Vec<u8>) {
        write!(v, "{self}").expect("writing to a Vec cannot fail");
    }
}

impl BytePart for u32 {
    fn append_to_vec(self, v: &mut Vec<u8>) {
        write!(v, "{self}").expect("writing to a Vec cannot fail");
    }
}

impl BytePart for Dec5 {
    fn append_to_vec(self, v: &mut Vec<u8>) {
        write!(v, "{:05}", self.0).expect("writing to a Vec cannot fail");
    }
}

/// The `u32` casts at the call sites are not cosmetic: C's `%x` reads an
/// `unsigned int`, so a negative `int` argument prints as its 32-bit two's
/// complement -- eight digits, not four. `as u32` reproduces exactly that,
/// and widens a `u16` the same way C's default promotion does.
impl BytePart for Hex4 {
    fn append_to_vec(self, v: &mut Vec<u8>) {
        write!(v, "{:04x}", self.0).expect("writing to a Vec cannot fail");
    }
}

impl BytePart for Hex4Upper {
    fn append_to_vec(self, v: &mut Vec<u8>) {
        write!(v, "{:04X}", self.0).expect("writing to a Vec cannot fail");
    }
}

impl BytePart for Hex2 {
    fn append_to_vec(self, v: &mut Vec<u8>) {
        write!(v, "{:02x}", self.0).expect("writing to a Vec cannot fail");
    }
}

impl BytePart for Hex2Upper {
    fn append_to_vec(self, v: &mut Vec<u8>) {
        write!(v, "{:02X}", self.0).expect("writing to a Vec cannot fail");
    }
}

/// Append pieces to a growing `Vec<u8>`, in order, and evaluate to the
/// result.
///
/// ```ignore
/// bytesbuild!(b"lookup_", name, b"_", Hex2(kind as u32), b"_", index)
/// ```
/// Each piece is appended through [`BytePart`], so its type decides how it
/// is rendered.
#[macro_export]
macro_rules! bytesbuild {
    ($($part:expr),* $(,)?) => {{
        let mut __v: ::std::vec::Vec<u8> = ::std::vec::Vec::new();
        $(
            $crate::support::fmt::BytePart::append_to_vec($part, &mut __v);
        )*
        __v
    }};
}

/// A from-scratch, byte-exact reimplementation of C's `%.Pg` conversion
/// with `P = precision` (glibc's `sprintf(buf, "%.Pg", val)`; the CFF
/// writer uses P = 13, the CLI's step times plain `%g`, i.e. P = 6). `%.Pg` rounds
/// `val` to `P` significant decimal digits, then picks `%f`-style
/// (`P-1-X` fractional digits, where `X` is the decimal exponent of the
/// rounded value) when `-4 <= X < P`, else `%e`-style (`P-1` fractional
/// digits, exponent as `e+XX`/`e-XX` with the sign always shown and at
/// least 2 digits), and finally strips trailing fractional zeros (and the
/// bare decimal point if none remain) since the `#` flag is never set at
/// any call site. Rust's `{:.N}`/`{:.N}e` formatting is, like glibc's,
/// correctly-rounded (round-to-nearest, ties-to-even) fixed-precision
/// decimal conversion -- unlike `vendor/emyg_dtoa.rs`'s *shortest*
/// round-tripping Grisu2 output, a fixed digit count has exactly one
/// correct answer, so the two must agree bit-for-bit. Verified against
/// CPython's `"%.13g" % val` (itself glibc-equivalent) across 200,000
/// pseudo-random f64 bit patterns with zero mismatches; see this file's
/// `libcff/codecs.rs`'s `format_g13` tests for the representative cases pinned from that
/// sweep.
pub fn format_g(val: f64, precision: usize) -> String {
    let precision = precision as i32;
    let e_form = format!("{:.*e}", (precision - 1) as usize, val);
    let e_pos = e_form.find('e').expect("Rust's `{:e}` always emits 'e'");
    let exponent: i32 = e_form[e_pos + 1..]
        .parse()
        .expect("Rust's `{:e}` exponent is always a plain decimal integer");
    let s = if (-4..precision).contains(&exponent) {
        let frac_digits = (precision - 1 - exponent).max(0) as usize;
        format!("{:.*}", frac_digits, val)
    } else {
        let mantissa = strip_trailing_fraction_zeros(&e_form[..e_pos]);
        format!(
            "{}e{}{:02}",
            mantissa,
            if exponent < 0 { '-' } else { '+' },
            exponent.abs()
        )
    };
    if (-4..precision).contains(&exponent) {
        strip_trailing_fraction_zeros(&s).to_string()
    } else {
        s
    }
}
fn strip_trailing_fraction_zeros(s: &str) -> &str {
    match s.find('.') {
        None => s,
        Some(dot) => {
            let stripped = s[..dot + 1].len() + s[dot + 1..].trim_end_matches('0').len();
            if stripped == dot + 1 {
                &s[..dot]
            } else {
                &s[..stripped]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What C's `printf` makes of the same conversion and argument.
    ///
    /// The helpers are checked against the C library rather than against
    /// hand-written expectations: the whole point of them is to reproduce
    /// a `printf`-family conversion byte for byte, and only libc can settle
    /// what that was.
    macro_rules! assert_matches_printf {
        ($fmt:expr, $c_arg:expr, $built:expr) => {{
            let mut expect = [0 as ::core::ffi::c_char; 64];
            let n = libc::snprintf(
                expect.as_mut_ptr(),
                expect.len(),
                concat!($fmt, "\0").as_ptr() as *const ::core::ffi::c_char,
                $c_arg,
            );
            assert!(n >= 0 && (n as usize) < expect.len(), "snprintf overflowed");
            let expect =
                ::core::slice::from_raw_parts(expect.as_ptr() as *const u8, n as usize).to_vec();
            let got: Vec<u8> = $built;
            assert_eq!(
                String::from_utf8_lossy(&got),
                String::from_utf8_lossy(&expect),
                "conversion {} disagrees with libc",
                $fmt
            );
        }};
    }

    #[test]
    #[cfg_attr(
        miri,
        ignore = "calls libc::snprintf via assert_matches_printf!, unsupported under Miri"
    )]
    fn decimal_matches_printf() {
        unsafe {
            for v in [0, 1, -1, 42, -42, i32::MAX, i32::MIN] {
                assert_matches_printf!("%d", v, bytesbuild!(v));
                assert_matches_printf!("%05d", v, bytesbuild!(Dec5(v)));
            }
            for v in [0u32, 1, 65535, u32::MAX] {
                assert_matches_printf!("%u", v, bytesbuild!(v));
            }
        }
    }

    // A negative `int` reaches `%x` as an `unsigned int`, so it prints eight
    // digits and not four. Casting to `u16` at the call site -- the obvious
    // reading of "%04x" -- would silently drop the top half.
    #[test]
    #[cfg_attr(
        miri,
        ignore = "calls libc::snprintf via assert_matches_printf!, unsupported under Miri"
    )]
    fn hex_matches_printf_including_negatives() {
        unsafe {
            for v in [0i32, 1, 0x0a, 0xabcd, 0xfffff, -1, -32768] {
                assert_matches_printf!("%04x", v, bytesbuild!(Hex4(v as u32)));
                assert_matches_printf!("%04X", v, bytesbuild!(Hex4Upper(v as u32)));
                assert_matches_printf!("%02x", v, bytesbuild!(Hex2(v as u32)));
                assert_matches_printf!("%02X", v, bytesbuild!(Hex2Upper(v as u32)));
            }
        }
    }

    // `%c` is a byte, not a character: 0xe9 is one byte for C, and would be
    // the two bytes of U+00E9 if it went through Rust's `char` formatting.
    #[test]
    #[cfg_attr(
        miri,
        ignore = "calls libc::snprintf via assert_matches_printf!, unsupported under Miri"
    )]
    fn byte_is_one_byte_not_a_char() {
        unsafe {
            for v in [b'A' as i32, 0, 0x7f, 0x80, 0xe9, 0xff] {
                assert_matches_printf!("%c", v, bytesbuild!(Byte(v as u8)));
            }
            let got = bytesbuild!(Byte(0xe9));
            assert_eq!(got.len(), 1);
            assert_eq!('\u{e9}'.to_string().len(), 2); // ...which this is not
        }
    }

    #[test]
    fn pieces_are_appended_in_order() {
        let got = bytesbuild!(
            b"lookup_",
            "ccmp",
            b"_",
            Hex2(0x1f),
            b"_",
            7_i32,
        );
        assert_eq!(got, b"lookup_ccmp_1f_7");
    }

    #[test]
    fn str_piece_appends_as_utf8_bytes() {
        let got = bytesbuild!(b"op_", "vmoveto");
        assert_eq!(got, b"op_vmoveto");
    }

    // `format_g(v, 6)` replaced `snprintf("%g")` for the step times. Checked
    // against libc over typical timings, the format's switch points
    // (1e-4, 1e6) and a deterministic pseudo-random sweep.
    #[test]
    #[cfg_attr(
        miri,
        ignore = "calls libc::snprintf via assert_matches_printf!, unsupported under Miri"
    )]
    fn format_g_6_matches_printf() {
        let mut values = vec![
            0.0, 0.5, 1.0, 0.000148, 0.00169652, 12.5, 999999.4, 999999.6, 1e6, 1e-4,
            9.99995e-5, 0.000099999, 123456.5, 1.5e-7, 3.2e12,
        ];
        let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
        for _ in 0..20_000 {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            let mantissa = (x >> 11) as f64 / (1u64 << 53) as f64;
            let exponent = ((x & 0x3f) as i32) - 20;
            values.push(mantissa * 10f64.powi(exponent));
        }
        for v in values {
            unsafe {
                assert_matches_printf!("%g", v, format_g(v, 6).into_bytes());
            }
        }
    }

}
