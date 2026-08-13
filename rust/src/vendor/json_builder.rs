// This file used to hold the vendored `json-builder` library's builder
// (`JsonBuilderValue`, `builderize`, `json_object_new`/`json_array_push`/
// every other `json_*_new`/`json_*_push` constructor, `json_object_sort`/
// `json_object_merge`, `json_measure_ex`/`json_serialize_ex`, and
// `json_builder_free`). Stage 6-2.5 replaced it on the write side with
// `support::built_json::{BuiltValue, ...}` (C-3) and confirmed, once every
// real consumer had moved over, that nothing in this crate still called
// any of it (C-4) -- so the builder was deleted rather than kept as dead
// weight. `JsonSerializeOpts` and the mode/option constants below are the
// one piece that survives: they're plain data with no dependency on the
// builder's own `JsonValue`-based representation, and
// `support::built_json` still re-exports them for real use by
// `bin/otfccdump.rs` and its own `preserialize`.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JsonSerializeOpts {
    pub mode: ::core::ffi::c_int,
    pub opts: ::core::ffi::c_int,
    pub indent_size: ::core::ffi::c_int,
}
pub const JSON_SERIALIZE_MODE_MULTILINE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_SERIALIZE_MODE_SINGLE_LINE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JSON_SERIALIZE_MODE_PACKED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const JSON_SERIALIZE_OPT_CRLF: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const JSON_SERIALIZE_OPT_PACK_BRACKETS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COMMA: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COLON: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const JSON_SERIALIZE_OPT_USE_TABS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
