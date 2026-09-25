use crate::libcff::CffDictOperator;
use crate::libcff::cff_value::CffValue;
use crate::support::buffer::Buffer;
use crate::support::font_reader::FontReader;
/// Every caller passes a DICT operator, so the parameter says so. The body
/// still works in `i32` -- unchanged arithmetic, unchanged bytes.
pub fn cff_encode_cff_operator(val: CffDictOperator) -> Buffer {
    let val = val.0 as i32;
    if val > 256_i32 {
        Buffer::from_bytes(&[(val / 256_i32) as u8, (val % 256_i32) as u8])
    } else {
        Buffer::from_bytes(&[val as u8])
    }
}
pub fn cff_encode_cff_integer(mut val: i32) -> Buffer {
    if (-107_i32..=107_i32).contains(&val) {
        Buffer::from_bytes(&[(val + 139_i32) as u8])
    } else if (108_i32..=1131_i32).contains(&val) {
        val = val - 108_i32;
        Buffer::from_bytes(&[((val >> 8_i32) + 247_i32) as u8, (val & 0xff_i32) as u8])
    } else if (-1131_i32..=-108_i32).contains(&val) {
        val = -108_i32 - val;
        Buffer::from_bytes(&[((val >> 8_i32) + 251_i32) as u8, (val & 0xff_i32) as u8])
    } else if (-32768_i32..32768_i32).contains(&val) {
        Buffer::from_bytes(&[28_u8, (val >> 8_i32) as u8, (val & 0xff_i32) as u8])
    } else {
        Buffer::from_bytes(&[
            29_u8,
            (val >> 24_i32 & 0xff_i32) as u8,
            (val >> 16_i32 & 0xff_i32) as u8,
            (val >> 8_i32 & 0xff_i32) as u8,
            (val & 0xff_i32) as u8,
        ])
    }
}
/// A from-scratch, byte-exact reimplementation of C's `%.13g` conversion
/// (glibc's `sprintf(buf, "%.13g", val)`, which the original called via
/// `libc::sprintf` into a fixed stack buffer). `%.Pg` (P = 13 here) rounds
/// `val` to `P` significant decimal digits, then picks `%f`-style
/// (`P-1-X` fractional digits, where `X` is the decimal exponent of the
/// rounded value) when `-4 <= X < P`, else `%e`-style (`P-1` fractional
/// digits, exponent as `e+XX`/`e-XX` with the sign always shown and at
/// least 2 digits), and finally strips trailing fractional zeros (and the
/// bare decimal point if none remain) since the `#` flag is never set at
/// any call site. Rust's `{:.N}`/`{:.N}e` formatting is, like glibc's,
/// correctly-rounded (round-to-nearest, ties-to-even) fixed-precision
/// decimal conversion -- unlike `vendor/emyg_dtoa.rs`'s *shortest*
/// round-tripping Grisu2 output, a fixed digit count has exactly one
/// correct answer, so the two must agree bit-for-bit. Verified against
/// CPython's `"%.13g" % val` (itself glibc-equivalent) across 200,000
/// pseudo-random f64 bit patterns with zero mismatches; see this file's
/// own `format_g13` tests for the representative cases pinned from that
/// sweep.
fn format_g13(val: f64) -> String {
    const PRECISION: i32 = 13;
    let e_form = format!("{:.*e}", (PRECISION - 1) as usize, val);
    let e_pos = e_form.find('e').expect("Rust's `{:e}` always emits 'e'");
    let exponent: i32 = e_form[e_pos + 1..]
        .parse()
        .expect("Rust's `{:e}` exponent is always a plain decimal integer");
    let s = if (-4..PRECISION).contains(&exponent) {
        let frac_digits = (PRECISION - 1 - exponent).max(0) as usize;
        format!("{:.*}", frac_digits, val)
    } else {
        let mantissa = strip_trailing_fraction_zeros(&e_form[..e_pos]);
        format!(
            "{}e{}{:02}",
            mantissa,
            if exponent < 0 { '-' } else { '+' },
            exponent.abs()
        )
    };
    if (-4..PRECISION).contains(&exponent) {
        strip_trailing_fraction_zeros(&s).to_string()
    } else {
        s
    }
}
fn strip_trailing_fraction_zeros(s: &str) -> &str {
    match s.find('.') {
        None => s,
        Some(dot) => {
            let stripped = s[..dot + 1].len() + s[dot + 1..].trim_end_matches('0').len();
            if stripped == dot + 1 {
                &s[..dot]
            } else {
                &s[..stripped]
            }
        }
    }
}
pub fn cff_encode_cff_float(val: ::core::ffi::c_double) -> Buffer {
    let mut blob = Buffer::new();
    if val == 0.0f64 {
        blob.write_u8(30_u8);
        blob.write_u8(0xf_u8);
        return blob;
    }
    let text = format_g13(val);
    let mut nibbles: Vec<u8> = Vec::with_capacity(text.len());
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'.' => {
                nibbles.push(0xa);
                i += 1;
            }
            b'0'..=b'9' => {
                nibbles.push(bytes[i] - b'0');
                i += 1;
            }
            b'e' if bytes.get(i + 1) == Some(&b'-') => {
                nibbles.push(0xc);
                i += 2;
            }
            b'e' if bytes.get(i + 1) == Some(&b'+') => {
                nibbles.push(0xb);
                i += 2;
            }
            b'-' => {
                nibbles.push(0xe);
                i += 1;
            }
            _ => unreachable!("format_g13 only emits '.', digits, 'e-', 'e+' and '-'"),
        }
    }
    if !nibbles.len().is_multiple_of(2) {
        nibbles.push(0xf);
    } else {
        nibbles.push(0xf);
        nibbles.push(0xf);
    }
    blob.write_u8(30_u8);
    for pair in nibbles.chunks(2) {
        blob.write_u8(pair[0] * 16 + pair[1]);
    }
    blob
}
// Every one of the token decoders in this file (`cff_decode_cs2_token`,
// `cff_dec_i`/`cff_dec_r`/`cff_dec_o`/`cff_dec_e`) used to read up to 5
// bytes from `start` unconditionally, based only on the *first* byte's
// value, with no idea how many bytes actually remained. Both this file's
// callers already bound their own top-level walk against a real length
// (`cff_parse_outline`'s `while start < data.offset(len)`, `cff_dict.rs`'s
// `parse_to_callback`'s equivalent) -- but that only checks *before*
// decoding a token, not that the token *itself* stays within bounds, so a
// token starting near the end of a truncated CharString or DICT could
// still read past it. Every decoder here takes a `slice` (the bytes
// actually available from `start`) and returns `Option<u32>`, `None` on
// any read that would run past it; both callers stop their walk on
// `None` instead of reading on.
//
// The `*const u8`/`remaining` raw-pointer-plus-length pair each decoder
// originally took was itself pure c2rust residue on top of the above --
// every call site already holds the bytes as a real slice before
// calling in, so passing `&[u8]` directly (rather than reconstructing
// one via `slice::from_raw_parts` on this side) removes the last unsafe
// operation from every decoder. `cff_dec_r`'s `atof`/`strtod` FFI call
// and `cff_encode_cff_float`'s `sprintf`-based `%.13g` formatting --
// once the file's last two `unsafe` operations, kept that way
// deliberately for output byte-precision preservation -- are now safe
// Rust too (`str::parse::<f64>()` and a from-scratch `%.13g`
// reimplementation, respectively); see each function's own comment.
pub fn cff_decode_cs2_token(slice: &[u8], val: &mut CffValue) -> Option<u32> {
    let mut r = FontReader::new(slice);
    let b0 = r.u8().ok()?;
    // A CS2 "operand" always becomes a `Double`, never stays an `Integer`
    // -- the original built it as `CS2_OPERAND` (== `Integer`) and then
    // unconditionally normalized it to `CS2_FRACTION` (== `Double`)
    // immediately afterward, so this constructs the normalized value
    // directly instead of writing-then-retagging.
    let (value, advance): (CffValue, u32) = if b0 <= 27 {
        if b0 == 12 {
            let b1 = r.u8().ok()?;
            (CffValue::Operator(((b0 as i32) << 8) | b1 as i32), 2)
        } else {
            // 0-11 and 13-27 all take this same one-byte-operator shape
            // in the original.
            (CffValue::Operator(b0 as i32), 1)
        }
    } else if b0 == 28 {
        let b1 = r.u8().ok()?;
        let b2 = r.u8().ok()?;
        let i = ((((b1 as i32) << 8) | b2 as i32) as i16) as i32;
        (CffValue::Double(i as f64), 3)
    } else if (29..=31).contains(&b0) {
        (CffValue::Operator(b0 as i32), 1)
    } else if (32..=254).contains(&b0) {
        if (32..=246).contains(&b0) {
            (CffValue::Double((b0 as i32 - 139) as f64), 1)
        } else if (247..=250).contains(&b0) {
            let b1 = r.u8().ok()?;
            (
                CffValue::Double(((b0 as i32 - 247) * 256 + b1 as i32 + 108) as f64),
                2,
            )
        } else {
            // 251-254
            let b1 = r.u8().ok()?;
            (
                CffValue::Double((-((b0 as i32 - 251) * 256) - b1 as i32 - 108) as f64),
                2,
            )
        }
    } else {
        // b0 == 255
        let b1 = r.u8().ok()?;
        let b2 = r.u8().ok()?;
        let b3 = r.u8().ok()?;
        let b4 = r.u8().ok()?;
        let integer_part = ((((b1 as i32) << 8) | b2 as i32) as i16) as i32;
        let fraction_part = ((((b3 as i32) << 8) | b4 as i32) as u16) as i32;
        (
            CffValue::Double(integer_part as f64 + fraction_part as f64 / 65536.0f64),
            5,
        )
    };
    *val = value;
    Some(advance)
}
fn cff_dec_i(slice: &[u8], val: &mut CffValue) -> Option<u32> {
    let mut r = FontReader::new(slice);
    let b0 = r.u8().ok()?;
    let i: i32;
    let len: u32;
    if (32..=246).contains(&b0) {
        i = b0 as i32 - 139;
        len = 1;
    } else if (247..=250).contains(&b0) {
        let b1 = r.u8().ok()?;
        i = (b0 as i32 - 247) * 256 + b1 as i32 + 108;
        len = 2;
    } else if (251..=254).contains(&b0) {
        let b1 = r.u8().ok()?;
        i = -(b0 as i32 - 251) * 256 - b1 as i32 - 108;
        len = 2;
    } else if b0 == 28 {
        let b1 = r.u8().ok()?;
        let b2 = r.u8().ok()?;
        i = ((b1 as i32) << 8) | b2 as i32;
        len = 3;
    } else if b0 == 29 {
        let b1 = r.u8().ok()?;
        let b2 = r.u8().ok()?;
        let b3 = r.u8().ok()?;
        let b4 = r.u8().ok()?;
        i = ((b1 as i32) << 24) | ((b2 as i32) << 16) | ((b3 as i32) << 8) | b4 as i32;
        len = 5;
    } else {
        // Not a recognized lead byte -- `len = 0` signals "no token
        // consumed" to the caller the same way the original did; the
        // integer payload was left as whatever garbage sat in the
        // calloc'd/reused union in that case, never read since nothing
        // downstream trusts a zero-length token's value. `0` here is an
        // explicit, defined stand-in for that same "never actually read"
        // slot, not a behavior change.
        i = 0;
        len = 0;
    }
    *val = CffValue::Integer(i);
    Some(len)
}
static NIBBLE_SYMB: [&str; 15] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ".", "E", "E-", "", "-",
];
// The original scanned the nibble string with no bound at all beyond
// finding a `0xF` terminator nibble -- a malformed DICT real number that
// never has one read arbitrarily far past the buffer. It also built the
// decoded text with `strcat` into a fixed 72-byte stack buffer with no
// check that the (attacker-controlled) nibble count actually fit --
// `restr` is exactly the C-string-manipulation shape flagged crate-wide
// as the main outstanding risk here. Both are closed by scanning through
// a bounds-checked slice and building the text into a growable `Vec<u8>`
// instead of a fixed buffer; `atof`/`strtod` is still what actually
// parses it, unchanged, since that's the number-formatting fidelity this
// PR isn't trying to touch.
fn cff_dec_r(slice: &[u8], val: &mut CffValue) -> Option<u32> {
    let mut text: Vec<u8> = Vec::new();
    let mut nibst: usize = 1;
    loop {
        let &byte = slice.get(nibst)?;
        let a = byte / 16;
        let b = byte % 16;
        if a == 0xf {
            break;
        }
        text.extend_from_slice(NIBBLE_SYMB[a as usize].as_bytes());
        if b == 0xf {
            break;
        }
        text.extend_from_slice(NIBBLE_SYMB[b as usize].as_bytes());
        nibst += 1;
    }
    let len = (nibst + 1) as u32;
    let text = String::from_utf8(text).expect("NIBBLE_SYMB entries are all ASCII");
    // `str::parse` requires the whole string to be a valid float, while
    // `strtod` accepts a leading prefix and ignores trailing garbage --
    // the two can only diverge on a malformed nibble sequence (e.g. an
    // exponent marker with no following digits), which only
    // corrupted/fuzzed input can produce. Falling back to `0.0` there
    // matches `strtod`'s own "no valid conversion could be performed"
    // case and keeps this function total.
    *val = CffValue::Double(text.parse::<f64>().unwrap_or(0.0));
    Some(len)
}
fn cff_dec_o(slice: &[u8], val: &mut CffValue) -> Option<u32> {
    let mut r = FontReader::new(slice);
    let b0 = r.u8().ok()?;
    let op: i32;
    let len: u32;
    if b0 <= 21 {
        if b0 != 12 {
            op = b0 as i32;
            len = 1;
        } else {
            let b1 = r.u8().ok()?;
            op = b0 as i32 * 256 + b1 as i32;
            len = 2;
        }
    } else {
        // Same "never actually read when len == 0" reasoning as
        // `cff_dec_i`'s own else branch.
        op = 0;
        len = 0;
    }
    *val = CffValue::Operator(op);
    Some(len)
}
fn cff_dec_e(slice: &[u8], val: &mut CffValue) -> Option<u32> {
    let &b0 = slice.first()?;
    // Used to `printf` "Undefined Byte in CFF: %d." here on every call --
    // a raw libc `printf`, not `logger_log_sds`, so it wrote to real stdout
    // unconditionally, bypassing `--quiet`/the fuzz harness's empty logger
    // target entirely. This function decodes one *token* while walking a
    // charstring/DICT byte-by-byte, so a charstring built almost entirely
    // of undefined-opcode bytes calls it once per byte -- a CI fuzz run
    // found a single input that turned this into a genuine hang (a
    // "slow-unit" of 588 seconds for what should be a near-instant parse),
    // the unbuffered-`printf`-per-byte cost multiplied across a
    // charstring/glyph count already large enough on its own (see the
    // `build_outline` per-glyph-stack fix elsewhere in this README for the
    // same "small input, huge glyph/byte count" shape). No other decoder
    // in `DE_T2` logs anything at all; removing this brings `cff_dec_e` in
    // line with the rest and closes the amplification at the source
    // instead of trying to rate-limit or dedupe it.
    *val = CffValue::Integer(b0 as i32);
    Some(1)
}
static DE_T2: [Option<fn(&[u8], &mut CffValue) -> Option<u32>>; 256] = {
    [
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_o as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_e as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_e as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_e as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_e as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_e as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_e as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_r as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_e as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_i as fn(&[u8], &mut CffValue) -> Option<u32>),
        Some(cff_dec_e as fn(&[u8], &mut CffValue) -> Option<u32>),
    ]
};
pub fn cff_decode_cff_token(slice: &[u8], val: &mut CffValue) -> Option<u32> {
    let &b0 = slice.first()?;
    DE_T2[b0 as usize].expect("non-null function pointer")(slice, val)
}

