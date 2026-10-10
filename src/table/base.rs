use otfcc_binary::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use otfcc_binary::bk::graph::bk_build_block;
use crate::font::sfnt::Packet;
use otfcc_binary::Buffer;
use otfcc_json::BuiltValue;
use otfcc_binary::{FontReader, ReadError};
use otfcc_json::ParsedValue;
use crate::support::primitives::{Pos, TableId};
use otfcc_json::JsonType;

#[derive(Copy, Clone, Debug)]
pub struct BaseValue {
    pub tag: u32,
    pub coordinate: Pos,
}
#[derive(Debug)]
pub struct BaseScriptEntry {
    pub tag: u32,
    pub default_baseline_tag: u32,
    pub base_values: Vec<BaseValue>,
}
#[derive(Debug)]
pub struct BaseAxis {
    pub entries: Vec<BaseScriptEntry>,
}
#[derive(Debug)]
pub struct BaseTable {
    pub horizontal: Option<Box<BaseAxis>>,
    pub vertical: Option<Box<BaseAxis>>,
}
#[derive(Debug)]
pub struct BaseTagList {
    pub items: Vec<u32>,
}
fn read_base_value(data: &[u8], offset: usize) -> i16 {
    FontReader::new(data)
        .at(offset)
        .and_then(|mut r| {
            r.skip(2)?;
            r.i16()
        })
        .unwrap_or(0)
}
/// Reads a BaseScript at `offset` into `(default_baseline_tag,
/// base_values)`; `(0, empty)` if any check fails.
///
/// `offset`, and every offset derived from it, is a `usize`: two `u16`
/// offsets added together can pass 65535, and narrowing the sum back to
/// `u16` would wrap it to the wrong place.
fn read_base_script(
    data: &[u8],
    offset: usize,
    base_tag_list: &[u32],
    n_base_tags: u16,
) -> (u32, Vec<BaseValue>) {
    let Ok(mut r) = FontReader::new(data).at(offset) else {
        return (0, Vec::new());
    };
    let Ok(base_values_rel) = r.u16() else {
        return (0, Vec::new());
    };
    if base_values_rel == 0 {
        return (0, Vec::new());
    }
    let base_values_offset = offset + base_values_rel as usize;
    let Ok(mut r2) = FontReader::new(data).at(base_values_offset) else {
        return (0, Vec::new());
    };
    let Ok(default_index_raw) = r2.u16() else {
        return (0, Vec::new());
    };
    let default_index = (default_index_raw % n_base_tags) as usize;
    let default_baseline_tag: u32 = base_tag_list[default_index];
    let Ok(base_values_count) = r2.u16() else {
        return (0, Vec::new());
    };
    if base_values_count != n_base_tags {
        return (0, Vec::new());
    }
    if r2.require_room(base_values_count as usize, 2).is_err() {
        return (0, Vec::new());
    }
    let mut base_values: Vec<BaseValue> = Vec::with_capacity(base_values_count as usize);
    for j in 0..base_values_count {
        let tag = base_tag_list[j as usize];
        let val_offset = r2.u16().unwrap();
        let coordinate = if val_offset != 0 {
            read_base_value(data, base_values_offset + val_offset as usize) as Pos
        } else {
            0_i32 as Pos
        };
        base_values.push(BaseValue { tag, coordinate });
    }
    (default_baseline_tag, base_values)
}
/// Reads a BaseAxis; `None` if any of the format checks fail. Offsets stay
/// `usize` for the same reason as in `read_base_script`.
fn read_axis(data: &[u8], offset: usize) -> Option<Box<BaseAxis>> {
    let mut r = FontReader::new(data).at(offset).ok()?;
    let base_tag_list_rel = r.u16().ok()?;
    let base_script_list_rel = r.u16().ok()?;
    if base_tag_list_rel == 0 || base_script_list_rel == 0 {
        return None;
    }
    let base_tag_list_offset = offset + base_tag_list_rel as usize;
    let mut tl = FontReader::new(data).at(base_tag_list_offset).ok()?;
    let n_base_tags = tl.u16().ok()?;
    if n_base_tags == 0 {
        return None;
    }
    tl.require_room(n_base_tags as usize, 4).ok()?;
    let mut base_tag_list: Vec<u32> = Vec::with_capacity(n_base_tags as usize);
    for _ in 0..n_base_tags {
        base_tag_list.push(tl.u32().unwrap());
    }
    let base_script_list_offset = offset + base_script_list_rel as usize;
    let mut sl = FontReader::new(data).at(base_script_list_offset).ok()?;
    let n_base_scripts = sl.u16().ok()?;
    sl.require_room(n_base_scripts as usize, 6).ok()?;
    let mut entries: Vec<BaseScriptEntry> = Vec::with_capacity(n_base_scripts as usize);
    for _ in 0..n_base_scripts {
        let tag = sl.u32().unwrap();
        let base_script_rel = sl.u16().unwrap();
        if base_script_rel != 0 {
            let (default_baseline_tag, base_values) = read_base_script(
                data,
                base_script_list_offset + base_script_rel as usize,
                &base_tag_list,
                n_base_tags,
            );
            entries.push(BaseScriptEntry {
                tag,
                default_baseline_tag,
                base_values,
            });
        } else {
            entries.push(BaseScriptEntry {
                tag,
                default_baseline_tag: 0,
                base_values: Vec::new(),
            });
        }
    }
    Some(Box::new(BaseAxis { entries }))
}
/// `decode_base`'s own (horizontal, vertical) axis pair -- named once so the
/// return type isn't spelled out twice (its own signature and every match
/// arm's destructuring stay tuple-shaped either way, so no call site needs
/// updating).
type BaseAxisPair = (Option<Box<BaseAxis>>, Option<Box<BaseAxis>>);
fn decode_base(data: &[u8]) -> Result<BaseAxisPair, ReadError> {
    let mut r = FontReader::new(data);
    r.skip(4)?; // majorVersion(2) + minorVersion(2), unused
    let offset_h = r.u16()?;
    let offset_v = r.u16()?;
    let horizontal = (offset_h != 0)
        .then(|| read_axis(data, offset_h as usize))
        .flatten();
    let vertical = (offset_v != 0)
        .then(|| read_axis(data, offset_v as usize))
        .flatten();
    Ok((horizontal, vertical))
}
pub fn read_base(packet: &Packet) -> Option<Box<BaseTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_BASE)?;
    let (horizontal, vertical) = match decode_base(&table.data) {
        Ok(parsed) => parsed,
        Err(_) => {
            tracing::warn!("Table 'BASE' Corrupted");
            return None;
        }
    };
    Some(Box::new(BaseTable {
        horizontal,
        vertical,
    }))
}
fn axis_to_json(axis: &BaseAxis) -> BuiltValue {
    let mut _axis = BuiltValue::new_object(axis.entries.len());
    for entry in axis.entries.iter() {
        if entry.tag != 0 {
            let mut _entry = BuiltValue::new_object(3);
            if entry.default_baseline_tag != 0 {
                let tag_bytes: [u8; 4] = [
                    ((entry.default_baseline_tag & 0xff000000u32) >> 24) as u8,
                    ((entry.default_baseline_tag & 0xff0000u32) >> 16) as u8,
                    ((entry.default_baseline_tag & 0xff00u32) >> 8) as u8,
                    (entry.default_baseline_tag & 0xffu32) as u8,
                ];
                _entry.push_field(b"defaultBaseline", BuiltValue::Str(tag_bytes.to_vec()));
            }
            let mut _values = BuiltValue::new_object(entry.base_values.len());
            for bv in entry.base_values.iter() {
                if bv.tag != 0 {
                    _values.push_tag(bv.tag, BuiltValue::position(bv.coordinate));
                }
            }
            _entry.push_field(b"baselines", _values);
            _axis.push_tag(entry.tag, _entry);
        }
    }
    _axis
}
pub fn dump_base(base: Option<&BaseTable>, root: &mut BuiltValue) {
    let Some(base) = base else { return };
    let stage = crate::logger::stage("BASE");
    {
        let mut _base = BuiltValue::new_object(2);
        if let Some(horizontal) = base.horizontal.as_deref() {
            _base.push_field(b"horizontal", axis_to_json(horizontal));
        }
        if let Some(vertical) = base.vertical.as_deref() {
            _base.push_field(b"vertical", axis_to_json(vertical));
        }
        root.push_field(b"BASE", _base);
        stage.finish();
    }
}
/// Returns `(default_baseline_tag, base_values)`, the JSON-side twin of
/// `read_base_script`.
fn base_script_from_json(sr: Option<&ParsedValue>) -> (u32, Vec<BaseValue>) {
    let default_baseline_tag = str2tag(sr.and_then(|v| v.get_bytes(b"defaultBaseline")));
    let Some(basevalues) = sr.and_then(|v| v.get_typed(b"baselines", JsonType::Object)) else {
        return (default_baseline_tag, Vec::new());
    };
    let fields = basevalues.as_object().unwrap();
    let mut base_values: Vec<BaseValue> = Vec::with_capacity(fields.len());
    for (key, val) in fields {
        base_values.push(BaseValue {
            tag: str2tag(Some(&key[..key.len() - 1])),
            coordinate: val.as_num().unwrap_or(0.0) as Pos,
        });
    }
    (default_baseline_tag, base_values)
}
/// `entries` gets only the object-typed values, then is sorted by tag
/// (stably).
fn axis_from_json(axis: Option<&ParsedValue>) -> Option<Box<BaseAxis>> {
    let axis = axis?;
    let mut entries: Vec<BaseScriptEntry> = Vec::new();
    if let Some(fields) = axis.as_object() {
        for (key, val) in fields {
            if val.as_object().is_some() {
                let tag = str2tag(Some(&key[..key.len() - 1]));
                let (default_baseline_tag, base_values) = base_script_from_json(Some(val));
                entries.push(BaseScriptEntry {
                    tag,
                    default_baseline_tag,
                    base_values,
                });
            }
        }
    }
    entries.sort_by_key(|e| e.tag);
    Some(Box::new(BaseAxis { entries }))
}
pub fn parse_base(root: &ParsedValue) -> Option<Box<BaseTable>> {
    let mut base: Option<Box<BaseTable>> = None;
    let table = root.get_typed(b"BASE", JsonType::Object);
    if let Some(table) = table {
        let stage = crate::logger::stage("BASE");
        {
            let horizontal = axis_from_json(table.get_typed(b"horizontal", JsonType::Object));
            let vertical = axis_from_json(table.get_typed(b"vertical", JsonType::Object));
            base = Some(Box::new(BaseTable {
                horizontal,
                vertical,
            }));
            stage.finish();
        }
    }
    return base;
}
pub fn axis_to_bk(axis: &BaseAxis) -> BkBlock {
    let mut taglist: BaseTagList = BaseTagList { items: Vec::new() };
    for entry in axis.entries.iter() {
        if entry.default_baseline_tag != 0 && !taglist.items.contains(&entry.default_baseline_tag) {
            taglist.items.push(entry.default_baseline_tag);
        }
        for bv in entry.base_values.iter() {
            if !taglist.items.contains(&bv.tag) {
                taglist.items.push(bv.tag);
            }
        }
    }
    taglist.items.sort();
    let mut base_tag_list: BkBlock = bk_new_block(vec![bk_int(
        BkCellType::B16,
        (taglist.items.len() as i32) as u32,
    )]);
    for &tag in taglist.items.iter() {
        bk_push(&mut base_tag_list, vec![bk_int(BkCellType::B32, tag)]);
    }
    let mut base_script_list: BkBlock = bk_new_block(vec![bk_int(
        BkCellType::B16,
        (axis.entries.len() as i32) as u32,
    )]);
    for entry_0 in axis.entries.iter() {
        let mut base_values: BkBlock = bk_new_block(Vec::new());
        // A default baseline tag missing from `taglist` (not expected, since
        // every default tag was added above) falls back to index 0.
        let default_index = taglist
            .items
            .iter()
            .position(|&t| t == entry_0.default_baseline_tag)
            .unwrap_or(0) as TableId;
        bk_push(
            &mut base_values,
            vec![bk_int(
                BkCellType::B16,
                (default_index as i32) as u32,
            )],
        );
        bk_push(
            &mut base_values,
            vec![bk_int(
                BkCellType::B16,
                (taglist.items.len() as i32) as u32,
            )],
        );
        for &tag in taglist.items.iter() {
            let found_index = entry_0.base_values.iter().position(|bv| bv.tag == tag);
            if let Some(found_index) = found_index {
                bk_push(
                    &mut base_values,
                    vec![bk_ptr(
                        BkCellType::P16,
                        Some(bk_new_block(vec![
                            bk_int(BkCellType::B16, 1_u32),
                            bk_int(
                                BkCellType::B16,
                                (entry_0.base_values[found_index].coordinate as i16 as i32) as u32,
                            ),
                        ])),
                    )],
                );
            } else {
                bk_push(
                    &mut base_values,
                    vec![bk_ptr(
                        BkCellType::P16,
                        Some(bk_new_block(vec![
                            bk_int(BkCellType::B16, 1_u32),
                            bk_int(BkCellType::B16, 0_u32),
                        ])),
                    )],
                );
            }
        }
        let script_record: BkBlock = bk_new_block(vec![
            bk_ptr(BkCellType::P16, Some(base_values)),
            bk_ptr(BkCellType::P16, None),
            bk_int(BkCellType::B16, 0_u32),
        ]);
        bk_push(
            &mut base_script_list,
            vec![
                bk_int(BkCellType::B32, entry_0.tag),
                bk_ptr(BkCellType::P16, Some(script_record)),
            ],
        );
    }
    return bk_new_block(vec![
        bk_ptr(BkCellType::P16, Some(base_tag_list)),
        bk_ptr(BkCellType::P16, Some(base_script_list)),
    ]);
}
pub fn build_base(base: Option<&BaseTable>) -> Option<Buffer> {
    let base = base?;
    let horizontal_bk = base.horizontal.as_deref().map(axis_to_bk);
    let vertical_bk = base.vertical.as_deref().map(axis_to_bk);
    let root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B32, 0x10000_u32),
        bk_ptr(BkCellType::P16, horizontal_bk),
        bk_ptr(BkCellType::P16, vertical_bk),
    ]);
    Some(bk_build_block(root))
}
#[inline]
fn str2tag(tags: Option<&[u8]>) -> u32 {
    let Some(tags) = tags else {
        return 0_u32;
    };
    let mut tag: u32 = 0_u32;
    let mut len: u8 = 0_u8;
    for &b in tags.iter().take(4) {
        tag = tag << 8_i32 | b as u32;
        len = len.wrapping_add(1);
    }
    for _ in len..4_u8 {
        tag = tag << 8_i32 | ' ' as i32 as u32;
    }
    tag
}

