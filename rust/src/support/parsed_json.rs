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
///
/// **`Str`'s `Vec<u8>` and every object key carry one trailing NUL byte not
/// counted in their logical length** (added by `parse_string`/
/// `parse_object`, stripped back off by this module's `pj_str_len`/
/// `pj_obj_key_len_at` accessors) -- matching `JsonStringValue`/
/// `JsonObjectEntry.name`'s own C-string convention exactly. This exists
/// for one reason: dozens of call sites across the consumer files this
/// type is about to be wired into (`table/otl/parse.rs` alone has ~20)
/// take a parsed key or string value straight into `CStr::from_ptr(..)
/// .to_bytes().to_vec()` or `strcmp(..)`, both of which require a
/// NUL-terminated buffer. Baking the terminator into storage here means
/// every one of those call sites keeps working completely unchanged
/// instead of needing an individually-reasoned-about rewrite. Code
/// constructing or reading a `Str`/key through anything other than this
/// module's own parsing/accessor functions must account for the extra
/// byte -- nothing outside this module should ever match `Str`'s `Vec<u8>`
/// directly for that reason.
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
    /// Keys are NUL-terminated `Vec<u8>`s, same convention as `Str` above.
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
        // Trailing NUL, not counted in the logical length -- see
        // `ParsedValue`'s doc comment. Safe unconditionally: the `0 =>
        // return None` arm above already makes an embedded NUL byte
        // impossible in `out`, so this terminator is never ambiguous with
        // real content.
        out.push(0);
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

/// Raw-pointer entry point matching `vendor::json::json_parse`'s own
/// signature exactly (`*const c_char`/`usize` in, null on any parse
/// failure) -- lets `bin/otfccbuild.rs`/`ffi/dll.rs` swap the call site
/// without reshaping the surrounding code. Owns the result as a `Box`;
/// pair with `json_value_free` below.
pub unsafe fn json_parse(
    json: *const ::core::ffi::c_char,
    length: usize,
) -> *mut ParsedValue {
    let bytes = unsafe { ::core::slice::from_raw_parts(json as *const u8, length) };
    match parse_json(bytes) {
        Some(v) => Box::into_raw(Box::new(v)),
        None => ::core::ptr::null_mut(),
    }
}

/// Frees a tree returned by `json_parse` above. No-op on null, matching
/// `vendor::json::json_value_free`.
pub unsafe fn json_value_free(v: *mut ParsedValue) {
    if !v.is_null() {
        drop(unsafe { Box::from_raw(v) });
    }
}

// The accessor layer below mirrors `support::json_funcs`'s parse-side API
// (both its original 16 helpers and C-1's 11 additions) name-for-name,
// targeting `*const ParsedValue` instead of `*const vendor::json::
// JsonValue` -- the point being that a consumer file's call *expressions*
// (`json_obj_get_type(v, b"key\0"..., JsonType::Object)`, etc.) stay
// textually identical when it's switched over; only its `use` line and its
// own function signatures' parameter types change. Every accessor here is
// self-guarding (null/wrong-variant returns 0/null/false) exactly like its
// `json_funcs` counterpart, even though `ParsedValue` being a real Rust
// enum makes most of that redundant for direct pattern-matching -- kept
// for signature and behavioral parity with the layer this is replacing,
// since the 40-odd files this gets wired into next were written against
// that contract.
use crate::vendor::json::JsonType;

/// The `JsonType` tag for a value -- replaces every `(*v).type_0` read
/// (`vendor::json::JsonValue`'s type tag is a field; `ParsedValue`'s is
/// the enum discriminant itself, so there's nothing to read without a
/// function). `JsonType::None` for null.
pub unsafe fn json_type_of(v: *const ParsedValue) -> JsonType {
    if v.is_null() {
        return JsonType::None;
    }
    match unsafe { &*v } {
        ParsedValue::Null => JsonType::Null,
        ParsedValue::Bool(_) => JsonType::Boolean,
        ParsedValue::Int(_) => JsonType::Integer,
        ParsedValue::Double(_) => JsonType::Double,
        ParsedValue::Str(_) => JsonType::String,
        ParsedValue::Array(_) => JsonType::Array,
        ParsedValue::Object(_) => JsonType::Object,
    }
}

pub unsafe fn json_obj_len(obj: *const ParsedValue) -> u32 {
    match unsafe { obj.as_ref() } {
        Some(ParsedValue::Object(fields)) => fields.len() as u32,
        _ => 0,
    }
}

