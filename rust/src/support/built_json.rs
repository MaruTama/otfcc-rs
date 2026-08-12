//! Stage 6-2.5, C-3 (in progress): a safe Rust representation for the JSON
//! *build/dump* side, meant to eventually replace `vendor::json::JsonValue`
//! for every consumer that only *constructs* a value (the whole
//! `table/*/dump.rs` family, reached today through
//! `vendor::json_builder`'s constructor API).
//!
//! `vendor::json::JsonValue` stays exactly as it is for now -- this module
//! is a deliberately *separate* type, following the same reasoning C-2
//! established for the parse side: the build-side object graph never
//! intersects `JsonValue`'s union-based one at runtime, so introducing
//! `BuiltValue` costs nothing on the existing type.
//!
//! `vendor::json_builder`'s real contract, once actually read end to end,
//! turned out narrower than its generality suggests -- mirroring what C-2's
//! survey found for `json_parse_ex`:
//!
//! - `builderize()` -- the "upgrade a bare `JsonValue` produced by the
//!   *parser* into a builder value in place" escape hatch -- never fires in
//!   practice. Every value ever passed to a `json_*_push` call in this
//!   crate was itself produced by a `json_*_new` call; now that C-2 split
//!   `ParsedValue` (parse) from `JsonValue` (build) into distinct Rust
//!   types, a parsed value reaching this API is a compile error, not a
//!   runtime "maybe" -- so `BuiltValue` needs no such upgrade path at all.
//! - `json_object_sort`/`json_object_merge` have zero callers anywhere in
//!   this crate (confirmed by grep) -- dropped rather than ported.
//! - `.parent`/`length_iterated` exist purely so `json_measure_ex`/
//!   `json_serialize_ex`/`json_builder_free` can walk the tree
//!   *iteratively* (stack-frugal C recursion avoidance) rather than
//!   recursively. A `Vec`/`Box`-owned tree needs none of that -- this
//!   module's serializer is ordinary recursion over `&BuiltValue`, and
//!   there is no `BuiltValue`-side `free` at all (`Drop` does it).
//! - **`json_measure_ex` itself turns out to exist purely to pre-size a
//!   `calloc`'d C buffer before `json_serialize_ex` fills it -- and it
//!   deliberately *over*-estimates** (its own arithmetic double-counts
//!   indent width; see `bin/otfccdump.rs`'s post-serialize "scan backward
//!   over trailing zero bytes to find where the real content ends" step,
//!   which exists *because* the buffer is oversized). A `Vec<u8>`-returning
//!   serializer needs no upfront size at all -- it grows exactly as far as
//!   the real content requires, so this module has no `json_measure_ex`
//!   equivalent; `json_serialize_ex` below returns the exact bytes
//!   directly. Wiring this in (a later PR) will delete the "trim trailing
//!   zeros" step in `bin/otfccdump.rs` as dead weight along with it.
//! - `JSON_SERIALIZE_MODE_SINGLE_LINE` (the fallback `DEFAULT_OPTS` mode)
//!   is never reached by any real call site either -- both callers
//!   (`bin/otfccdump.rs`, `support/json_funcs.rs`'s `preserialize`) always
//!   pass `PACKED` or `MULTILINE` explicitly. `get_serialize_flags` is
//!   still ported in full below (it is cheap and already written), so
//!   nothing is lost by not narrowing further here.
//!
//! This module is not wired into anything yet -- `BuiltValue` and its
//! constructor/serializer functions exist and are validated by the
//! differential test suite at the bottom of this file (which builds the
//! same tree shape with both this module and `vendor::json_builder`, then
//! compares the serialized bytes), but no `table/*/dump.rs` consumer has
//! been switched over. That's the next PR, once this one has proven the
//! new serializer matches byte-for-byte.

use ::core::ffi::{c_char, c_int, c_uint};

pub use crate::vendor::json_builder::{
    JsonSerializeOpts, JSON_SERIALIZE_MODE_MULTILINE, JSON_SERIALIZE_MODE_PACKED,
    JSON_SERIALIZE_MODE_SINGLE_LINE, JSON_SERIALIZE_OPT_CRLF, JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COLON,
    JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COMMA, JSON_SERIALIZE_OPT_PACK_BRACKETS,
    JSON_SERIALIZE_OPT_USE_TABS,
};
use crate::vendor::emyg_dtoa::emyg_dtoa;

