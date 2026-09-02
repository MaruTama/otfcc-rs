#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
// `read_16u`/friends -- factored out of the ~40 per-file private copies
// c2rust emitted (one per translation unit that #included
// c/lib/support/bin-io.h's `static inline` helpers) -- lost their last
// caller in Stage 11 Phase 11 (`table/cvt.rs`'s `otfcc_parse_cvt`, the only
// remaining parse-side consumer, converted to `u16::from_be_bytes` directly
// on its own already-safe `&[u8]`) and were deleted; every binary-format
// reader in the crate now goes through `support/font_reader.rs`'s
// bounds-checked `FontReader` instead.
//
// What's left here is `pos_to_u16`, the one *write*-side conversion that
// cannot be spelled inline without inviting someone to "simplify" it — see
// its comment.

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
/// c2rust got this wrong; `rust/scripts/archive/fix-float-narrowing.py`
/// records the original call-site list and the full diagnosis.
#[inline]
pub(crate) fn pos_to_u16(x: f64) -> u16 {
    x as i16 as u16
}

#[cfg(test)]
mod tests {
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
