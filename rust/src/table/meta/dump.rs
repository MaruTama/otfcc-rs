#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};
use crate::logger::{ILogger};
use crate::support::options::{Options};

use crate::table::meta::types::{MetaEntry, MetaTable};
use crate::support::base64::{base64_encode};
use crate::support::built_json::{BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_string_new_length};
use crate::vendor::sds::{sdsempty};
#[inline]
unsafe extern "C" fn is_string_tag(mut tag: u32) -> bool {
    return tag == crate::tag::TAG_DLNG || tag == crate::tag::TAG_SLNG;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_dump_meta(
    meta: Option<&MetaTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
) {
    let meta = match meta {
        Some(m) => m,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"meta"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _meta: *mut BuiltValue = json_object_new(3 as usize);
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
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < entries.len() {
            let mut e: *const MetaEntry = &entries[__caryll_index];
            while keep != 0 {
                let mut _e: *mut BuiltValue = json_object_new(2 as usize);
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
                    let mut out_len: usize = 0 as usize;
                    let mut out: *mut u8 = base64_encode(
                        (*e).data.as_ptr(),
                        (*e).data.len(),
                        &raw mut out_len,
                    );
                    json_object_push(
                        _e,
                        b"base64\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new_length(
                            out_len as ::core::ffi::c_uint,
                            out as *mut ::core::ffi::c_char,
                        ),
                    );
                    free(out as *mut ::core::ffi::c_void);
                    out = ::core::ptr::null_mut::<u8>();
                }
                json_array_push(_entries, _e);
                keep = (keep == 0) as ::core::ffi::c_int as usize;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
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
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
#[inline]
unsafe extern "C" fn tag2str(mut tag: u32, mut tags: *mut ::core::ffi::c_char) {
    *tags.offset(0 as ::core::ffi::c_int as isize) =
        (tag >> 24 as ::core::ffi::c_int & 0xff as u32) as ::core::ffi::c_char;
    *tags.offset(1 as ::core::ffi::c_int as isize) =
        (tag >> 16 as ::core::ffi::c_int & 0xff as u32) as ::core::ffi::c_char;
    *tags.offset(2 as ::core::ffi::c_int as isize) =
        (tag >> 8 as ::core::ffi::c_int & 0xff as u32) as ::core::ffi::c_char;
    *tags.offset(3 as ::core::ffi::c_int as isize) =
        (tag & 0xff as u32) as ::core::ffi::c_char;
}
