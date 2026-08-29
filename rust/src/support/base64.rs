#![forbid(unsafe_code)]

static BASE64_TABLE: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_encode(src: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(src.len().div_ceil(3) * 4);
    let mut chunks = src.chunks_exact(3);
    for chunk in &mut chunks {
        out.push(BASE64_TABLE[(chunk[0] >> 2) as usize]);
        out.push(BASE64_TABLE[(((chunk[0] & 0x3) << 4) | (chunk[1] >> 4)) as usize]);
        out.push(BASE64_TABLE[(((chunk[1] & 0xf) << 2) | (chunk[2] >> 6)) as usize]);
        out.push(BASE64_TABLE[(chunk[2] & 0x3f) as usize]);
    }
    let rem = chunks.remainder();
    if !rem.is_empty() {
        out.push(BASE64_TABLE[(rem[0] >> 2) as usize]);
        if rem.len() == 1 {
            out.push(BASE64_TABLE[((rem[0] & 0x3) << 4) as usize]);
        } else {
            out.push(BASE64_TABLE[(((rem[0] & 0x3) << 4) | (rem[1] >> 4)) as usize]);
            out.push(BASE64_TABLE[((rem[1] & 0xf) << 2) as usize]);
        }
        out.push(b'=');
        if rem.len() == 1 {
            out.push(b'=');
        }
    }
    out
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
}
