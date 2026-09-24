#![forbid(unsafe_code)]

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;

static BASE64_TABLE: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Standard RFC 4648 alphabet with `=` padding, no line wrapping -- exactly
/// what the hand-rolled encoder this replaced produced. A thin wrapper over
/// the `base64` crate's `STANDARD` engine: encoding has a single well-defined
/// output for any input (there's no "invalid" byte sequence to encode), so
/// there's no behavior for the crate to diverge on, unlike `base64_decode`
/// below.
pub fn base64_encode(src: &[u8]) -> Vec<u8> {
    STANDARD.encode(src).into_bytes()
}

/// `None` on malformed input (a count of base64 alphabet characters not a
/// multiple of 4) -- the original signaled this the same way malloc
/// failure was signaled, by returning a null pointer with the out-param
/// length left unset; every call site already treated that null return as
/// "no decoded value" (either substituting an empty buffer or, in
/// `table/meta/parse.rs`, skipping the JSON entry), so this is the same
/// outcome through a real `Option` instead of a null/uninitialized-length
/// pair. Bytes outside the base64 alphabet (and not `=`) are silently
/// skipped rather than rejected, matching the original.
///
/// Deliberately left hand-rolled rather than swapped to the `base64` crate
/// (unlike `base64_encode` above), because it has a second, more subtle
/// non-standard behavior the crate's `STANDARD` engine does not reproduce:
/// a `=` that appears anywhere other than in the final 4-character group is
/// *not* treated as padding here -- it looks up as data value `0` (the same
/// table slot as `'A'`) and is decoded into the output like any other
/// alphabet character, with truncation only checked against the very last
/// processed 4-byte group. E.g. `base64_decode(b"Z=g=")` returns
/// `Some(vec![100, 8])`, not `None` and not a 1-byte result. The `base64`
/// crate's decoders, including its most permissive `GeneralPurposeConfig`,
/// validate `=` position and reject a misplaced one instead. Since this
/// divergence only shows up on malformed/non-conformant input (this
/// encoder, and any RFC 4648-conformant one, never emits a `=` anywhere but
/// the final group), and this codebase treats byte-exact output as sacred,
/// this function stays hand-rolled rather than risk a silent behavior
/// change here; see `decode_treats_a_non_trailing_equals_sign_as_data_not_
/// padding` below, which pins the exact quirk.
pub fn base64_decode(src: &[u8]) -> Option<Vec<u8>> {
    let mut dtable = [0x80_u8; 256];
    for (i, &c) in BASE64_TABLE.iter().enumerate() {
        dtable[c as usize] = i as u8;
    }
    dtable[b'=' as usize] = 0;

    let count = src.iter().filter(|&&c| dtable[c as usize] != 0x80).count();
    if count % 4 != 0 {
        return None;
    }

    let mut out = Vec::with_capacity(count / 4 * 3);
    let mut in_block = [0u8; 4];
    let mut block = [0u8; 4];
    let mut n = 0usize;
    for &c in src {
        let tmp = dtable[c as usize];
        if tmp != 0x80 {
            in_block[n] = c;
            block[n] = tmp;
            n += 1;
            if n == 4 {
                out.push((block[0] << 2) | (block[1] >> 4));
                out.push((block[1] << 4) | (block[2] >> 2));
                out.push((block[2] << 6) | block[3]);
                n = 0;
            }
        }
    }
    if !out.is_empty() {
        if in_block[2] == b'=' {
            out.truncate(out.len() - 2);
        } else if in_block[3] == b'=' {
            out.truncate(out.len() - 1);
        }
    }
    Some(out)
}

#[cfg(test)]
mod base64_tests {
    use super::*;

    // RFC 4648 section 10 test vectors.
    #[test]
    fn encode_matches_rfc_4648_test_vectors() {
        assert_eq!(base64_encode(b""), b"");
        assert_eq!(base64_encode(b"f"), b"Zg==");
        assert_eq!(base64_encode(b"fo"), b"Zm8=");
        assert_eq!(base64_encode(b"foo"), b"Zm9v");
        assert_eq!(base64_encode(b"foob"), b"Zm9vYg==");
        assert_eq!(base64_encode(b"fooba"), b"Zm9vYmE=");
        assert_eq!(base64_encode(b"foobar"), b"Zm9vYmFy");
    }

