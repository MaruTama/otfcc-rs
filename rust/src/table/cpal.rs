#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::Packet;
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::buffer::Buffer;
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::ParsedValue;
use crate::support::primitives::{ColorId, TableId};
use crate::vendor::json::JsonType;

use crate::bk::bkgraph::bk_build_block;
use crate::support::built_json::BuiltValue;
#[derive(Copy, Clone)]
pub struct CpalColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
    pub label: u16,
}
#[derive(Clone)]
pub struct CpalPalette {
    pub colorset: Vec<CpalColor>,
    pub type_0: u32,
    pub label: u32,
}
// Stage 6-4 "Box化": every field this struct owns is already a
// `Vec`/scalar, so no `Drop` impl is needed -- `Box::new` construction
// plus the standard drop glue is sufficient. `init_cpal`/`dispose_cpal`/
// `table_cpal_{init,dispose,create,copy,free}` all deleted: grepping
// confirmed `table_cpal_copy` was never called anywhere (not even
// self-referentially), and `table_cpal_free` was the only one of these
// ever called from outside this file (from `caryll_font.rs`'s table
// disposal).
#[derive(Clone)]
pub struct CpalTable {
    pub version: u16,
    pub palettes: Vec<CpalPalette>,
}
pub static WHITE: CpalColor = CpalColor {
    red: 0xff_u8,
    green: 0xff_u8,
    blue: 0xff_u8,
    alpha: 0xff_u8,
    label: 0xffff_u16,
};
/// The 3 v1 offset arrays (`offsetPaletteTypeArray`/`...LabelArray`/
/// `...EntryLabelArray`) are read at absolute offsets `16`/`20`/`24 + 2 *
/// num_palettes` -- 4 bytes further into the table than the OpenType 'CPAL'
/// spec places them (`12`/`16`/`20 + 2 * num_palettes`, immediately after
/// `colorRecordIndices`). Preserved exactly as the original C read them:
/// this migration's job is bounds-checking the existing byte offsets, not
/// re-deriving what they "should" be.
///
/// `offset_first_color_record`, `offset_palette_type_array`,
/// `offset_palette_label_array` and `offset_palette_entry_label_array` are
/// each a raw `u32` read straight from the file (full attacker control, up
/// to `u32::MAX`) that the original guarded with `x.wrapping_add(count *
/// stride)`. A value close enough to `u32::MAX` wraps that addition back
/// down to something small, so `length < wrapped_small_value` could pass
/// even though `x` itself points nowhere near the table -- the same
/// overflow-defeats-guard shape `otl/coverage.rs`'s `read_coverage` and
/// `table/cmap.rs`'s plan writeup describe, and (unlike `table/gdef.rs`'s
/// or `table/svg.rs`'s offsets, which are sums of a few `u16` fields and so
/// can never reach anywhere near `u32::MAX`) a *directly attacker-supplied*
/// `u32`, so this one is really reachable. `FontReader::at`/`require_room`
/// use `checked_add`/`checked_mul` throughout, closing all four instances
/// of it in this table at once.
fn parse_cpal(data: &[u8]) -> Result<(u16, Vec<CpalPalette>), ReadError> {
    if data.len() < 2 {
        return Err(ReadError { needed: 2, available: data.len() });
    }
    let version = FontReader::new(data).u16()?;
    let table_header_length: usize = if version == 0 { 14 } else { 26 };
    if data.len() < table_header_length {
        return Err(ReadError {
            needed: table_header_length,
            available: data.len(),
        });
    }
    let mut h = FontReader::new(data).at(2)?;
    let num_palettes_entries = h.u16()? as usize;
    let num_palettes = h.u16()? as usize;
    let num_color_records = h.u16()? as usize;
    let offset_first_color_record = h.u32()? as usize;

    let mut cr = FontReader::new(data).at(offset_first_color_record)?;
    cr.require_room(num_color_records, 4)?;
    let mut color_list: Vec<CpalColor> = Vec::with_capacity(num_color_records);
    for _ in 0..num_color_records {
        let blue = cr.u8()?;
        let green = cr.u8()?;
        let red = cr.u8()?;
        let alpha = cr.u8()?;
        color_list.push(CpalColor {
            red,
            green,
            blue,
            alpha,
            label: 0xffff,
        });
    }

    if data.len() < table_header_length + 2 * num_palettes {
        return Err(ReadError {
            needed: table_header_length + 2 * num_palettes,
            available: data.len(),
        });
    }
    let mut idx = FontReader::new(data).at(12)?;
    let mut palettes: Vec<CpalPalette> = Vec::with_capacity(num_palettes);
    for _ in 0..num_palettes {
        // `label: 0xffff`, not `0` -- matches what the deleted
        // `CPAL_I_PALETTE.init` call used to leave here (nothing
        // overwrites `.label` afterward in this function, unlike
        // `.type_0`/`.colorset`, which init also touched but every caller
        // re-sets).
        let palette_start_index = idx.u16()? as usize;
        let mut colorset = Vec::with_capacity(num_palettes_entries);
        for j in 0..num_palettes_entries {
            let color = palette_start_index
                .checked_add(j)
                .filter(|&i| i < num_color_records)
                .map_or(WHITE, |i| color_list[i]);
            colorset.push(color);
        }
        palettes.push(CpalPalette {
            colorset,
            type_0: 0,
            label: 0xffff,
        });
    }

    if version > 0 {
        if let Some(offset_palette_type_array) =
            FontReader::new(data).at(16 + 2 * num_palettes).ok().and_then(|mut r| r.u32().ok())
        {
            let offset_palette_type_array = offset_palette_type_array as usize;
            if offset_palette_type_array != 0 {
                if let Ok(mut tr) = FontReader::new(data).at(offset_palette_type_array) {
                    if tr.require_room(num_palettes, 4).is_ok() {
                        for p in palettes.iter_mut() {
                            p.type_0 = tr.u32().unwrap();
                        }
                    }
                }
            }
        }
        if let Some(offset_palette_label_array) =
            FontReader::new(data).at(20 + 2 * num_palettes).ok().and_then(|mut r| r.u32().ok())
        {
            let offset_palette_label_array = offset_palette_label_array as usize;
            if offset_palette_label_array != 0 {
                if let Ok(mut lr) = FontReader::new(data).at(offset_palette_label_array) {
                    if lr.require_room(num_palettes, 2).is_ok() {
                        for p in palettes.iter_mut() {
                            p.label = lr.u16().unwrap() as u32;
                        }
                    }
                }
            }
        }
        if let Some(offset_palette_entry_label_array) =
            FontReader::new(data).at(24 + 2 * num_palettes).ok().and_then(|mut r| r.u32().ok())
        {
            let offset_palette_entry_label_array = offset_palette_entry_label_array as usize;
            if offset_palette_entry_label_array != 0 {
                if let Ok(mut er) = FontReader::new(data).at(offset_palette_entry_label_array) {
                    if er.require_room(num_palettes_entries, 4).is_ok() {
                        for j in 0..num_palettes_entries {
                            let label = er.u16().unwrap();
                            for p in palettes.iter_mut() {
                                p.colorset[j].label = label;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok((version, palettes))
}
pub fn otfcc_read_cpal(packet: &Packet) -> Option<Box<CpalTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_CPAL)?;
    let (version, palettes) = parse_cpal(&table.data).ok()?;
    Some(Box::new(CpalTable { version, palettes }))
}
#[inline]
unsafe fn dump_color(color: *const CpalColor) -> BuiltValue {
    let mut _color = BuiltValue::new_object(5);
    _color.push_field(b"red", BuiltValue::Int((*color).red as i64));
    _color.push_field(b"green", BuiltValue::Int((*color).green as i64));
    _color.push_field(b"blue", BuiltValue::Int((*color).blue as i64));
    if (*color).alpha as i32 != 0xff_i32 {
        _color.push_field(b"alpha", BuiltValue::Int((*color).alpha as i64));
    }
    if (*color).label as i32 != 0xffff_i32 {
        _color.push_field(b"label", BuiltValue::Int((*color).label as i64));
    }
    _color.preserialize()
}
#[inline]
unsafe fn dump_palette(palette: *const CpalPalette) -> BuiltValue {
    let mut _palette = BuiltValue::new_object(3);
    if (*palette).type_0 != 0 {
        _palette.push_field(b"type", BuiltValue::Int((*palette).type_0 as i64));
    }
    if (*palette).label != 0xffff_u32 {
        _palette.push_field(b"label", BuiltValue::Int((*palette).label as i64));
    }
    let colorset: &Vec<CpalColor> = &(*palette).colorset;
    let mut a = BuiltValue::new_array(colorset.len());
    let mut j: ColorId = 0 as ColorId;
    while (j as usize) < colorset.len() {
        a.push_item(dump_color(&colorset[j as usize] as *const CpalColor));
        j = j.wrapping_add(1);
    }
    _palette.push_field(b"colors", a);
    _palette
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_cpal(
    table: Option<&CpalTable>,
    root: &mut BuiltValue,
    options: &Options,
) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"CPAL"),
    );
    let palettes: &Vec<CpalPalette> = &(*table).palettes;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _t = BuiltValue::new_object(2);
        _t.push_field(b"version", BuiltValue::Int((*table).version as i64));
        let mut _a = BuiltValue::new_array(palettes.len());
        let mut j: TableId = 0 as TableId;
        while (j as usize) < palettes.len() {
            _a.push_item(dump_palette(&palettes[j as usize] as *const CpalPalette));
            j = j.wrapping_add(1);
        }
        _t.push_field(b"palettes", _a);
        root.push_field(b"CPAL", _t);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
#[inline]
fn parse_color(color: Option<&ParsedValue>) -> CpalColor {
    let mut c: CpalColor = WHITE;
    let Some(color) = color.filter(|v| v.as_object().is_some()) else {
        return c;
    };
    c.red = color.get_int_or(b"red", 0) as u8;
    c.green = color.get_int_or(b"green", 0) as u8;
    c.blue = color.get_int_or(b"blue", 0) as u8;
    c.alpha = color.get_int_or(b"alpha", 0xff) as u8;
    c.label = color.get_int_or(b"label", 0xffff) as u16;
    c
}
pub unsafe fn otfcc_parse_cpal(
    root: &ParsedValue,
    options: &Options,
) -> Option<Box<CpalTable>> {
    let table = root.get_typed(b"CPAL", JsonType::Object)?;
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"CPAL"),
    );
    // Matches the pre-migration control flow exactly: an empty/missing
    // `palettes` array returns `None` here without ever calling
    // `logger_finish` -- an unbalanced logger start/finish that predates
    // this conversion, preserved rather than fixed (see the crate's
    // "no behavior change" rule).
    let palette_items = table
        .get_typed(b"palettes", JsonType::Array)
        .and_then(ParsedValue::as_array)
        .filter(|items| !items.is_empty())?;
    let version = table.get_int(b"version") as u16;
    let mut cpal: Box<CpalTable> = Box::new(CpalTable {
        version,
        palettes: Vec::new(),
    });
    for _palette in palette_items {
        if _palette.as_object().is_none() {
            continue;
        }
        let Some(color_items) = _palette
            .get_typed(b"colors", JsonType::Array)
            .and_then(ParsedValue::as_array)
        else {
            continue;
        };
        let mut palette: CpalPalette = CpalPalette {
            colorset: Vec::new(),
            type_0: _palette.get_int(b"type") as u32,
            label: _palette.get_int_or(b"type", 0xffff) as u32,
        };
        for _color in color_items {
            palette.colorset.push(parse_color(Some(_color)));
        }
        cpal.palettes.push(palette);
    }
    logger_finish(&mut *options.logger.borrow_mut());
    Some(cpal)
}
#[inline]
unsafe fn build_palette_type(cpal: *const CpalTable) -> *mut BkBlock {
    let palettes: &Vec<CpalPalette> = &(*cpal).palettes;
    let mut needs_palette_type: bool = false;
    let mut j: TableId = 0 as TableId;
    while (j as usize) < palettes.len() {
        if palettes[j as usize].type_0 != 0 {
            needs_palette_type = true;
        }
        j = j.wrapping_add(1);
    }
    if !needs_palette_type {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let block: *mut BkBlock = bk_new_block(&[]);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < palettes.len() {
        bk_push(
            block,
            &[bk_int(
                BkCellType::B32,
                (palettes[j_0 as usize].type_0) as u32,
            )],
        );
        j_0 = j_0.wrapping_add(1);
    }
    return block;
}
#[inline]
unsafe fn build_palette_label(cpal: *const CpalTable) -> *mut BkBlock {
    let palettes: &Vec<CpalPalette> = &(*cpal).palettes;
    let mut needs_palette_label: bool = false;
    let mut j: TableId = 0 as TableId;
    while (j as usize) < palettes.len() {
        if palettes[j as usize].label != 0xffff_u32 {
            needs_palette_label = true;
        }
        j = j.wrapping_add(1);
    }
    if !needs_palette_label {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let block: *mut BkBlock = bk_new_block(&[]);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < palettes.len() {
        bk_push(
            block,
            &[bk_int(
                BkCellType::B16,
                (palettes[j_0 as usize].label) as u32,
            )],
        );
        j_0 = j_0.wrapping_add(1);
    }
    return block;
}
#[inline]
unsafe fn build_palette_entry_label(cpal: *const CpalTable) -> *mut BkBlock {
    let palettes: &Vec<CpalPalette> = &(*cpal).palettes;
    let mut needs_palette_entry_label: bool = false;
    let palette: &CpalPalette = &palettes[0_usize];
    let mut j: ColorId = 0 as ColorId;
    while (j as usize) < palette.colorset.len() {
        if palette.colorset[j as usize].label as i32 != 0xffff_i32
        {
            needs_palette_entry_label = true;
        }
        j = j.wrapping_add(1);
    }
    if !needs_palette_entry_label {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let block: *mut BkBlock = bk_new_block(&[]);
    let mut j_0: ColorId = 0 as ColorId;
    while (j_0 as usize) < palette.colorset.len() {
        bk_push(
            block,
            &[bk_int(
                BkCellType::B16,
                (palette.colorset[j_0 as usize].label as i32) as u32,
            )],
        );
        j_0 = j_0.wrapping_add(1);
    }
    return block;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_cpal(cpal: Option<&CpalTable>) -> Option<Buffer> {
    let cpal = cpal? as *const CpalTable;
    let palettes: &Vec<CpalPalette> = &(*cpal).palettes;
    if palettes.is_empty() {
        return None;
    }
    let num_palettes: u16 = palettes.len() as u16;
    let num_palettes_entries: u16 = palettes[0_usize].colorset.len() as u16;
    let num_color_records: u16 =
        (num_palettes as i32 * num_palettes_entries as i32) as u16;
    let color_records: *mut BkBlock = bk_new_block(&[]);
    let mut j: TableId = 0 as TableId;
    while (j as i32) < num_palettes as i32 {
        let palette: &CpalPalette = &palettes[j as usize];
        let total_colors: ColorId = palette.colorset.len() as ColorId;
        let mut k: ColorId = 0 as ColorId;
        while (k as i32) < num_palettes_entries as i32 {
            let color: *const CpalColor;
            if (k as i32) < total_colors as i32 {
                color = &palette.colorset[k as usize] as *const CpalColor;
            } else {
                color = &raw const WHITE;
            }
            bk_push(
                color_records,
                &[
                    bk_int(BkCellType::B8, ((*color).blue as i32) as u32),
                    bk_int(
                        BkCellType::B8,
                        ((*color).green as i32) as u32,
                    ),
                    bk_int(BkCellType::B8, ((*color).red as i32) as u32),
                    bk_int(
                        BkCellType::B8,
                        ((*color).alpha as i32) as u32,
                    ),
                ],
            );
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(
            BkCellType::B16,
            ((*cpal).version as i32) as u32,
        ),
        bk_int(
            BkCellType::B16,
            (num_palettes_entries as i32) as u32,
        ),
        bk_int(BkCellType::B16, (num_palettes as i32) as u32),
        bk_int(
            BkCellType::B16,
            (num_color_records as i32) as u32,
        ),
        bk_ptr(BkCellType::P32, color_records),
    ]);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as i32) < num_palettes as i32 {
        bk_push(
            root,
            &[bk_int(
                BkCellType::B16,
                (num_palettes_entries as i32 * j_0 as i32) as u32,
            )],
        );
        j_0 = j_0.wrapping_add(1);
    }
    if (*cpal).version as i32 > 0_i32 {
        bk_push(
            root,
            &[
                bk_ptr(BkCellType::P32, build_palette_type(cpal)),
                bk_ptr(BkCellType::P32, build_palette_label(cpal)),
                bk_ptr(BkCellType::P32, build_palette_entry_label(cpal)),
            ],
        );
    }
    Some(bk_build_block(root))
}

#[cfg(test)]
mod parse_cpal_tests {
    use super::*;

    // header(12) + colorRecordIndices(2, one palette) + one color record(4)
    fn well_formed_v0_table() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&0u16.to_be_bytes()); // version
        b.extend_from_slice(&1u16.to_be_bytes()); // numPaletteEntries
        b.extend_from_slice(&1u16.to_be_bytes()); // numPalettes
        b.extend_from_slice(&1u16.to_be_bytes()); // numColorRecords
        b.extend_from_slice(&14u32.to_be_bytes()); // offsetFirstColorRecord
        b.extend_from_slice(&0u16.to_be_bytes()); // colorRecordIndices[0]
        b.extend_from_slice(&[10, 20, 30, 255]); // blue, green, red, alpha
        b
    }

    #[test]
    fn well_formed_v0_table_reads_one_palette_one_color() {
        let data = well_formed_v0_table();
        let (version, palettes) = parse_cpal(&data).unwrap();
        assert_eq!(version, 0);
        assert_eq!(palettes.len(), 1);
        let color = palettes[0].colorset[0];
        assert_eq!((color.red, color.green, color.blue, color.alpha), (30, 20, 10, 255));
        assert_eq!(color.label, 0xffff);
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        assert!(parse_cpal(&well_formed_v0_table()[..10]).is_err());
    }

    #[test]
    fn palette_entry_past_num_color_records_falls_back_to_white() {
        // colorRecordIndices[0] (palette_start_index) pointing past the
        // one real color record must fall back to WHITE, not index OOB
        // into color_list.
        let mut data = well_formed_v0_table();
        data[12..14].copy_from_slice(&5u16.to_be_bytes());
        let (_, palettes) = parse_cpal(&data).unwrap();
        let color = palettes[0].colorset[0];
        assert_eq!((color.red, color.green, color.blue, color.alpha), (255, 255, 255, 255));
    }

    #[test]
    fn color_record_offset_near_u32_max_is_rejected_not_wrapped() {
        // The original guarded `offset_first_color_record` (a raw,
        // fully attacker-controlled u32 read straight from the file)
        // with `x.wrapping_add(4 * num_color_records)`: a value this
        // close to u32::MAX wraps that addition back down to something
        // small, which could pass `length < wrapped_small_value` even
        // though the real offset points nowhere near this small table.
        let mut data = well_formed_v0_table();
        data[8..12].copy_from_slice(&0xFFFF_FFF0u32.to_be_bytes());
        assert!(parse_cpal(&data).is_err());
    }

    // version=1, one palette/entry/color record, plus a palette-type array
    // so the v1-only offset arithmetic is exercised. `table_header_length`
    // is 26 for v1, and the colorRecordIndices-region guard conservatively
    // demands `table_header_length + 2 * num_palettes` (28 bytes here) of
    // total table length even though colorRecordIndices itself only needs
    // 14 -- inherited from the original C, not something this migration
    // tightens -- so the table must reach at least 28 bytes before the v1
    // arrays are even considered.
    fn well_formed_v1_table_with_palette_type() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&1u16.to_be_bytes()); // version
        b.extend_from_slice(&1u16.to_be_bytes()); // numPaletteEntries
        b.extend_from_slice(&1u16.to_be_bytes()); // numPalettes
        b.extend_from_slice(&1u16.to_be_bytes()); // numColorRecords
        b.extend_from_slice(&14u32.to_be_bytes()); // offsetFirstColorRecord
        b.extend_from_slice(&0u16.to_be_bytes()); // colorRecordIndices[0], @12
        b.extend_from_slice(&[10, 20, 30, 255]); // color record, @14
        // offsetPaletteTypeArray lives at absolute offset 16 + 2*numPalettes
        // = 18 (this crate's CPAL reads it 4 bytes later than the spec
        // position -- see parse_cpal's doc comment).
        b.extend_from_slice(&28u32.to_be_bytes()); // @18: offsetPaletteTypeArray = 28
        b.resize(28, 0); // padding up to the guard2-mandated 28-byte minimum
        b.extend_from_slice(&0xCAFEBABEu32.to_be_bytes()); // @28: palette 0's type
        b
    }

    #[test]
    fn v1_palette_type_array_is_read_at_its_shifted_offset() {
        let data = well_formed_v1_table_with_palette_type();
        let (version, palettes) = parse_cpal(&data).unwrap();
        assert_eq!(version, 1);
        assert_eq!(palettes[0].type_0, 0xCAFEBABE);
    }

    #[test]
    fn palette_type_array_offset_near_u32_max_is_rejected_not_wrapped() {
        let mut data = well_formed_v1_table_with_palette_type();
        data[18..22].copy_from_slice(&0xFFFF_FFF0u32.to_be_bytes());
        let (_, palettes) = parse_cpal(&data).unwrap();
        // The optional array is simply left unpopulated on rejection --
        // the whole table isn't corrupted by one bad optional offset.
        assert_eq!(palettes[0].type_0, 0);
    }
}
