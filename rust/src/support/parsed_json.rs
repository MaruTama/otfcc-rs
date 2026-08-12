//! Stage 6-2.5, C-2 (in progress): a safe, single-pass JSON parser and
//! tree, meant to eventually replace `vendor::json::JsonValue` for every
//! consumer that only *reads* a parsed value (the whole `table/*/parse.rs`
//! family, reached today through `support::json_funcs`'s C-1 accessor
//! layer).
//!
//! `vendor::json::JsonValue` stays exactly as it is and keeps serving the
//! build side (`vendor::json_builder`, every `table/*/dump.rs`, the
//! writer) -- this is a deliberately *separate* type, not a
//! representation change to the existing one. That's sound because the
//! two object graphs never intersect at runtime: the entire parse tree is
//! freed via `json_value_free` before any build tree is constructed, and
//! no dump-side code ever reads *through* a value the way parse-side code
//! does (dump-side code only ever calls `json_funcs::preserialize`/
//! `otfcc_dump_flags`, both pure build-side constructors that never read
//! an existing `JsonValue`'s payload).
//!
//! `vendor::json::json_parse_ex`'s real contract, as actually used in this
//! crate (`json_parse`, called from exactly two places -- `bin/
//! otfccbuild.rs` and `ffi/dll.rs` -- both just check the result for
//! null): always passes `error_buf = null` (parse-error text/position has
//! never been surfaced anywhere in this codebase) and
//! `settings.settings = 0` (`JSON_ENABLE_COMMENTS` unset, so comment
//! support is dead code in this crate's actual usage even though the
//! vendored parser implements it). That narrows what this replacement
//! needs to do: parse standard JSON, or don't -- no line/column tracking,
//! no comment syntax, no custom allocator hookup (moot once `Vec`/`Box`
//! own the memory).
//!
//! This module is not wired into anything yet -- `ParsedValue`/
//! `parse_json` exist and are validated by the differential test suite at
//! the bottom of this file (which parses the same bytes with both this
//! parser and `vendor::json::json_parse` and compares the resulting
//! trees), but no `table/*/parse.rs` consumer has been switched over.
//! That's the next PR, once this one has proven the new parser matches
//! byte-for-byte on real payloads and a battery of edge cases.

/// The parse-side JSON tree. String content is `Vec<u8>`, not `String`:
/// glyph names can be non-UTF-8 (Latin-1), a documented invariant carried
/// over unchanged from `JsonValue.u.string`.
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedValue {
    Null,
    Bool(bool),
    Int(i64),
    Double(f64),
    Str(Vec<u8>),
    Array(Vec<ParsedValue>),
    /// `Vec`, not `HashMap`/`BTreeMap`: object member order and duplicate
    /// keys must survive verbatim -- a documented invariant carried over
    /// unchanged from `JsonObjectValue`/`JsonObjectEntry` (see
    /// `support::json_funcs`'s C-1 doc comments and `rust/README.md`).
    Object(Vec<(Vec<u8>, ParsedValue)>),
}

