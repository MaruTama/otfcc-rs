//! The one piece of C's `strtol` this crate still needs, over `&[u8]`.
//!
//! Every caller used to reach for `libc::strtol` (or `atoi`, a one-line
//! wrapper around it) on a raw `*const c_char`, which meant an `unsafe fn`
//! and a NUL-terminated buffer at each site even though the bytes always
//! came from somewhere Rust already owned: a `ParsedValue` object key, or a
//! CLI argument that had been a Rust `String` moments earlier. Three
//! near-identical hand-rolled copies had already grown out of that
//! (`table/cmap.rs`'s base-10 `atoi` and base-16 `parse_hex`, from Stage E);
//! this module is their single home.
//!
//! What it reproduces, deliberately and no more: optional leading ASCII
//! whitespace, an optional `+`/`-` sign, then the longest run of digits
//! valid in `base`, with `0` for "no digits at all". It stops there -- no
//! `errno`, no end pointer, no base-0 prefix detection.
//!
//! `support/ttinstr.rs` keeps its own `strtol_base0`/`strtol_base2`: those
//! return how many bytes they consumed (their caller is a lexer that must
//! advance past the number) and base 0 has to sniff `0x`/`0` prefixes.
//! Folding them in here would mean giving every caller an end position it
//! does not want; they are noted as follow-up rather than forced together.

/// `strtol(s, NULL, base)` truncated to `i32`, over bytes instead of a C
/// string. `base` must be 2..=16.
///
/// Overflow wraps rather than saturating at `LONG_MAX` the way C does.
/// That is what the release build of the hand-rolled predecessors already
/// did (`i64` accumulate, then `as i32`); spelling it `wrapping_*` makes
/// debug and Miri builds agree with release instead of panicking on a
/// 19-digit JSON object key, which is reachable from fuzzed input.
pub fn strtol(s: &[u8], base: u32) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::strtol;

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