/// The build-side JSON tree. Unlike `parsed_json::ParsedValue`, no
/// NUL-termination convention is needed on `Str`/keys: nothing on the
/// build side ever reads a value back out through a C-string accessor
/// (`preserialize`'s own `Vec<u8>` output is measured by `.len()`, not
/// `strlen`) -- `BuiltValue` is only ever constructed, then serialized.
#[derive(Debug, Clone, PartialEq)]
pub enum BuiltValue {
    Null,
    Bool(bool),
    Int(i64),
    Double(f64),
    Str(Vec<u8>),
    /// Pre-rendered bytes spliced in verbatim by the serializer, matching
    /// `JsonType::PreSerialized`'s role exactly -- see [`preserialize`].
    PreSerialized(Vec<u8>),
    Array(Vec<BuiltValue>),
    /// `Vec`, not `HashMap`/`BTreeMap`: object member order must survive
    /// verbatim, same invariant `ParsedValue::Object` documents.
    Object(Vec<(Vec<u8>, BuiltValue)>),
}

// The constructor layer below mirrors `vendor::json_builder`'s API
// name-for-name, targeting `*mut BuiltValue` instead of `*mut
// vendor::json::JsonValue` -- the point, same as `parsed_json`'s own
// accessor layer, is that a `table/*/dump.rs` file's call *expressions*
// stay textually identical when it's switched over; only the `use` line
// and the function's own signature change.

/// Pre-sized array constructor; `length` is a capacity hint only (unlike
/// the old `json_array_new`, `Vec::push` beyond it just reallocates --
/// there is no separate `additional_length_allocated` bookkeeping to get
/// wrong).
pub unsafe fn json_array_new(length: usize) -> *mut BuiltValue {
    Box::into_raw(Box::new(BuiltValue::Array(Vec::with_capacity(length))))
}

/// Appends `value` to `array`, taking ownership of it. No-op (but still
/// consumes `value`) if `array` isn't actually an array -- every real call
/// site only ever passes a value it got from `json_array_new` itself, so
/// this never fires in practice; kept lenient rather than panicking to
/// match this layer's read-accessor style.
pub unsafe fn json_array_push(array: *mut BuiltValue, value: *mut BuiltValue) -> *mut BuiltValue {
    let v = *unsafe { Box::from_raw(value) };
    if let Some(BuiltValue::Array(items)) = unsafe { array.as_mut() } {
        items.push(v);
    }
    array
}

/// Pre-sized object constructor; see [`json_array_new`] on the capacity
/// hint.
pub unsafe fn json_object_new(length: usize) -> *mut BuiltValue {
    Box::into_raw(Box::new(BuiltValue::Object(Vec::with_capacity(length))))
}

/// Pushes `(name, value)`, `name` taken as a NUL-terminated C string.
pub unsafe fn json_object_push(
    object: *mut BuiltValue,
    name: *const c_char,
    value: *mut BuiltValue,
) -> *mut BuiltValue {
    let name_length = unsafe { ::libc::strlen(name) } as c_uint;
    unsafe { json_object_push_length(object, name_length, name, value) }
}

/// [`json_object_push`], but with an explicit byte length instead of
/// `strlen` -- copies exactly `name_length` bytes verbatim (embedded NULs
/// included), matching the old `json_object_push_length`'s raw `memcpy`.
pub unsafe fn json_object_push_length(
    object: *mut BuiltValue,
    name_length: c_uint,
    name: *const c_char,
    value: *mut BuiltValue,
) -> *mut BuiltValue {
    let key = unsafe { ::core::slice::from_raw_parts(name as *const u8, name_length as usize) }
        .to_vec();
    let v = *unsafe { Box::from_raw(value) };
    if let Some(BuiltValue::Object(fields)) = unsafe { object.as_mut() } {
        fields.push((key, v));
    }
    object
}

/// [`json_object_push`], for a `Handle.name`-shaped `Vec<u8>` key (no
/// longer a NUL-terminated C string) -- truncates at the first embedded
/// NUL the same way `strlen` would, matching
/// `parsed_json`'s and the old `json_builder`'s own
/// `json_object_push_bytes_key`.
pub(crate) unsafe fn json_object_push_bytes_key(
    object: *mut BuiltValue,
    name: &[u8],
    value: *mut BuiltValue,
) -> *mut BuiltValue {
    let len = match name.iter().position(|&b| b == 0) {
        Some(p) => p,
        None => name.len(),
    };
    unsafe {
        json_object_push_length(object, len as c_uint, name.as_ptr() as *const c_char, value)
    }
}