/// The `i`th member's raw NUL-terminated key pointer (see `ParsedValue`'s
/// doc comment on why every key is NUL-terminated); null out of range or
/// on a non-object.
pub unsafe fn json_obj_key_at(obj: *const ParsedValue, i: u32) -> *mut ::core::ffi::c_char {
    match unsafe { obj.as_ref() } {
        Some(ParsedValue::Object(fields)) => match fields.get(i as usize) {
            Some((k, _)) => k.as_ptr() as *mut ::core::ffi::c_char,
            None => ::core::ptr::null_mut(),
        },
        _ => ::core::ptr::null_mut(),
    }
}

/// The `i`th member's key length in bytes, excluding the trailing NUL; 0
/// out of range or on a non-object.
pub unsafe fn json_obj_key_len_at(obj: *const ParsedValue, i: u32) -> u32 {
    match unsafe { obj.as_ref() } {
        Some(ParsedValue::Object(fields)) => match fields.get(i as usize) {
            Some((k, _)) => (k.len() - 1) as u32,
            None => 0,
        },
        _ => 0,
    }
}

/// The `i`th member's value; null out of range or on a non-object.
pub unsafe fn json_obj_val_at(obj: *const ParsedValue, i: u32) -> *mut ParsedValue {
    match unsafe { obj.as_ref() } {
        Some(ParsedValue::Object(fields)) => match fields.get(i as usize) {
            Some((_, v)) => v as *const ParsedValue as *mut ParsedValue,
            None => ::core::ptr::null_mut(),
        },
        _ => ::core::ptr::null_mut(),
    }
}

/// Overwrites the `i`th member's value with `new_value`, dropping whatever
/// was there before. Replaces the old parser's `json_value_free` + build a
/// replacement + manual `.parent` splice pattern (used by `glyf.rs` to
/// release each glyph's parse subtree as it's consumed, and by
/// `table/otl/parse.rs`'s duplicate-feature/lookup merger to turn a
/// duplicate definition into an alias string) -- a plain assignment does
/// the same job here since `ParsedValue` owns its children. No-op if `obj`
/// isn't an object or `i` is out of range.
pub unsafe fn json_obj_set_val_at(obj: *mut ParsedValue, i: u32, new_value: ParsedValue) {
    if let Some(ParsedValue::Object(fields)) = unsafe { obj.as_mut() } {
        if let Some((_, v)) = fields.get_mut(i as usize) {
            *v = new_value;
        }
    }
}

/// Overwrites the `i`th member's value with `Null`. See
/// [`json_obj_set_val_at`].
pub unsafe fn json_obj_null_out_val_at(obj: *mut ParsedValue, i: u32) {
    unsafe { json_obj_set_val_at(obj, i, ParsedValue::Null) };
}

pub unsafe fn json_arr_len(arr: *const ParsedValue) -> u32 {
    match unsafe { arr.as_ref() } {
        Some(ParsedValue::Array(items)) => items.len() as u32,
        _ => 0,
    }
}

/// The `i`th array element; null out of range or on a non-array.
pub unsafe fn json_arr_at(arr: *const ParsedValue, i: u32) -> *mut ParsedValue {
    match unsafe { arr.as_ref() } {
        Some(ParsedValue::Array(items)) => match items.get(i as usize) {
            Some(v) => v as *const ParsedValue as *mut ParsedValue,
            None => ::core::ptr::null_mut(),
        },
        _ => ::core::ptr::null_mut(),
    }
}

/// A string value's raw NUL-terminated pointer; null for anything but a
/// string.
pub unsafe fn json_str_ptr(v: *const ParsedValue) -> *mut ::core::ffi::c_char {
    match unsafe { v.as_ref() } {
        Some(ParsedValue::Str(s)) => s.as_ptr() as *mut ::core::ffi::c_char,
        _ => ::core::ptr::null_mut(),
    }
}

/// A string value's length in bytes, excluding the trailing NUL; 0 for
/// anything but a string.
pub unsafe fn json_str_len(v: *const ParsedValue) -> u32 {
    match unsafe { v.as_ref() } {
        Some(ParsedValue::Str(s)) => (s.len() - 1) as u32,
        _ => 0,
    }
}

/// An integer value's raw `i64`; 0 for anything but `Int`.
pub unsafe fn json_int_val(v: *const ParsedValue) -> i64 {
    match unsafe { v.as_ref() } {
        Some(ParsedValue::Int(i)) => *i,
        _ => 0,
    }
}

/// A double value's raw `f64`; 0.0 for anything but `Double`.
pub unsafe fn json_dbl_val(v: *const ParsedValue) -> f64 {
    match unsafe { v.as_ref() } {
        Some(ParsedValue::Double(d)) => *d,
        _ => 0.0,
    }
}

