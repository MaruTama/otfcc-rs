#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::free;
use crate::support::json_funcs::{json_obj_get_type};
use crate::logger::{ILogger};
use crate::support::options::{Options};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};

use crate::table::meta::types::{MetaEntry, MetaTable};
use crate::support::base64::{base64_decode};
use crate::table::meta::types::{meta_iEntries, table_iMeta};
use crate::vendor::sds::{sdsempty, sdsnewlen};
pub unsafe extern "C" fn parseMetaData(mut v: *const JsonValue) -> SdsRaw {
    if (*v).type_0 == JsonType::String
    {
        return sdsnewlen(
            (*v).u.string.ptr as *const ::core::ffi::c_void,
            (*v).u.string.length as usize,
        );
    } else if (*v).type_0 == JsonType::Object
    {
        let mut _string: *mut JsonValue = json_obj_get_type(
            v,
            b"string\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::String,
        );
        if !_string.is_null() {
            return sdsnewlen(
                (*_string).u.string.ptr as *const ::core::ffi::c_void,
                (*_string).u.string.length as usize,
            );
        }
        let mut _base64: *mut JsonValue = json_obj_get_type(
            v,
            b"base64\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::String,
        );
        if !_base64.is_null() {
            let mut strLen: usize = 0 as usize;
            let mut str: *mut ::core::ffi::c_char = base64_decode(
                (*_base64).u.string.ptr as *mut u8,
                (*_base64).u.string.length as usize,
                &raw mut strLen,
            ) as *mut ::core::ffi::c_char;
            let mut s: SdsRaw = sdsnewlen(str as *const ::core::ffi::c_void, strLen);
            free(str as *mut ::core::ffi::c_void);
            str = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return s;
        }
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
pub unsafe extern "C" fn otfcc_parseMeta(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut MetaTable {
    let mut _meta: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    _meta = json_obj_get_type(
        root,
        b"meta\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if _meta.is_null() {
        return ::core::ptr::null_mut::<MetaTable>();
    }
    let mut _meta_entries: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    _meta_entries = json_obj_get_type(
        _meta,
        b"entries\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _meta_entries.is_null() {
        return ::core::ptr::null_mut::<MetaTable>();
    }
    let mut meta: *mut MetaTable = (
        table_iMeta.create.expect("non-null function pointer"))();
    (*(*options).logger)
        .startSDS
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
                let mut str: SdsRaw = parseMetaData(_e);
                if !str.is_null() {
                    meta_iEntries.push.expect("non-null function pointer")(
                        &raw mut (*meta).entries,
                        MetaEntry {
                            tag: tag,
                            data: str,
                        },
                    );
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    return meta;
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
