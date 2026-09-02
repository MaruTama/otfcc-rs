//! Stage 6-2.5, C-3: a safe Rust representation for the JSON *build/dump*
//! side, replacing the old vendored `JsonValue`-based builder for every
//! consumer that only *constructs* a value (the whole `table/*/dump.rs`
//! family, plus `bin/otfccdump.rs`'s own serialize call). Every consumer
//! has been switched over to this module's constructor API and its
//! `json_serialize_ex`.
//!
//! `BuiltValue` is a genuinely separate type from `parsed_json::ParsedValue`
//! (the parse side's own safe representation, from Stage 6-2.5 C-2) even
//! though both ultimately replaced pieces of the same old vendored
//! `JsonValue` union: the two object graphs never intersected at runtime
//! (the whole parse tree was freed before any build tree existed), so
//! there was never a reason to unify them.
//!
//! The old vendored parser (`vendor/json.rs`) and builder
//! (`vendor/json_builder.rs`) were deleted entirely in Stage 6-2.5 C-4,
//! once grep confirmed neither had any remaining caller in this crate --
//! this module's own differential test suite (which used to build the same
//! sample tree with both this module and the old builder, then compare the
//! serialized bytes) was rewritten to assert against fixed byte fixtures
//! instead, captured from this module's own output after that comparison
//! had already confirmed it matched. Their real contracts, once actually
//! read end to end while porting them here, turned out narrower than their
//! generality suggested:
//!
//! - `builderize()` -- the old builder's "upgrade a bare `JsonValue`
//!   produced by the *parser* into a builder value in place" escape hatch
//!   -- never fired in practice. Every value ever passed to a
//!   `json_*_push` call in this crate was itself produced by a
//!   `json_*_new` call; once C-2 split `ParsedValue` (parse) from
//!   `JsonValue` (build) into distinct Rust types, a parsed value reaching
//!   this API became a compile error, not a runtime "maybe" -- so
//!   `BuiltValue` needs no such upgrade path at all.
//! - `json_object_sort`/`json_object_merge` had zero callers anywhere in
//!   this crate (confirmed by grep) -- dropped rather than ported.
//! - `.parent`/`length_iterated` existed purely so the old builder's
//!   `json_measure_ex`/`json_serialize_ex`/`json_builder_free` could walk
//!   the tree *iteratively* (stack-frugal C recursion avoidance) rather
//!   than recursively. A `Vec`/`Box`-owned tree needs none of that -- this
//!   module's serializer is ordinary recursion over `&BuiltValue`, and
//!   there is no `BuiltValue`-side `free` at all (`Drop` does it).
//! - **`json_measure_ex` itself turned out to exist purely to pre-size a
//!   `calloc`'d C buffer before `json_serialize_ex` fills it -- and it
//!   deliberately *over*-estimated** (its own arithmetic double-counted
//!   indent width; `bin/otfccdump.rs` used to scan backward over the
//!   resulting buffer's trailing zero padding to find where the real
//!   content actually ended, purely to work around the over-estimate). A
//!   `Vec<u8>`-returning serializer needs no upfront size at all -- it
//!   grows exactly as far as the real content requires, so this module has
//!   no `json_measure_ex` equivalent; `json_serialize_ex` below returns
//!   the exact bytes directly, and `bin/otfccdump.rs`'s scan-for-trailing-
//!   zeros step was deleted along with it.
//! - `JSON_SERIALIZE_MODE_SINGLE_LINE` (the old builder's fallback
//!   `DEFAULT_OPTS` mode) was never reached by any real call site either --
//!   both callers (`bin/otfccdump.rs`, this module's own `preserialize`)
//!   always pass `PACKED` or `MULTILINE` explicitly. `get_serialize_flags`
//!   is still ported in full below (it was cheap and already written), so
//!   nothing was lost by not narrowing further here.

use ::core::ffi::{c_char, c_int};

use crate::vendor::emyg_dtoa::emyg_dtoa;
pub use crate::vendor::json_builder::{
    JSON_SERIALIZE_MODE_MULTILINE, JSON_SERIALIZE_MODE_PACKED, JSON_SERIALIZE_MODE_SINGLE_LINE,
    JSON_SERIALIZE_OPT_CRLF, JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COLON,
    JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COMMA, JSON_SERIALIZE_OPT_PACK_BRACKETS,
    JSON_SERIALIZE_OPT_USE_TABS, JsonSerializeOpts,
};

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

