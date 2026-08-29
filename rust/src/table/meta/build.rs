use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::support::buffer::Buffer;

use crate::bk::bkblock::bk_new_block_from_string_len;
use crate::bk::bkgraph::bk_build_block;
use crate::table::meta::types::{MetaEntry, MetaTable};
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_meta(meta: Option<&MetaTable>) -> *mut Buffer {
    let meta = match meta {
        Some(m) if !m.entries.is_empty() => m,
        _ => return ::core::ptr::null_mut::<Buffer>(),
    };
    let entries: &Vec<MetaEntry> = &meta.entries;
    let root: *mut BkBlock = unsafe {
        bk_new_block(&[
            bk_int(BkCellType::B32, (meta.version) as u32),
            bk_int(BkCellType::B32, (meta.flags) as u32),
            bk_int(BkCellType::B32, 0_u32),
            bk_int(BkCellType::B32, (entries.len() as u32) as u32),
        ])
    };
    let mut __caryll_index: usize = 0_usize;
    let mut keep: usize = 1_usize;
    while keep != 0 && __caryll_index < entries.len() {
        let e: &MetaEntry = &entries[__caryll_index];
        while keep != 0 {
            unsafe {
                bk_push(
                    root,
                    &[
                        bk_int(BkCellType::B32, (e.tag) as u32),
                        bk_ptr(
                            BkCellType::P32,
                            bk_new_block_from_string_len(
                                e.data.len(),
                                e.data.as_ptr() as *const ::core::ffi::c_char,
                            ),
                        ),
                        bk_int(BkCellType::B32, (e.data.len()) as u32),
                    ],
                )
            };
            keep = (keep == 0) as i32 as usize;
        }
        keep = (keep == 0) as i32 as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    return unsafe { bk_build_block(root) };
}