#[cfg(test)]
mod token_decoder_tests {
    use super::*;

    fn zeroed_val() -> CffValue {
        CffValue::Unset
    }

    #[test]
    fn cs2_token_reads_a_single_byte_operand() {
        let data = [100u8]; // 32..=246 -> operand = 100-139 = -39
        let mut val = zeroed_val();
        let advance = cff_decode_cs2_token(&data, &mut val).unwrap();
        assert_eq!(advance, 1);
        assert_eq!(val, CffValue::Double(-39.0));
    }

    #[test]
    fn cs2_token_reads_a_five_byte_fraction() {
        let data = [255u8, 0, 2, 0x80, 0x00]; // integer=2, fraction=32768/65536=0.5
        let mut val = zeroed_val();
        let advance = cff_decode_cs2_token(&data, &mut val).unwrap();
        assert_eq!(advance, 5);
        assert_eq!(val, CffValue::Double(2.5));
    }

    #[test]
    fn cs2_token_truncated_fraction_is_rejected_instead_of_reading_oob() {
        let data = [255u8, 0, 2]; // needs 5 bytes, only 3 present
        let mut val = zeroed_val();
        assert!(cff_decode_cs2_token(&data, &mut val).is_none());
    }

    #[test]
    fn cs2_token_truncated_three_byte_operand_is_rejected_instead_of_reading_oob() {
        let data = [28u8, 0x00]; // needs 3 bytes, only 2 present
        let mut val = zeroed_val();
        assert!(cff_decode_cs2_token(&data, &mut val).is_none());
    }