/// A NUL-terminated C string, copied verbatim (embedded NULs impossible
/// here since `strlen` finds the length).
pub unsafe fn json_string_new(buf: *const c_char) -> *mut BuiltValue {
    let length = unsafe { ::libc::strlen(buf) } as c_uint;
    unsafe { json_string_new_length(length, buf) }
}

/// [`json_string_new`], with an explicit byte length -- copies exactly
/// `length` bytes verbatim (embedded NULs included), matching the old
/// `json_string_new_length`'s raw `memcpy`.
pub unsafe fn json_string_new_length(length: c_uint, buf: *const c_char) -> *mut BuiltValue {
    let bytes =
        unsafe { ::core::slice::from_raw_parts(buf as *const u8, length as usize) }.to_vec();
    Box::into_raw(Box::new(BuiltValue::Str(bytes)))
}

/// [`json_string_new`], for a `Handle.name`-shaped `Vec<u8>` value --
/// truncates at the first embedded NUL the same way `strlen` would,
/// matching `parsed_json`'s and the old `json_builder`'s own
/// `json_string_new_from_bytes`.
pub(crate) unsafe fn json_string_new_from_bytes(buf: &[u8]) -> *mut BuiltValue {
    let len = match buf.iter().position(|&b| b == 0) {
        Some(p) => p,
        None => buf.len(),
    };
    Box::into_raw(Box::new(BuiltValue::Str(buf[..len].to_vec())))
}

pub unsafe fn json_integer_new(integer: i64) -> *mut BuiltValue {
    Box::into_raw(Box::new(BuiltValue::Int(integer)))
}

pub unsafe fn json_double_new(dbl: f64) -> *mut BuiltValue {
    Box::into_raw(Box::new(BuiltValue::Double(dbl)))
}

pub unsafe fn json_boolean_new(b: c_int) -> *mut BuiltValue {
    Box::into_raw(Box::new(BuiltValue::Bool(b != 0)))
}

pub unsafe fn json_null_new() -> *mut BuiltValue {
    Box::into_raw(Box::new(BuiltValue::Null))
}

/// Serializes `x` now (packed mode) and keeps the bytes, so the writer can
/// splice them in verbatim later instead of descending into `x` a second
/// time. Consumes `x` -- matches `json_funcs::preserialize`'s contract
/// exactly, minus the separate `json_measure_ex`/`malloc`/
/// `json_builder_free` steps that a `Vec<u8>`-returning serializer makes
/// unnecessary.
pub unsafe fn preserialize(x: *mut BuiltValue) -> *mut BuiltValue {
    let v = *unsafe { Box::from_raw(x) };
    let opts = JsonSerializeOpts {
        mode: JSON_SERIALIZE_MODE_PACKED,
        opts: 0,
        indent_size: 0,
    };
    let bytes = json_serialize_ex(&v, opts);
    Box::into_raw(Box::new(BuiltValue::PreSerialized(bytes)))
}

const F_SPACES_AROUND_BRACKETS: c_int = 1 << 0;
const F_SPACES_AFTER_COMMAS: c_int = 1 << 1;
const F_SPACES_AFTER_COLONS: c_int = 1 << 2;
const F_TABS: c_int = 1 << 3;

/// Ported verbatim from `vendor::json_builder::get_serialize_flags` --
/// pure, cheap, and already fully general, so there's no reason to narrow
/// it to just the two mode/opts combinations real call sites use.
fn get_serialize_flags(opts: JsonSerializeOpts) -> c_int {
    if opts.mode == JSON_SERIALIZE_MODE_PACKED {
        return 0;
    }
    let mut flags = 0;
    if opts.mode == JSON_SERIALIZE_MODE_MULTILINE {
        if opts.opts & JSON_SERIALIZE_OPT_USE_TABS != 0 {
            flags |= F_TABS;
        }
    } else {
        if opts.opts & JSON_SERIALIZE_OPT_PACK_BRACKETS == 0 {
            flags |= F_SPACES_AROUND_BRACKETS;
        }
        if opts.opts & JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COMMA == 0 {
            flags |= F_SPACES_AFTER_COMMAS;
        }
    }
    if opts.opts & JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COLON == 0 {
        flags |= F_SPACES_AFTER_COLONS;
    }
    flags
}

