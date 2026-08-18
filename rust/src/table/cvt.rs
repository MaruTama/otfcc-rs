#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};


use crate::support::parsed_json::{ParsedValue, json_arr_at, json_arr_len, json_dbl_val, json_int_val, json_obj_get_type, json_str_len, json_str_ptr, json_type_of};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::support::font_reader::{FontReader};
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::vendor::json::{JsonType};
use crate::font::caryll_sfnt::{Packet};
use crate::support::base64::{base64_decode};
use crate::support::buffer::{bufnew, bufwrite16b};
use crate::support::built_json::{BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_push};

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
// Unlike every other table in this batch, the original C (and the first
// Rust translation) here was already memory-safe without a separate length
// guard: `table_length` is derived directly from the table's own declared
// length (`length >> 1`, i.e. `length / 2`), and the read loop is bounded
// by that exact same `table_length` -- so `2 * table_length <= length`
// always holds and no read can go past the end. Migrated anyway for
// consistency with the rest of this batch (dropping `__fortable_*`/
// `.offset()`), not because it fixes a bug.
pub unsafe fn otfcc_read_cvt(
    packet: &Packet,
    mut _options: *const Options,
    mut tag: u32,
) -> Option<Box<CvtTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == tag)?;
    let table_length = (table.data.len() / 2) as u32;
    let words = __caryll_allocate_clean(
        (::core::mem::size_of::<u16>() as usize)
            .wrapping_mul(table_length.wrapping_add(1 as u32) as usize),
        18 as ::core::ffi::c_ulong,
    ) as *mut u16;
    let mut r = FontReader::new(&table.data);
    for j in 0..table_length as usize {
        *words.offset(j as isize) = r
            .u16()
            .expect("table_length is derived from data.len(), so table_length u16 reads always fit");
    }
    Some(Box::new(CvtTable { length: table_length, words }))
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_cvt(
    table: Option<&CvtTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    logger_start_sds(
        (*options).logger,
        crate::bytesbuild!(b"cvt"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut arr: *mut BuiltValue = json_array_new((*table).length as usize);
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
        logger_finish((*options).logger);
    }
}
pub unsafe fn otfcc_parse_cvt(
    mut root: *const ParsedValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> Option<Box<CvtTable>> {
    let mut t: Option<Box<CvtTable>> = None;
    let mut table: *const ParsedValue = ::core::ptr::null();
    table = json_obj_get_type(root, tag, JsonType::Array);
    if !table.is_null() {
        logger_start_sds(
            (*options).logger,
            crate::bytesbuild!(b"cvt"),
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
                let mut record: *const ParsedValue = json_arr_at(table, j as u32);
                if json_type_of(record) == JsonType::Integer
                {
                    *words.offset(j as isize) = json_int_val(record) as u16;
                } else if json_type_of(record) == JsonType::Double
                {
                    *words.offset(j as isize) = json_dbl_val(record) as u16;
                } else {
                    *words.offset(j as isize) = 0 as u16;
                }
                j = j.wrapping_add(1);
            }
            t = Some(Box::new(CvtTable { length: table_length, words }));
            ___loggedstep_v = false;
            logger_finish(
                (*options).logger
            );
        }
    } else {
        table = json_obj_get_type(root, tag, JsonType::String);
        if !table.is_null() {
            logger_start_sds(
                (*options).logger,
                crate::bytesbuild!(b"cvt"),
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
                logger_finish(
                    (*options).logger
                );
            }
        }
    }
    return t;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_cvt(
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
