//! Stage 6-2.5, C-2: a safe, single-pass JSON parser and tree, replacing
//! the old vendored `JsonValue`-based parser for every consumer that only
//! *reads* a parsed value (the whole `table/*/parse.rs` family, plus
//! `bin/otfccbuild.rs`'s and `ffi/dll.rs`'s own `json_parse` calls). Every
//! consumer has been switched over to this module's `parse_json`/accessor
//! API.
//!
//! `ParsedValue` is a genuinely separate type from `built_json::BuiltValue`
//! (the build side's own safe representation, from Stage 6-2.5 C-3) even
//! though both ultimately replaced pieces of the same old vendored
//! `JsonValue` union: the two object graphs never intersected at runtime
//! (the entire parse tree was freed before any build tree was
//! constructed), so there was never a reason to unify them.
//!
//! The old vendored parser (`vendor/json.rs`) was deleted entirely in
//! Stage 6-2.5 C-4, once grep confirmed it had no remaining caller in this
//! crate -- this module's own test suite below no longer differential-
//! tests against it for the same reason; it asserts directly against
//! `parse_json`'s own output instead. Its real contract, once actually
//! read end to end while porting it here, turned out narrower than its
//! generality suggested: the old `json_parse_ex`, as actually used in this
//! crate (`json_parse`, called from exactly two places -- `bin/
//! otfccbuild.rs` and `ffi/dll.rs` -- both just checking the result for
//! null), always passed `error_buf = null` (parse-error text/position was
//! never surfaced anywhere in this codebase) and `settings.settings = 0`
//! (`JSON_ENABLE_COMMENTS` unset, so comment support was dead code in this
//! crate's actual usage even though the vendored parser implemented it).
//! That's why this replacement only needs to parse standard JSON, or not
//! -- no line/column tracking, no comment syntax, no custom allocator
//! hookup (moot once `Vec`/`Box` own the memory).

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