/// Renders `value` to bytes exactly, no upfront size pass -- see this
/// module's doc comment on why `json_measure_ex` has no equivalent here.
pub fn json_serialize_ex(value: &BuiltValue, opts: JsonSerializeOpts) -> Vec<u8> {
    let flags = get_serialize_flags(opts);
    let mut out = Vec::new();
    write_value(value, opts, flags, 0, &mut out);
    out
}

fn push_newline_indent(out: &mut Vec<u8>, opts: JsonSerializeOpts, flags: c_int, depth: c_int) {
    if opts.mode != JSON_SERIALIZE_MODE_MULTILINE {
        return;
    }
    if opts.opts & JSON_SERIALIZE_OPT_CRLF != 0 {
        out.push(b'\r');
    }
    out.push(b'\n');
    let indent_char = if flags & F_TABS != 0 { b'\t' } else { b' ' };
    for _ in 0..(depth * opts.indent_size) {
        out.push(indent_char);
    }
}

fn write_value(
    value: &BuiltValue,
    opts: JsonSerializeOpts,
    flags: c_int,
    depth: c_int,
    out: &mut Vec<u8>,
) {
    match value {
        BuiltValue::Array(items) => {
            if items.is_empty() {
                out.extend_from_slice(b"[]");
                return;
            }
            out.push(b'[');
            if flags & F_SPACES_AROUND_BRACKETS != 0 {
                out.push(b' ');
            }
            let inner_depth = depth + 1;
            push_newline_indent(out, opts, flags, inner_depth);
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                    if flags & F_SPACES_AFTER_COMMAS != 0 {
                        out.push(b' ');
                    }
                    push_newline_indent(out, opts, flags, inner_depth);
                }
                write_value(item, opts, flags, inner_depth, out);
            }
            push_newline_indent(out, opts, flags, depth);
            if flags & F_SPACES_AROUND_BRACKETS != 0 {
                out.push(b' ');
            }
            out.push(b']');
        }
        BuiltValue::Object(fields) => {
            if fields.is_empty() {
                out.extend_from_slice(b"{}");
                return;
            }
            out.push(b'{');
            if flags & F_SPACES_AROUND_BRACKETS != 0 {
                out.push(b' ');
            }
            let inner_depth = depth + 1;
            push_newline_indent(out, opts, flags, inner_depth);
            for (i, (key, val)) in fields.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                    if flags & F_SPACES_AFTER_COMMAS != 0 {
                        out.push(b' ');
                    }
                    push_newline_indent(out, opts, flags, inner_depth);
                }
                out.push(b'"');
                escape_string_into(key, out);
                out.push(b'"');
                out.push(b':');
                if flags & F_SPACES_AFTER_COLONS != 0 {
                    out.push(b' ');
                }
                write_value(val, opts, flags, inner_depth, out);
            }
            push_newline_indent(out, opts, flags, depth);
            if flags & F_SPACES_AROUND_BRACKETS != 0 {
                out.push(b' ');
            }
            out.push(b'}');
        }
        BuiltValue::PreSerialized(bytes) => out.extend_from_slice(bytes),
        BuiltValue::Str(s) => {
            out.push(b'"');
            escape_string_into(s, out);
            out.push(b'"');
        }
        BuiltValue::Int(n) => {
            out.extend_from_slice(n.to_string().as_bytes());
        }
        BuiltValue::Double(d) => {
            let mut buffer = [0 as c_char; 256];
            unsafe { emyg_dtoa(*d, buffer.as_mut_ptr()) };
            let text = unsafe { ::core::ffi::CStr::from_ptr(buffer.as_ptr()) };
            out.extend_from_slice(text.to_bytes());
        }
        BuiltValue::Bool(b) => out.extend_from_slice(if *b { b"true" } else { b"false" }),
        BuiltValue::Null => out.extend_from_slice(b"null"),
    }
}

