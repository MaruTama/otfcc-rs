#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::free;

use crate::logger::{LOG_VL_PROGRESS, LoggerType, logger_log_sds};
use crate::support::alloc::__caryll_allocate_clean;
use crate::support::buffer::Buffer;
use crate::support::options::Options;
use crate::support::fmt::Byte;
pub struct SfntTableEntry {
    pub tag: i32,
    pub length: u32,
    pub checksum: u32,
    pub buffer: Buffer,
}
pub struct SfntBuilder {
    pub count: u32,
    pub header: u32,
    pub tables: std::collections::BTreeMap<i32, SfntTableEntry>,
    pub options: *const Options,
}
// sfnt table checksums sum the table's bytes as big-endian u32 words.
// `buflongalign` (called by both callers just before this) always pads
// `data` out to a multiple of 4 bytes, so `chunks_exact(4)` covers every
// byte with no remainder -- reading each chunk with `from_be_bytes` avoids
// the alignment requirement a `*const u32` cast onto `Vec<u8>`'s
// 1-byte-aligned storage would need (that cast used to be UB here, caught
// by miri once `Font` construction stopped masking it earlier in the same
// test; see rust/README.md).
fn buf_checksum_bytes(data: &[u8]) -> u32 {
    data.chunks_exact(4)
        .fold(0u32, |sum, word| sum.wrapping_add(u32::from_be_bytes(word.try_into().unwrap())))
}
fn create_segment(tag: u32, mut buffer: Buffer) -> SfntTableEntry {
    let length = buffer.len() as u32;
    buffer.long_align();
    let sum = buf_checksum_bytes(&buffer.data);
    SfntTableEntry {
        tag: tag as i32,
        length,
        checksum: sum,
        buffer,
    }
}
pub unsafe fn otfcc_new_sfnt_builder(header: u32, options: &Options) -> *mut SfntBuilder {
    let builder: *mut SfntBuilder = __caryll_allocate_clean(
        ::core::mem::size_of::<SfntBuilder>() as usize,
        40 as ::core::ffi::c_ulong,
    ) as *mut SfntBuilder;
    (*builder).count = 0_u32;
    (*builder).header = header;
    // `BTreeMap` (a `std` type, but the caution established for
    // `IndexMap` in `table/fvar.rs` applies just as much: no documented
    // guarantee that an all-zero-bytes value is a safe-to-drop empty map)
    // gets `ptr::write`, not a plain assignment, so the calloc'd garbage
    // sitting here is never read or dropped.
    ::core::ptr::write(
        &raw mut (*builder).tables,
        std::collections::BTreeMap::new(),
    );
    (*builder).options = options;
    return builder;
}
pub unsafe fn otfcc_delete_sfnt_builder(builder: *mut SfntBuilder) {
    if builder.is_null() {
        return;
    }
    // `tables` (a `BTreeMap`, each entry now owning its `Buffer` directly)
    // needs its `Drop` glue run explicitly before the raw `free()` below --
    // `free()` only reclaims the allocation itself, it does not run Rust
    // destructors. `drop_in_place` does that in place, without needing to
    // move the map out first the way the old per-entry `Buffer::from_raw`
    // loop did.
    ::core::ptr::drop_in_place(&raw mut (*builder).tables);
    free(builder as *mut ::core::ffi::c_void);
}
// Deduplicates by `tag`, first registration wins -- a later
// `otfcc_sfnt_builder_push_table` call for a tag already present just
// frees the newly-passed `buffer` and returns, silently, no warning
// logged (unlike the `consolidate/otl/*.rs` uthash instances earlier in
// this migration, which mostly do warn on a duplicate). `BTreeMap`, not
// `IndexMap`/`Vec`: `otfcc_sfnt_builder_serialize` (below) sorts entries
// by tag before writing the table directory -- required by the SFNT
// format itself, which mandates the directory be sorted ascending by
// tag -- so `BTreeMap`'s always-sorted iteration is exactly right here,
// the same shape as the six `consolidate/otl/*.rs` instances earlier in
// this migration and unlike `ScriptStatHash`/`FvarMaster`'s insertion
// order.
pub unsafe fn otfcc_sfnt_builder_push_table(
    builder: *mut SfntBuilder,
    tag: u32,
    buffer: Option<Buffer>,
) {
    if builder.is_null() {
        return;
    }
    let Some(buffer) = buffer else {
        return;
    };
    let options: *const Options = (*builder).options;
    if (*builder).tables.contains_key(&(tag as i32)) {
        // `buffer` just drops here -- same as the old
        // `Buffer::from_raw(buffer)` + implicit drop.
        return;
    }
    let entry = create_segment(tag, buffer);
    (*builder).tables.insert(tag as i32, entry);
    logger_log_sds(
        &mut *(*options).logger.borrow_mut(),
        LOG_VL_PROGRESS,
        LoggerType::Progress,
        crate::bytesbuild!(
            b"OpenType table ",
            Byte((tag >> 24_i32 & 0xff_u32) as u8),
            Byte((tag >> 16_i32 & 0xff_u32) as u8),
            Byte((tag >> 8_i32 & 0xff_u32) as u8),
            Byte((tag & 0xff_u32) as u8),
            b" successfully built.\n",
        ),
    );
}
pub unsafe fn otfcc_sfnt_builder_serialize(builder: *mut SfntBuilder) -> Buffer {
    let mut buffer = Buffer::new();
    if builder.is_null() {
        return buffer;
    }
    let n_tables: u16 = (*builder).tables.len() as u16;
    let search_range: u16 = ((if (n_tables as i32) < 16_i32 {
        8_i32
    } else {
        if (n_tables as i32) < 32_i32 {
            16_i32
        } else {
            if (n_tables as i32) < 64_i32 {
                32_i32
            } else {
                64_i32
            }
        }
    }) * 16_i32) as u16;
    buffer.write_u32be((*builder).header);
    buffer.write_u16be(n_tables);
    buffer.write_u16be(search_range);
    buffer.write_u16be(
        (if (n_tables as i32) < 16_i32 {
            3_i32
        } else if (n_tables as i32) < 32_i32 {
            4_i32
        } else if (n_tables as i32) < 64_i32 {
            5_i32
        } else {
            6_i32
        }) as u16,
    );
    buffer.write_u16be(
        (n_tables as i32 * 16_i32
            - search_range as i32) as u16,
    );
    let mut offset: usize = (12_i32
        + n_tables as i32 * 16_i32)
        as usize;
    let mut head_offset: usize = offset;
    for (tag, table) in (*builder).tables.iter() {
        buffer.write_u32be(*tag as u32);
        buffer.write_u32be(table.checksum);
        buffer.write_u32be(offset as u32);
        buffer.write_u32be(table.length);
        let cp: usize = buffer.pos();
        buffer.seek(offset);
        buffer.write_buffer(&table.buffer);
        buffer.seek(cp);
        if *tag == crate::tag::TAG_HEAD as i32 {
            head_offset = offset;
        }
        offset = offset.wrapping_add(table.buffer.len());
    }
    buffer.long_align();
    let whole_checksum: u32 = buf_checksum_bytes(&buffer.data);
    buffer.seek(head_offset.wrapping_add(8_usize));
    buffer.write_u32be(0xb1b0afba_u32.wrapping_sub(whole_checksum));
    buffer
}
