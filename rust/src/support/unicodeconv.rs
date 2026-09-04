#![forbid(unsafe_code)]
// `utf16be_to_utf8`/`utf8toutf16be` return `Vec<u8>` now instead of
// `SdsRaw`, each with its only caller (`table/name.rs`) a direct Rust
// call site (never a real FFI boundary) -- same rationale as every other
// instance of `#[allow(improper_ctypes_definitions)]` in this crate.

/// Decodes big-endian UTF-16 (`name` table string data, per its own
/// on-the-wire encoding) to UTF-8. An unpaired trailing byte is dropped
/// silently (matches the original's `inlenb -= 1` truncation); a high
/// surrogate not followed by a matching low surrogate is encoded as-is (a
/// 3-byte UTF-8 sequence for the raw, technically-invalid surrogate
/// value) rather than rejected -- preserved verbatim from the C-derived
/// original, not a deliberate design choice being made here.
#[allow(improper_ctypes_definitions)]
pub fn utf16be_to_utf8(inb: &[u8]) -> Vec<u8> {
    let inlen = inb.len() & !1;
    let inb = &inb[..inlen];
    let read_u16 = |i: usize| u16::from_be_bytes([inb[i], inb[i + 1]]) as u32;
    let mut out: Vec<u8> = Vec::with_capacity(inlen);
    let mut i = 0;
    while i < inlen {
        let mut c = read_u16(i);
        i += 2;
        if c & 0xfc00 == 0xd800 {
            if i >= inlen {
                break;
            }
            let d = read_u16(i);
            i += 2;
            if d & 0xfc00 == 0xdc00 {
                c = ((c & 0x3ff) << 10 | (d & 0x3ff)).wrapping_add(0x10000);
            }
        }
        let bits_start;
        if c < 0x80 {
            out.push(c as u8);
            bits_start = -6_i32;
        } else if c < 0x800 {
            out.push((c >> 6 & 0x1f | 0xc0) as u8);
            bits_start = 0;
        } else if c < 0x10000 {
            out.push((c >> 12 & 0xf | 0xe0) as u8);
            bits_start = 6;
        } else {
            out.push((c >> 18 & 0x7 | 0xf0) as u8);
            bits_start = 12;
        }
        let mut bits = bits_start;
        while bits >= 0 {
            out.push((c >> bits & 0x3f | 0x80) as u8);
            bits -= 6;
        }
    }
    out
}
/// Encodes UTF-8 to big-endian UTF-16 (the `name` table's own on-the-wire
/// string encoding). Malformed UTF-8 truncates the output at the point of
/// failure (a lead byte outside `0x80..0xf8`'s valid ranges, or a
/// continuation byte that doesn't match `0x80..0xc0`, or not enough bytes
/// left for a multi-byte sequence's declared length) rather than
/// substituting a replacement character -- preserved verbatim from the
/// C-derived original, not a deliberate design choice being made here.
/// Code points at or above `0x110000` (out of Unicode's range) are
/// silently dropped, contributing zero bytes to the output.
#[allow(improper_ctypes_definitions)]
pub fn utf8toutf16be(_in: &[u8]) -> Vec<u8> {
    let inlen = _in.len();
    let mut out: Vec<u8> = Vec::with_capacity(inlen * 2);
    let mut i = 0;
    while i < inlen {
        let d = _in[i];
        i += 1;
        let mut c: u32;
        let mut trailing: u8;
        if (d as i32) < 0x80 {
            c = d as u32;
            trailing = 0;
        } else if (d as i32) < 0xc0 {
            break;
        } else if (d as i32) < 0xe0 {
            c = (d as u32) & 0x1f;
            trailing = 1;
        } else if (d as i32) < 0xf0 {
            c = (d as u32) & 0xf;
            trailing = 2;
        } else if (d as i32) < 0xf8 {
            c = (d as u32) & 0x7;
            trailing = 3;
        } else {
            break;
        }
        if inlen - i < trailing as usize {
            break;
        }
        while trailing != 0 {
            if i >= inlen {
                break;
            }
            let d = _in[i];
            i += 1;
            if (d as i32) & 0xc0 != 0x80 {
                break;
            }
            c = c << 6 | (d as u32 & 0x3f);
            trailing -= 1;
        }
        if c < 0x10000 {
            out.push((c >> 8 & 0xff) as u8);
            out.push((c & 0xff) as u8);
        } else if c < 0x110000 {
            let tmp1 = (0xd800_u32 | c >> 10) as u16;
            out.push((tmp1 >> 8) as u8);
            out.push((tmp1 & 0xff) as u8);
            let tmp2 = (0xdc00_u32 | c & 0x3ff) as u16;
            out.push((tmp2 >> 8) as u8);
            out.push((tmp2 & 0xff) as u8);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16be_to_utf8_decodes_ascii() {
        assert_eq!(utf16be_to_utf8(&[0x00, 0x41, 0x00, 0x42]), b"AB");
    }

    #[test]
    fn utf16be_to_utf8_decodes_a_bmp_character() {
        // U+00E9 (é), a 2-byte UTF-8 sequence.
        assert_eq!(utf16be_to_utf8(&[0x00, 0xe9]), [0xc3, 0xa9]);
    }

    #[test]
    fn utf16be_to_utf8_drops_a_trailing_odd_byte() {
        assert_eq!(utf16be_to_utf8(&[0x00, 0x41, 0xff]), b"A");
    }

    // Pins the existing surrogate-pair math verbatim (`0xd800 | c >> 10` /
    // `0xdc00 | c & 0x3ff` on the encode side, `+ 0x10000` after combining
    // the two halves on the decode side) rather than the textbook
    // `- 0x10000` / `+ 0x10000` inverse pair -- these two are *not*
    // mutual inverses for an astral code point (confirmed by hand:
    // encoding U+1F600 and decoding the result back does not return
    // U+1F600), which is a preexisting quirk of the C-derived formula,
    // not something introduced or fixed by this conversion. Both
    // directions are pinned independently against hand-computed
    // expected bytes for that reason, not cross-checked against each
    // other.
    #[test]
    fn utf16be_to_utf8_combines_a_well_formed_surrogate_pair() {
        // High surrogate 0xd87d, low surrogate 0xde00 (the exact pair
        // `utf8toutf16be_encodes_an_astral_character_via_a_surrogate_pair`
        // below produces for U+1F600) decodes to 3 bytes starting with
        // 0xf0 -- not U+1F600's own UTF-8 encoding, by the quirk above.
        assert_eq!(
            utf16be_to_utf8(&[0xd8, 0x7d, 0xde, 0x00]),
            [0xf0, 0xaf, 0x98, 0x80]
        );
    }

    #[test]
    fn utf16be_to_utf8_encodes_an_unpaired_high_surrogate_as_is() {
        // High surrogate 0xd800 followed by a non-surrogate unit (0x0041,
        // "A") -- the low unit is still consumed (never re-examined as
        // its own character) but doesn't change `c`, so the raw
        // surrogate value itself gets encoded as a 3-byte sequence.
        assert_eq!(
            utf16be_to_utf8(&[0xd8, 0x00, 0x00, 0x41]),
            [0xed, 0xa0, 0x80]
        );
    }

    #[test]
    fn utf16be_to_utf8_drops_a_high_surrogate_truncated_at_eof() {
        assert_eq!(utf16be_to_utf8(&[0x00, 0x41, 0xd8, 0x00]), b"A");
    }

    #[test]
    fn utf8toutf16be_encodes_ascii() {
        assert_eq!(utf8toutf16be(b"AB"), [0x00, 0x41, 0x00, 0x42]);
    }

    #[test]
    fn utf8toutf16be_encodes_a_bmp_character() {
        assert_eq!(utf8toutf16be(&[0xc3, 0xa9]), [0x00, 0xe9]);
    }

    #[test]
    fn utf8toutf16be_encodes_an_astral_character_via_a_surrogate_pair() {
        // U+1F600 (😀), UTF-8 F0 9F 98 80.
        assert_eq!(
            utf8toutf16be(&[0xf0, 0x9f, 0x98, 0x80]),
            [0xd8, 0x7d, 0xde, 0x00]
        );
    }

    #[test]
    fn utf8toutf16be_truncates_at_an_invalid_continuation_byte() {
        // A 2-byte lead (0xc3) followed by plain ASCII instead of a
        // continuation byte: the bad byte is still consumed, the partial
        // (garbage) code point accumulated so far is still emitted, and
        // nothing further is read -- matches the original's fallthrough
        // exactly, not a deliberate validation choice made here.
        assert_eq!(utf8toutf16be(&[0xc3, 0x41]), [0x00, 0x03]);
    }

    #[test]
    fn utf8toutf16be_stops_when_not_enough_bytes_remain_for_the_sequence() {
        // A 3-byte lead (0xe2, declares 2 trailing bytes) with only 1
        // byte left in the input -- breaks the outer loop immediately,
        // emitting nothing.
        assert!(utf8toutf16be(&[0xe2, 0x82]).is_empty());
    }
}
