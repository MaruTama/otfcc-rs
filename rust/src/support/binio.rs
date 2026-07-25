// Shared big-endian byte readers, factored out of the ~40 per-file private
// copies c2rust emitted (one per translation unit that #included
// c/lib/support/bin-io.h's `static inline` helpers). Never externally linked
// (no #[no_mangle]) even in their per-file form, so consolidating them
// changes no ABI. Bodies rewritten to u*::from_be_bytes (matching the
// idiom already used on the write side in support/buffer.rs), but
// every signature is byte-for-byte identical to the original so callers
// need no changes beyond `use`.
//
// Also holds `pos_to_u16`, the one *write*-side conversion that cannot be
// spelled inline without inviting someone to "simplify" it — see its comment.

#[inline]
pub(crate) unsafe fn read_8u(src: *const u8) -> u8 {
    *src
}

#[inline]
pub(crate) unsafe fn read_8s(src: *const u8) -> i8 {
    read_8u(src) as i8
}

#[inline]
pub(crate) unsafe fn read_16u(src: *const u8) -> u16 {
    u16::from_be_bytes([*src, *src.offset(1)])
}

#[inline]
pub(crate) unsafe fn read_16s(src: *const u8) -> i16 {
    read_16u(src) as i16
}

#[inline]
pub(crate) unsafe fn read_24u(src: *const u8) -> u32 {
    u32::from_be_bytes([0, *src, *src.offset(1), *src.offset(2)])
}

#[inline]
pub(crate) unsafe fn read_32u(src: *const u8) -> u32 {
    u32::from_be_bytes([*src, *src.offset(1), *src.offset(2), *src.offset(3)])
}

#[inline]
pub(crate) unsafe fn read_32s(src: *const u8) -> i32 {
    read_32u(src) as i32
}

#[inline]
pub(crate) unsafe fn read_64u(src: *const u8) -> u64 {
    u64::from_be_bytes([
        *src,
        *src.offset(1),
        *src.offset(2),
        *src.offset(3),
        *src.offset(4),
        *src.offset(5),
        *src.offset(6),
        *src.offset(7),
    ])
}

/// C's *implicit* `pos_t` (f64) -> `uint16_t` narrowing, as it happens at
/// `bufwrite16b()` call sites whose C source has no explicit intermediate
/// cast — `bufwrite16b(buf, hmtx->metrics[j].lsb)` and friends.
///
/// **`x as u16` is not the same thing and must never be substituted here.**
/// Rust's float-to-unsigned conversion *saturates*, so every negative value
/// would become 0, silently zeroing negative `hmtx.lsb`, `vmtx.tsb` and
/// `VORG.defaultVerticalOrigin` in the built font. C converts through a
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

#[derive(Copy, Clone)]
#[repr(C)]
pub union otfcc_EndianProbe32 {
    pub i1: [u8; 4],
    pub i4: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union otfcc_EndianProbe16 {
    pub i1: [u8; 2],
    pub i2: u16,
}
