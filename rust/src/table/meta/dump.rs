#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::options::Options;

use crate::support::base64::base64_encode;
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new,
    json_object_push, json_string_new_length,
};
use crate::table::meta::types::{MetaEntry, MetaTable};
#[inline]
unsafe fn is_string_tag(tag: u32) -> bool {
    return tag == crate::tag::TAG_DLNG || tag == crate::tag::TAG_SLNG;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_meta(
    meta: Option<&MetaTable>,
    root: *mut BuiltValue,
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
        let mut _meta: *mut BuiltValue = json_object_new(3_usize);
        json_object_push(
            _meta,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*meta).version as i64),
        );
        json_object_push(
            _meta,
            b"flags\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*meta).flags as i64),
        );
        let entries: &Vec<MetaEntry> = &(*meta).entries;
        let mut _entries: *mut BuiltValue = json_array_new(entries.len());
        let mut __caryll_index: usize = 0_usize;
        let mut keep: usize = 1_usize;
        while keep != 0 && __caryll_index < entries.len() {
            let e: *const MetaEntry = &entries[__caryll_index];
            while keep != 0 {
                let mut _e: *mut BuiltValue = json_object_new(2_usize);
                let mut _tag: [::core::ffi::c_char; 4] = [0; 4];
                tag2str((*e).tag, &raw mut _tag as *mut ::core::ffi::c_char);
                json_object_push(
                    _e,
                    b"tag\0" as *const u8 as *const ::core::ffi::c_char,
                    json_string_new_length(
                        4 as ::core::ffi::c_uint,
                        &raw mut _tag as *mut ::core::ffi::c_char,
                    ),
                );
                if is_string_tag((*e).tag) {
                    json_object_push(
                        _e,
                        b"string\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new_length(
                            (*e).data.len() as ::core::ffi::c_uint,
                            (*e).data.as_ptr() as *const ::core::ffi::c_char,
                        ),
                    );
                } else {
                    let encoded = base64_encode(&(*e).data);
                    json_object_push(
                        _e,
                        b"base64\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new_length(
                            encoded.len() as ::core::ffi::c_uint,
                            encoded.as_ptr() as *mut ::core::ffi::c_char,
                        ),
                    );
                }
                json_array_push(_entries, _e);
                keep = (keep == 0) as i32 as usize;
            }
            keep = (keep == 0) as i32 as usize;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        json_object_push(
            _meta,
            b"entries\0" as *const u8 as *const ::core::ffi::c_char,
            _entries,
        );
        json_object_push(
            root,
            b"meta\0" as *const u8 as *const ::core::ffi::c_char,
            _meta,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
#[inline]
unsafe fn tag2str(tag: u32, tags: *mut ::core::ffi::c_char) {
    *tags.offset(0_i32 as isize) =
        (tag >> 24_i32 & 0xff_u32) as ::core::ffi::c_char;
    *tags.offset(1_i32 as isize) =
        (tag >> 16_i32 & 0xff_u32) as ::core::ffi::c_char;
    *tags.offset(2_i32 as isize) =
        (tag >> 8_i32 & 0xff_u32) as ::core::ffi::c_char;
    *tags.offset(3_i32 as isize) = (tag & 0xff_u32) as ::core::ffi::c_char;
}