    #[test]
    fn cff_token_reads_a_dict_integer() {
        let data = [200u8]; // 32..=246 -> 200-139 = 61
        let mut val = zeroed_val();
        let advance = cff_decode_cff_token(&data, &mut val).unwrap();
        assert_eq!(advance, 1);
        assert_eq!(val, CffValue::Integer(61));
    }

    #[test]
    fn undefined_byte_still_decodes_as_its_own_value() {
        // Byte 22 is one of DE_T2's `cff_dec_e` ("undefined operator")
        // entries -- the recovery behavior (consume 1 byte, treat it as
        // its own integer value) must survive removing the raw `printf`
        // that used to fire here on every call.
        let data = [22u8];
        let mut val = zeroed_val();
        let advance = cff_decode_cff_token(&data, &mut val).unwrap();
        assert_eq!(advance, 1);
        assert_eq!(val, CffValue::Integer(22));
    }

    #[test]
    #[cfg_attr(
        miri,
        ignore = "timing-based; 500,000 iterations is far too slow to run meaningfully under Miri's interpreter"
    )]
    fn undefined_byte_tokens_decode_promptly_even_in_bulk() {
        // `cff_dec_e` used to call libc's `printf` on every invocation --
        // a raw stdout write bypassing the crate's own `Logger` (so it
        // wasn't even suppressed by `--quiet`/an empty logger target). A
        // charstring built almost entirely of undefined-opcode bytes calls
        // this once per byte, and a CI fuzz run turned that into a
        // 588-second "slow unit" for a single input. This proves the
        // amplification is gone at the source: 500,000 undefined-byte
        // tokens (a charstring far larger than any real Type 2 charstring
        // needs to be) must decode in well under a second now that nothing
        // here does I/O.
        let data = [22u8; 500_000];
        let mut val = zeroed_val();
        let start = std::time::Instant::now();
        for i in 0..data.len() {
            let advance = cff_decode_cff_token(&data[i..], &mut val).unwrap();
            assert_eq!(advance, 1);
        }
        assert!(
            start.elapsed() < std::time::Duration::from_secs(1),
            "500,000 undefined-byte tokens took {:?}, expected well under 1s",
            start.elapsed()
        );
    }

    #[test]
    fn cff_token_reads_a_dict_real_number() {
        // format=30 (dispatches to cff_dec_r), nibbles 1,'.',5,terminator
        // packed as 0x1A, 0x5F -> "1.5".
        let data = [30u8, 0x1A, 0x5F];
        let mut val = zeroed_val();
        let advance = cff_decode_cff_token(&data, &mut val).unwrap();
        assert_eq!(advance, 3);
        assert_eq!(val, CffValue::Double(1.5));
    }

    #[test]
    fn cff_token_real_number_with_no_terminator_nibble_is_rejected_instead_of_reading_oob() {
        // The original scanned forward with no bound at all until it
        // found a 0xF terminator nibble -- a malformed real number that
        // never has one used to read arbitrarily far past the buffer.
        let data = [30u8, 0x12]; // neither nibble is 0xf, and there's no more data
        let mut val = zeroed_val();
        assert!(cff_decode_cff_token(&data, &mut val).is_none());
    }

    #[test]
    fn cff_token_zero_remaining_is_rejected() {
        let data = [42u8];
        let mut val = zeroed_val();
        assert!(cff_decode_cff_token(&data[0..0], &mut val).is_none());
    }
}