    #[test]
    fn decode_matches_rfc_4648_test_vectors() {
        assert_eq!(base64_decode(b""), Some(b"".to_vec()));
        assert_eq!(base64_decode(b"Zg=="), Some(b"f".to_vec()));
        assert_eq!(base64_decode(b"Zm8="), Some(b"fo".to_vec()));
        assert_eq!(base64_decode(b"Zm9v"), Some(b"foo".to_vec()));
        assert_eq!(base64_decode(b"Zm9vYg=="), Some(b"foob".to_vec()));
        assert_eq!(base64_decode(b"Zm9vYmE="), Some(b"fooba".to_vec()));
        assert_eq!(base64_decode(b"Zm9vYmFy"), Some(b"foobar".to_vec()));
    }

    #[test]
    fn round_trip_is_stable_across_every_remainder_length() {
        for len in 0..40 {
            let data: Vec<u8> = (0..len).map(|i| (i * 37) as u8).collect();
            assert_eq!(base64_decode(&base64_encode(&data)), Some(data));
        }
    }

    #[test]
    fn decode_rejects_a_length_not_a_multiple_of_four() {
        // "Zg=" has 3 base64-alphabet characters (Z, g, and '=' both count,
        // per the original's dtable), not a multiple of 4.
        assert_eq!(base64_decode(b"Zg="), None);
    }

    #[test]
    fn decode_silently_skips_characters_outside_the_alphabet() {
        // A newline in the middle of an otherwise-valid encoding of "foo"
        // is dropped rather than rejected, matching the original's dtable
        // lookup (any byte that isn't a table entry or '=' reads as the
        // 0x80 sentinel and is excluded from both the count and the output).
        assert_eq!(base64_decode(b"Zm\n9v"), Some(b"foo".to_vec()));
    }

    // Adversarial cases added for Stage M-18 (the `base64` crate swap),
    // pinning old-vs-new equivalence on malformed/embedded-junk inputs now
    // that `base64_encode` goes through the crate and `base64_decode`
    // stays hand-rolled.

    #[test]
    fn decode_empty_input_is_empty_output() {
        assert_eq!(base64_decode(b""), Some(Vec::new()));
    }

    #[test]
    fn decode_skips_embedded_whitespace_of_several_kinds() {
        // Spaces, tabs, and carriage returns are all outside the alphabet
        // and outside '=', so all get filtered the same as the newline
        // case above.
        assert_eq!(base64_decode(b"Zm 9v"), Some(b"foo".to_vec()));
        assert_eq!(base64_decode(b"Zm\t9v"), Some(b"foo".to_vec()));
        assert_eq!(base64_decode(b"Zm\r\n9v"), Some(b"foo".to_vec()));
    }

    #[test]
    fn decode_rejects_wrong_padding_length() {
        // One base64-alphabet character short of a full group (after
        // filtering) is still rejected by the `count % 4 != 0` check.
        assert_eq!(base64_decode(b"Zm9"), None);
        assert_eq!(base64_decode(b"Z"), None);
    }

    #[test]
    fn decode_treats_a_non_trailing_equals_sign_as_data_not_padding() {
        // Pins the exact quirk documented on `base64_decode`: a `=` that
        // isn't in the final processed 4-byte group decodes as data value
        // 0 (same table slot as 'A'), not as padding, and truncation is
        // only checked against the *last* group's `=` positions. This is
        // why `base64_decode` was kept hand-rolled instead of routed
        // through the `base64` crate for Stage M-18 -- the crate's
        // decoders reject a misplaced '=' instead.
        assert_eq!(base64_decode(b"Z=g="), Some(vec![100, 8]));
        assert_eq!(base64_decode(b"===="), Some(vec![0]));
        assert_eq!(base64_decode(b"AA=A"), Some(vec![0]));
        assert_eq!(base64_decode(b"A=A="), Some(vec![0, 0]));
        // A stray '=' in a non-final group ("AA=A" decodes to a single
        // zero byte on its own, per above), followed by a well-formed
        // final group ("Zm9v" -> "foo"): no truncation happens because
        // in_block/block only retain the *last* processed group's values,
        // so all three of the first group's bytes survive.
        assert_eq!(base64_decode(b"AA=AZm9v"), Some(b"\x00\x00\x00foo".to_vec()));
    }

    #[test]
    fn encode_matches_the_base64_crate_directly() {
        // Cross-check `base64_encode`'s thin-wrapper output against the
        // crate it wraps, independent of the RFC test vectors above.
        use base64::Engine as _;
        for len in 0..40 {
            let data: Vec<u8> = (0..len).map(|i| (i * 53) as u8).collect();
            assert_eq!(
                base64_encode(&data),
                base64::engine::general_purpose::STANDARD.encode(&data).into_bytes()
            );
        }
    }
}
