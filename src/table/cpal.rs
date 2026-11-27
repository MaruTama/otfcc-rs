use crate::font::sfnt::Packet;
use crate::support::primitives::ColorId;
use otfcc_binary::Buffer;
use otfcc_binary::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use otfcc_binary::{FontReader, ReadError};
use otfcc_json::JsonType;
use otfcc_json::ParsedValue;

use otfcc_binary::bk::graph::bk_build_block;
use otfcc_json::BuiltValue;
/// One palette entry. `label` is the entry's name id, shared by the same entry
/// in every palette, or 0xFFFF for "no label".
#[derive(Copy, Clone, Debug)]
pub struct CpalColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
    pub label: u16,
}
/// One palette: its colors, plus (version 1 only) its type flags and name id
/// (0xFFFF for "no label").
#[derive(Clone, Debug)]
pub struct CpalPalette {
    pub colorset: Vec<CpalColor>,
    pub palette_type: u32,
    pub label: u32,
}
/// The 'CPAL' table: the palettes that 'COLR' layers refer to by index. Every
/// palette has the same number of entries.
#[derive(Clone, Debug)]
pub struct CpalTable {
    pub version: u16,
    pub palettes: Vec<CpalPalette>,
}
pub static WHITE: CpalColor = CpalColor {
    red: 0xff,
    green: 0xff,
    blue: 0xff,
    alpha: 0xff,
    label: 0xffff,
};
/// The 3 v1 offsets (`paletteTypesArrayOffset`, `paletteLabelsArrayOffset`,
/// `paletteEntryLabelsArrayOffset`) follow `colorRecordIndices` directly, at
/// `12`/`16`/`20 + 2 * num_palettes`. (otfcc used to read them 4 bytes
/// further on, so each array was read from the next one's offset.)
///
/// `offset_first_color_record` and the three v1 array offsets are raw `u32`s
/// straight from the file (up to `u32::MAX`), so every position is computed
/// through `FontReader::at`/`require_room` (`checked_add`/`checked_mul`)
/// rather than by plain addition that could wrap past a length check.
fn decode_cpal(data: &[u8]) -> Result<(u16, Vec<CpalPalette>), ReadError> {
    if data.len() < 2 {
        return Err(ReadError {
            needed: 2,
            available: data.len(),
        });
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

    let color_list = read_color_records(data, offset_first_color_record, num_color_records)?;

    if data.len() < table_header_length + 2 * num_palettes {
        return Err(ReadError {
            needed: table_header_length + 2 * num_palettes,
            available: data.len(),
        });
    }
    let mut palettes = read_palettes(data, &color_list, num_palettes, num_palettes_entries)?;

    if version > 0 {
        read_v1_extensions(data, &mut palettes, num_palettes_entries);
    }

    Ok((version, palettes))
}

/// Reads the shared pool of BGRA color records that every palette indexes into.
fn read_color_records(
    data: &[u8],
    offset_first_color_record: usize,
    num_color_records: usize,
) -> Result<Vec<CpalColor>, ReadError> {
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
    Ok(color_list)
}

/// Reads `colorRecordIndices` and materialises each palette's colors from the
/// shared pool. An entry that points past the pool becomes `WHITE`. Palette
/// type and label start as "none" (`0` / `0xffff`); v1 data fills them in.
fn read_palettes(
    data: &[u8],
    color_list: &[CpalColor],
    num_palettes: usize,
    num_palettes_entries: usize,
) -> Result<Vec<CpalPalette>, ReadError> {
    let mut idx = FontReader::new(data).at(12)?;
    let mut palettes: Vec<CpalPalette> = Vec::with_capacity(num_palettes);
    for _ in 0..num_palettes {
        let palette_start_index = idx.u16()? as usize;
        let mut colorset = Vec::with_capacity(num_palettes_entries);
        for j in 0..num_palettes_entries {
            let color = palette_start_index
                .checked_add(j)
                .filter(|&i| i < color_list.len())
                .map_or(WHITE, |i| color_list[i]);
            colorset.push(color);
        }
        palettes.push(CpalPalette {
            colorset,
            palette_type: 0,
            label: 0xffff,
        });
    }
    Ok(palettes)
}

/// Follows one of the three v1 offset fields (stored at `field_offset`) to its
/// array. `None` when the field is unreadable, zero ("absent"), points outside
/// the table, or the array of `count` elements of `element_size` bytes does
/// not fit; those arrays are optional, so the palettes keep their defaults.
fn open_v1_array(
    data: &[u8],
    field_offset: usize,
    count: usize,
    element_size: usize,
) -> Option<FontReader<'_>> {
    let offset = FontReader::new(data)
        .at(field_offset)
        .ok()
        .and_then(|mut r| r.u32().ok())? as usize;
    if offset == 0 {
        return None;
    }
    let reader = FontReader::new(data).at(offset).ok()?;
    reader.require_room(count, element_size).ok()?;
    Some(reader)
}

