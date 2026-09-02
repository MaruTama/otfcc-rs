#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::options::Options;

use crate::support::base64::base64_encode;
use crate::support::built_json::BuiltValue;
use crate::table::meta::types::{MetaEntry, MetaTable};
#[inline]
unsafe fn is_string_tag(tag: u32) -> bool {
    return tag == crate::tag::TAG_DLNG || tag == crate::tag::TAG_SLNG;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_meta(
    meta: Option<&MetaTable>,
    root: &mut BuiltValue,
    options: &Options,
) {
    let meta = match meta {
        Some(m) => m,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"meta"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _meta = BuiltValue::new_object(3);
        _meta.push_field(b"version", BuiltValue::Int((*meta).version as i64));
        _meta.push_field(b"flags", BuiltValue::Int((*meta).flags as i64));
        let entries: &Vec<MetaEntry> = &(*meta).entries;
        let mut _entries = BuiltValue::new_array(entries.len());
        for e in entries.iter() {
            let mut _e = BuiltValue::new_object(2);
            let tag_bytes: [u8; 4] = [
                ((e.tag & 0xff000000u32) >> 24) as u8,
                ((e.tag & 0xff0000u32) >> 16) as u8,
                ((e.tag & 0xff00u32) >> 8) as u8,
                (e.tag & 0xffu32) as u8,
            ];
            _e.push_field(b"tag", BuiltValue::Str(tag_bytes.to_vec()));
            if is_string_tag(e.tag) {
                _e.push_field(b"string", BuiltValue::Str(e.data.clone()));
            } else {
                let encoded = base64_encode(&e.data);
                _e.push_field(b"base64", BuiltValue::Str(encoded));
            }
            _entries.push_item(_e);
        }
        _meta.push_field(b"entries", _entries);
        root.push_field(b"meta", _meta);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
