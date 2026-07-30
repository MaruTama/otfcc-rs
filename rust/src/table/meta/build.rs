#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::table::meta::types::{MetaEntry, MetaTable};
use crate::bk::bkblock::{bk_new_block_from_string_len};
use crate::bk::bkgraph::{bk_build_block};
#[inline]
unsafe extern "C" fn sdslen(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
pub unsafe extern "C" fn otfcc_build_meta(
    mut meta: *const MetaTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if meta.is_null() || (*meta).entries.length == 0 {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B32, ((*meta).version) as u32), bk_int(BkCellType::B32, ((*meta).flags) as u32), bk_int(BkCellType::B32, 0 as u32), bk_int(BkCellType::B32, ((*meta).entries.length as u32) as u32)]);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*meta).entries.length {
        let mut e: *mut MetaEntry = (*meta).entries.items.offset(__caryll_index as isize);
        while keep != 0 {
            bk_push(root, &[bk_int(BkCellType::B32, ((*e).tag) as u32), bk_ptr(BkCellType::P32, bk_new_block_from_string_len(
                    sdslen((*e).data),
                    (*e).data as *const ::core::ffi::c_char,
                )), bk_int(BkCellType::B32, (sdslen((*e).data)) as u32)]);
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    return bk_build_block(root);
}
