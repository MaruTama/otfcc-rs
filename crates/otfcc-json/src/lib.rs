//! otfcc's JSON layer: the parser that reads a font's JSON, the value tree
//! and serializer that write it, and the float formatting both directions
//! rely on.
//!
//! It knows nothing about fonts. The rules that are about fonts, such as
//! the 16-bit limit on collection sizes, stay in `otfcc_rust`
//! (`support::json_limits`).
//!
//! The output format is fixed: otfcc's golden tests pin the exact bytes, so
//! floats go through the vendored Grisu2 `dtoa` rather than Rust's own
//! formatting, which would sometimes pick different digits.
#![deny(unsafe_code)]

mod build;
mod dtoa;
mod kind;
mod parse;

pub use build::{
    BuiltValue, JSON_SERIALIZE_MODE_MULTILINE, JSON_SERIALIZE_MODE_PACKED,
    JSON_SERIALIZE_MODE_SINGLE_LINE, JSON_SERIALIZE_OPT_CRLF, JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COLON,
    JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COMMA, JSON_SERIALIZE_OPT_PACK_BRACKETS,
    JSON_SERIALIZE_OPT_USE_TABS, JsonSerializeOpts, json_serialize_ex,
};
pub use kind::JsonType;
pub use parse::{ParsedValue, parse_json};