/// A boolean value's raw `bool`; false for anything but `Bool`.
pub unsafe fn json_bool_val(v: *const ParsedValue) -> bool {
    match unsafe { v.as_ref() } {
        Some(ParsedValue::Bool(b)) => *b,
        _ => false,
    }
}

/// Look up `key` in a JSON object, of whatever type; null when there is no
/// such member (or `obj` is not an object). The first member whose name
/// matches wins, which matters because the parser keeps duplicate keys
/// rather than collapsing them -- matches `json_funcs::json_obj_get`.
pub unsafe fn json_obj_get(
    obj: *const ParsedValue,
    key: *const ::core::ffi::c_char,
) -> *mut ParsedValue {
    match unsafe { obj.as_ref() } {
        Some(ParsedValue::Object(fields)) => {
            for (k, v) in fields {
                if unsafe { ::libc::strcmp(k.as_ptr() as *const ::core::ffi::c_char, key) } == 0 {
                    return v as *const ParsedValue as *mut ParsedValue;
                }
            }
            ::core::ptr::null_mut()
        }
        _ => ::core::ptr::null_mut(),
    }
}

/// [`json_obj_get`], but null unless the member has the type asked for.
pub unsafe fn json_obj_get_type(
    obj: *const ParsedValue,
    key: *const ::core::ffi::c_char,
    type_0: JsonType,
) -> *mut ParsedValue {
    let v = unsafe { json_obj_get(obj, key) };
    if !v.is_null() && unsafe { json_type_of(v) } == type_0 {
        return v;
    }
    ::core::ptr::null_mut()
}

/// Look up `key` in a JSON object and read it as a boolean; false for
/// anything that is not a boolean-valued member of an object.
pub unsafe fn json_obj_getbool(obj: *const ParsedValue, key: *const ::core::ffi::c_char) -> bool {
    unsafe { json_bool_val(json_obj_get_type(obj, key, JsonType::Boolean)) }
}

/// A number, whether the JSON spelled it as an integer or a double; 0.0
/// for anything else, including null.
pub unsafe fn json_numof(cv: *const ParsedValue) -> f64 {
    match unsafe { cv.as_ref() } {
        Some(ParsedValue::Int(i)) => *i as f64,
        Some(ParsedValue::Double(d)) => *d,
        _ => 0.0,
    }
}

/// A boolean; false for anything else, including null.
pub unsafe fn json_boolof(cv: *const ParsedValue) -> bool {
    unsafe { json_bool_val(cv) }
}

/// A member's numeric value; 0.0 when absent or non-numeric.
pub unsafe fn json_obj_getnum(obj: *const ParsedValue, key: *const ::core::ffi::c_char) -> f64 {
    unsafe { json_obj_getnum_fallback(obj, key, 0.0) }
}

/// A member's numeric value, truncated to an `i32`; 0 when absent or
/// non-numeric.
pub unsafe fn json_obj_getint(obj: *const ParsedValue, key: *const ::core::ffi::c_char) -> i32 {
    unsafe { json_obj_getint_fallback(obj, key, 0) }
}

// The numeric lookups below walk the object themselves instead of going
// through `json_obj_get`, matching `json_funcs`'s own reasoning: on a name
// match whose value has the wrong type they *keep looking*, since the
// parser permits (and this crate relies on) duplicate members.

/// A member's numeric value, or `fallback` when absent or non-numeric.
pub unsafe fn json_obj_getnum_fallback(
    obj: *const ParsedValue,
    key: *const ::core::ffi::c_char,
    fallback: f64,
) -> f64 {
    if let Some(ParsedValue::Object(fields)) = unsafe { obj.as_ref() } {
        for (k, v) in fields {
            if unsafe { ::libc::strcmp(k.as_ptr() as *const ::core::ffi::c_char, key) } == 0 {
                match v {
                    ParsedValue::Int(i) => return *i as f64,
                    ParsedValue::Double(d) => return *d,
                    _ => {}
                }
            }
        }
    }
    fallback
}

/// A member's numeric value truncated to an `i32`, or `fallback` when
/// absent or non-numeric.
pub unsafe fn json_obj_getint_fallback(
    obj: *const ParsedValue,
    key: *const ::core::ffi::c_char,
    fallback: i32,
) -> i32 {
    if let Some(ParsedValue::Object(fields)) = unsafe { obj.as_ref() } {
        for (k, v) in fields {
            if unsafe { ::libc::strcmp(k.as_ptr() as *const ::core::ffi::c_char, key) } == 0 {
                match v {
                    ParsedValue::Int(i) => return *i as i32,
                    ParsedValue::Double(d) => return *d as i32,
                    _ => {}
                }
            }
        }
    }
    fallback
}