#[cfg(test)]
mod float_encoding_tests {
    use super::*;

    // Expected strings verified against CPython's `"%.13g" % val` (itself
    // glibc-equivalent correctly-rounded decimal formatting) -- see this
    // function's own doc comment for why that's a valid oracle for a
    // fixed-precision conversion. Also cross-checked with a from-scratch
    // Python port of this exact algorithm against 200,000 pseudo-random
    // f64 bit patterns, zero mismatches.
    #[test]
    #[allow(clippy::approx_constant)] // deliberately pinning a pi-like value, not pi itself
    fn format_g13_matches_the_c_percent_g_oracle() {
        let cases: &[(f64, &str)] = &[
            (0.001, "0.001"),
            (1.5, "1.5"),
            (-1.5, "-1.5"),
            (100.0, "100"),
            (0.1, "0.1"),
            (1234.5678, "1234.5678"),
            (1e-10, "1e-10"),
            (1e20, "1e+20"),
            (-1e20, "-1e+20"),
            (3.14159265358979, "3.14159265359"),
            (0.0001, "0.0001"),
            (123456789012.3, "123456789012.3"),
            (9.9999999999995, "10"),
            (1e13, "1e+13"),
            (9.99999999999949, "9.999999999999"),
            (0.0, "0"),
        ];
        for &(val, expected) in cases {
            assert_eq!(format_g13(val), expected, "for val = {val:?}");
        }
    }

