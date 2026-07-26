#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};
unsafe extern "C" {
    fn sdsempty() -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn base64_decode(src: *const u8, len: usize, out_len: *mut usize) -> *mut u8;
}


use crate::support::json_funcs::{json_obj_get_type};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::logger::{otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_array, json_double, json_integer, json_string, json_value};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_cvt {
    pub length: u32,
    pub words: *mut u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_cvt {
    pub init: Option<unsafe extern "C" fn(*mut table_cvt) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_cvt, *const table_cvt) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_cvt, *mut table_cvt) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_cvt) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_cvt, table_cvt) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_cvt, table_cvt) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_cvt>,
    pub free: Option<unsafe extern "C" fn(*mut table_cvt) -> ()>,
}
#[inline]
unsafe extern "C" fn disposeCvt(mut table: *mut table_cvt) {
    if !(*table).words.is_null() {
        free((*table).words as *mut ::core::ffi::c_void);
        (*table).words = ::core::ptr::null_mut::<u16>();
    }
}
#[inline]
unsafe extern "C" fn table_cvt_copy(mut dst: *mut table_cvt, mut src: *const table_cvt) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_cvt>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_cvt_free(mut x: *mut table_cvt) {
    if x.is_null() {
        return;
    }
    table_cvt_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_cvt_create() -> *mut table_cvt {
    let mut x: *mut table_cvt =
        malloc(::core::mem::size_of::<table_cvt>() as usize) as *mut table_cvt;
    table_cvt_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_cvt_init(mut x: *mut table_cvt) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_cvt>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_cvt_copyReplace(mut dst: *mut table_cvt, src: table_cvt) {
    table_cvt_dispose(dst);
    table_cvt_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_cvt_move(mut dst: *mut table_cvt, mut src: *mut table_cvt) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_cvt>() as usize,
    );
    table_cvt_init(src);
}
#[inline]
unsafe extern "C" fn table_cvt_replace(mut dst: *mut table_cvt, src: table_cvt) {
    table_cvt_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_cvt>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_cvt_dispose(mut x: *mut table_cvt) {
    disposeCvt(x);
}
#[unsafe(no_mangle)]
pub static table_iCvt: __caryll_elementinterface_table_cvt = {
    __caryll_elementinterface_table_cvt {
        init: Some(table_cvt_init as unsafe extern "C" fn(*mut table_cvt) -> ()),
        copy: Some(table_cvt_copy as unsafe extern "C" fn(*mut table_cvt, *const table_cvt) -> ()),
        move_0: Some(table_cvt_move as unsafe extern "C" fn(*mut table_cvt, *mut table_cvt) -> ()),
        dispose: Some(table_cvt_dispose as unsafe extern "C" fn(*mut table_cvt) -> ()),
        replace: Some(table_cvt_replace as unsafe extern "C" fn(*mut table_cvt, table_cvt) -> ()),
        copyReplace: Some(
            table_cvt_copyReplace as unsafe extern "C" fn(*mut table_cvt, table_cvt) -> (),
        ),
        create: Some(table_cvt_create),
        free: Some(table_cvt_free as unsafe extern "C" fn(*mut table_cvt) -> ()),
    }
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_readCvt(
    packet: otfcc_Packet,
    mut _options: *const otfcc_Options,
    mut tag: u32,
) -> *mut table_cvt {
    let mut t: *mut table_cvt = ::core::ptr::null_mut::<table_cvt>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == tag {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: u32 = table.length;
                    t = __caryll_allocate_clean(
                        ::core::mem::size_of::<table_cvt>() as usize,
                        16 as ::core::ffi::c_ulong,
                    ) as *mut table_cvt;
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
    return ::core::ptr::null_mut::<table_cvt>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_dumpCvt(
    mut table: *const table_cvt,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
    mut tag: *const ::core::ffi::c_char,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"cvt"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut arr: *mut json_value = json_array_new((*table).length as usize);
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
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_parseCvt(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut table_cvt {
    let mut t: *mut table_cvt = ::core::ptr::null_mut::<table_cvt>();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(root, tag, json_array);
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), b"cvt"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            t = __caryll_allocate_clean(
                ::core::mem::size_of::<table_cvt>() as usize,
                44 as ::core::ffi::c_ulong,
            ) as *mut table_cvt;
            (*t).length = (*table).u.array.length as u32;
            (*t).words = __caryll_allocate_clean(
                (::core::mem::size_of::<u16>() as usize)
                    .wrapping_mul((*t).length.wrapping_add(1 as u32) as usize),
                46 as ::core::ffi::c_ulong,
            ) as *mut u16;
            let mut j: u16 = 0 as u16;
            while (j as u32) < (*t).length {
                let mut record: *mut json_value =
                    *(*table).u.array.values.offset(j as isize) as *mut json_value;
                if (*record).type_0 == json_integer
                {
                    *(*t).words.offset(j as isize) = (*record).u.integer as u16;
                } else if (*record).type_0 == json_double
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
                (*options).logger as *mut otfcc_ILogger
            );
        }
    } else {
        table = json_obj_get_type(root, tag, json_string);
        if !table.is_null() {
            (*(*options).logger)
                .startSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                crate::sdsbuild!(sdsempty(), b"cvt"),
            );
            let mut ___loggedstep_v_0: bool = true;
            while ___loggedstep_v_0 {
                t = __caryll_allocate_clean(
                    ::core::mem::size_of::<table_cvt>() as usize,
                    61 as ::core::ffi::c_ulong,
                ) as *mut table_cvt;
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
                    (*options).logger as *mut otfcc_ILogger
                );
            }
        }
    }
    return t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_buildCvt(
    mut table: *const table_cvt,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if table.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    let mut j: u16 = 0 as u16;
    while (j as u32) < (*table).length {
        bufwrite16b(buf, *(*table).words.offset(j as isize));
        j = j.wrapping_add(1);
    }
    return buf;
}
