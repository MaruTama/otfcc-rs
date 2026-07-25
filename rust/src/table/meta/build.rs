
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, sds, sdshdr16, sdshdr32, sdshdr64, sdshdr8};
use crate::bk::bkblock::{b32, bk_Block, bkover, p32};

use crate::table::meta::types::{meta_Entry, table_meta};
extern "C" {
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromStringLen(len: usize, str: *const ::core::ffi::c_char) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}
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
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildMeta(
    mut meta: *const table_meta,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if meta.is_null() || (*meta).entries.length == 0 {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut root: *mut bk_Block = bk_new_Block(
        b32 as ::core::ffi::c_int,
        (*meta).version,
        b32 as ::core::ffi::c_int,
        (*meta).flags,
        b32 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        b32 as ::core::ffi::c_int,
        (*meta).entries.length as u32,
        bkover as ::core::ffi::c_int,
    );
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*meta).entries.length {
        let mut e: *mut meta_Entry = (*meta).entries.items.offset(__caryll_index as isize);
        while keep != 0 {
            bk_push(
                root,
                b32 as ::core::ffi::c_int,
                (*e).tag,
                p32 as ::core::ffi::c_int,
                bk_newBlockFromStringLen(
                    sdslen((*e).data),
                    (*e).data as *const ::core::ffi::c_char,
                ),
                b32 as ::core::ffi::c_int,
                sdslen((*e).data),
                bkover as ::core::ffi::c_int,
            );
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    return bk_build_Block(root);
}
