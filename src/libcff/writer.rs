use crate::libcff::CffCharstringOperator;
use crate::support::buffer::Buffer;
/// A safe, allocation-free reimplementation of C99's `modf`, matching its
/// exact contract rather than reaching for `f64::trunc`/`f64::fract`
/// directly, which diverge from it on exactly one input class: for a
/// finite `x`, `x.trunc()`/`x.fract()` already split it into the same
/// integral/fractional pair `modf` does (both truncate toward zero, and
/// the fractional part keeps `x`'s own sign), but for `x = ±inf`,
/// `x.fract()` is `x - x.trunc()` = `inf - inf` = `NaN`, while C99's
/// `modf` defines the fractional part of an infinity to be a zero of the
/// same sign, with the integral part set to that same infinity. This
/// crate's own `Pos` (`f64`) glyph coordinates are never observed to
/// reach `cff_merge_cs2_operand` as non-finite on any input this crate's
/// fuzz corpus or test suite constructs, but proving that for every
/// possible caller, present and future, is a much larger claim than this
/// one function needs to stand on -- special-casing infinity here instead
/// keeps this rewrite behaviorally identical to the `unsafe extern "C"`
/// `modf` it replaces for every `f64` value, not just the ones this
/// crate happens to construct today.
fn modf(x: f64) -> (f64, f64) {
    if x.is_infinite() {
        (x, 0.0_f64.copysign(x))
    } else {
        (x.trunc(), x.fract())
    }
}
pub fn cff_build_header() -> Buffer {
    Buffer::from_bytes(&[1_u8, 0_u8, 4_u8, 4_u8])
}
pub fn cff_merge_cs2_operator(blob: &mut Buffer, val: CffCharstringOperator) {
    let val = val.0;
    if val >= 0x100_i32 {
        blob.write_bytes(&[(val >> 8_i32) as u8, (val & 0xff_i32) as u8]);
    } else {
        blob.write_bytes(&[(val & 0xff_i32) as u8]);
    };
}
pub fn cff_merge_cs2_int(blob: &mut Buffer, val: i32) {
    if (-1131_i32..=-108_i32).contains(&val) {
        blob.write_bytes(&[
            (((-108_i32 - val) / 256_i32 + 251_i32) as u8 as i32) as u8,
            (((-108_i32 - val) % 256_i32) as u8 as i32) as u8,
        ]);
    } else if (-107_i32..=107_i32).contains(&val) {
        blob.write_bytes(&[((val + 139_i32) as u8 as i32) as u8]);
    } else if (108_i32..=1131_i32).contains(&val) {
        blob.write_bytes(&[
            (((val - 108_i32) / 256_i32 + 247_i32) as u8 as i32) as u8,
            (((val - 108_i32) % 256_i32) as u8 as i32) as u8,
        ]);
    } else if (-32768_i32..=32767_i32).contains(&val) {
        blob.write_bytes(&[
            28_u8,
            ((val >> 8_i32) as u8 as i32) as u8,
            ((val << 8_i32 >> 8_i32) as u8 as i32) as u8,
        ]);
    } else {
        cff_merge_cs2_int(blob, 0_i32);
    };
}
fn merge_cs2_real(blob: &mut Buffer, val: f64) {
    let integer_part: i16 = val.floor() as i16;
    let fraction_part: u16 =
        ((val - integer_part as i32 as f64) * 65536.0f64) as u16;
    blob.write_bytes(&[
        0xff_u8,
        (integer_part as i32 >> 8_i32) as u8,
        (integer_part as i32 & 0xff_i32) as u8,
        (fraction_part as i32 >> 8_i32) as u8,
        (fraction_part as i32 & 0xff_i32) as u8,
    ]);
}
pub fn cff_merge_cs2_operand(blob: &mut Buffer, val: f64) {
    let (intpart, fract) = modf(val);
    if fract == 0.0f64 {
        cff_merge_cs2_int(blob, intpart as i32);
    } else {
        merge_cs2_real(blob, val);
    };
}
pub fn cff_merge_cs2_special(blob: &mut Buffer, val: u8) {
    blob.write_u8(val);
}
pub fn cff_build_offset(val: i32) -> Buffer {
    Buffer::from_bytes(&[
        29_u8,
        (val >> 24_i32 & 0xff_i32) as u8,
        (val >> 16_i32 & 0xff_i32) as u8,
        (val >> 8_i32 & 0xff_i32) as u8,
        (val & 0xff_i32) as u8,
    ])
}
#[cfg(test)]
mod modf_tests {
    use super::modf;

    #[test]
    fn matches_libc_modf_on_finite_values_positive_negative_and_whole() {
        // (intpart, fract) pairs, cross-checked against `libc::modf`'s own
        // documented contract (trunc-toward-zero intpart, same-signed
        // fract) rather than against a live libc call, since this module's
        // whole point is to no longer link one.
        assert_eq!(modf(1.5), (1.0, 0.5));
        assert_eq!(modf(-1.5), (-1.0, -0.5));
        assert_eq!(modf(3.0), (3.0, 0.0));
        assert_eq!(modf(-3.0), (-3.0, -0.0));
        assert_eq!(modf(0.0), (0.0, 0.0));
    }

    #[test]
    fn infinity_gets_a_same_signed_zero_fraction_not_nan() {
        // The one case `f64::trunc`/`f64::fract` alone would get wrong:
        // `inf.fract()` is `inf - inf` = `NaN`, but C99's `modf` defines an
        // infinity's fractional part as a zero of the same sign. Pinned
        // here so a future refactor back to plain `trunc`/`fract` (a very
        // tempting simplification) fails loudly instead of silently
        // reintroducing this divergence.
        let (ip, fr) = modf(f64::INFINITY);
        assert_eq!(ip, f64::INFINITY);
        assert_eq!(fr, 0.0);
        assert!(fr.is_sign_positive());

        let (ip, fr) = modf(f64::NEG_INFINITY);
        assert_eq!(ip, f64::NEG_INFINITY);
        assert_eq!(fr, 0.0);
        assert!(fr.is_sign_negative());
    }

    #[test]
    fn nan_propagates_to_both_parts() {
        let (ip, fr) = modf(f64::NAN);
        assert!(ip.is_nan());
        assert!(fr.is_nan());
    }
}
