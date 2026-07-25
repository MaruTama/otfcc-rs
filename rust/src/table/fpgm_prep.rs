#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, strcmp};
unsafe extern "C" {
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite_bytes(buf: *mut caryll_Buffer, size: usize, str: *const u8);
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn parse_ttinstr(
        col: *mut json_value,
        context: *mut ::core::ffi::c_void,
        Make: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut u8, u32) -> ()>,
        Wrong: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> (),
        >,
    );
    fn dump_ttinstr(
        instructions: *mut u8,
        length: u32,
        options: *const otfcc_Options,
    ) -> *mut json_value;
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_object, json_value};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_fpgm_prep {
    pub tag: sds,
    pub length: u32,
    pub bytes: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_fpgm_prep {
    pub init: Option<unsafe extern "C" fn(*mut table_fpgm_prep) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_fpgm_prep, *const table_fpgm_prep) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_fpgm_prep, *mut table_fpgm_prep) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_fpgm_prep) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_fpgm_prep, table_fpgm_prep) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_fpgm_prep, table_fpgm_prep) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_fpgm_prep>,
    pub free: Option<unsafe extern "C" fn(*mut table_fpgm_prep) -> ()>,
}
#[inline]
unsafe extern "C" fn disposeFpgmPrep(mut table: *mut table_fpgm_prep) {
    if !(*table).tag.is_null() {
        sdsfree((*table).tag);
    }
    if !(*table).bytes.is_null() {
        free((*table).bytes as *mut ::core::ffi::c_void);
        (*table).bytes = ::core::ptr::null_mut::<u8>();
    }
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_init(mut x: *mut table_fpgm_prep) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_fpgm_prep>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_free(mut x: *mut table_fpgm_prep) {
    if x.is_null() {
        return;
    }
    table_fpgm_prep_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_create() -> *mut table_fpgm_prep {
    let mut x: *mut table_fpgm_prep =
        malloc(::core::mem::size_of::<table_fpgm_prep>() as usize) as *mut table_fpgm_prep;
    table_fpgm_prep_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_replace(mut dst: *mut table_fpgm_prep, src: table_fpgm_prep) {
    table_fpgm_prep_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_fpgm_prep>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_copyReplace(
    mut dst: *mut table_fpgm_prep,
    src: table_fpgm_prep,
) {
    table_fpgm_prep_dispose(dst);
    table_fpgm_prep_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_copy(
    mut dst: *mut table_fpgm_prep,
    mut src: *const table_fpgm_prep,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_fpgm_prep>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_move(
    mut dst: *mut table_fpgm_prep,
    mut src: *mut table_fpgm_prep,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_fpgm_prep>() as usize,
    );
    table_fpgm_prep_init(src);
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_dispose(mut x: *mut table_fpgm_prep) {
    disposeFpgmPrep(x);
}
#[unsafe(no_mangle)]
pub static table_iFpgm_prep: __caryll_elementinterface_table_fpgm_prep = {
    __caryll_elementinterface_table_fpgm_prep {
        init: Some(table_fpgm_prep_init as unsafe extern "C" fn(*mut table_fpgm_prep) -> ()),
        copy: Some(
            table_fpgm_prep_copy
                as unsafe extern "C" fn(*mut table_fpgm_prep, *const table_fpgm_prep) -> (),
        ),
        move_0: Some(
            table_fpgm_prep_move
                as unsafe extern "C" fn(*mut table_fpgm_prep, *mut table_fpgm_prep) -> (),
        ),
        dispose: Some(table_fpgm_prep_dispose as unsafe extern "C" fn(*mut table_fpgm_prep) -> ()),
        replace: Some(
            table_fpgm_prep_replace
                as unsafe extern "C" fn(*mut table_fpgm_prep, table_fpgm_prep) -> (),
        ),
        copyReplace: Some(
            table_fpgm_prep_copyReplace
                as unsafe extern "C" fn(*mut table_fpgm_prep, table_fpgm_prep) -> (),
        ),
        create: Some(table_fpgm_prep_create),
        free: Some(table_fpgm_prep_free as unsafe extern "C" fn(*mut table_fpgm_prep) -> ()),
    }
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_readFpgmPrep(
    packet: otfcc_Packet,
    mut _options: *const otfcc_Options,
    mut tag: u32,
) -> *mut table_fpgm_prep {
    let mut t: *mut table_fpgm_prep = ::core::ptr::null_mut::<table_fpgm_prep>();
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
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: u32 = table.length;
                    t = (
                        table_iFpgm_prep.create.expect("non-null function pointer"))();
                    (*t).tag = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    (*t).length = length;
                    (*t).bytes = __caryll_allocate_clean(
                        (::core::mem::size_of::<u8>() as usize)
                            .wrapping_mul(length as usize),
                        22 as ::core::ffi::c_ulong,
                    ) as *mut u8;
                    if (*t).bytes.is_null() {
                        table_iFpgm_prep.free.expect("non-null function pointer")(t);
                        t = ::core::ptr::null_mut::<table_fpgm_prep>();
                    } else {
                        memcpy(
                            (*t).bytes as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            length as usize,
                        );
                        return t;
                    }
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<table_fpgm_prep>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn table_dumpTableFpgmPrep(
    mut table: *const table_fpgm_prep,
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
        crate::sdsbuild!(sdsempty(), tag),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        json_object_push(
            root,
            tag,
            dump_ttinstr((*table).bytes, (*table).length, options),
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn makeFpgmPrepInstr(
    mut _t: *mut ::core::ffi::c_void,
    mut instrs: *mut u8,
    mut length: u32,
) {
    let mut t: *mut table_fpgm_prep = _t as *mut table_fpgm_prep;
    (*t).length = length;
    (*t).bytes = instrs;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wrongFpgmPrepInstr(
    mut _t: *mut ::core::ffi::c_void,
    mut _reason: *mut ::core::ffi::c_char,
    mut _pos: ::core::ffi::c_int,
) {
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_parseFpgmPrep(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut table_fpgm_prep {
    let mut t: *mut table_fpgm_prep = ::core::ptr::null_mut::<table_fpgm_prep>();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get(root, tag);
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), tag),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            t = (
                table_iFpgm_prep.create.expect("non-null function pointer"))();
            (*t).tag = sdsnew(tag);
            parse_ttinstr(
                table,
                t as *mut ::core::ffi::c_void,
                Some(
                    makeFpgmPrepInstr
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut u8,
                            u32,
                        ) -> (),
                ),
                Some(
                    wrongFpgmPrepInstr
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
            );
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    return t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_buildFpgmPrep(
    mut table: *const table_fpgm_prep,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if table.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite_bytes(buf, (*table).length as usize, (*table).bytes);
    return buf;
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