// Stage 10 (complete): `BuiltValue`'s data has been fully safe from the
// start (it's a plain enum over `Vec`/`Box`-owned variants) -- the
// unsafety was entirely in a free-function shell (`json_array_new`/
// `json_object_push`/etc., kept raw-pointer-shaped on purpose so
// `table/*/dump.rs` call sites could stay textually identical to the old C
// `json_builder` idiom during the mechanical c2rust port, the same
// reasoning `support/buffer.rs` used for `Buffer`, Stage 9's target). This
// `impl` is the safe replacement API; every consumer across the crate now
// calls it directly, so the free-function shell itself has been deleted
// (see the plan doc's Stage 10 Phase 12). `into_raw`/`from_raw` below
// remain as the one legitimate bridge across the `FontSerializer`
// type-erasure boundary (`json_writer.rs`/`bin/otfccdump.rs`), matching
// `Buffer::into_raw`/`Buffer::from_raw`'s equivalent role after Stage 9.
impl BuiltValue {
    /// Pre-sized array constructor; `capacity` is a capacity hint only
    /// (`Vec::push` beyond it just reallocates).
    pub fn new_array(capacity: usize) -> BuiltValue {
        BuiltValue::Array(Vec::with_capacity(capacity))
    }

    /// Pre-sized object constructor; see [`new_array`](Self::new_array) on
    /// the capacity hint.
    pub fn new_object(capacity: usize) -> BuiltValue {
        BuiltValue::Object(Vec::with_capacity(capacity))
    }

    /// Appends `value` to `self`, taking ownership of it. No-op (but still
    /// drops `value`) if `self` isn't actually an array -- every real call
    /// site only ever pushes into a value it got from
    /// [`new_array`](Self::new_array) itself, so this never fires in
    /// practice; kept lenient rather than panicking to match this layer's
    /// long-standing read-accessor style.
    pub fn push_item(&mut self, value: BuiltValue) {
        if let BuiltValue::Array(items) = self {
            items.push(value);
        }
    }

    /// Pushes `(key, value)`, `key` copied verbatim (embedded NULs
    /// included) -- matches the old `json_object_push`/
    /// `json_object_push_length`'s raw `memcpy`. No-op (but still drops
    /// `value`) if `self` isn't actually an object.
    pub fn push_field(&mut self, key: &[u8], value: BuiltValue) {
        if let BuiltValue::Object(fields) = self {
            fields.push((key.to_vec(), value));
        }
    }

    /// [`push_field`](Self::push_field), for a `Handle.name`-shaped `&[u8]`
    /// key -- truncates at the first embedded NUL the same way `strlen`
    /// would, matching `parsed_json`'s and the old `json_builder`'s own
    /// `json_object_push_bytes_key`.
    pub fn push_field_bytes_key(&mut self, key: &[u8], value: BuiltValue) {
        let len = key.iter().position(|&b| b == 0).unwrap_or(key.len());
        self.push_field(&key[..len], value);
    }

    /// Push `value` under a four-character OpenType tag, unpacked
    /// big-endian from `tag`.
    pub fn push_tag(&mut self, tag: u32, value: BuiltValue) {
        let tag_bytes: [u8; 4] = [
            ((tag & 0xff000000u32) >> 24) as u8,
            ((tag & 0xff0000u32) >> 16) as u8,
            ((tag & 0xff00u32) >> 8) as u8,
            (tag & 0xffu32) as u8,
        ];
        self.push_field(&tag_bytes, value);
    }

    /// A string value, for a `Handle.name`-shaped `&[u8]` -- truncates at
    /// the first embedded NUL the same way `strlen` would, matching the old
    /// `json_string_new_from_bytes`.
    pub fn str_truncated_at_nul(bytes: &[u8]) -> BuiltValue {
        let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        BuiltValue::Str(bytes[..len].to_vec())
    }

    /// A coordinate, written as an integer when it is one so the JSON stays
    /// readable -- matches the old `json_new_position` exactly. Uses
    /// `f64::round` directly rather than an `extern "C" { fn round(...) }`
    /// declaration: both round half away from zero identically, so there
    /// is nothing left to import libm for.
    pub fn position(z: crate::support::primitives::Pos) -> BuiltValue {
        if z.round() == z {
            BuiltValue::Int(z as i64)
        } else {
            BuiltValue::Double(z)
        }
    }