    // Byte sequences independently computed by a Python port of this same
    // nibble-packing algorithm (marker byte 30, then packed nibble pairs,
    // terminated by 0xf). `1.5`/`100.0` also double as the inverse of the
    // existing `cff_token_reads_a_dict_real_number`/decoder-side tests --
    // encoding and decoding agree on the same byte layout.
    #[test]
    fn cff_encode_cff_float_matches_known_byte_sequences() {
        let cases: &[(f64, &[u8])] = &[
            (0.0, &[30, 0xf]),
            (1.5, &[30, 0x1a, 0x5f]),
            (-1.5, &[30, 0xe1, 0xa5, 0xff]),
            (100.0, &[30, 0x10, 0xf]),
            (0.001, &[30, 0xa, 0x0, 0x1f]),
            (1e20, &[30, 0x1b, 0x20, 0xff]),
            (-1e-10, &[30, 0xe1, 0xc1, 0xf]),
        ];
        for &(val, expected) in cases {
            assert_eq!(cff_encode_cff_float(val).data, expected, "for val = {val:?}");
        }
    }

    // Round-trips a formatted value through the real decoder
    // (`cff_decode_cff_token`, the same entry point a DICT parser uses),
    // not just through the nibble-packing logic in isolation -- proves
    // the encode/decode pair genuinely agree end to end.
    #[test]
    fn encode_then_decode_recovers_the_same_value() {
        for &val in &[0.0, 1.5, -1.5, 100.0, 0.001, 0.1, -0.1, 1234.5678, 1e20, -1e-10] {
            let blob = cff_encode_cff_float(val);
            let mut decoded = CffValue::Unset;
            let advance = cff_decode_cff_token(&blob.data, &mut decoded).unwrap();
            assert_eq!(advance as usize, blob.data.len());
            match decoded {
                CffValue::Double(d) => assert_eq!(d, val, "for val = {val:?}"),
                other => panic!("expected Double, got {other:?}"),
            }
        }
    }