/// Parses `input` as JSON. `None` on any malformed input -- matches
/// `vendor::json::json_parse`'s contract exactly (see this module's doc
/// comment for why no error detail needs to be preserved).
pub fn parse_json(input: &[u8]) -> Option<ParsedValue> {
    let mut p = Parser { input, pos: 0 };
    p.skip_ws();
    let v = p.parse_value()?;
    p.skip_ws();
    if p.pos != p.input.len() {
        return None; // trailing garbage after the top-level value
    }
    Some(v)
}

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.pos += 1;
        Some(b)
    }

    fn expect(&mut self, b: u8) -> Option<()> {
        if self.bump()? == b { Some(()) } else { None }
    }

    /// Space, tab, CR, LF only -- matches the vendored parser's
    /// `32 | 9 | 13` no-op arm plus its `10` (newline) arm (which only
    /// additionally tracks line/column, irrelevant here since this
    /// parser reports no position).
    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            self.pos += 1;
        }
    }

    fn parse_value(&mut self) -> Option<ParsedValue> {
        match self.peek()? {
            b'{' => self.parse_object(),
            b'[' => self.parse_array(),
            b'"' => self.parse_string().map(ParsedValue::Str),
            b't' => {
                self.expect_literal(b"true")?;
                Some(ParsedValue::Bool(true))
            }
            b'f' => {
                self.expect_literal(b"false")?;
                Some(ParsedValue::Bool(false))
            }
            b'n' => {
                self.expect_literal(b"null")?;
                Some(ParsedValue::Null)
            }
            b'-' | b'0'..=b'9' => self.parse_number(),
            _ => None,
        }
    }

    fn expect_literal(&mut self, lit: &[u8]) -> Option<()> {
        if self.input[self.pos..].starts_with(lit) {
            self.pos += lit.len();
            Some(())
        } else {
            None
        }
    }

    fn parse_object(&mut self) -> Option<ParsedValue> {
        self.expect(b'{')?;
        let mut fields = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Some(ParsedValue::Object(fields));
        }
        loop {
            self.skip_ws();
            if self.peek() != Some(b'"') {
                return None;
            }
            let key = self.parse_string()?;
            self.skip_ws();
            self.expect(b':')?;
            self.skip_ws();
            let value = self.parse_value()?;
            fields.push((key, value));
            self.skip_ws();
            match self.bump()? {
                b',' => {
                    // A single trailing comma before '}' is tolerated --
                    // matches vendor::json::json.rs's leniency, confirmed
                    // empirically (`json_parse(b"{\"a\":1,}")` succeeds). A
                    // second consecutive comma, or a comma before the
                    // first member, is still rejected (confirmed
                    // separately: both make the vendored parser return
                    // null too).
                    self.skip_ws();
                    if self.peek() == Some(b'}') {
                        self.pos += 1;
                        break;
                    }
                    continue;
                }
                b'}' => break,
                _ => return None,
            }
        }
        Some(ParsedValue::Object(fields))
    }

    fn parse_array(&mut self) -> Option<ParsedValue> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.pos += 1;
            return Some(ParsedValue::Array(items));
        }
        loop {
            self.skip_ws();
            items.push(self.parse_value()?);
            self.skip_ws();
            match self.bump()? {
                b',' => {
                    // Single trailing comma tolerated -- see the matching
                    // comment in `parse_object`.
                    self.skip_ws();
                    if self.peek() == Some(b']') {
                        self.pos += 1;
                        break;
                    }
                    continue;
                }
                b']' => break,
                _ => return None,
            }
        }
        Some(ParsedValue::Array(items))
    }

    /// Raw bytes pass through unescaped verbatim (non-UTF-8 preserved, the
    /// same invariant `JsonValue.u.string` carries). `\uXXXX` decodes to
    /// the UTF-8 encoding of the codepoint, combining UTF-16 surrogate
    /// pairs for astral characters -- matches `vendor::json::json.rs`'s
    /// `uc_b1..b4`/`0xf800`-masked surrogate handling.
    fn parse_string(&mut self) -> Option<Vec<u8>> {
        self.expect(b'"')?;
        let mut out = Vec::new();
        loop {
            let b = self.bump()?;
            match b {
                b'"' => break,
                b'\\' => {
                    let esc = self.bump()?;
                    match esc {
                        b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'),
                        b'/' => out.push(b'/'),
                        b'b' => out.push(0x08),
                        b'f' => out.push(0x0c),
                        b'n' => out.push(b'\n'),
                        b'r' => out.push(b'\r'),
                        b't' => out.push(b'\t'),
                        b'u' => {
                            let mut cp = self.parse_hex4()? as u32;
                            if (0xd800..=0xdbff).contains(&cp) {
                                // High surrogate: a low surrogate must follow.
                                self.expect(b'\\')?;
                                self.expect(b'u')?;
                                let lo = self.parse_hex4()? as u32;
                                if !(0xdc00..=0xdfff).contains(&lo) {
                                    return None;
                                }
                                cp = 0x10000 + ((cp - 0xd800) << 10) + (lo - 0xdc00);
                            }
                            let ch = char::from_u32(cp)?;
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        _ => return None,
                    }
                }
                0 => return None, // NUL: treated as EOF-in-string by the vendored parser
                _ => out.push(b),
            }
        }
        Some(out)
    }

    fn parse_hex4(&mut self) -> Option<u16> {
        let mut v: u16 = 0;
        for _ in 0..4 {
            let b = self.bump()?;
            let digit = match b {
                b'0'..=b'9' => b - b'0',
                b'a'..=b'f' => b - b'a' + 10,
                b'A'..=b'F' => b - b'A' + 10,
                _ => return None,
            };
            v = v.wrapping_mul(16).wrapping_add(digit as u16);
        }
        Some(v)
    }

    /// Integer vs `Double` is a *syntactic* decision, not a magnitude-based
    /// one: presence of `.` or `e`/`E` anywhere in the literal makes it
    /// `Double` -- matches `vendor::json::json.rs`'s `FLAG_NUM_E`-gated
    /// dispatch exactly, including the same
    /// `int_part as f64 + fraction / 10^digits`, then `*= 10^exponent`
    /// assembly (via `f64::powf`, not a decimal string parse) so the
    /// rounding matches bit-for-bit rather than just numerically.
    /// Leading zeros (`01`, `00`) are rejected, matching the vendored
    /// parser's `Unexpected '0' before ...` error. Integer accumulation
    /// wraps silently on overflow, matching the vendored parser's bare
    /// `integer = integer * 10 + digit` (no overflow check).
    fn parse_number(&mut self) -> Option<ParsedValue> {
        let negative = if self.peek() == Some(b'-') {
            self.pos += 1;
            true
        } else {
            false
        };
        let mut int_val: i64 = 0;
        let mut digits = 0u32;
        let mut leading_zero = false;
        while let Some(b @ b'0'..=b'9') = self.peek() {
            if digits == 1 && leading_zero {
                return None; // "01", "00", etc.
            }
            if digits == 0 && b == b'0' {
                leading_zero = true;
            }
            int_val = int_val.wrapping_mul(10).wrapping_add((b - b'0') as i64);
            digits += 1;
            self.pos += 1;
        }
        // NOTE: unlike the fraction/exponent digit counts below, the
        // vendored parser has no "at least one digit" check on the
        // integer part itself -- confirmed empirically: `json_parse` on
        // `{"v":-}` succeeds and produces `Int(0)` (the value's `.integer`
        // starts calloc'd-zero and a bare `-` never gets any digit added
        // to it). So a lone `-` is accepted here too, not rejected.

        let mut is_double = false;
        let mut dbl = 0.0f64;

        if self.peek() == Some(b'.') {
            is_double = true;
            dbl = int_val as f64;
            self.pos += 1;
            let mut frac: i64 = 0;
            let mut frac_digits = 0u32;
            while let Some(b @ b'0'..=b'9') = self.peek() {
                frac = frac.wrapping_mul(10).wrapping_add((b - b'0') as i64);
                frac_digits += 1;
                self.pos += 1;
            }
            if frac_digits == 0 {
                return None; // "5." with no digit after the dot
            }
            dbl += frac as f64 / 10.0f64.powf(frac_digits as f64);
        }

        if matches!(self.peek(), Some(b'e' | b'E')) {
            if !is_double {
                is_double = true;
                dbl = int_val as f64;
            }
            self.pos += 1;
            let exp_negative = match self.peek() {
                Some(b'-') => {
                    self.pos += 1;
                    true
                }
                Some(b'+') => {
                    self.pos += 1;
                    false
                }
                _ => false,
            };
            let mut exp: i64 = 0;
            let mut exp_digits = 0u32;
            while let Some(b @ b'0'..=b'9') = self.peek() {
                exp = exp.wrapping_mul(10).wrapping_add((b - b'0') as i64);
                exp_digits += 1;
                self.pos += 1;
            }
            if exp_digits == 0 {
                return None; // "5e" with no digit after the e
            }
            let signed_exp = if exp_negative { -exp } else { exp };
            dbl *= 10.0f64.powf(signed_exp as f64);
        }

        if is_double {
            Some(ParsedValue::Double(if negative { -dbl } else { dbl }))
        } else {
            Some(ParsedValue::Int(if negative { -int_val } else { int_val }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::json_funcs::{
        json_arr_at, json_arr_len, json_bool_val, json_dbl_val, json_int_val, json_obj_key_at,
        json_obj_key_len_at, json_obj_len, json_obj_val_at, json_str_len, json_str_ptr,
    };
    use crate::vendor::json::{JsonType, JsonValue, json_parse, json_value_free};

    /// Structural equality between the old (union-based) tree and the new
    /// (enum-based) one -- object member order and duplicate keys must
    /// match positionally, not just as a multiset.
    unsafe fn trees_equal(old: *const JsonValue, new: &ParsedValue) -> bool {
        unsafe {
            match new {
                ParsedValue::Null => (*old).type_0 == JsonType::Null,
                ParsedValue::Bool(b) => {
                    (*old).type_0 == JsonType::Boolean && json_bool_val(old) == *b
                }
                ParsedValue::Int(i) => {
                    (*old).type_0 == JsonType::Integer && json_int_val(old) == *i
                }
                ParsedValue::Double(d) => {
                    (*old).type_0 == JsonType::Double
                        && (json_dbl_val(old) == *d
                            || (json_dbl_val(old).is_nan() && d.is_nan()))
                }
                ParsedValue::Str(s) => {
                    (*old).type_0 == JsonType::String
                        && ::core::slice::from_raw_parts(
                            json_str_ptr(old) as *const u8,
                            json_str_len(old) as usize,
                        ) == s.as_slice()
                }
                ParsedValue::Array(items) => {
                    (*old).type_0 == JsonType::Array
                        && json_arr_len(old) as usize == items.len()
                        && items
                            .iter()
                            .enumerate()
                            .all(|(i, v)| trees_equal(json_arr_at(old, i as u32), v))
                }
                ParsedValue::Object(fields) => {
                    (*old).type_0 == JsonType::Object
                        && json_obj_len(old) as usize == fields.len()
                        && fields.iter().enumerate().all(|(i, (k, v))| {
                            let old_key = ::core::slice::from_raw_parts(
                                json_obj_key_at(old, i as u32) as *const u8,
                                json_obj_key_len_at(old, i as u32) as usize,
                            );
                            old_key == k.as_slice()
                                && trees_equal(json_obj_val_at(old, i as u32), v)
                        })
                }
            }
        }
    }

    fn check(bytes: &[u8]) {
        unsafe {
            let old_root = json_parse(bytes.as_ptr() as *const ::core::ffi::c_char, bytes.len());
            let new_root = parse_json(bytes);
            match (old_root.is_null(), &new_root) {
                (true, None) => {} // both correctly rejected
                (false, Some(new)) => {
                    assert!(
                        trees_equal(old_root, new),
                        "tree mismatch for {:?}: new = {:?}",
                        String::from_utf8_lossy(bytes),
                        new
                    );
                    json_value_free(old_root);
                }
                (true, Some(new)) => panic!(
                    "old parser rejected but new parser accepted {:?} as {:?}",
                    String::from_utf8_lossy(bytes),
                    new
                ),
                (false, None) => {
                    json_value_free(old_root);
                    panic!(
                        "old parser accepted but new parser rejected {:?}",
                        String::from_utf8_lossy(bytes)
                    );
                }
            }
        }
    }

    #[test]
    fn every_committed_payload_json_matches() {
        let mut any = false;
        for dir in ["../tests/payload", "../build"] {
            let Ok(entries) = std::fs::read_dir(dir) else { continue };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    let bytes = std::fs::read(&path).unwrap();
                    check(&bytes);
                    any = true;
                }
            }
        }
        assert!(any, "no .json payload files found to differential-test against");
    }

    #[test]
    fn number_edge_cases_match() {
        for s in [
            "5", "5.0", "5e0", "5E2", "-0", "-0.0", "0", "-5",
            "1e309",                    // overflow-to-infinity
            "-1e309",                   // overflow-to-negative-infinity
            "9223372036854775807",      // i64::MAX
            // NOTE: literals beyond i64::MAX with no '.'/'e' (still
            // syntactically Integer) are deliberately not covered here --
            // the *vendored* parser's `integer = integer * 10 + digit`
            // overflows i64 with no guard, which panics under Rust's
            // debug-build overflow checks (a pre-existing latent bug in
            // src/vendor/json.rs, not something this new parser
            // introduces or needs to reproduce the crash of). Flagged
            // separately; out of scope for this differential suite, whose
            // job is comparing two parsers' *results*, not exercising the
            // old one's crash bugs.
            "0.1", "1.7976931348623157e308",
            "123.456e-7", "1e-300", "0.0000001",
        ] {
            check(format!(r#"{{"v":{s}}}"#).as_bytes());
        }
    }

    #[test]
    fn number_syntax_errors_match() {
        for s in ["01", "00", "00.5", "-01", "5.", "5e", "5.e2", ".5", "+5"] {
            check(format!(r#"{{"v":{s}}}"#).as_bytes());
        }
    }

    #[test]
    fn string_edge_cases_match() {
        check(b"{\"a\":\"\\u00e9\\ud83d\\ude00\\t\\n\\\\\\\"\"}"); // \u escape + surrogate pair (é, 😀)
        check(&[b'{', b'"', b'a', b'"', b':', b'"', 0xff, 0xfe, b'"', b'}']); // raw non-UTF-8 bytes
        check(br#"{"a":"","b":[],"c":{}}"#); // empty containers
        check(br#"{"dup":1,"dup":2}"#); // duplicate keys, order preserved
        check(br#"[true,false,null,1,1.5,"s"]"#);
        check(br#"{"nested":{"a":[1,{"b":2}]}}"#);
    }

    /// Both single-trailing-comma tolerance (before `]`/`}`) and a bare
    /// `-` value (accepted as `Int(0)`, since the vendored parser's number
    /// has no "at least one digit" check on the integer part -- only the
    /// fraction and exponent parts do) are genuine leniency quirks of
    /// `vendor::json::json.rs`, confirmed empirically before this test was
    /// written (not assumptions -- see git history / PR description for
    /// the probe that found them). Matched here rather than treated as
    /// malformed input.
    #[test]
    fn parser_leniency_quirks_match() {
        for s in ["[1,2,]", r#"{"a":1,}"#, r#"{"v":-}"#, "[1,]", "[]", "{}"] {
            check(s.as_bytes());
        }
    }

    #[test]
    fn malformed_input_rejected_the_same_way() {
        for bad in [
            "{", r#"{"a":}"#, r#""unterminated"#, "nul", "{,}", "[1 2]",
            r#"{"a" "b"}"#, "", "   ", "[1,2,,]", "[,1]", "[,]",
        ] {
            check(bad.as_bytes());
        }
    }
}