    /// Serialize a bitfield as a JSON object of `label: true` pairs, one
    /// per set bit -- matches the old `otfcc_dump_flags` exactly.
    pub fn dump_flags(flags: c_int, labels: &[&::core::ffi::CStr]) -> BuiltValue {
        let mut v = BuiltValue::new_object(0);
        for (j, label) in labels.iter().enumerate() {
            if flags & (1 as c_int) << j != 0 {
                v.push_field(label.to_bytes(), BuiltValue::Bool(true));
            }
        }
        v
    }

    /// Serializes `self` now (packed mode) and keeps the bytes, so the
    /// writer can splice them in verbatim later instead of descending into
    /// `self` a second time. Consumes `self` -- matches the old
    /// `preserialize`'s contract exactly, minus the separate
    /// `json_measure_ex`/`malloc`/`json_builder_free` steps that a
    /// `Vec<u8>`-returning serializer makes unnecessary.
    pub fn preserialize(self) -> BuiltValue {
        let opts = JsonSerializeOpts {
            mode: JSON_SERIALIZE_MODE_PACKED,
            opts: 0,
            indent_size: 0,
        };
        let bytes = json_serialize_ex(&self, opts);
        BuiltValue::PreSerialized(bytes)
    }

    // The pair below is the bridge across the `FontSerializer` type-erasure
    // boundary (`json_writer.rs`'s single `.into_raw()` at the very end of
    // `serialize`; `bin/otfccdump.rs`'s matching `from_raw` on the way
    // back), mirroring `Buffer::into_raw`/`Buffer::from_raw`'s role after
    // Stage 9. Not for use anywhere else.
    pub fn into_raw(self) -> *mut BuiltValue {
        Box::into_raw(Box::new(self))
    }
    /// # Safety
    /// `ptr` must either be null or have come from [`BuiltValue::into_raw`]
    /// and not have been freed already.
    pub unsafe fn from_raw(ptr: *mut BuiltValue) -> Option<BuiltValue> {
        if ptr.is_null() {
            None
        } else {
            Some(*unsafe { Box::from_raw(ptr) })
        }
    }
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

    /// The same nested tree shape a former differential test suite built
    /// against `vendor::json_builder` (object containing an array,
    /// strings needing every escape case, positive/negative/zero
    /// integers, a double, both booleans, null, and a nested empty
    /// array/object), built solely through this module's own safe
    /// constructor API.
    fn build_sample_tree() -> BuiltValue {
        let mut root = BuiltValue::new_object(8);

        root.push_field(b"name", BuiltValue::Str(b"quote\" back\\slash\ttab".to_vec()));

        let control_bytes: &[u8] = &[0, 8, 11, 12, 10, 13, b'a', 0xff];
        root.push_field(b"controls", BuiltValue::Str(control_bytes.to_vec()));

        let mut arr = BuiltValue::new_array(5);
        // `i64::MIN` is deliberately excluded, matching what the
        // former differential test against the old builder also had
        // to exclude: this module's own `write_value` uses
        // `i64::to_string()` and handles it correctly, but is kept out
        // of this fixture for continuity with that history rather than
        // for any correctness reason of its own.
        for n in [0i64, 1, -1, 12345, i64::MIN + 1, i64::MAX] {
            arr.push_item(BuiltValue::Int(n));
        }
        root.push_field(b"ints", arr);

        for (dbl, key) in [
            (0.0f64, "zero"),
            (-0.0f64, "negzero"),
            (3.5f64, "frac"),
            (-123.456f64, "negfrac"),
            (1.0e20f64, "big"),
            (1.0e-20f64, "small"),
        ] {
            root.push_field(key.as_bytes(), BuiltValue::Double(dbl));
        }

        root.push_field(b"true", BuiltValue::Bool(true));
        root.push_field(b"false", BuiltValue::Bool(false));
        root.push_field(b"null", BuiltValue::Null);

        root.push_field(b"emptyarr", BuiltValue::new_array(0));
        root.push_field(b"emptyobj", BuiltValue::new_object(0));

        root
    }

