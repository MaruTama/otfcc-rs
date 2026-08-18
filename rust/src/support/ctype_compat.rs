//! The three `<ctype.h>` classifications and two conversions otfcc actually
//! uses, in the only locale it ever runs in.
//!
//! c2rust translated the C source's `<ctype.h>` macros literally, so on glibc
//! every call went through the internal table-pointer functions
//! `__ctype_b_loc`/`__ctype_tolower_loc`/`__ctype_toupper_loc` and a bitmask
//! test against `_ISdigit` & co. Those symbols do not exist on macOS, so this
//! module used to *supply* them: three 384-entry tables and three fake
//! `#[no_mangle]` functions returning pointers into them, which let every call
//! site keep its glibc shape unchanged.
//!
//! That shape is gone now — the call sites ask these functions directly — so
//! the tables, the fake symbols and the twelve `_IS*` bit constants went with
//! it. Nine of those twelve classes were never tested by anything.
//!
//! The definitions are the plain "C" locale's, which is the only locale in
//! effect: otfcc never calls `setlocale`. Argument and return types are
//! `c_int` because that is what the call sites have — a byte read from a font,
//! sign-extended through `c_char`, so `-128..=127` — and because the C
//! functions are specified that way. Anything outside `0..=127` is
//! unclassified and unchanged, which is what glibc's own "C" table says and
//! what makes `EOF` (-1) fall out for free rather than needing a special case.
//!
//! The three predicates are checked against the platform's own libc over all
//! 384 inputs. The two case conversions cannot be, because **the two libcs
//! disagree** where C leaves the answer undefined: for `-128..=-2` glibc
//! returns `c + 256` (its tables are indexed from -128 so a sign-extended
//! `char` lands on the unsigned byte's entry) while Darwin returns `c`. Both
//! agree on `-1..=255`. This module returns `c`, and the disagreement is
//! unobservable at both call sites — `the_negative_range_disagreement_is_unobservable`
//! is the proof, not the assurance.

use core::ffi::c_int;

/// `isdigit`: decimal digits only — not hex, not Unicode.
pub const fn c_isdigit(c: c_int) -> bool {
    matches!(c, 0x30..=0x39)
}

/// `isspace`: space, and the five control characters `\t \n \v \f \r`.
///
/// `\v` (0x0B) is why this is not `u8::is_ascii_whitespace`, which omits it:
/// swapping the two would silently change where the JSON parser and
/// `sdssplitargs` stop.
pub const fn c_isspace(c: c_int) -> bool {
    matches!(c, 0x20 | 0x09..=0x0D)
}

/// `isprint`: the printable ASCII range, **space included**.
///
/// Which is why this is not `u8::is_ascii_graphic`, whose range starts one
/// character later.
pub const fn c_isprint(c: c_int) -> bool {
    matches!(c, 0x20..=0x7E)
}

/// `tolower`; for unclassified input, the identity.
pub const fn c_tolower(c: c_int) -> c_int {
    if matches!(c, 0x41..=0x5A) { c + 32 } else { c }
}

/// `toupper`; for unclassified input, the identity.
pub const fn c_toupper(c: c_int) -> c_int {
    if matches!(c, 0x61..=0x7A) { c - 32 } else { c }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These replace calls that went to the platform's libc on Linux and to a
    // hand-built table on macOS, so "the C locale, obviously" is not enough --
    // check them against the real thing, over the whole range a call site can
    // produce (a `c_char` byte is -128..=127; 128..=255 and EOF are included
    // because glibc's tables are indexable there and C code does reach them).
    #[test]
    // Miri doesn't implement libc's isdigit/isspace/isprint/tolower/toupper
    // on the macOS target ("unsupported operation", not a bug finding) --
    // this whole file's tests exist specifically to call the platform's
    // real libc and compare, so there is no Miri-friendly way to run them.
    #[cfg_attr(miri, ignore = "calls real libc ctype functions, unsupported under Miri")]
    fn ctype_predicates_match_libc() {
        for c in -128..=255 {
            unsafe {
                assert_eq!(c_isdigit(c), libc::isdigit(c) != 0, "isdigit({c})");
                assert_eq!(c_isspace(c), libc::isspace(c) != 0, "isspace({c})");
                assert_eq!(c_isprint(c), libc::isprint(c) != 0, "isprint({c})");
            }
        }
    }

    #[test]
    #[cfg_attr(miri, ignore = "calls real libc ctype functions, unsupported under Miri")]
    fn case_conversion_matches_libc_where_c_defines_it() {
        // `EOF` and any value of an `unsigned char` -- everything C actually
        // specifies, and everything the two libcs agree on.
        for c in -1..=255 {
            unsafe {
                assert_eq!(c_tolower(c), libc::tolower(c), "tolower({c})");
                assert_eq!(c_toupper(c), libc::toupper(c), "toupper({c})");
            }
        }
    }

    /// glibc's answer for the range C leaves undefined: its tables start at
    /// -128, so a sign-extended `char` reads the unsigned byte's entry. (`-1`
    /// is `EOF` and comes back unchanged, which is why it is not in the range.)
    fn glibc_tolower(c: c_int) -> c_int {
        if (-128..=-2).contains(&c) {
            c + 256
        } else {
            c_tolower(c)
        }
    }

    // Where the two libcs disagree, C is a byte >= 0x80 in a glyph name, a
    // table string or hinting text -- reachable, so the disagreement has to be
    // shown harmless rather than assumed to be. Two properties do it, one per
    // call site.
    #[test]
    #[cfg_attr(miri, ignore = "calls real libc ctype functions, unsupported under Miri")]
    fn the_negative_range_disagreement_is_unobservable() {
        // sdstolower/sdstoupper store the result back into a `c_char`, and
        // `128 as c_char == -128`, so the byte written is the same either way.
        // Checked against whichever libc is present, over the full range.
        for c in -128..=127 {
            unsafe {
                assert_eq!(
                    c_tolower(c) as ::core::ffi::c_char,
                    libc::tolower(c) as ::core::ffi::c_char,
                    "tolower({c}) truncated"
                );
                assert_eq!(
                    c_toupper(c) as ::core::ffi::c_char,
                    libc::toupper(c) as ::core::ffi::c_char,
                    "toupper({c}) truncated"
                );
            }
        }

        // strnmatch only ever asks whether two folded bytes are *equal* -- both
        // of its callers test `== 0` and neither looks at the sign of the
        // difference. Both foldings keep the negatives injective and disjoint
        // from the ASCII image, so they agree on every such question.
        for a in -128..=127 {
            for b in -128..=127 {
                assert_eq!(
                    c_tolower(a) == c_tolower(b),
                    glibc_tolower(a) == glibc_tolower(b),
                    "tolower({a}) == tolower({b})"
                );
            }
        }
    }

    #[test]
    fn the_two_rust_lookalikes_are_not_the_same_function() {
        // `\v`: whitespace to C, not to Rust.
        assert!(c_isspace(0x0B));
        assert!(!(0x0B_u8).is_ascii_whitespace());
        // space: printable to C, not "graphic" to Rust.
        assert!(c_isprint(0x20));
        assert!(!(0x20_u8).is_ascii_graphic());
    }
}
