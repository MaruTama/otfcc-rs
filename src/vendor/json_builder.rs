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
pub struct JsonSerializeOpts {
    pub mode: i32,
    pub opts: i32,
    pub indent_size: i32,
}
pub const JSON_SERIALIZE_MODE_MULTILINE: i32 = 0_i32;
pub const JSON_SERIALIZE_MODE_SINGLE_LINE: i32 = 1_i32;
pub const JSON_SERIALIZE_MODE_PACKED: i32 = 2_i32;
pub const JSON_SERIALIZE_OPT_CRLF: i32 =
    1_i32 << 1_i32;
pub const JSON_SERIALIZE_OPT_PACK_BRACKETS: i32 =
    1_i32 << 2_i32;
pub const JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COMMA: i32 =
    1_i32 << 3_i32;
pub const JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COLON: i32 =
    1_i32 << 4_i32;
pub const JSON_SERIALIZE_OPT_USE_TABS: i32 =
    1_i32 << 5_i32;
