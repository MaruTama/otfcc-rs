
use crate::logger::ByteStr;
use otfcc_binary::Buffer;
use crate::support::options::Options;
use crate::support::fmt::Byte;
#[derive(Debug)]
pub struct SfntTableEntry {
    pub tag: i32,
    pub length: u32,
    pub checksum: u32,
    pub buffer: Buffer,
}
#[derive(Debug)]
pub struct SfntBuilder<'a> {
    pub count: u32,
    pub header: u32,
    pub tables: std::collections::BTreeMap<i32, SfntTableEntry>,
    pub options: &'a Options,
}

impl<'a> SfntBuilder<'a> {
    pub fn new(header: u32, options: &'a Options) -> Self {
        SfntBuilder {
            count: 0,
            header,
            tables: std::collections::BTreeMap::new(),
            options,
        }
    }
}
// sfnt table checksums sum the table's bytes as big-endian u32 words.
// `buflongalign` (called by both callers just before this) always pads
// `data` out to a multiple of 4 bytes, so `chunks_exact(4)` covers every
// byte with no remainder. `from_be_bytes` on each chunk avoids any
// alignment requirement on `Vec<u8>`'s 1-byte-aligned storage.
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
// Deduplicates by `tag`, first registration wins -- a later call for a tag
// already present drops the new `buffer` silently. `BTreeMap` because the
// SFNT format requires the table directory sorted ascending by tag, which
// `sfnt_builder_serialize` (below) gets from the map's iteration order.
pub fn sfnt_builder_push_table(builder: &mut SfntBuilder, tag: u32, buffer: Option<Buffer>) {
    let Some(buffer) = buffer else {
        return;
    };
    if builder.tables.contains_key(&(tag as i32)) {
        // `buffer` just drops here -- same as the old
        // `Buffer::from_raw(buffer)` + implicit drop.
        return;
    }
    let entry = create_segment(tag, buffer);
    builder.tables.insert(tag as i32, entry);
    tracing::debug!("OpenType table {}{}{}{} successfully built.\n", ByteStr(Byte((tag >> 24_i32 & 0xff_u32) as u8)), ByteStr(Byte((tag >> 16_i32 & 0xff_u32) as u8)), ByteStr(Byte((tag >> 8_i32 & 0xff_u32) as u8)), ByteStr(Byte((tag & 0xff_u32) as u8)));
}
// `builder.is_null()` was dead here too, same reasoning as
// `sfnt_builder_push_table` above.
pub fn sfnt_builder_serialize(builder: &SfntBuilder) -> Buffer {
    let mut buffer = Buffer::new();
    let n_tables: u16 = builder.tables.len() as u16;
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
    buffer.write_u32be(builder.header);
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
    for (tag, table) in builder.tables.iter() {
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
