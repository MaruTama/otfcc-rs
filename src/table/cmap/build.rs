//! Writing the binary `cmap` table: a format 4 subtable for the BMP, format
//! 12 when the mappings need it, and format 14 for variation sequences.

use crate::support::options::Options;
use crate::support::primitives::{GlyphId, Unicode};
use crate::table::cmap::{CmapTable, cmap_lookup};
use otfcc_binary::Buffer;
use otfcc_binary::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use otfcc_binary::bk::block::{bk_new_block_from_buffer, bk_new_block_from_buffer_copy};
use otfcc_binary::bk::graph::bk_build_block;

fn build_cmap_format4(cmap: &CmapTable) -> Buffer {
    let mut buf = Buffer::new();
    let mut end_count = Buffer::new();
    let mut start_count = Buffer::new();
    let mut id_delta = Buffer::new();
    let mut id_range_offset = Buffer::new();
    let mut glyph_id_array = Buffer::new();
    let mut started: bool = false;
    let mut last_unicode_start: i32 = 0xffffff;
    let mut last_unicode_end: i32 = 0xffffff;
    let mut last_gid_start: i32 = 0xffffff;
    let mut last_gid_end: i32 = 0xffffff;
    let mut last_glyph_id_array_offset: usize = 0;
    let mut is_sequencial: bool = true;
    let mut segments_count: u16 = 0;
    for (&unicode, glyph) in cmap.unicodes.iter() {
        if unicode <= 0xffff {
            if !started {
                started = true;
                last_unicode_end = unicode;
                last_unicode_start = last_unicode_end;
                last_gid_end = i32::from(glyph.index);
                last_gid_start = last_gid_end;
                is_sequencial = true;
            } else if unicode == last_unicode_end + 1
                && !(i32::from(glyph.index) != last_gid_end + 1
                    && i32::from(is_sequencial) != 0
                    && last_gid_end - last_gid_start >= 4)
            {
                if i32::from(is_sequencial) != 0 && !(i32::from(glyph.index) == last_gid_end + 1) {
                    last_glyph_id_array_offset = glyph_id_array.cursor;
                    for j in last_gid_start..=last_gid_end {
                        glyph_id_array.write_u16be(j as u16);
                    }
                }
                last_unicode_end = unicode;
                is_sequencial =
                    i32::from(is_sequencial) != 0 && i32::from(glyph.index) == last_gid_end + 1;
                last_gid_end = i32::from(glyph.index);
                if !is_sequencial {
                    glyph_id_array.write_u16be(last_gid_end as u16);
                }
            } else {
                end_count.write_u16be(last_unicode_end as u16);
                start_count.write_u16be(last_unicode_start as u16);
                if is_sequencial {
                    id_delta.write_u16be((last_gid_start - last_unicode_start) as u16);
                    id_range_offset.write_u16be(0);
                } else {
                    id_delta.write_u16be(0);
                    id_range_offset.write_u16be((last_glyph_id_array_offset + 1) as u16);
                }
                segments_count = segments_count.wrapping_add(1);
                last_unicode_end = unicode;
                last_unicode_start = last_unicode_end;
                last_gid_end = i32::from(glyph.index);
                last_gid_start = last_gid_end;
                is_sequencial = true;
            }
        }
    }
    end_count.write_u16be(last_unicode_end as u16);
    start_count.write_u16be(last_unicode_start as u16);
    if is_sequencial {
        id_delta.write_u16be((last_gid_start - last_unicode_start) as u16);
        id_range_offset.write_u16be(0);
    } else {
        id_delta.write_u16be(0);
        id_range_offset.write_u16be((last_glyph_id_array_offset + 1) as u16);
    }
    segments_count = segments_count.wrapping_add(1);
    // The closing segment for U+FFFF.
    if last_gid_end < 0xffff {
        end_count.write_u16be(0xffff);
        start_count.write_u16be(0xffff);
        id_delta.write_u16be(1);
        id_range_offset.write_u16be(0);
        segments_count = segments_count.wrapping_add(1);
    }
    // Each idRangeOffset so far is the glyph ID array position plus one;
    // make it relative to the entry itself, which sits (segCount - j)
    // entries before the glyph ID array.
    for j in 0..segments_count as usize {
        let at = j * 2;
        let ro = u16::from_be_bytes([id_range_offset.data[at], id_range_offset.data[at + 1]]);
        if ro != 0 {
            let distance = 2 * (u32::from(segments_count) - j as u32);
            id_range_offset.seek(at);
            id_range_offset.write_u16be((ro - 1).wrapping_add(distance as u16));
        }
    }
    buf.write_u16be(4); // format
    buf.write_u16be(0); // length, patched below
    buf.write_u16be(0); // language
    buf.write_u16be(segments_count << 1);
    // searchRange is twice the largest power of two not above segCount,
    // entrySelector the log2 of that power, and rangeShift what remains.
    let mut search_range: u32 = 1;
    let mut entry_selector: u32 = 0;
    while search_range <= u32::from(segments_count) {
        search_range <<= 1;
        entry_selector += 1;
    }
    buf.write_u16be(search_range as u16);
    buf.write_u16be((entry_selector - 1) as u16);
    buf.write_u16be((2 * u32::from(segments_count) - search_range) as u16);
    buf.write_buffer(&end_count);
    buf.write_u16be(0); // reservedPad
    buf.write_buffer(&start_count);
    buf.write_buffer(&id_delta);
    buf.write_buffer(&id_range_offset);
    buf.write_buffer(&glyph_id_array);
    buf.seek(2);
    buf.write_u16be(buf.len() as u16);
    buf
}
fn try_build_cmap_format4(cmap: &CmapTable) -> Option<Buffer> {
    let buf = build_cmap_format4(cmap);
    if buf.len() > u16::MAX as usize {
        None
    } else {
        Some(buf)
    }
}
fn build_cmap_format12(cmap: &CmapTable) -> Buffer {
    let mut buf = Buffer::new();
    buf.write_u16be(12);
    buf.write_u16be(0);
    buf.write_u32be(0);
    buf.write_u32be(0);
    buf.write_u32be(0);
    let mut n_groups: u32 = 0;
    let mut started: bool = false;
    let mut last_unicode_start: i32 = 0xffffff;
    let mut last_unicode_end: i32 = 0xffffff;
    let mut last_gid_start: i32 = 0xffffff;
    let mut last_gid_end: i32 = 0xffffff;
    for (&unicode, glyph) in cmap.unicodes.iter() {
        if !started {
            started = true;
            last_unicode_end = unicode;
            last_unicode_start = last_unicode_end;
            last_gid_end = i32::from(glyph.index);
            last_gid_start = last_gid_end;
        } else if unicode == last_unicode_end + 1 && i32::from(glyph.index) == last_gid_end + 1 {
            last_unicode_end = unicode;
            last_gid_end = i32::from(glyph.index);
        } else {
            buf.write_u32be(last_unicode_start as u32);
            buf.write_u32be(last_unicode_end as u32);
            buf.write_u32be(last_gid_start as u32);
            n_groups += 1;
            last_unicode_end = unicode;
            last_unicode_start = last_unicode_end;
            last_gid_end = i32::from(glyph.index);
            last_gid_start = last_gid_end;
        }
    }
    buf.write_u32be(last_unicode_start as u32);
    buf.write_u32be(last_unicode_end as u32);
    buf.write_u32be(last_gid_start as u32);
    n_groups += 1;
    buf.seek(4);
    buf.write_u32be(buf.len() as u32);
    buf.seek(12);
    buf.write_u32be(n_groups);
    buf
}
/// The length of the per-code-point tables in format 14 building: every
/// code point up to U+10FFFF, plus one last slot kept empty so every range
/// ends inside the table.
pub const MAX_UNICODE: Unicode = 0x110001;
/// A selector has default variation sequences: ones whose glyph is the
/// code point's plain `cmap` glyph.
pub const HAS_DEFAULT: u8 = 1;
/// A selector has non-default variation sequences: ones with a glyph of
/// their own.
pub const HAS_NON_DEFAULT: u8 = 2;
#[inline]
/// Writes the default UVS range `start..=end`, split into ranges of at most
/// 256 code points (additionalCount is one byte).
fn write_default_range(dflt: &mut Buffer, n_ranges: &mut u32, mut start: Unicode, end: Unicode) {
    while end - start > 0xff {
        dflt.write_u24be(start);
        dflt.write_u8(0xff);
        start += 0x100;
        *n_ranges += 1;
    }
    dflt.write_u24be(start);
    dflt.write_u8((end - start) as u8);
    *n_ranges += 1;
}
fn build_format14_for_selector(
    cmap: &CmapTable,
    selector: Unicode,
    dflt: &mut Buffer,
    nondflt: &mut Buffer,
) -> u8 {
    // Per code point, the glyph a default or non-default mapping for this
    // selector gives, or 0xffff for none.
    let mut defaults: Vec<GlyphId> = vec![0xffff; MAX_UNICODE as usize];
    let mut non_defaults: Vec<GlyphId> = vec![0xffff; MAX_UNICODE as usize];
    for (key, glyph) in cmap.uvs.iter() {
        let u: Unicode = key.unicode as Unicode;
        if !(key.selector != selector || u >= MAX_UNICODE) && !glyph.name.is_empty() {
            let uvs_gid: GlyphId = glyph.index;
            match cmap_lookup(cmap, u as i32) {
                None => {
                    non_defaults[u as usize] = uvs_gid;
                }
                Some(g) if i32::from(uvs_gid) == i32::from(g.index) => {
                    defaults[u as usize] = uvs_gid;
                }
                Some(_) => {
                    non_defaults[u as usize] = uvs_gid;
                }
            }
        }
    }
    non_defaults[0] = 0xffff;
    defaults[0] = 0xffff;
    non_defaults[(MAX_UNICODE - 1) as usize] = 0xffff;
    defaults[(MAX_UNICODE - 1) as usize] = 0xffff;
    let mut num_unicode_value_ranges: u32 = 0;
    let mut start_unicode_value: Unicode = 0;
    let mut num_uvs_mappings: u32 = 0;
    dflt.write_u32be(0);
    nondflt.write_u32be(0);
    // Both ends of the tables are kept unset above, so every run of
    // default mappings has a start and an end inside the walk.
    for u in 1..MAX_UNICODE {
        let here = defaults[u as usize] != 0xffff;
        let before = defaults[u as usize - 1] != 0xffff;
        if here && !before {
            start_unicode_value = u;
        }
        if !here && before {
            write_default_range(
                dflt,
                &mut num_unicode_value_ranges,
                start_unicode_value,
                u - 1,
            );
        }
        if non_defaults[u as usize] != 0xffff {
            nondflt.write_u24be(u);
            nondflt.write_u16be(non_defaults[u as usize]);
            num_uvs_mappings += 1;
        }
    }
    dflt.seek(0);
    dflt.write_u32be(num_unicode_value_ranges);
    nondflt.seek(0);
    nondflt.write_u32be(num_uvs_mappings);
    let mut results: u8 = 0;
    if num_unicode_value_ranges != 0 {
        results |= HAS_DEFAULT;
    }
    if num_uvs_mappings != 0 {
        results |= HAS_NON_DEFAULT;
    }
    results
}
fn build_cmap_format14(cmap: &CmapTable) -> Buffer {
    let mut valid_selectors: Vec<bool> = vec![false; MAX_UNICODE as usize];
    for key in cmap.uvs.keys() {
        if key.selector < MAX_UNICODE {
            valid_selectors[key.selector as usize] = true;
        }
    }
    let n_selectors = valid_selectors.iter().filter(|&&valid| valid).count() as u32;
    let mut st: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, 14),
        bk_int(BkCellType::B32, 0),
        bk_int(BkCellType::B32, n_selectors),
    ]);
    for selector in 0..MAX_UNICODE {
        if valid_selectors[selector as usize] {
            let mut dflt = Buffer::new();
            let mut nondflt = Buffer::new();
            let results: u8 = build_format14_for_selector(cmap, selector, &mut dflt, &mut nondflt);
            let dflt = if results & HAS_DEFAULT == 0 {
                None
            } else {
                Some(dflt)
            };
            let nondflt = if results & HAS_NON_DEFAULT == 0 {
                None
            } else {
                Some(nondflt)
            };
            bk_push(
                &mut st,
                vec![
                    // varSelector is a 24-bit value.
                    bk_int(BkCellType::B8, selector >> 16 & 0xff),
                    bk_int(BkCellType::B8, selector >> 8 & 0xff),
                    bk_int(BkCellType::B8, selector & 0xff),
                    bk_ptr(BkCellType::P32, bk_new_block_from_buffer(dflt)),
                    bk_ptr(BkCellType::P32, bk_new_block_from_buffer(nondflt)),
                ],
            );
        }
    }
    let mut buf = bk_build_block(st);
    buf.seek(2);
    buf.write_u32be(buf.len() as u32);
    buf
}
/// Serializes the table. Returns `None` when no code point is mapped.
///
/// The result always has a format 4 subtable for the BMP, except that with
/// `options.stub_cmap4` a font that needs format 12 gets a minimal placeholder
/// instead. A format 12 subtable is added when some code point is above
/// 0xFFFF, or when the format 4 one would not fit in 64 KB. Variation
/// sequences, if any, add a format 14 subtable.
pub fn build_cmap(cmap: Option<&CmapTable>, options: &Options) -> Option<Buffer> {
    let cmap = match cmap {
        Some(c) if !c.unicodes.is_empty() => c,
        _ => return None,
    };
    let mut requires_format12: bool = false;
    let has_uvs: bool = !cmap.uvs.is_empty();
    for &unicode in cmap.unicodes.keys() {
        if unicode > 0xffff {
            requires_format12 = true;
        }
    }
    let mut format4: Option<Buffer> = None;
    if !requires_format12 || !options.stub_cmap4 {
        format4 = try_build_cmap_format4(cmap);
        if format4.is_none() {
            requires_format12 = true;
        }
    }
    let mut n_tables: u8 = (if i32::from(requires_format12) != 0 {
        4
    } else {
        2
    }) as u8;
    if has_uvs {
        n_tables = (i32::from(n_tables) + 1) as u8;
    }
    let format4: Buffer = format4.unwrap_or_else(|| {
        let mut stub = Buffer::new();
        stub.write_u16be(4);
        stub.write_u16be(32);
        stub.write_u16be(0);
        stub.write_u16be(4);
        stub.write_u16be(4);
        stub.write_u16be(1);
        stub.write_u16be(0);
        stub.write_u16be(0);
        stub.write_u16be(0xffff);
        stub.write_u16be(0);
        stub.write_u16be(0);
        stub.write_u16be(0xffff);
        stub.write_u16be(0);
        stub.write_u16be(1);
        stub.write_u16be(0);
        stub.write_u16be(0);
        stub
    });
    let format12 = build_cmap_format12(cmap);
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, 0),
        bk_int(BkCellType::B16, i32::from(n_tables) as u32),
    ]);
    bk_push(
        &mut root,
        vec![
            bk_int(BkCellType::B16, 0),
            bk_int(BkCellType::B16, 3),
            bk_ptr(
                BkCellType::P32,
                bk_new_block_from_buffer_copy(Some(&format4)),
            ),
        ],
    );
    if requires_format12 {
        bk_push(
            &mut root,
            vec![
                bk_int(BkCellType::B16, 0),
                bk_int(BkCellType::B16, 4),
                bk_ptr(
                    BkCellType::P32,
                    bk_new_block_from_buffer_copy(Some(&format12)),
                ),
            ],
        );
    }
    if has_uvs {
        let format14 = build_cmap_format14(cmap);
        bk_push(
            &mut root,
            vec![
                bk_int(BkCellType::B16, 0),
                bk_int(BkCellType::B16, 5),
                bk_ptr(BkCellType::P32, bk_new_block_from_buffer(Some(format14))),
            ],
        );
    }
    bk_push(
        &mut root,
        vec![
            bk_int(BkCellType::B16, 3),
            bk_int(BkCellType::B16, 1),
            bk_ptr(
                BkCellType::P32,
                bk_new_block_from_buffer_copy(Some(&format4)),
            ),
        ],
    );
    if requires_format12 {
        bk_push(
            &mut root,
            vec![
                bk_int(BkCellType::B16, 3),
                bk_int(BkCellType::B16, 10),
                bk_ptr(
                    BkCellType::P32,
                    bk_new_block_from_buffer_copy(Some(&format12)),
                ),
            ],
        );
    }
    Some(bk_build_block(root))
}
