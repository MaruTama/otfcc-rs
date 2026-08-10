#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::free;
use crate::support::json_funcs::{json_obj_get_type};
use crate::logger::{ILogger};
use crate::support::options::{Options};
use crate::vendor::json::{JsonType, JsonValue};

use crate::table::meta::types::{MetaEntry, MetaTable};
use crate::support::base64::{base64_decode};
use crate::vendor::sds::{sdsempty};
// `extern "C"` is a c2rust artifact -- this is only ever called from
// `otfcc_parse_meta` in this same file, never across a real FFI boundary,
// same reasoning as every other `#[allow(improper_ctypes_definitions)]`
// in this migration.
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn parse_meta_data(mut v: *const JsonValue) -> Option<Vec<u8>> {
    if (*v).type_0 == JsonType::String
    {
        return Some(::core::slice::from_raw_parts(
            (*v).u.string.ptr as *const u8,
            (*v).u.string.length as usize,
        ).to_vec());
    } else if (*v).type_0 == JsonType::Object
    {
        let mut _string: *mut JsonValue = json_obj_get_type(
            v,
            b"string\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::String,
        );
        if !_string.is_null() {
            return Some(::core::slice::from_raw_parts(
                (*_string).u.string.ptr as *const u8,
                (*_string).u.string.length as usize,
            ).to_vec());
        }
        let mut _base64: *mut JsonValue = json_obj_get_type(
            v,
            b"base64\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::String,
        );
        if !_base64.is_null() {
            let mut str_len: usize = 0 as usize;
            let mut str: *mut u8 = base64_decode(
                (*_base64).u.string.ptr as *mut u8,
                (*_base64).u.string.length as usize,
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
pub unsafe extern "C" fn otfcc_parse_meta(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> Option<Box<MetaTable>> {
    let mut _meta: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    _meta = json_obj_get_type(
        root,
        b"meta\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if _meta.is_null() {
        return None;
    }
    let mut _meta_entries: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    _meta_entries = json_obj_get_type(
        _meta,
        b"entries\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _meta_entries.is_null() {
        return None;
    }
    let mut meta: Box<MetaTable> = Box::new(MetaTable { version: 1, flags: 0, entries: Vec::new() });
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"meta"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: usize = 0 as usize;
        while j < (*_meta_entries).u.array.length as usize {
            let mut _e: *mut JsonValue =
                *(*_meta_entries).u.array.values.offset(j as isize) as *mut JsonValue;
            let mut _tag: *mut JsonValue = json_obj_get_type(
                _e,
                b"tag\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            if !(_tag.is_null() || (*_tag).u.string.length != 4 as ::core::ffi::c_uint) {
                let mut tag: u32 = str2tag((*_tag).u.string.ptr);
                if let Some(data) = parse_meta_data(_e) {
                    (*meta).entries.push(MetaEntry {
                        tag: tag,
                        data,
                    });
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    return Some(meta);
}
#[inline]
unsafe extern "C" fn str2tag(mut tags: *const ::core::ffi::c_char) -> u32 {
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