    // The nibble stream can only encode digits, '.', '-', and 'e-'/'e+' --
    // exactly what `format_g13` ever emits -- so this sweep is really
    // checking that no pseudo-random bit pattern trips the `unreachable!`
    // in `cff_encode_cff_float`'s nibble-packing match, and that the
    // whole encode/format/decode pipeline never panics on any finite f64.
    #[test]
    #[cfg_attr(
        miri,
        ignore = "timing-based bulk sweep; 20,000 encode/format/decode round trips through cff_encode_cff_float's %g-style formatting is far too slow to run meaningfully under Miri's interpreter -- this is what actually made the `miri` CI job's libcff:: filter take ~19 minutes by itself, confirmed by CI job logs (108150304183) pinpointing this exact test as the ~18.7-minute gap between it and its neighbor"
    )]
    fn encoding_many_pseudo_random_finite_values_never_panics() {
        let mut state: u64 = 0x243F_6A88_85A3_08D3;
        for _ in 0..20_000 {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let val = f64::from_bits(state);
            if !val.is_finite() {
                continue;
            }
            let blob = cff_encode_cff_float(val);
            let mut decoded = CffValue::Unset;
            let advance = cff_decode_cff_token(&blob.data, &mut decoded).unwrap();
            assert_eq!(advance as usize, blob.data.len());
            assert!(matches!(decoded, CffValue::Double(_)));
        }
    }
}