/// [`json_obj_key_at`]/[`json_obj_key_len_at`], combined into one owned
/// copy -- see [`json_str_bytes`] for why. Returns `Vec::new()` out of
/// range or on a non-object, matching `json_obj_key_len_at`'s 0.
pub unsafe fn json_obj_key_bytes_at(obj: *const ParsedValue, i: u32) -> Vec<u8> {
    match unsafe { obj.as_ref() } {
        Some(ParsedValue::Object(fields)) => match fields.get(i as usize) {
            Some((k, _)) => k[..k.len() - 1].to_vec(),
            None => Vec::new(),
        },
        _ => Vec::new(),
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

/// [`json_str_ptr`]/[`json_str_len`], combined into one owned copy --
/// for callers that used to `sdsnewlen(json_str_ptr(v), json_str_len(v))`
/// right after each other and now just want a `Vec<u8>` (e.g. to hand to
/// `handle_from_name`). Returns `Vec::new()` for anything but a string,
/// matching `json_str_len`'s 0.
pub unsafe fn json_str_bytes(v: *const ParsedValue) -> Vec<u8> {
    match unsafe { v.as_ref() } {
        Some(ParsedValue::Str(s)) => s[..s.len() - 1].to_vec(),
        _ => Vec::new(),
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

/// A member's string value, copied into a fresh `Vec<u8>`; `None` if it is
/// not a string.
pub unsafe fn json_obj_getsds(
    obj: *const ParsedValue,
    key: *const ::core::ffi::c_char,
) -> Option<Vec<u8>> {
    let v = unsafe { json_obj_get_type(obj, key, JsonType::String) };
    if v.is_null() {
        None
    } else {
        unsafe {
            Some(::core::slice::from_raw_parts(json_str_ptr(v) as *const u8, json_str_len(v) as usize).to_vec())
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

    /// Wraps `bytes` as a `Str` the way `parse_string`/`parse_object` do --
    /// the trailing NUL is part of the storage convention, not the logical
    /// content (see `ParsedValue`'s doc comment); this saves every
    /// expected-value literal below from having to spell it out by hand.
    fn s(bytes: impl AsRef<[u8]>) -> ParsedValue {
        let mut v = bytes.as_ref().to_vec();
        v.push(0);
        ParsedValue::Str(v)
    }

    /// Parses `{"v": <json_body>}` and returns the `v` field's value alone
    /// -- lets the number/string test cases below assert against a bare
    /// `ParsedValue` instead of a whole one-field object literal each time.
    fn parse_v(json_body: &str) -> ParsedValue {
        let text = format!(r#"{{"v":{json_body}}}"#);
        match parse_json(text.as_bytes()) {
            Some(ParsedValue::Object(mut fields)) if fields.len() == 1 => fields.remove(0).1,
            other => panic!("expected {{\"v\": {json_body}}} to parse to a 1-field object, got {other:?}"),
        }
    }

    /// Direct coverage for the accessor layer's NUL-termination contract
    /// (see `ParsedValue`'s doc comment): confirms `json_str_ptr`/
    /// `json_obj_key_at`'s results are actually usable with
    /// `strcmp`/`CStr::from_ptr`, the exact pattern dozens of call sites
    /// across the crate rely on.
    #[test]
    fn accessor_strings_are_nul_terminated() {
        unsafe {
            let root = parse_json(br#"{"abc":"xyz"}"#).unwrap();
            let key = json_obj_key_at(&root, 0);
            assert_eq!(
                ::libc::strcmp(key, b"abc\0".as_ptr() as *const ::core::ffi::c_char),
                0
            );
            assert_eq!(json_obj_key_len_at(&root, 0), 3);
            assert_eq!(
                ::core::ffi::CStr::from_ptr(key).to_bytes(),
                b"abc"
            );

            let val = json_obj_val_at(&root, 0);
            let str_ptr = json_str_ptr(val);
            assert_eq!(
                ::libc::strcmp(str_ptr, b"xyz\0".as_ptr() as *const ::core::ffi::c_char),
                0
            );
            assert_eq!(json_str_len(val), 3);
            assert_eq!(::core::ffi::CStr::from_ptr(str_ptr).to_bytes(), b"xyz");
        }
    }

    /// Every `.json` file committed under `tests/payload/` or produced
    /// into `build/` parses successfully -- a smoke test against real
    /// font-derived JSON, not just the synthetic cases below.
    #[test]
    fn every_committed_payload_json_parses() {
        let mut any = false;
        for dir in ["../tests/payload", "../build"] {
            let Ok(entries) = std::fs::read_dir(dir) else { continue };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    let bytes = std::fs::read(&path).unwrap();
                    assert!(
                        parse_json(&bytes).is_some(),
                        "failed to parse {}",
                        path.display()
                    );
                    any = true;
                }
            }
        }
        assert!(any, "no .json payload files found to test against");
    }

    #[test]
    fn number_edge_cases() {
        assert_eq!(parse_v("5"), ParsedValue::Int(5));
        assert_eq!(parse_v("5.0"), ParsedValue::Double(5.0));
        assert_eq!(parse_v("5e0"), ParsedValue::Double(5.0));
        assert_eq!(parse_v("5E2"), ParsedValue::Double(500.0));
        assert_eq!(parse_v("-0"), ParsedValue::Int(0));
        assert_eq!(parse_v("-0.0"), ParsedValue::Double(-0.0));
        assert_eq!(parse_v("0"), ParsedValue::Int(0));
        assert_eq!(parse_v("-5"), ParsedValue::Int(-5));
        assert_eq!(parse_v("1e309"), ParsedValue::Double(f64::INFINITY)); // overflow-to-infinity
        assert_eq!(parse_v("-1e309"), ParsedValue::Double(f64::NEG_INFINITY));
        assert_eq!(parse_v("9223372036854775807"), ParsedValue::Int(i64::MAX));
        // Beyond i64::MAX with no '.'/'e' (still syntactically Integer)
        // wraps silently via `wrapping_mul`/`wrapping_add`, matching the
        // (now-deleted) vendored parser's own wrapping integer
        // accumulation -- both were fixed to wrap deliberately rather than
        // panic on overflow (see `parse_number`'s doc comment).
        assert_eq!(
            parse_v("99999999999999999999"),
            ParsedValue::Int(7766279631452241919)
        );
        assert_eq!(parse_v("0.1"), ParsedValue::Double(0.1));
        assert_eq!(
            parse_v("1.7976931348623157e308"),
            ParsedValue::Double(f64::MAX)
        );
        assert_eq!(parse_v("123.456e-7"), ParsedValue::Double(123.456e-7));
        assert_eq!(parse_v("1e-300"), ParsedValue::Double(1e-300));
        assert_eq!(parse_v("0.0000001"), ParsedValue::Double(0.0000001));
    }

    #[test]
    fn number_syntax_errors_are_rejected() {
        for bad in ["01", "00", "00.5", "-01", "5.", "5e", "5.e2", ".5", "+5"] {
            assert!(
                parse_json(format!(r#"{{"v":{bad}}}"#).as_bytes()).is_none(),
                "expected {bad:?} to be rejected"
            );
        }
    }

    #[test]
    fn string_edge_cases() {
        // `\u` escape + surrogate pair (é, 😀), plus \t \n \\ \".
        let mut expected: Vec<u8> = "é😀".bytes().collect();
        expected.extend(b"\t\n\\\"");
        assert_eq!(
            parse_v(r#""é😀\t\n\\\"""#),
            s(expected)
        );

        // Raw non-UTF-8 bytes survive verbatim (Latin-1 glyph names).
        assert_eq!(
            parse_json(&[b'{', b'"', b'a', b'"', b':', b'"', 0xff, 0xfe, b'"', b'}']),
            Some(ParsedValue::Object(vec![(
                b"a\0".to_vec(),
                s([0xffu8, 0xfe])
            )]))
        );

        // Empty containers.
        assert_eq!(
            parse_json(br#"{"a":"","b":[],"c":{}}"#),
            Some(ParsedValue::Object(vec![
                (b"a\0".to_vec(), s(b"")),
                (b"b\0".to_vec(), ParsedValue::Array(vec![])),
                (b"c\0".to_vec(), ParsedValue::Object(vec![])),
            ]))
        );

        // Duplicate keys, order preserved (not collapsed into a map).
        assert_eq!(
            parse_json(br#"{"dup":1,"dup":2}"#),
            Some(ParsedValue::Object(vec![
                (b"dup\0".to_vec(), ParsedValue::Int(1)),
                (b"dup\0".to_vec(), ParsedValue::Int(2)),
            ]))
        );

        assert_eq!(
            parse_json(br#"[true,false,null,1,1.5,"s"]"#),
            Some(ParsedValue::Array(vec![
                ParsedValue::Bool(true),
                ParsedValue::Bool(false),
                ParsedValue::Null,
                ParsedValue::Int(1),
                ParsedValue::Double(1.5),
                s(b"s"),
            ]))
        );

        assert_eq!(
            parse_json(br#"{"nested":{"a":[1,{"b":2}]}}"#),
            Some(ParsedValue::Object(vec![(
                b"nested\0".to_vec(),
                ParsedValue::Object(vec![(
                    b"a\0".to_vec(),
                    ParsedValue::Array(vec![
                        ParsedValue::Int(1),
                        ParsedValue::Object(vec![(b"b\0".to_vec(), ParsedValue::Int(2))]),
                    ])
                )])
            )]))
        );
    }

    /// Both single-trailing-comma tolerance (before `]`/`}`) and a bare
    /// `-` value (accepted as `Int(0)`, since the number grammar has no
    /// "at least one digit" check on the integer part -- only the
    /// fraction and exponent parts do) are genuine leniency quirks of the
    /// vendored parser this one replaced, confirmed empirically (not
    /// assumed) before this parser was written to match them. A second
    /// consecutive comma, or a comma before the first element, is still
    /// rejected -- see `malformed_input_is_rejected`.
    #[test]
    fn parser_leniency_quirks() {
        assert_eq!(
            parse_json(b"[1,2,]"),
            Some(ParsedValue::Array(vec![ParsedValue::Int(1), ParsedValue::Int(2)]))
        );
        assert_eq!(
            parse_json(br#"{"a":1,}"#),
            Some(ParsedValue::Object(vec![(b"a\0".to_vec(), ParsedValue::Int(1))]))
        );
        assert_eq!(parse_v("-"), ParsedValue::Int(0));
        assert_eq!(
            parse_json(b"[1,]"),
            Some(ParsedValue::Array(vec![ParsedValue::Int(1)]))
        );
        assert_eq!(parse_json(b"[]"), Some(ParsedValue::Array(vec![])));
        assert_eq!(parse_json(b"{}"), Some(ParsedValue::Object(vec![])));
    }

    #[test]
    fn malformed_input_is_rejected() {
        for bad in [
            "{", r#"{"a":}"#, r#""unterminated"#, "nul", "{,}", "[1 2]",
            r#"{"a" "b"}"#, "", "   ", "[1,2,,]", "[,1]", "[,]",
        ] {
            assert!(parse_json(bad.as_bytes()).is_none(), "expected {bad:?} to be rejected");
        }
    }
}