/// Ported from `vendor::json_builder::serialize_string`'s escape table
/// verbatim: NUL and vertical tab (0x0B) as `\uXXXX`, the standard
/// backslash escapes for the rest, everything else copied through
/// unchanged (including raw non-UTF-8/Latin-1 bytes -- object keys and
/// string values are `Vec<u8>`, not `String`, for exactly this reason).
fn escape_string_into(s: &[u8], out: &mut Vec<u8>) {
    for &c in s {
        match c {
            0 => out.extend_from_slice(b"\\u0000"),
            11 => out.extend_from_slice(b"\\u000b"),
            b'"' => out.extend_from_slice(b"\\\""),
            b'\\' => out.extend_from_slice(b"\\\\"),
            8 => out.extend_from_slice(b"\\b"),
            12 => out.extend_from_slice(b"\\f"),
            10 => out.extend_from_slice(b"\\n"),
            13 => out.extend_from_slice(b"\\r"),
            9 => out.extend_from_slice(b"\\t"),
            _ => out.push(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vendor::json::JsonValue;
    use crate::vendor::json_builder::{
        json_array_new as old_array_new, json_array_push as old_array_push,
        json_boolean_new as old_boolean_new, json_builder_free, json_double_new as old_double_new,
        json_integer_new as old_integer_new, json_measure_ex, json_null_new as old_null_new,
        json_object_new as old_object_new, json_object_push_length as old_object_push_length,
        json_serialize_ex as old_serialize_ex, json_string_new_length as old_string_new_length,
    };

    unsafe fn old_serialize(value: *mut JsonValue, opts: JsonSerializeOpts) -> Vec<u8> {
        let len = unsafe { json_measure_ex(value, opts) };
        let buf = unsafe { ::libc::calloc(1, len) } as *mut ::core::ffi::c_char;
        unsafe { old_serialize_ex(buf, value, opts) };
        // Matches `bin/otfccdump.rs`'s own "scan back over the
        // over-estimate's trailing zero padding" step -- see this
        // module's doc comment on why `json_measure_ex` over-estimates.
        let mut actual_len = len - 1;
        while actual_len > 0 && unsafe { *buf.add(actual_len) } == 0 {
            actual_len -= 1;
        }
        if unsafe { *buf.add(actual_len) } != 0 {
            actual_len += 1;
        }
        let bytes =
            unsafe { ::core::slice::from_raw_parts(buf as *const u8, actual_len) }.to_vec();
        unsafe { ::libc::free(buf as *mut ::core::ffi::c_void) };
        bytes
    }

    /// Builds the same nested tree shape (object containing an array,
    /// strings needing every escape case, positive/negative/zero
    /// integers, a double, both booleans, null, and a nested empty
    /// array/object) with both the old builder and this module, and
    /// returns `(old_tree, new_tree)`.
    unsafe fn build_sample_tree() -> (*mut JsonValue, BuiltValue) {
        unsafe {
            let old_root = old_object_new(8);
            let new_root = json_object_new(8);

            old_object_push_length(
                old_root,
                4,
                c"name".as_ptr(),
                old_string_new_length(
                    22,
                    c"quote\" back\\slash\ttab".as_ptr(),
                ),
            );
            json_object_push_length(
                new_root,
                4,
                c"name".as_ptr(),
                json_string_new_length(
                    22,
                    c"quote\" back\\slash\ttab".as_ptr(),
                ),
            );

            let control_bytes: &[u8] = &[0, 8, 11, 12, 10, 13, b'a', 0xff];
            old_object_push_length(
                old_root,
                8,
                c"controls".as_ptr(),
                old_string_new_length(
                    control_bytes.len() as c_uint,
                    control_bytes.as_ptr() as *const c_char,
                ),
            );
            json_object_push_length(
                new_root,
                8,
                c"controls".as_ptr(),
                json_string_new_length(
                    control_bytes.len() as c_uint,
                    control_bytes.as_ptr() as *const c_char,
                ),
            );

            let old_arr = old_array_new(5);
            let new_arr = json_array_new(5);
            // `i64::MIN` is deliberately excluded: the old builder's
            // `json_serialize_ex` does `integer = -integer` on a negative
            // value with no overflow guard, which is UB-but-wraps in C but
            // panics in a debug-mode Rust build's checked negation --
            // a pre-existing latent bug in `vendor/json_builder.rs`,
            // unrelated to and not reproduced by this module, same as the
            // parser's integer-overflow panic flagged during C-2.
            for n in [0i64, 1, -1, 12345, i64::MIN + 1, i64::MAX] {
                old_array_push(old_arr, old_integer_new(n));
                json_array_push(new_arr, json_integer_new(n));
            }
            old_object_push_length(old_root, 6, c"ints".as_ptr(), old_arr);
            json_object_push_length(new_root, 6, c"ints".as_ptr(), new_arr);

            for (dbl, key) in [
                (0.0f64, c"zero"),
                (-0.0f64, c"negzero"),
                (3.5f64, c"frac"),
                (-123.456f64, c"negfrac"),
                (1.0e20f64, c"big"),
                (1.0e-20f64, c"small"),
            ] {
                old_object_push_length(old_root, 7, key.as_ptr(), old_double_new(dbl));
                json_object_push_length(new_root, 7, key.as_ptr(), json_double_new(dbl));
            }

            old_object_push_length(old_root, 4, c"true".as_ptr(), old_boolean_new(1));
            json_object_push_length(new_root, 4, c"true".as_ptr(), json_boolean_new(1));
            old_object_push_length(old_root, 5, c"false".as_ptr(), old_boolean_new(0));
            json_object_push_length(new_root, 5, c"false".as_ptr(), json_boolean_new(0));
            old_object_push_length(old_root, 4, c"null".as_ptr(), old_null_new());
            json_object_push_length(new_root, 4, c"null".as_ptr(), json_null_new());

            old_object_push_length(old_root, 9, c"emptyarr".as_ptr(), old_array_new(0));
            json_object_push_length(new_root, 9, c"emptyarr".as_ptr(), json_array_new(0));
            old_object_push_length(old_root, 8, c"emptyobj".as_ptr(), old_object_new(0));
            json_object_push_length(new_root, 8, c"emptyobj".as_ptr(), json_object_new(0));

            let new_tree = *Box::from_raw(new_root);
            (old_root, new_tree)
        }
    }

    #[test]
    fn packed_matches_old_builder() {
        unsafe {
            let (old_root, new_tree) = build_sample_tree();
            let opts = JsonSerializeOpts {
                mode: JSON_SERIALIZE_MODE_PACKED,
                opts: 0,
                indent_size: 0,
            };
            let old_bytes = old_serialize(old_root, opts);
            let new_bytes = json_serialize_ex(&new_tree, opts);
            assert_eq!(old_bytes, new_bytes);
            json_builder_free(old_root);
        }
    }

    #[test]
    fn multiline_matches_old_builder() {
        unsafe {
            let (old_root, new_tree) = build_sample_tree();
            let opts = JsonSerializeOpts {
                mode: JSON_SERIALIZE_MODE_MULTILINE,
                opts: 0,
                indent_size: 4,
            };
            let old_bytes = old_serialize(old_root, opts);
            let new_bytes = json_serialize_ex(&new_tree, opts);
            assert_eq!(old_bytes, new_bytes);
            json_builder_free(old_root);
        }
    }

    #[test]
    fn preserialize_splices_verbatim_into_a_parent() {
        unsafe {
            // A subtree built and preserialized in isolation, then
            // spliced into a fresh parent, must serialize identically to
            // the same subtree built directly inside that parent -- the
            // whole point of `PreSerialized` is that splicing it changes
            // nothing observable.
            let sub_a = json_object_new(1);
            json_object_push_length(sub_a, 1, c"x".as_ptr(), json_integer_new(42));
            let pre = preserialize(sub_a);

            let parent_with_pre = json_object_new(1);
            json_object_push_length(parent_with_pre, 5, c"child".as_ptr(), pre);
            let parent_with_pre = *Box::from_raw(parent_with_pre);

            let sub_b = json_object_new(1);
            json_object_push_length(sub_b, 1, c"x".as_ptr(), json_integer_new(42));
            let sub_b = *Box::from_raw(sub_b);
            let parent_direct = BuiltValue::Object(vec![(b"child".to_vec(), sub_b)]);

            let opts = JsonSerializeOpts {
                mode: JSON_SERIALIZE_MODE_PACKED,
                opts: 0,
                indent_size: 0,
            };
            assert_eq!(
                json_serialize_ex(&parent_with_pre, opts),
                json_serialize_ex(&parent_direct, opts),
            );
        }
    }

    #[test]
    fn object_key_and_string_nul_truncation_matches_strlen_convention() {
        unsafe {
            let name: &[u8] = b"abc\0def";
            let value_bytes: &[u8] = b"xy\0z";
            let obj = json_object_new(1);
            let pushed = json_object_push_bytes_key(
                obj,
                name,
                json_string_new_from_bytes(value_bytes),
            );
            assert_eq!(obj, pushed);
            let tree = *Box::from_raw(obj);
            match tree {
                BuiltValue::Object(fields) => {
                    assert_eq!(fields.len(), 1);
                    assert_eq!(fields[0].0, b"abc".to_vec());
                    assert_eq!(fields[0].1, BuiltValue::Str(b"xy".to_vec()));
                }
                _ => panic!("expected an object"),
            }
        }
    }
}
