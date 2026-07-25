#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, strcmp};
use crate::logger::{otfcc_ILogger};
use crate::support::options::{otfcc_Options};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_array, json_object, json_string, json_type, json_value};

use crate::table::meta::types::{__caryll_elementinterface_table_meta, __caryll_vectorinterface_meta_Entries, meta_Entry, table_meta};
unsafe extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    static meta_iEntries: __caryll_vectorinterface_meta_Entries;
    static table_iMeta: __caryll_elementinterface_table_meta;
    fn base64_decode(src: *const u8, len: usize, out_len: *mut usize) -> *mut u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parseMetaData(mut v: *const json_value) -> sds {
    if (*v).type_0 as ::core::ffi::c_uint
        == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return sdsnewlen(
            (*v).u.string.ptr as *const ::core::ffi::c_void,
            (*v).u.string.length as usize,
        );
    } else if (*v).type_0 as ::core::ffi::c_uint
        == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut _string: *mut json_value = json_obj_get_type(
            v,
            b"string\0" as *const u8 as *const ::core::ffi::c_char,
            json_string,
        );
        if !_string.is_null() {
            return sdsnewlen(
                (*_string).u.string.ptr as *const ::core::ffi::c_void,
                (*_string).u.string.length as usize,
            );
        }
        let mut _base64: *mut json_value = json_obj_get_type(
            v,
            b"base64\0" as *const u8 as *const ::core::ffi::c_char,
            json_string,
        );
        if !_base64.is_null() {
            let mut strLen: usize = 0 as usize;
            let mut str: *mut ::core::ffi::c_char = base64_decode(
                (*_base64).u.string.ptr as *mut u8,
                (*_base64).u.string.length as usize,
                &raw mut strLen,
            ) as *mut ::core::ffi::c_char;
            let mut s: sds = sdsnewlen(str as *const ::core::ffi::c_void, strLen);
            free(str as *mut ::core::ffi::c_void);
            str = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return s;
        }
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_parseMeta(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_meta {
    let mut _meta: *mut json_value = ::core::ptr::null_mut::<json_value>();
    _meta = json_obj_get_type(
        root,
        b"meta\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if _meta.is_null() {
        return ::core::ptr::null_mut::<table_meta>();
    }
    let mut _meta_entries: *mut json_value = ::core::ptr::null_mut::<json_value>();
    _meta_entries = json_obj_get_type(
        _meta,
        b"entries\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    if _meta_entries.is_null() {
        return ::core::ptr::null_mut::<table_meta>();
    }
    let mut meta: *mut table_meta = (
        table_iMeta.create.expect("non-null function pointer"))();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"meta"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: usize = 0 as usize;
        while j < (*_meta_entries).u.array.length as usize {
            let mut _e: *mut json_value =
                *(*_meta_entries).u.array.values.offset(j as isize) as *mut json_value;
            let mut _tag: *mut json_value = json_obj_get_type(
                _e,
                b"tag\0" as *const u8 as *const ::core::ffi::c_char,
                json_string,
            );
            if !(_tag.is_null() || (*_tag).u.string.length != 4 as ::core::ffi::c_uint) {
                let mut tag: u32 = str2tag((*_tag).u.string.ptr);
                let mut str: sds = parseMetaData(_e);
                if !str.is_null() {
                    meta_iEntries.push.expect("non-null function pointer")(
                        &raw mut (*meta).entries,
                        meta_Entry {
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
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    return meta;
}
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<json_value>();
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        }
        _k = _k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_get_type(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    type_0: json_type,
) -> *mut json_value {
    let mut v: *mut json_value = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return v;
    }
    return ::core::ptr::null_mut::<json_value>();
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
