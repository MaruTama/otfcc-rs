use libc::{free};
use crate::logger::{otfcc_ILogger};
use crate::support::options::{otfcc_Options};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_value};
extern "C" {
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new_length(
        length: ::core::ffi::c_uint,
        _: *const ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn base64_encode(src: *const u8, len: usize, out_len: *mut usize) -> *mut u8;
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: u8,
    pub alloc: u8,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: u16,
    pub alloc: u16,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: u32,
    pub alloc: u32,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: u64,
    pub alloc: u64,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct meta_Entry {
    pub tag: u32,
    pub data: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct meta_Entries {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut meta_Entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_meta {
    pub version: u32,
    pub flags: u32,
    pub entries: meta_Entries,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SDS_TYPE_5: ::core::ffi::c_int = 0;
pub const SDS_TYPE_8: ::core::ffi::c_int = 1;
pub const SDS_TYPE_16: ::core::ffi::c_int = 2;
pub const SDS_TYPE_32: ::core::ffi::c_int = 3;
pub const SDS_TYPE_64: ::core::ffi::c_int = 4;
pub const SDS_TYPE_MASK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SDS_TYPE_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
#[inline]
unsafe extern "C" fn isStringTag(mut tag: u32) -> bool {
    return tag == 1684827751i32 as u32 || tag == 1936485991i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpMeta(
    mut meta: *const table_meta,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if meta.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"meta\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _meta: *mut json_value = json_object_new(3 as usize);
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
        let mut _entries: *mut json_value = json_array_new((*meta).entries.length);
        json_object_push(
            _meta,
            b"entries\0" as *const u8 as *const ::core::ffi::c_char,
            _entries,
        );
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < (*meta).entries.length {
            let mut e: *mut meta_Entry = (*meta).entries.items.offset(__caryll_index as isize);
            while keep != 0 {
                let mut _e: *mut json_value = json_object_new(2 as usize);
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
                if isStringTag((*e).tag) {
                    json_object_push(
                        _e,
                        b"string\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new_length(
                            sdslen((*e).data) as ::core::ffi::c_uint,
                            (*e).data as *const ::core::ffi::c_char,
                        ),
                    );
                } else {
                    let mut outLen: usize = 0 as usize;
                    let mut out: *mut u8 = base64_encode(
                        (*e).data as *mut u8,
                        sdslen((*e).data),
                        &raw mut outLen,
                    );
                    json_object_push(
                        _e,
                        b"base64\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new_length(
                            outLen as ::core::ffi::c_uint,
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
            root,
            b"meta\0" as *const u8 as *const ::core::ffi::c_char,
            _meta,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
