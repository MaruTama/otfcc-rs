// This file used to hold the vendored `json-parser` library's parser
// (`json_parse_ex` and its whole state machine, `JsonValue`/
// `JsonObjectEntry`/the union payload it read into, `JsonSettings`/
// `JsonState`). Stage 6-2.5 replaced it on the read side with
// `support::parsed_json::{ParsedValue, parse_json}` (C-2) and confirmed,
// once every real consumer had moved over, that nothing in this crate
// still called any of it (C-4) -- so the parser and its types were deleted
// rather than kept as dead weight. `JsonType` is the one piece that
// survives: `support::parsed_json`/`support::built_json`'s own APIs still
// use it as a plain type-tag argument (`json_obj_get_type`/`json_type_of`
// and friends), independent of the parser that originally defined it
// alongside.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum JsonType {
    None = 0,
    Object = 1,
    Array = 2,
    Integer = 3,
    Double = 4,
    String = 5,
    Boolean = 6,
    Null = 7,
    PreSerialized = 8,
}
