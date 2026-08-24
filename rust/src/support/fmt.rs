//! Byte-oriented `printf`-shaped formatting for names and log/JSON text that
//! must survive non-UTF-8 bytes unchanged.
//!
//! Was part of `vendor/sds.rs`, a transpiled port of redis's `sds` string
//! type -- but nothing in this crate actually builds an `sds` anymore
//! ([`bytesbuild!`] is the only builder macro any real call site uses; the
//! `SdsRaw`-returning `sdsbuild!` this trait's `append_to` half once served
//! had zero callers, confirmed by grep before this file split off). What's
//! left is purely this: a trait deciding how to append one typed piece to a
//! growing `Vec<u8>`, and the macro that chains pieces together.
use libc::strlen;

/// One piece of a [`bytesbuild!`] call: knows how to append itself to a
/// growing `Vec<u8>`.
///
/// Text may not always go through this as UTF-8 -- a glyph name that isn't
/// valid UTF-8 has to survive unchanged in the JSON output, so `format!`
/// (which would replace invalid bytes with U+FFFD) is never used on
/// caller-controlled bytes here, only on values this crate itself
/// generates as ASCII digits/hex.
pub trait SdsPart {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>);
}

impl SdsPart for &[u8] {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        v.extend_from_slice(self);
    }
}

impl<const N: usize> SdsPart for &[u8; N] {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        unsafe { (&self[..]).append_to_vec(v) };
    }
}

/// A `Handle`'s `name` (a `Vec<u8>`): appends up to the first embedded NUL,
/// matching a C `%s`/`strlen` conversion's truncation, since every existing
/// call site passing a `Handle.name` into [`bytesbuild!`] relied on that.
/// An empty `Vec` (an unset `Handle`) appends nothing -- unlike a null
/// C-string pointer's `"(null)"` below, there's no "null vs. empty"
/// distinction left once the storage is owned, which only changes
/// warning-log wording for a handle with no name, never any dumped/built
/// output.
impl SdsPart for &Vec<u8> {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        let bytes = match self.iter().position(|&b| b == 0) {
            Some(nul_pos) => &self[..nul_pos],
            None => &self[..],
        };
        unsafe { bytes.append_to_vec(v) };
    }
}

/// A C string (`%s`): the bytes up to the terminating NUL.
///
/// A null pointer appends `(null)`, which is what both glibc and Apple's
/// libc print for `%s`. The old code handed the pointer straight to
/// `vsnprintf`, so any call site that can pass null was already relying on
/// that.
impl SdsPart for *const ::core::ffi::c_char {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        if self.is_null() {
            return unsafe { b"(null)".append_to_vec(v) };
        }
        let bytes = unsafe { ::core::slice::from_raw_parts(self as *const u8, strlen(self)) };
        v.extend_from_slice(bytes);
    }
}

impl SdsPart for *mut ::core::ffi::c_char {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        unsafe { (self as *const ::core::ffi::c_char).append_to_vec(v) };
    }
}

/// A static C string (`%s`), for the label tables that reach the log and
/// the JSON output. Identical to the `*const c_char` impl above, minus the
/// `strlen` and the null check: a `CStr` carries its own length and cannot
/// be null.
impl SdsPart for &::core::ffi::CStr {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        unsafe { self.to_bytes().append_to_vec(v) };
    }
}

/// A single byte (`%c`).
///
/// C converts the argument to `unsigned char`, so this is one raw byte and
/// *not* a `char`: formatting a `char` would UTF-8 encode anything above
/// 0x7f into two bytes. `otl/read.rs` builds lookup names out of the four
/// bytes of an OpenType tag this way, and those names reach the JSON
/// output.
pub struct Byte(pub u8);

impl SdsPart for Byte {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        v.push(self.0);
    }
}

/// `%04x`
pub struct Hex4(pub u32);
/// `%04X`
pub struct Hex4Upper(pub u32);
/// `%02x`
pub struct Hex2(pub u32);
/// `%02X`
pub struct Hex2Upper(pub u32);
/// `%05d`
pub struct Dec5(pub ::core::ffi::c_int);

fn cat_ascii_vec(v: &mut Vec<u8>, digits: &str) {
    v.extend_from_slice(digits.as_bytes());
}