#[cfg(test)]
mod parse_base_tests {
    use super::*;

    const HANG: u32 = 0x68616e67; // "hang"

    // header(8) + axis(4) + tag list(6, one tag) + script list(8, one
    // record) + script table(2) + base values(6, one coord) + coord(4)
    fn well_formed_base_table() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&0u16.to_be_bytes()); // majorVersion
        b.extend_from_slice(&1u16.to_be_bytes()); // minorVersion
        b.extend_from_slice(&8u16.to_be_bytes()); // HorizAxisOffset
        b.extend_from_slice(&0u16.to_be_bytes()); // VertAxisOffset (none)
        // Axis table @8
        b.extend_from_slice(&4u16.to_be_bytes()); // BaseTagListOffset (rel to 8)
        b.extend_from_slice(&10u16.to_be_bytes()); // BaseScriptListOffset (rel to 8)
        // BaseTagList @12
        b.extend_from_slice(&1u16.to_be_bytes()); // BaseTagCount
        b.extend_from_slice(&HANG.to_be_bytes());
        // BaseScriptList @18
        b.extend_from_slice(&1u16.to_be_bytes()); // BaseScriptCount
        b.extend_from_slice(&HANG.to_be_bytes()); // BaseScriptTag
        b.extend_from_slice(&8u16.to_be_bytes()); // BaseScriptOffset (rel to 18)
        // BaseScript table @26
        b.extend_from_slice(&2u16.to_be_bytes()); // BaseValuesOffset (rel to 26)
        // BaseValues table @28
        b.extend_from_slice(&0u16.to_be_bytes()); // DefaultIndex
        b.extend_from_slice(&1u16.to_be_bytes()); // BaseCoordCount
        b.extend_from_slice(&6u16.to_be_bytes()); // BaseCoordOffset[0] (rel to 28)
        // BaseCoord @34
        b.extend_from_slice(&1u16.to_be_bytes()); // format (unread, format-agnostic)
        b.extend_from_slice(&500i16.to_be_bytes()); // Coordinate
        b
    }

    #[test]
    fn well_formed_table_reads_the_horizontal_axis() {
        let data = well_formed_base_table();
        let (horizontal, vertical) = decode_base(&data).unwrap();
        assert!(vertical.is_none());
        let axis = horizontal.unwrap();
        assert_eq!(axis.entries.len(), 1);
        assert_eq!(axis.entries[0].tag, HANG);
        assert_eq!(axis.entries[0].default_baseline_tag, HANG);
        assert_eq!(axis.entries[0].base_values.len(), 1);
        assert_eq!(axis.entries[0].base_values[0].tag, HANG);
        assert_eq!(axis.entries[0].base_values[0].coordinate, 500.0);
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        assert!(decode_base(&well_formed_base_table()[..6]).is_err());
    }

    #[test]
    fn zero_axis_offset_is_absent_not_an_error() {
        let mut data = well_formed_base_table();
        data[4..6].copy_from_slice(&0u16.to_be_bytes()); // HorizAxisOffset = 0
        let (horizontal, vertical) = decode_base(&data).unwrap();
        assert!(horizontal.is_none());
        assert!(vertical.is_none());
    }

    #[test]
    fn zero_base_tag_count_makes_the_axis_absent() {
        let mut data = well_formed_base_table();
        data[12..14].copy_from_slice(&0u16.to_be_bytes()); // BaseTagCount = 0
        let (horizontal, _) = decode_base(&data).unwrap();
        assert!(horizontal.is_none());
    }

    #[test]
    fn base_coord_count_mismatched_with_tag_count_is_rejected() {
        // BaseCoordCount (absolute offset 30) is 1 in the fixture, but
        // n_base_tags here is 2 -- must be rejected, not read with the
        // wrong count.
        let base_tag_list = vec![HANG, HANG];
        let data = well_formed_base_table();
        let (tag, values) = read_base_script(&data, 26, &base_tag_list, 2);
        assert_eq!(tag, 0);
        assert!(values.is_empty());
    }

    #[test]
    fn base_script_offset_sum_near_u16_boundary_does_not_wrap() {
        // `base_values_offset + offset` must not be truncated to u16. With
        // `offset` = 60000 and a BaseValuesOffset field of 10000, the true
        // combined offset is 70000, which a u16 would wrap down to
        // 70000 - 65536 = 4464. A well-formed BaseValues structure placed
        // only at the true offset (70000, left as zeros at the wrapped
        // address) must be read from there, not from the wrapped address.
        let base_tag_list = vec![HANG];
        let mut data = vec![0u8; 70010];
        data[60000..60002].copy_from_slice(&10000u16.to_be_bytes());
        data[70000..70002].copy_from_slice(&0u16.to_be_bytes()); // DefaultIndex
        data[70002..70004].copy_from_slice(&1u16.to_be_bytes()); // BaseCoordCount
        data[70004..70006].copy_from_slice(&6u16.to_be_bytes()); // BaseCoordOffset[0]
        data[70006..70008].copy_from_slice(&1u16.to_be_bytes()); // format
        data[70008..70010].copy_from_slice(&500i16.to_be_bytes()); // Coordinate
        let (default_baseline_tag, base_values) = read_base_script(&data, 60000, &base_tag_list, 1);
        assert_eq!(default_baseline_tag, HANG);
        assert_eq!(base_values.len(), 1);
        assert_eq!(base_values[0].coordinate, 500.0);
    }

    #[test]
    fn axis_tag_list_offset_sum_near_u16_boundary_does_not_wrap() {
        // Same wraparound shape as the BaseScript test above, but for
        // `read_axis`'s own `BaseTagListOffset`/`BaseScriptListOffset`
        // fields.
        let mut data = vec![0u8; 70020];
        data[60000..60002].copy_from_slice(&10000u16.to_be_bytes()); // BaseTagListOffset (rel)
        data[60002..60004].copy_from_slice(&10010u16.to_be_bytes()); // BaseScriptListOffset (rel)
        // BaseTagList @70000
        data[70000..70002].copy_from_slice(&1u16.to_be_bytes());
        data[70002..70006].copy_from_slice(&HANG.to_be_bytes());
        // BaseScriptList @70010
        data[70010..70012].copy_from_slice(&1u16.to_be_bytes());
        data[70012..70016].copy_from_slice(&HANG.to_be_bytes());
        data[70016..70018].copy_from_slice(&0u16.to_be_bytes()); // BaseScriptOffset = 0 (absent)
        let axis = read_axis(&data, 60000).unwrap();
        assert_eq!(axis.entries.len(), 1);
        assert_eq!(axis.entries[0].tag, HANG);
        assert_eq!(axis.entries[0].default_baseline_tag, 0);
        assert!(axis.entries[0].base_values.is_empty());
    }
}