    /// Fixed-fixture regression coverage for `json_serialize_ex`'s exact
    /// byte output, replacing what used to be a differential comparison
    /// against `vendor::json_builder`'s own serializer (deleted along with
    /// the rest of the now-fully-superseded old builder -- see
    /// `rust/README.md`'s Stage 6-2.5 C-4 entry). The expected bytes below
    /// were captured from this function's own output after confirming it
    /// matched the old builder byte-for-byte, so this still protects
    /// against an accidental future change to escaping, number formatting,
    /// or bracket/indent spacing -- just without a live second
    /// implementation to compare against.
    #[test]
    // vendor/emyg_dtoa.rs's `prettify` calls libc `memmove` to shift digits
    // when rounding carries a decimal point -- Miri (at least on the macOS
    // target) doesn't implement that foreign function ("unsupported
    // operation", not a bug finding). Untested whether this also reproduces
    // on the Linux target; if CI's miri job passes this test, the ignore
    // can come off.
    #[cfg_attr(
        miri,
        ignore = "libc memmove in vendor/emyg_dtoa.rs unsupported under Miri"
    )]
    fn packed_matches_the_known_good_fixture() {
        let tree = build_sample_tree();
        let opts = JsonSerializeOpts {
            mode: JSON_SERIALIZE_MODE_PACKED,
            opts: 0,
            indent_size: 0,
        };
        let expected: &[u8] = b"\x7b\x22\x6e\x61\x6d\x65\x22\x3a\x22\x71\x75\x6f\x74\x65\x5c\x22\x20\x62\x61\x63\x6b\x5c\x5c\x73\x6c\x61\x73\x68\x5c\x74\x74\x61\x62\x22\x2c\x22\x63\x6f\x6e\x74\x72\x6f\x6c\x73\x22\x3a\x22\x5c\x75\x30\x30\x30\x30\x5c\x62\x5c\x75\x30\x30\x30\x62\x5c\x66\x5c\x6e\x5c\x72\x61\xff\x22\x2c\x22\x69\x6e\x74\x73\x22\x3a\x5b\x30\x2c\x31\x2c\x2d\x31\x2c\x31\x32\x33\x34\x35\x2c\x2d\x39\x32\x32\x33\x33\x37\x32\x30\x33\x36\x38\x35\x34\x37\x37\x35\x38\x30\x37\x2c\x39\x32\x32\x33\x33\x37\x32\x30\x33\x36\x38\x35\x34\x37\x37\x35\x38\x30\x37\x5d\x2c\x22\x7a\x65\x72\x6f\x22\x3a\x30\x2e\x30\x2c\x22\x6e\x65\x67\x7a\x65\x72\x6f\x22\x3a\x30\x2e\x30\x2c\x22\x66\x72\x61\x63\x22\x3a\x33\x2e\x35\x2c\x22\x6e\x65\x67\x66\x72\x61\x63\x22\x3a\x2d\x31\x32\x33\x2e\x34\x35\x36\x2c\x22\x62\x69\x67\x22\x3a\x31\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x2e\x30\x2c\x22\x73\x6d\x61\x6c\x6c\x22\x3a\x31\x65\x2d\x32\x30\x2c\x22\x74\x72\x75\x65\x22\x3a\x74\x72\x75\x65\x2c\x22\x66\x61\x6c\x73\x65\x22\x3a\x66\x61\x6c\x73\x65\x2c\x22\x6e\x75\x6c\x6c\x22\x3a\x6e\x75\x6c\x6c\x2c\x22\x65\x6d\x70\x74\x79\x61\x72\x72\x22\x3a\x5b\x5d\x2c\x22\x65\x6d\x70\x74\x79\x6f\x62\x6a\x22\x3a\x7b\x7d\x7d";
        assert_eq!(json_serialize_ex(&tree, opts), expected);
    }

    #[test]
    // Same reason as packed_matches_the_known_good_fixture above.
    #[cfg_attr(
        miri,
        ignore = "libc memmove in vendor/emyg_dtoa.rs unsupported under Miri"
    )]
    fn multiline_matches_the_known_good_fixture() {
        let tree = build_sample_tree();
        let opts = JsonSerializeOpts {
            mode: JSON_SERIALIZE_MODE_MULTILINE,
            opts: 0,
            indent_size: 4,
        };
        let expected: &[u8] = b"\x7b\x0a\x20\x20\x20\x20\x22\x6e\x61\x6d\x65\x22\x3a\x20\x22\x71\x75\x6f\x74\x65\x5c\x22\x20\x62\x61\x63\x6b\x5c\x5c\x73\x6c\x61\x73\x68\x5c\x74\x74\x61\x62\x22\x2c\x0a\x20\x20\x20\x20\x22\x63\x6f\x6e\x74\x72\x6f\x6c\x73\x22\x3a\x20\x22\x5c\x75\x30\x30\x30\x30\x5c\x62\x5c\x75\x30\x30\x30\x62\x5c\x66\x5c\x6e\x5c\x72\x61\xff\x22\x2c\x0a\x20\x20\x20\x20\x22\x69\x6e\x74\x73\x22\x3a\x20\x5b\x0a\x20\x20\x20\x20\x20\x20\x20\x20\x30\x2c\x0a\x20\x20\x20\x20\x20\x20\x20\x20\x31\x2c\x0a\x20\x20\x20\x20\x20\x20\x20\x20\x2d\x31\x2c\x0a\x20\x20\x20\x20\x20\x20\x20\x20\x31\x32\x33\x34\x35\x2c\x0a\x20\x20\x20\x20\x20\x20\x20\x20\x2d\x39\x32\x32\x33\x33\x37\x32\x30\x33\x36\x38\x35\x34\x37\x37\x35\x38\x30\x37\x2c\x0a\x20\x20\x20\x20\x20\x20\x20\x20\x39\x32\x32\x33\x33\x37\x32\x30\x33\x36\x38\x35\x34\x37\x37\x35\x38\x30\x37\x0a\x20\x20\x20\x20\x5d\x2c\x0a\x20\x20\x20\x20\x22\x7a\x65\x72\x6f\x22\x3a\x20\x30\x2e\x30\x2c\x0a\x20\x20\x20\x20\x22\x6e\x65\x67\x7a\x65\x72\x6f\x22\x3a\x20\x30\x2e\x30\x2c\x0a\x20\x20\x20\x20\x22\x66\x72\x61\x63\x22\x3a\x20\x33\x2e\x35\x2c\x0a\x20\x20\x20\x20\x22\x6e\x65\x67\x66\x72\x61\x63\x22\x3a\x20\x2d\x31\x32\x33\x2e\x34\x35\x36\x2c\x0a\x20\x20\x20\x20\x22\x62\x69\x67\x22\x3a\x20\x31\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x2e\x30\x2c\x0a\x20\x20\x20\x20\x22\x73\x6d\x61\x6c\x6c\x22\x3a\x20\x31\x65\x2d\x32\x30\x2c\x0a\x20\x20\x20\x20\x22\x74\x72\x75\x65\x22\x3a\x20\x74\x72\x75\x65\x2c\x0a\x20\x20\x20\x20\x22\x66\x61\x6c\x73\x65\x22\x3a\x20\x66\x61\x6c\x73\x65\x2c\x0a\x20\x20\x20\x20\x22\x6e\x75\x6c\x6c\x22\x3a\x20\x6e\x75\x6c\x6c\x2c\x0a\x20\x20\x20\x20\x22\x65\x6d\x70\x74\x79\x61\x72\x72\x22\x3a\x20\x5b\x5d\x2c\x0a\x20\x20\x20\x20\x22\x65\x6d\x70\x74\x79\x6f\x62\x6a\x22\x3a\x20\x7b\x7d\x0a\x7d";
        assert_eq!(json_serialize_ex(&tree, opts), expected);
    }

    #[test]
    fn preserialize_splices_verbatim_into_a_parent() {
        // A subtree built and preserialized in isolation, then spliced
        // into a fresh parent, must serialize identically to the same
        // subtree built directly inside that parent -- the whole point of
        // `PreSerialized` is that splicing it changes nothing observable.
        let mut sub_a = BuiltValue::new_object(1);
        sub_a.push_field(b"x", BuiltValue::Int(42));
        let pre = sub_a.preserialize();

        let mut parent_with_pre = BuiltValue::new_object(1);
        parent_with_pre.push_field(b"child", pre);

        let sub_b = BuiltValue::Object(vec![(b"x".to_vec(), BuiltValue::Int(42))]);
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

    #[test]
    fn safe_api_builds_and_serializes_an_object_with_no_unsafe() {
        let mut obj = BuiltValue::new_object(2);
        obj.push_field(b"a", BuiltValue::Int(1));
        obj.push_field(b"b", BuiltValue::Bool(true));
        let mut arr = BuiltValue::new_array(2);
        arr.push_item(BuiltValue::Int(1));
        arr.push_item(BuiltValue::Null);
        obj.push_field(b"c", arr);
        let opts = JsonSerializeOpts {
            mode: JSON_SERIALIZE_MODE_PACKED,
            opts: 0,
            indent_size: 0,
        };
        assert_eq!(
            json_serialize_ex(&obj, opts),
            br#"{"a":1,"b":true,"c":[1,null]}"#
        );
    }

    #[test]
    fn safe_api_push_item_on_a_non_array_is_a_lenient_no_op() {
        let mut not_an_array = BuiltValue::Int(0);
        not_an_array.push_item(BuiltValue::Bool(true));
        assert_eq!(not_an_array, BuiltValue::Int(0));
    }

    #[test]
    fn safe_api_push_field_on_a_non_object_is_a_lenient_no_op() {
        let mut not_an_object = BuiltValue::Null;
        not_an_object.push_field(b"key", BuiltValue::Bool(true));
        assert_eq!(not_an_object, BuiltValue::Null);
    }

    #[test]
    fn safe_api_push_field_bytes_key_truncates_at_the_first_nul() {
        let mut obj = BuiltValue::new_object(1);
        obj.push_field_bytes_key(b"abc\0def", BuiltValue::Int(1));
        assert_eq!(obj, BuiltValue::Object(vec![(b"abc".to_vec(), BuiltValue::Int(1))]));
    }

    #[test]
    fn safe_api_push_tag_unpacks_big_endian() {
        let mut obj = BuiltValue::new_object(1);
        obj.push_tag(0x47535542u32, BuiltValue::Bool(true)); // "GSUB"
        assert_eq!(
            obj,
            BuiltValue::Object(vec![(b"GSUB".to_vec(), BuiltValue::Bool(true))])
        );
    }

    #[test]
    fn safe_api_str_truncated_at_nul_matches_the_free_function() {
        assert_eq!(
            BuiltValue::str_truncated_at_nul(b"xy\0z"),
            BuiltValue::Str(b"xy".to_vec())
        );
        assert_eq!(
            BuiltValue::str_truncated_at_nul(b"no-nul"),
            BuiltValue::Str(b"no-nul".to_vec())
        );
    }

    #[test]
    fn safe_api_position_picks_int_or_double() {
        assert_eq!(BuiltValue::position(3.0), BuiltValue::Int(3));
        assert_eq!(BuiltValue::position(-2.0), BuiltValue::Int(-2));
        assert_eq!(BuiltValue::position(3.5), BuiltValue::Double(3.5));
    }

    #[test]
    fn safe_api_dump_flags_emits_only_set_bits() {
        let labels: &[&::core::ffi::CStr] = &[c"bold", c"italic", c"underline"];
        // bit 0 (bold) and bit 2 (underline) set, bit 1 (italic) clear.
        let v = BuiltValue::dump_flags(0b101, labels);
        assert_eq!(
            v,
            BuiltValue::Object(vec![
                (b"bold".to_vec(), BuiltValue::Bool(true)),
                (b"underline".to_vec(), BuiltValue::Bool(true)),
            ])
        );
    }

    #[test]
    fn safe_api_preserialize_matches_a_direct_serialize() {
        let mut obj = BuiltValue::new_object(1);
        obj.push_field(b"x", BuiltValue::Int(42));
        let pre = obj.clone().preserialize();
        let opts = JsonSerializeOpts {
            mode: JSON_SERIALIZE_MODE_PACKED,
            opts: 0,
            indent_size: 0,
        };
        assert_eq!(json_serialize_ex(&pre, opts), json_serialize_ex(&obj, opts));
        match pre {
            BuiltValue::PreSerialized(bytes) => assert_eq!(bytes, br#"{"x":42}"#),
            _ => panic!("expected PreSerialized"),
        }
    }
}