impl SdsPart for ::core::ffi::c_int {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{self}"));
    }
}

impl SdsPart for ::core::ffi::c_uint {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{self}"));
    }
}

impl SdsPart for Dec5 {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{:05}", self.0));
    }
}

/// The `u32` casts at the call sites are not cosmetic: C's `%x` reads an
/// `unsigned int`, so a negative `int` argument prints as its 32-bit two's
/// complement -- eight digits, not four. `as u32` reproduces exactly that,
/// and widens a `u16` the same way C's default promotion does.
impl SdsPart for Hex4 {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{:04x}", self.0));
    }
}

impl SdsPart for Hex4Upper {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{:04X}", self.0));
    }
}

impl SdsPart for Hex2 {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{:02x}", self.0));
    }
}

impl SdsPart for Hex2Upper {
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{:02X}", self.0));
    }
}

/// Append pieces to a growing `Vec<u8>`, in order, and evaluate to the
/// result.
///
/// ```ignore
/// bytesbuild!(b"lookup_", name, b"_", Hex2(kind as u32), b"_", index)
/// ```
/// Each piece is appended through [`SdsPart`], so its type decides how it
/// is rendered.
#[macro_export]
macro_rules! bytesbuild {
    ($($part:expr),* $(,)?) => {{
        let mut __v: ::std::vec::Vec<u8> = ::std::vec::Vec::new();
        $(
            // Callers in the middle of the `unsafe_op_in_unsafe_fn` burn-down
            // increasingly wrap their whole function body in one `unsafe {}`
            // block rather than fine-grain every raw-pointer operation --
            // this macro's own `unsafe` then nests inside that caller block,
            // which `unused_unsafe` (correctly) flags as redundant. Silenced
            // here, once, rather than forcing every call site (this macro is
            // used at ~50+ of them) to restructure around it.
            #[allow(unused_unsafe)]
            unsafe { $crate::support::fmt::SdsPart::append_to_vec($part, &mut __v); }
        )*
        __v
    }};
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

    // The reason these helpers exist instead of `format!`: a glyph name that
    // is not valid UTF-8 has to survive unchanged. `to_string_lossy` would
    // replace the 0xe9 with U+FFFD and the font would come out with a
    // different name.
    #[test]
    fn c_string_is_copied_as_bytes_even_when_not_utf8() {
        let name = b"caf\xe9\0";
        let got = bytesbuild!(name.as_ptr() as *const ::core::ffi::c_char);
        assert_eq!(got, b"caf\xe9");
    }

    #[test]
    #[cfg_attr(
        miri,
        ignore = "calls libc::snprintf via assert_matches_printf!, unsupported under Miri"
    )]
    fn null_c_string_prints_like_libc() {
        unsafe {
            assert_matches_printf!(
                "%s",
                ::core::ptr::null::<::core::ffi::c_char>(),
                bytesbuild!(::core::ptr::null::<::core::ffi::c_char>())
            );
        }
    }

    // `&[u8]`/`&[u8; N]` append every byte, embedded NULs included; a C
    // string (`*const c_char`) stops at the first one, like `strlen`. This
    // used to be `%S` (an `SdsRaw`'s stored length) vs. `%s` (`strlen`) --
    // the distinction survives here as "raw byte slice" vs. "C string",
    // since nothing left in this crate builds a length-prefixed `SdsRaw`.
    #[test]
    fn byte_slice_keeps_embedded_nul_but_c_string_does_not() {
        let by_slice = bytesbuild!(b"ab\0cd");
        assert_eq!(by_slice, b"ab\0cd");
        let by_c_string = bytesbuild!(b"ab\0cd\0".as_ptr() as *const ::core::ffi::c_char);
        assert_eq!(by_c_string, b"ab");
    }

    #[test]
    fn pieces_are_appended_in_order() {
        let got = bytesbuild!(
            b"lookup_",
            b"ccmp\0".as_ptr() as *const ::core::ffi::c_char,
            b"_",
            Hex2(0x1f),
            b"_",
            7 as ::core::ffi::c_int,
        );
        assert_eq!(got, b"lookup_ccmp_1f_7");
    }
}
