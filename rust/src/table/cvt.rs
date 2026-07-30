#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};


use crate::support::json_funcs::{json_obj_get_type};
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CvtTable {
    pub length: u32,
    pub words: *mut u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CvtTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut CvtTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut CvtTable, *const CvtTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CvtTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut CvtTable>,
    pub free: Option<unsafe extern "C" fn(*mut CvtTable) -> ()>,
}
#[inline]
unsafe extern "C" fn dispose_cvt(mut table: *mut CvtTable) {
    if !(*table).words.is_null() {
        free((*table).words as *mut ::core::ffi::c_void);
        (*table).words = ::core::ptr::null_mut::<u16>();
    }
}
#[inline]
unsafe extern "C" fn table_cvt_copy(mut dst: *mut CvtTable, mut src: *const CvtTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CvtTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_cvt_free(mut x: *mut CvtTable) {
    if x.is_null() {
        return;
    }
    table_cvt_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_cvt_create() -> *mut CvtTable {
    let mut x: *mut CvtTable =
        malloc(::core::mem::size_of::<CvtTable>() as usize) as *mut CvtTable;
    table_cvt_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_cvt_init(mut x: *mut CvtTable) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<CvtTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_cvt_dispose(mut x: *mut CvtTable) {
    dispose_cvt(x);
}
pub static TABLE_I_CVT: CvtTableElementInterface = {
    CvtTableElementInterface {
        init: Some(table_cvt_init as unsafe extern "C" fn(*mut CvtTable) -> ()),
        copy: Some(table_cvt_copy as unsafe extern "C" fn(*mut CvtTable, *const CvtTable) -> ()),
        dispose: Some(table_cvt_dispose as unsafe extern "C" fn(*mut CvtTable) -> ()),
        create: Some(table_cvt_create),
        free: Some(table_cvt_free as unsafe extern "C" fn(*mut CvtTable) -> ()),
    }
};
pub unsafe extern "C" fn otfcc_read_cvt(
    packet: Packet,
    mut _options: *const Options,
    mut tag: u32,
) -> *mut CvtTable {
    let mut t: *mut CvtTable = ::core::ptr::null_mut::<CvtTable>();
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
                    t = __caryll_allocate_clean(
                        ::core::mem::size_of::<CvtTable>() as usize,
                        16 as ::core::ffi::c_ulong,
                    ) as *mut CvtTable;
                    (*t).length = length >> 1 as ::core::ffi::c_int;
                    (*t).words = __caryll_allocate_clean(
                        (::core::mem::size_of::<u16>() as usize)
                            .wrapping_mul((*t).length.wrapping_add(1 as u32) as usize),
                        18 as ::core::ffi::c_ulong,
                    ) as *mut u16;
                    let mut j: u16 = 0 as u16;
                    while (j as u32) < (*t).length {
                        *(*t).words.offset(j as isize) =
                            read_16u(data.offset(
                                (2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                            ) as *const u8);
                        j = j.wrapping_add(1);
                    }
                    return t;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<CvtTable>();
}
pub unsafe extern "C" fn otfcc_dump_cvt(
    mut table: *const CvtTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) {
    if table.is_null() {
        return;
    }
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
) -> *mut CvtTable {
    let mut t: *mut CvtTable = ::core::ptr::null_mut::<CvtTable>();
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
            t = __caryll_allocate_clean(
                ::core::mem::size_of::<CvtTable>() as usize,
                44 as ::core::ffi::c_ulong,
            ) as *mut CvtTable;
            (*t).length = (*table).u.array.length as u32;
            (*t).words = __caryll_allocate_clean(
                (::core::mem::size_of::<u16>() as usize)
                    .wrapping_mul((*t).length.wrapping_add(1 as u32) as usize),
                46 as ::core::ffi::c_ulong,
            ) as *mut u16;
            let mut j: u16 = 0 as u16;
            while (j as u32) < (*t).length {
                let mut record: *mut JsonValue =
                    *(*table).u.array.values.offset(j as isize) as *mut JsonValue;
                if (*record).type_0 == JsonType::Integer
                {
                    *(*t).words.offset(j as isize) = (*record).u.integer as u16;
                } else if (*record).type_0 == JsonType::Double
                {
                    *(*t).words.offset(j as isize) = (*record).u.dbl as u16;
                } else {
                    *(*t).words.offset(j as isize) = 0 as u16;
                }
                j = j.wrapping_add(1);
            }
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
                t = __caryll_allocate_clean(
                    ::core::mem::size_of::<CvtTable>() as usize,
                    61 as ::core::ffi::c_ulong,
                ) as *mut CvtTable;
                let mut len: usize = 0;
                let mut raw: *mut u8 = base64_decode(
                    (*table).u.string.ptr as *mut u8,
                    (*table).u.string.length as usize,
                    &raw mut len,
                );
                (*t).length = (len >> 1 as ::core::ffi::c_int) as u32;
                (*t).words = __caryll_allocate_clean(
                    (::core::mem::size_of::<u16>() as usize)
                        .wrapping_mul((*t).length.wrapping_add(1 as u32) as usize),
                    66 as ::core::ffi::c_ulong,
                ) as *mut u16;
                let mut j_0: u16 = 0 as u16;
                while (j_0 as u32) < (*t).length {
                    *(*t).words.offset(j_0 as isize) = read_16u(
                        raw.offset((2 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize),
                    );
                    j_0 = j_0.wrapping_add(1);
                }
                free(raw as *mut ::core::ffi::c_void);
                raw = ::core::ptr::null_mut::<u8>();
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
pub unsafe extern "C" fn otfcc_build_cvt(
    mut table: *const CvtTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if table.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    let mut j: u16 = 0 as u16;
    while (j as u32) < (*table).length {
        bufwrite16b(buf, *(*table).words.offset(j as isize));
        j = j.wrapping_add(1);
    }
    return buf;
}
