#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::options::Options;
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_str_len, json_str_ptr,
    json_type_of,
};
use crate::vendor::json::JsonType;
use libc::free;

use crate::support::base64::base64_decode;
use crate::table::meta::types::{MetaEntry, MetaTable};
// `extern "C"` is a c2rust artifact -- this is only ever called from
// `otfcc_parse_meta` in this same file, never across a real FFI boundary,
// same reasoning as every other `#[allow(improper_ctypes_definitions)]`
// in this migration.
#[allow(improper_ctypes_definitions)]
pub unsafe fn parse_meta_data(mut v: *const ParsedValue) -> Option<Vec<u8>> {
    if json_type_of(v) == JsonType::String {
        return Some(
            ::core::slice::from_raw_parts(json_str_ptr(v) as *const u8, json_str_len(v) as usize)
                .to_vec(),
        );
    } else if json_type_of(v) == JsonType::Object {
        let mut _string: *const ParsedValue = json_obj_get_type(
            v,
            b"string\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::String,
        );
        if !_string.is_null() {
            return Some(
                ::core::slice::from_raw_parts(
                    json_str_ptr(_string) as *const u8,
                    json_str_len(_string) as usize,
                )
                .to_vec(),
            );
        }
        let mut _base64: *const ParsedValue = json_obj_get_type(
            v,
            b"base64\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::String,
        );
        if !_base64.is_null() {
            let mut str_len: usize = 0 as usize;
            let mut str: *mut u8 = base64_decode(
                json_str_ptr(_base64) as *mut u8,
                json_str_len(_base64) as usize,
                &raw mut str_len,
            );
            let s: Vec<u8> = ::core::slice::from_raw_parts(str, str_len).to_vec();
            free(str as *mut ::core::ffi::c_void);
            str = ::core::ptr::null_mut::<u8>();
            return Some(s);
        }
    }
    None
}
pub unsafe fn otfcc_parse_meta(
    mut root: *const ParsedValue,
    options: &Options,
) -> Option<Box<MetaTable>> {
    let mut _meta: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    _meta = json_obj_get_type(
        root,
        b"meta\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if _meta.is_null() {
        return None;
    }
    let mut _meta_entries: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    _meta_entries = json_obj_get_type(
        _meta,
        b"entries\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _meta_entries.is_null() {
        return None;
    }
    let mut meta: Box<MetaTable> = Box::new(MetaTable {
        version: 1,
        flags: 0,
        entries: Vec::new(),
    });
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"meta"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: usize = 0 as usize;
        while j < json_arr_len(_meta_entries) as usize {
            let mut _e: *const ParsedValue = json_arr_at(_meta_entries, j as u32);
            let mut _tag: *const ParsedValue = json_obj_get_type(
                _e,
                b"tag\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            if !(_tag.is_null() || json_str_len(_tag) != 4 as ::core::ffi::c_uint) {
                let mut tag: u32 = str2tag(json_str_ptr(_tag));
                if let Some(data) = parse_meta_data(_e) {
                    (*meta).entries.push(MetaEntry { tag: tag, data });
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    return Some(meta);
}
#[inline]
unsafe fn str2tag(mut tags: *const ::core::ffi::c_char) -> u32 {
    if tags.is_null() {
        return 0 as u32;
    }
    let mut tag: u32 = 0 as u32;
    let mut len: u8 = 0 as u8;
    while *tags as ::core::ffi::c_int != 0 && (len as ::core::ffi::c_int) < 4 as ::core::ffi::c_int
    {
        tag = tag << 8 as ::core::ffi::c_int | *tags as u32;
        tags = tags.offset(1);
        len = len.wrapping_add(1);
    }
    while (len as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
        tag = tag << 8 as ::core::ffi::c_int | ' ' as i32 as u32;
        len = len.wrapping_add(1);
    }
    return tag;
}
