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
/// multiple of 4). Bytes outside the base64 alphabet (and not `=`) are
/// silently skipped rather than rejected.
///
/// Hand-rolled rather than using the `base64` crate (unlike `base64_encode`
/// above) because of a second non-standard behavior the crate does not
/// reproduce: a `=` anywhere other than in the final 4-character group is
/// *not* padding here -- it decodes as data value `0` (the same table slot
/// as `'A'`), with truncation only checked against the very last 4-byte
/// group. E.g. `base64_decode(b"Z=g=")` returns `Some(vec![100, 8])`. The
/// crate's decoders, including its most permissive `GeneralPurposeConfig`,
/// reject a misplaced `=` instead. This only shows up on non-conformant
/// input, but output must stay byte-identical; see
/// `decode_treats_a_non_trailing_equals_sign_as_data_not_padding` below,
/// which pins the exact quirk.
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
        // "Zg=" has 3 base64-alphabet characters (Z, g, and '=' all count),
        // not a multiple of 4.
        assert_eq!(base64_decode(b"Zg="), None);
    }

    #[test]
    fn decode_silently_skips_characters_outside_the_alphabet() {
        // A newline in the middle of an otherwise-valid encoding of "foo"
        // is dropped rather than rejected (any byte that isn't a table
        // entry or '=' reads as the 0x80 sentinel and is excluded from both
        // the count and the output).
        assert_eq!(base64_decode(b"Zm\n9v"), Some(b"foo".to_vec()));
    }

    // Malformed and embedded-junk inputs, pinning that `base64_encode` (the
    // crate) and `base64_decode` (hand-rolled) still round-trip as before.

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
        // only checked against the *last* group's `=` positions. The
        // `base64` crate's decoders reject a misplaced '=' instead.
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
