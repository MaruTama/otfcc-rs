#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::options::Options;
use crate::support::parsed_json::ParsedValue;
use crate::vendor::json::JsonType;

use crate::support::base64::base64_decode;
use crate::table::meta::types::{MetaEntry, MetaTable};
// `extern "C"` is a c2rust artifact -- this is only ever called from
// `otfcc_parse_meta` in this same file, never across a real FFI boundary,
// same reasoning as every other `#[allow(improper_ctypes_definitions)]`
// in this migration.
#[allow(improper_ctypes_definitions)]
pub fn parse_meta_data(v: Option<&ParsedValue>) -> Option<Vec<u8>> {
    let v = v?;
    if let Some(bytes) = v.as_str_bytes() {
        return Some(bytes.to_vec());
    }
    if v.as_object().is_some() {
        if let Some(bytes) = v.get_bytes(b"string") {
            return Some(bytes.to_vec());
        }
        if let Some(bytes) = v.get_bytes(b"base64") {
            // Unlike the `string` field above, a malformed `base64` field
            // (a character count not a multiple of 4) now makes this whole
            // entry `None` instead of silently keeping an empty decoded
            // value -- the same "drop what doesn't parse" choice already
            // made everywhere else malformed JSON-build input is handled.
            return base64_decode(bytes);
        }
    }
    None
}
pub unsafe fn otfcc_parse_meta(
    root: &ParsedValue,
    options: &Options,
) -> Option<Box<MetaTable>> {
    let _meta = root.get_typed(b"meta", JsonType::Object)?;
    let entries = _meta
        .get_typed(b"entries", JsonType::Array)
        .and_then(ParsedValue::as_array)?;
    let mut meta: Box<MetaTable> = Box::new(MetaTable {
        version: 1,
        flags: 0,
        entries: Vec::new(),
    });
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"meta"),
    );
    for _e in entries {
        let Some(tag_bytes) = _e
            .get_typed(b"tag", JsonType::String)
            .and_then(ParsedValue::as_str_bytes)
            .filter(|b| b.len() == 4)
        else {
            continue;
        };
        let tag: u32 = str2tag(Some(tag_bytes));
        if let Some(data) = parse_meta_data(Some(_e)) {
            meta.entries.push(MetaEntry { tag, data });
        }
    }
    logger_finish(&mut *options.logger.borrow_mut());
    Some(meta)
}
#[inline]
fn str2tag(tags: Option<&[u8]>) -> u32 {
    let Some(tags) = tags else {
        return 0_u32;
    };
    let mut tag: u32 = 0_u32;
    let mut len: u8 = 0_u8;
    for &b in tags.iter().take(4) {
        tag = tag << 8_i32 | b as u32;
        len = len.wrapping_add(1);
    }
    while (len as i32) < 4_i32 {
        tag = tag << 8_i32 | ' ' as i32 as u32;
        len = len.wrapping_add(1);
    }
    tag
}
