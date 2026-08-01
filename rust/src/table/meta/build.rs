#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::vendor::sds::{sdslen};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::table::meta::types::{MetaEntry, MetaTable};
use crate::bk::bkblock::{bk_new_block_from_string_len};
use crate::bk::bkgraph::{bk_build_block};
pub unsafe extern "C" fn otfcc_build_meta(
    mut meta: *const MetaTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if meta.is_null() || (*meta).entries.is_empty() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let entries: &Vec<MetaEntry> = &(*meta).entries;
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B32, ((*meta).version) as u32), bk_int(BkCellType::B32, ((*meta).flags) as u32), bk_int(BkCellType::B32, 0 as u32), bk_int(BkCellType::B32, (entries.len() as u32) as u32)]);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < entries.len() {
        let mut e: *const MetaEntry = &entries[__caryll_index];
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