/// Reads the optional v1 arrays: palette types, palette labels and the
/// per-entry labels shared by all palettes.
fn read_v1_extensions(data: &[u8], palettes: &mut [CpalPalette], num_palettes_entries: usize) {
    let num_palettes = palettes.len();
    if let Some(mut tr) = open_v1_array(data, 12 + 2 * num_palettes, num_palettes, 4) {
        for p in palettes.iter_mut() {
            p.palette_type = tr.u32().expect("require_room above covered this read");
        }
    }
    if let Some(mut lr) = open_v1_array(data, 16 + 2 * num_palettes, num_palettes, 2) {
        for p in palettes.iter_mut() {
            p.label = u32::from(lr.u16().expect("require_room above covered this read"));
        }
    }
    // A uint16 name ID per entry.
    if let Some(mut er) = open_v1_array(data, 20 + 2 * num_palettes, num_palettes_entries, 2) {
        for j in 0..num_palettes_entries {
            let label = er.u16().expect("require_room above covered this read");
            for p in palettes.iter_mut() {
                p.colorset[j].label = label;
            }
        }
    }
}
/// Reads the 'CPAL' table from `packet`. Returns `None` when the table is absent
/// or corrupted.
pub fn read_cpal(packet: &Packet) -> Option<Box<CpalTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_CPAL)?;
    let (version, palettes) = decode_cpal(&table.data).ok()?;
    Some(Box::new(CpalTable { version, palettes }))
}
#[inline]
fn dump_color(color: &CpalColor) -> BuiltValue {
    let mut _color = BuiltValue::new_object(5);
    _color.push_field(b"red", BuiltValue::Int(i64::from(color.red)));
    _color.push_field(b"green", BuiltValue::Int(i64::from(color.green)));
    _color.push_field(b"blue", BuiltValue::Int(i64::from(color.blue)));
    if i32::from(color.alpha) != 0xff {
        _color.push_field(b"alpha", BuiltValue::Int(i64::from(color.alpha)));
    }
    if i32::from(color.label) != 0xffff {
        _color.push_field(b"label", BuiltValue::Int(i64::from(color.label)));
    }
    _color.preserialize()
}
#[inline]
fn dump_palette(palette: &CpalPalette) -> BuiltValue {
    let mut _palette = BuiltValue::new_object(3);
    if palette.palette_type != 0 {
        _palette.push_field(b"type", BuiltValue::Int(i64::from(palette.palette_type)));
    }
    if palette.label != 0xffff {
        _palette.push_field(b"label", BuiltValue::Int(i64::from(palette.label)));
    }
    let colorset: &Vec<CpalColor> = &palette.colorset;
    let mut a = BuiltValue::new_array(colorset.len());
    for color in colorset.iter() {
        a.push_item(dump_color(color));
    }
    _palette.push_field(b"colors", a);
    _palette
}
/// Dumps the table as the `"CPAL"` object (`version` and `palettes`).
pub fn dump_cpal(table: Option<&CpalTable>, root: &mut BuiltValue) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    let stage = crate::logger::stage("CPAL");
    let palettes: &Vec<CpalPalette> = &table.palettes;
    {
        let mut _t = BuiltValue::new_object(2);
        _t.push_field(b"version", BuiltValue::Int(i64::from(table.version)));
        let mut _a = BuiltValue::new_array(palettes.len());
        for p in palettes.iter() {
            _a.push_item(dump_palette(p));
        }
        _t.push_field(b"palettes", _a);
        root.push_field(b"CPAL", _t);
        stage.finish();
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
/// Reads the `"CPAL"` object. Returns `None` when `palettes` is missing or
/// empty. A color that is not an object becomes opaque white, and missing channels
/// default to 0 (alpha to 0xFF).
pub fn parse_cpal(root: &ParsedValue) -> Option<Box<CpalTable>> {
    let table = root.get_typed(b"CPAL", JsonType::Object)?;
    let stage = crate::logger::stage("CPAL");
    // An empty/missing `palettes` array returns `None`, closing the stage
    // without a `Finish` line.
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
            palette_type: _palette.get_int(b"type") as u32,
            label: _palette.get_int_or(b"label", 0xffff) as u32,
        };
        for _color in color_items {
            palette.colorset.push(parse_color(Some(_color)));
        }
        cpal.palettes.push(palette);
    }
    stage.finish();
    Some(cpal)
}
#[inline]
fn build_palette_type(cpal: &CpalTable) -> Option<BkBlock> {
    let palettes: &Vec<CpalPalette> = &cpal.palettes;
    if !palettes.iter().any(|p| p.palette_type != 0) {
        return None;
    }
    let mut block: BkBlock = bk_new_block(Vec::new());
    for p in palettes.iter() {
        bk_push(&mut block, vec![bk_int(BkCellType::B32, p.palette_type)]);
    }
    Some(block)
}
#[inline]
fn build_palette_label(cpal: &CpalTable) -> Option<BkBlock> {
    let palettes: &Vec<CpalPalette> = &cpal.palettes;
    if !palettes.iter().any(|p| p.label != 0xffff) {
        return None;
    }
    let mut block: BkBlock = bk_new_block(Vec::new());
    for p in palettes.iter() {
        bk_push(&mut block, vec![bk_int(BkCellType::B16, p.label)]);
    }
    Some(block)
}
#[inline]
fn build_palette_entry_label(cpal: &CpalTable) -> Option<BkBlock> {
    let palettes: &Vec<CpalPalette> = &cpal.palettes;
    let palette: &CpalPalette = &palettes[0];
    if !palette
        .colorset
        .iter()
        .any(|c| i32::from(c.label) != 0xffff)
    {
        return None;
    }
    let mut block: BkBlock = bk_new_block(Vec::new());
    for c in palette.colorset.iter() {
        bk_push(
            &mut block,
            vec![bk_int(BkCellType::B16, u32::from(c.label))],
        );
    }
    Some(block)
}
/// Serializes the table. Returns `None` when there are no palettes. The entry
/// count is taken from the first palette; a shorter palette is padded with white.
/// The version 1 arrays (types, labels, entry labels) are only written for
/// version above 0.
pub fn build_cpal(cpal: Option<&CpalTable>) -> Option<Buffer> {
    let cpal = cpal?;
    let palettes: &Vec<CpalPalette> = &cpal.palettes;
    if palettes.is_empty() {
        return None;
    }
    let num_palettes: u16 = palettes.len() as u16;
    let num_palettes_entries: u16 = palettes[0].colorset.len() as u16;
    let num_color_records: u16 = (i32::from(num_palettes) * i32::from(num_palettes_entries)) as u16;
    let mut color_records: BkBlock = bk_new_block(Vec::new());
    for palette in palettes.iter().take(num_palettes as usize) {
        let total_colors: ColorId = palette.colorset.len() as ColorId;
        for k in 0..num_palettes_entries {
            let color: &CpalColor = if i32::from(k) < i32::from(total_colors) {
                &palette.colorset[k as usize]
            } else {
                &WHITE
            };
            bk_push(
                &mut color_records,
                vec![
                    bk_int(BkCellType::B8, u32::from(color.blue)),
                    bk_int(BkCellType::B8, u32::from(color.green)),
                    bk_int(BkCellType::B8, u32::from(color.red)),
                    bk_int(BkCellType::B8, u32::from(color.alpha)),
                ],
            );
        }
    }
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, u32::from(cpal.version)),
        bk_int(BkCellType::B16, u32::from(num_palettes_entries)),
        bk_int(BkCellType::B16, u32::from(num_palettes)),
        bk_int(BkCellType::B16, u32::from(num_color_records)),
        bk_ptr(BkCellType::P32, Some(color_records)),
    ]);
    for j_0 in 0..num_palettes {
        bk_push(
            &mut root,
            vec![bk_int(
                BkCellType::B16,
                (i32::from(num_palettes_entries) * i32::from(j_0)) as u32,
            )],
        );
    }
    if i32::from(cpal.version) > 0 {
        bk_push(
            &mut root,
            vec![
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
        let (version, palettes) = decode_cpal(&data).unwrap();
        assert_eq!(version, 0);
        assert_eq!(palettes.len(), 1);
        let color = palettes[0].colorset[0];
        assert_eq!(
            (color.red, color.green, color.blue, color.alpha),
            (30, 20, 10, 255)
        );
        assert_eq!(color.label, 0xffff);
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        assert!(decode_cpal(&well_formed_v0_table()[..10]).is_err());
    }

    #[test]
    fn palette_entry_past_num_color_records_falls_back_to_white() {
        // colorRecordIndices[0] (palette_start_index) pointing past the
        // one real color record must fall back to WHITE, not index OOB
        // into color_list.
        let mut data = well_formed_v0_table();
        data[12..14].copy_from_slice(&5u16.to_be_bytes());
        let (_, palettes) = decode_cpal(&data).unwrap();
        let color = palettes[0].colorset[0];
        assert_eq!(
            (color.red, color.green, color.blue, color.alpha),
            (255, 255, 255, 255)
        );
    }

    #[test]
    fn color_record_offset_near_u32_max_is_rejected_not_wrapped() {
        // `offset_first_color_record` is a raw u32 from the file: a value
        // this close to u32::MAX must not wrap `x + 4 * num_color_records`
        // back down to something small that passes the length check.
        let mut data = well_formed_v0_table();
        data[8..12].copy_from_slice(&0xFFFF_FFF0u32.to_be_bytes());
        assert!(decode_cpal(&data).is_err());
    }

    // version=1, one palette, entry and color record, and all three v1
    // arrays, laid out as the specification has it: the 12-byte header,
    // colorRecordIndices (2 bytes per palette), then the three offsets.
    fn well_formed_v1_table_with_palette_type() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&1u16.to_be_bytes()); // version
        b.extend_from_slice(&1u16.to_be_bytes()); // numPaletteEntries
        b.extend_from_slice(&1u16.to_be_bytes()); // numPalettes
        b.extend_from_slice(&1u16.to_be_bytes()); // numColorRecords
        b.extend_from_slice(&26u32.to_be_bytes()); // colorRecordsArrayOffset
        b.extend_from_slice(&0u16.to_be_bytes()); // colorRecordIndices[0], @12
        b.extend_from_slice(&30u32.to_be_bytes()); // @14: paletteTypesArrayOffset
        b.extend_from_slice(&34u32.to_be_bytes()); // @18: paletteLabelsArrayOffset
        b.extend_from_slice(&36u32.to_be_bytes()); // @22: paletteEntryLabelsArrayOffset
        b.extend_from_slice(&[10, 20, 30, 255]); // @26: the color record
        b.extend_from_slice(&0xCAFEBABEu32.to_be_bytes()); // @30: palette 0's type
        b.extend_from_slice(&256u16.to_be_bytes()); // @34: palette 0's label
        b.extend_from_slice(&257u16.to_be_bytes()); // @36: entry 0's label
        b
    }

    #[test]
    fn v1_arrays_are_read_where_the_specification_puts_them() {
        let data = well_formed_v1_table_with_palette_type();
        let (version, palettes) = decode_cpal(&data).unwrap();
        assert_eq!(version, 1);
        assert_eq!(palettes[0].palette_type, 0xCAFEBABE);
        assert_eq!(palettes[0].label, 256);
        // The entry labels are uint16, so an array at the very end of the
        // table, two bytes long, is read.
        assert_eq!(palettes[0].colorset[0].label, 257);
    }

    #[test]
    fn palette_type_array_offset_near_u32_max_is_rejected_not_wrapped() {
        let mut data = well_formed_v1_table_with_palette_type();
        data[14..18].copy_from_slice(&0xFFFF_FFF0u32.to_be_bytes());
        let (_, palettes) = decode_cpal(&data).unwrap();
        // The optional array is simply left unpopulated on rejection --
        // the whole table isn't corrupted by one bad optional offset.
        assert_eq!(palettes[0].palette_type, 0);
    }
}
