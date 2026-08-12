#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};


use crate::support::json_funcs::{json_arr_at, json_arr_len, json_dbl_val, json_int_val, json_obj_get_type, json_str_len, json_str_ptr};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer};
use crate::vendor::json::{JsonType, JsonValue};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::base64::{base64_decode};
use crate::support::buffer::{bufnew, bufwrite16b};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_push};
use crate::vendor::sds::{sdsempty};

#[repr(C)]
pub struct CvtTable {
    pub length: u32,
    pub words: *mut u16,
}
// Stage 6-4 "Box化": `words` is the only allocation this struct owns, same
// shape as `LtshTable`/`VorgTable`. Grepping confirmed only `.free` was
// ever called from outside this file (from `caryll_font.rs`'s table
// disposal); the entire vtable is deleted.
impl Drop for CvtTable {
    fn drop(&mut self) {
        unsafe {
            if !self.words.is_null() {
                free(self.words as *mut ::core::ffi::c_void);
                self.words = ::core::ptr::null_mut::<u16>();
            }
        }
    }
}
pub unsafe extern "C" fn otfcc_read_cvt(
    packet: Packet,
    mut _options: *const Options,
    mut tag: u32,
) -> Option<Box<CvtTable>> {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == tag {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    let table_length = length >> 1 as ::core::ffi::c_int;
                    let words = __caryll_allocate_clean(
                        (::core::mem::size_of::<u16>() as usize)
                            .wrapping_mul(table_length.wrapping_add(1 as u32) as usize),
                        18 as ::core::ffi::c_ulong,
                    ) as *mut u16;
                    let mut j: u16 = 0 as u16;
                    while (j as u32) < table_length {
                        *words.offset(j as isize) =
                            read_16u(data.offset(
                                (2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                            ) as *const u8);
                        j = j.wrapping_add(1);
                    }
                    return Some(Box::new(CvtTable { length: table_length, words }));
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return None;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_dump_cvt(
    table: Option<&CvtTable>,
    mut root: *mut JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"cvt"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut arr: *mut JsonValue = json_array_new((*table).length as usize);
        let mut j: u16 = 0 as u16;
        while (j as u32) < (*table).length {
            json_array_push(
                arr,
                json_integer_new(*(*table).words.offset(j as isize) as i64),
            );
            j = j.wrapping_add(1);
        }
        json_object_push(root, tag, arr);
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_cvt(
    mut root: *const JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> Option<Box<CvtTable>> {
    let mut t: Option<Box<CvtTable>> = None;
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(root, tag, JsonType::Array);
    if !table.is_null() {
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"cvt"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let table_length = json_arr_len(table);
            let words = __caryll_allocate_clean(
                (::core::mem::size_of::<u16>() as usize)
                    .wrapping_mul(table_length.wrapping_add(1 as u32) as usize),
                46 as ::core::ffi::c_ulong,
            ) as *mut u16;
            let mut j: u16 = 0 as u16;
            while (j as u32) < table_length {
                let mut record: *mut JsonValue = json_arr_at(table, j as u32);
                if (*record).type_0 == JsonType::Integer
                {
                    *words.offset(j as isize) = json_int_val(record) as u16;
                } else if (*record).type_0 == JsonType::Double
                {
                    *words.offset(j as isize) = json_dbl_val(record) as u16;
                } else {
                    *words.offset(j as isize) = 0 as u16;
                }
                j = j.wrapping_add(1);
            }
            t = Some(Box::new(CvtTable { length: table_length, words }));
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    } else {
        table = json_obj_get_type(root, tag, JsonType::String);
        if !table.is_null() {
            (*(*options).logger)
                .start_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                crate::sdsbuild!(sdsempty(), b"cvt"),
            );
            let mut ___loggedstep_v_0: bool = true;
            while ___loggedstep_v_0 {
                let mut len: usize = 0;
                let mut raw: *mut u8 = base64_decode(
                    json_str_ptr(table) as *mut u8,
                    json_str_len(table) as usize,
                    &raw mut len,
                );
                let table_length = (len >> 1 as ::core::ffi::c_int) as u32;
                let words = __caryll_allocate_clean(
                    (::core::mem::size_of::<u16>() as usize)
                        .wrapping_mul(table_length.wrapping_add(1 as u32) as usize),
                    66 as ::core::ffi::c_ulong,
                ) as *mut u16;
                let mut j_0: u16 = 0 as u16;
                while (j_0 as u32) < table_length {
                    *words.offset(j_0 as isize) = read_16u(
                        raw.offset((2 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize),
                    );
                    j_0 = j_0.wrapping_add(1);
                }
                free(raw as *mut ::core::ffi::c_void);
                raw = ::core::ptr::null_mut::<u8>();
                t = Some(Box::new(CvtTable { length: table_length, words }));
                ___loggedstep_v_0 = false;
                (*(*options).logger)
                    .finish
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger
                );
            }
        }
    }
    return t;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_cvt(
    table: Option<&CvtTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let table = match table {
        Some(t) => t,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    let mut j: u16 = 0 as u16;
    while (j as u32) < (*table).length {
        bufwrite16b(buf, *(*table).words.offset(j as isize));
        j = j.wrapping_add(1);
    }
    return buf;
}
