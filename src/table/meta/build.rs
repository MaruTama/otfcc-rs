use crate::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::support::buffer::Buffer;

use crate::bk::block::bk_new_block_from_bytes;
use crate::bk::graph::bk_build_block;
use crate::table::meta::types::{MetaEntry, MetaTable};
#[allow(improper_ctypes_definitions)]
pub fn build_meta(meta: Option<&MetaTable>) -> Option<Buffer> {
    let meta = match meta {
        Some(m) if !m.entries.is_empty() => m,
        _ => return None,
    };
    let entries: &Vec<MetaEntry> = &meta.entries;
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B32, meta.version),
        bk_int(BkCellType::B32, meta.flags),
        bk_int(BkCellType::B32, 0_u32),
        bk_int(BkCellType::B32, entries.len() as u32),
    ]);
    for e in entries.iter() {
        bk_push(
            &mut root,
            vec![
                bk_int(BkCellType::B32, e.tag),
                bk_ptr(
                    BkCellType::P32,
                    Some(bk_new_block_from_bytes(&e.data)),
                ),
                bk_int(BkCellType::B32, (e.data.len()) as u32),
            ],
        );
    }
    Some(bk_build_block(root))
}