/// A member's string value, copied into a fresh [`crate::vendor::sds::
/// SdsRaw`]; null if it is not a string. The caller owns the copy.
pub unsafe fn json_obj_getsds(
    obj: *const ParsedValue,
    key: *const ::core::ffi::c_char,
) -> crate::vendor::sds::SdsRaw {
    let v = unsafe { json_obj_get_type(obj, key, JsonType::String) };
    if v.is_null() {
        ::core::ptr::null_mut()
    } else {
        unsafe {
            crate::vendor::sds::sdsnewlen(
                json_str_ptr(v) as *const ::core::ffi::c_void,
                json_str_len(v) as usize,
            )
        }
    }
}

/// [`json_obj_getsds`] without the copy: the pointer belongs to the parse
/// tree and dies with it.
pub unsafe fn json_obj_getstr_share(
    obj: *const ParsedValue,
    key: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe { json_str_ptr(json_obj_get_type(obj, key, JsonType::String)) }
}

/// Serialize a bitfield as a JSON object of `label: true` pairs -- see
/// `json_funcs::otfcc_dump_flags` for the build-side inverse (unaffected
/// by this module, since it never reads an existing value).
///
/// A number is taken as the raw field value; an object is read label by
/// label. Anything else -- including a missing key, which arrives here as
/// null -- is 0.
pub unsafe fn otfcc_parse_flags(v: *const ParsedValue, labels: &[&::core::ffi::CStr]) -> u32 {
    if v.is_null() {
        return 0;
    }
    match unsafe { json_type_of(v) } {
        JsonType::Integer => unsafe { json_int_val(v) as u32 },
        JsonType::Double => unsafe { json_dbl_val(v) as u32 },
        JsonType::Object => {
            let mut flags: u32 = 0;
            for (j, label) in labels.iter().enumerate() {
                if unsafe { json_obj_getbool(v, label.as_ptr()) } {
                    flags |= (1u32) << j;
                }
            }
            flags
        }
        _ => 0,
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
                    // `s` carries a trailing NUL not counted in its logical
                    // content (see `ParsedValue`'s doc comment) -- strip it
                    // before comparing against the old tree's bytes.
                    (*old).type_0 == JsonType::String
                        && ::core::slice::from_raw_parts(
                            json_str_ptr(old) as *const u8,
                            json_str_len(old) as usize,
                        ) == &s[..s.len() - 1]
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
                            // `k` carries a trailing NUL, see the `Str` arm
                            // above.
                            old_key == &k[..k.len() - 1]
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

    /// Direct coverage for the accessor layer's NUL-termination contract
    /// (see `ParsedValue`'s doc comment) -- the differential tests above
    /// only exercise it indirectly through `trees_equal`'s own stripping.
    /// This confirms `json_str_ptr`/`json_obj_key_at`'s results are
    /// actually usable with `strcmp`/`CStr::from_ptr`, the exact pattern
    /// dozens of call sites in the files this accessor layer is about to
    /// be wired into rely on.
    #[test]
    fn accessor_strings_are_nul_terminated() {
        // Explicit `super::` qualification: this module's `use super::*`
        // (bringing in this module's own ParsedValue-based accessors) and
        // its explicit `use json_funcs::{...}` (the old JsonValue-based
        // ones, needed by `trees_equal` above) share the same names --
        // Rust's explicit-import-wins-over-glob rule means the bare names
        // resolve to `json_funcs`'s versions everywhere else in this
        // module, so this test needs `super::` to reach the ones it's
        // actually testing.
        unsafe {
            let root = parse_json(br#"{"abc":"xyz"}"#).unwrap();
            let key = super::json_obj_key_at(&root, 0);
            assert_eq!(
                ::libc::strcmp(key, b"abc\0".as_ptr() as *const ::core::ffi::c_char),
                0
            );
            assert_eq!(super::json_obj_key_len_at(&root, 0), 3);
            assert_eq!(
                ::core::ffi::CStr::from_ptr(key).to_bytes(),
                b"abc"
            );

            let val = super::json_obj_val_at(&root, 0);
            let s = super::json_str_ptr(val);
            assert_eq!(
                ::libc::strcmp(s, b"xyz\0".as_ptr() as *const ::core::ffi::c_char),
                0
            );
            assert_eq!(super::json_str_len(val), 3);
            assert_eq!(::core::ffi::CStr::from_ptr(s).to_bytes(), b"xyz");
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
            "99999999999999999999",    // beyond i64::MAX, no '.'/'e' -- wraps
                                        // silently in both parsers (see
                                        // `parse_number`'s doc comment and
                                        // `json_parse_ex`'s wrapping integer
                                        // accumulation)
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
