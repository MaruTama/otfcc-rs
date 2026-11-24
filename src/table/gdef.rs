use crate::font::sfnt::Packet;
use crate::support::handle::{GlyphHandle, Handle, HandleState, handle_from_name};
use crate::support::primitives::Pos;
use crate::table::otl::budget::OtlReadBudget;
use crate::table::otl::classdef::{ClassDef, read_class_def};
use crate::table::otl::coverage::{
    Coverage, dump_coverage, parse_coverage, push_to_coverage, read_coverage,
};
use otfcc_binary::Buffer;
use otfcc_binary::FontReader;
use otfcc_binary::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use otfcc_json::JsonType;
use otfcc_json::ParsedValue;

use crate::table::otl::classdef::{build_class_def, dump_class_def, parse_class_def};
use crate::table::otl::coverage::build_coverage;
use otfcc_binary::bk::block::bk_new_block_from_buffer;
use otfcc_binary::bk::graph::bk_build_block;
use otfcc_json::BuiltValue;
/// One ligature caret: `format` is the GDEF CaretValue format (1: a
/// coordinate, 2: a contour point index; device tables are not kept).
/// `coordiante` (sic) holds the coordinate and `point_index` the point.
#[derive(Copy, Clone, Debug)]
pub struct CaretValue {
    pub format: i8,
    pub coordiante: Pos,
    pub point_index: i16,
}
/// The carets of one ligature glyph, in order.
pub type CaretValueList = Vec<CaretValue>;
/// The carets of a single ligature `glyph`.
#[derive(Debug)]
pub struct CaretValueRecord {
    pub glyph: GlyphHandle,
    pub carets: CaretValueList,
}
/// All ligature caret records of the font.
pub type LigCaretTable = Vec<CaretValueRecord>;
// Shared by `dispose_gdef` and `consolidate_gdef`.
pub(crate) fn clear_lig_carets(lc: &mut LigCaretTable) {
    lc.clear();
}
/// The `GDEF` table: glyph classes, mark attachment classes, ligature
/// carets and mark glyph sets (the other GDEF parts are not kept).
#[derive(Debug)]
pub struct GdefTable {
    pub glyph_class_def: Option<Box<ClassDef>>,
    pub mark_attach_class_def: Option<Box<ClassDef>>,
    pub lig_carets: LigCaretTable,
    /// The mark glyph sets (GDEF 1.2), which lookups name by index in
    /// their `markFilteringSet`. A set left empty keeps its place, so the
    /// indices of the others do not change.
    pub mark_glyph_sets: Vec<Coverage>,
}
/// GDEF version 1.2, the first with a `markGlyphSetsDefOffset`.
const GDEF_VERSION_1_2: u32 = 0x0001_0002;
/// Reads the MarkGlyphSets table at `offset`: format 1, a count, and one
/// 32-bit coverage offset per set from the table's start. A set whose
/// coverage cannot be read is empty; a table that cannot be read has none.
fn read_mark_glyph_sets(data: &[u8], offset: usize, budget: &mut OtlReadBudget) -> Vec<Coverage> {
    let header = FontReader::new(data).at(offset).and_then(|mut r| {
        let format = r.u16()?;
        let count = r.u16()?;
        r.require_room(usize::from(count), 4)?;
        let mut offsets = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            offsets.push(r.u32()?);
        }
        Ok((format, offsets))
    });
    let Ok((1, offsets)) = header else {
        return Vec::new();
    };
    offsets
        .into_iter()
        .map(
            |rel| match u32::try_from(offset).ok().and_then(|o| o.checked_add(rel)) {
                Some(at) => read_coverage(data, at, budget),
                None => Coverage::new(),
            },
        )
        .collect()
}
fn read_caret_value(data: &[u8], offset: usize) -> CaretValue {
    let mut v: CaretValue = CaretValue {
        format: 0,
        coordiante: 0.,
        point_index: 0xffffu16 as i16,
    };
    if let Ok(bytes) = FontReader::new(data)
        .at(offset)
        .and_then(|mut r| r.bytes(4))
    {
        let format = u16::from_be_bytes([bytes[0], bytes[1]]);
        let value = u16::from_be_bytes([bytes[2], bytes[3]]);
        v.format = format as i8;
        if format == 2 {
            v.point_index = value as i16;
        } else {
            // Formats 1 and 3 hold a signed (int16) coordinate.
            v.coordiante = Pos::from(value as i16);
        }
    }
    v
}
fn read_lig_caret_record(data: &[u8], offset: usize) -> CaretValueRecord {
    let mut g: CaretValueRecord = CaretValueRecord {
        glyph: Handle::new(HandleState::Empty, 0, Vec::new()),
        carets: Vec::new(),
    };
    let Ok(mut r) = FontReader::new(data).at(offset) else {
        return g;
    };
    let Ok(caret_count) = r.u16() else { return g };
    if r.require_room(caret_count as usize, 2).is_err() {
        return g;
    }
    for _ in 0..caret_count {
        let caret_rel = r.u16().expect("require_room above covered this read");
        g.carets
            .push(read_caret_value(data, offset + caret_rel as usize));
    }
    g
}
/// The LigCaretList (`CoverageOffset`/`LigGlyphCount`/`LigGlyphOffset[]`).
/// Any of its three checks failing fails the whole GDEF table (`None`);
/// `lig_caret_offset == 0` means there is no list, `Some(Vec::new())`.
fn read_lig_carets(
    data: &[u8],
    lig_caret_offset: usize,
    budget: &mut OtlReadBudget,
) -> Option<LigCaretTable> {
    if lig_caret_offset == 0 {
        return Some(Vec::new());
    }
    if data.len() < lig_caret_offset + 4 {
        return None;
    }
    let coverage_rel = FontReader::new(data)
        .at(lig_caret_offset)
        .ok()?
        .u16()
        .ok()? as usize;
    let cov: Coverage = read_coverage(data, (lig_caret_offset + coverage_rel) as u32, budget);
    let lig_glyph_count = FontReader::new(data)
        .at(lig_caret_offset + 2)
        .ok()?
        .u16()
        .ok()?;
    if cov.len() != lig_glyph_count as usize {
        return None;
    }
    if data.len() < lig_caret_offset + 4 + cov.len() * 2 {
        return None;
    }
    let mut off_reader = FontReader::new(data).at(lig_caret_offset + 4).ok()?;
    let mut result = Vec::with_capacity(cov.len());
    for glyph in &cov {
        let lig_glyph_rel = off_reader.u16().ok()?;
        let mut v = read_lig_caret_record(data, lig_caret_offset + lig_glyph_rel as usize);
        v.glyph = glyph.clone();
        result.push(v);
    }
    Some(result)
}
/// Reads the `GDEF` table from `packet`, or `None` if it is missing or
/// malformed.
pub fn read_gdef(packet: &Packet) -> Option<Box<GdefTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_GDEF)?;
    let data: &[u8] = &table.data;
    if data.len() < 12 {
        return None;
    }
    // One budget for this whole table's `read_coverage` calls (reached
    // below via `LigCaretList`'s own coverage table) -- see `OtlReadBudget`.
    let mut budget = OtlReadBudget::for_table("GDEF");
    let classdef_offset = FontReader::new(data).at(4).ok()?.u16().ok()?;
    // is purely a narrow bridge.
    let glyph_class_def = if classdef_offset != 0 {
        Some(Box::new(read_class_def(data, u32::from(classdef_offset))))
    } else {
        None
    };

    let lig_caret_offset = FontReader::new(data).at(8).ok()?.u16().ok()? as usize;
    let lig_carets = read_lig_carets(data, lig_caret_offset, &mut budget)?;

    let mark_attach_def_offset = FontReader::new(data).at(10).ok()?.u16().ok()?;
    let mark_attach_class_def = if mark_attach_def_offset != 0 {
        Some(Box::new(read_class_def(
            data,
            u32::from(mark_attach_def_offset),
        )))
    } else {
        None
    };

    let version = FontReader::new(data).at(0).ok()?.u32().ok()?;
    let mark_glyph_sets_offset = if version >= GDEF_VERSION_1_2 {
        FontReader::new(data)
            .at(12)
            .and_then(|mut r| r.u16())
            .unwrap_or(0)
    } else {
        0
    };
    let mark_glyph_sets = if mark_glyph_sets_offset != 0 {
        read_mark_glyph_sets(data, usize::from(mark_glyph_sets_offset), &mut budget)
    } else {
        Vec::new()
    };

    Some(Box::new(GdefTable {
        glyph_class_def,
        mark_attach_class_def,
        lig_carets,
        mark_glyph_sets,
    }))
}
fn dump_gdef_lig_carets(gdef: &GdefTable) -> BuiltValue {
    let lig_carets = &gdef.lig_carets;
    let mut _carets = BuiltValue::new_object(lig_carets.len());
    for record in lig_carets {
        let mut _record = BuiltValue::new_array(record.carets.len());
        for caret in &record.carets {
            let mut _cv = BuiltValue::new_object(1);
            if caret.format == 2 {
                _cv.push_field(b"atPoint", BuiltValue::Int(i64::from(caret.point_index)));
            } else {
                _cv.push_field(b"at", BuiltValue::Int(caret.coordiante as i64));
            }
            _record.push_item(_cv);
        }
        _carets.push_field_bytes_key(&record.glyph.name, _record.preserialize());
    }
    _carets
}
/// Writes `gdef` into `root` as the JSON `GDEF` object (nothing if `None`).
pub fn dump_gdef(gdef: Option<&GdefTable>, root: &mut BuiltValue) {
    let Some(gdef) = gdef else {
        return;
    };
    let stage = crate::logger::stage("GDEF");
    let mut _gdef = BuiltValue::new_object(4);
    if let Some(cd) = gdef.glyph_class_def.as_deref() {
        _gdef.push_field(b"glyphClassDef", dump_class_def(cd));
    }
    if let Some(cd) = gdef.mark_attach_class_def.as_deref() {
        _gdef.push_field(b"markAttachClassDef", dump_class_def(cd));
    }
    if !gdef.lig_carets.is_empty() {
        _gdef.push_field(b"ligCarets", dump_gdef_lig_carets(gdef));
    }
    if !gdef.mark_glyph_sets.is_empty() {
        let mut sets = BuiltValue::new_array(gdef.mark_glyph_sets.len());
        for set in &gdef.mark_glyph_sets {
            sets.push_item(dump_coverage(set));
        }
        _gdef.push_field(b"markGlyphSets", sets);
    }
    root.push_field(b"GDEF", _gdef);
    stage.finish();
}
fn lig_caret_from_json(carets: Option<&ParsedValue>, lc: &mut LigCaretTable) {
    let Some(fields) = carets.and_then(ParsedValue::as_object) else {
        return;
    };
    for (key, a) in fields {
        let Some(items) = a.as_array() else {
            continue;
        };
        let mut v: CaretValueRecord = CaretValueRecord {
            glyph: Handle::new(HandleState::Empty, 0, Vec::new()),
            carets: Vec::new(),
        };
        v.glyph = handle_from_name(Some(key[..key.len() - 1].to_vec()));
        for _caret in items {
            let mut caret: CaretValue = CaretValue {
                format: 1,
                coordiante: Pos::from(0_i32),
                point_index: 0xffff_i32 as i16,
            };
            if _caret.as_object().is_some() {
                if _caret.get_typed(b"atPoint", JsonType::Integer).is_some() {
                    caret.format = 2;
                    caret.point_index = _caret.get_int(b"atPoint") as i16;
                } else {
                    caret.coordiante = _caret.get_num(b"at") as Pos;
                }
            }
            v.carets.push(caret);
        }
        lc.push(v);
    }
}
/// Reads a `GDEF` table from the JSON `root`, or `None` if it has none.
pub fn parse_gdef(root: &ParsedValue) -> Option<Box<GdefTable>> {
    let table = root.get_typed(b"GDEF", JsonType::Object)?;
    let stage = crate::logger::stage("GDEF");
    let mut gdef: Box<GdefTable> = Box::new(GdefTable {
        glyph_class_def: None,
        mark_attach_class_def: None,
        lig_carets: Vec::new(),
        mark_glyph_sets: Vec::new(),
    });
    // is purely a narrow bridge.
    gdef.glyph_class_def = parse_class_def(table.get(b"glyphClassDef")).map(Box::new);
    gdef.mark_attach_class_def = parse_class_def(table.get(b"markAttachClassDef")).map(Box::new);
    lig_caret_from_json(table.get(b"ligCarets"), &mut gdef.lig_carets);
    if let Some(sets) = table
        .get_typed(b"markGlyphSets", JsonType::Array)
        .and_then(ParsedValue::as_array)
    {
        // Every item keeps its index, as lookups refer to sets by it.
        gdef.mark_glyph_sets = sets.iter().map(|set| parse_coverage(Some(set))).collect();
    }
    stage.finish();
    Some(gdef)
}
fn write_lig_caret_rec(cr: &CaretValueRecord) -> BkBlock {
    let carets = &cr.carets;
    let mut bcr: BkBlock = bk_new_block(vec![bk_int(BkCellType::B16, (carets.len()) as u32)]);
    for caret in carets {
        bk_push(
            &mut bcr,
            vec![bk_ptr(
                BkCellType::P16,
                Some(bk_new_block(vec![
                    bk_int(BkCellType::B16, i32::from(caret.format) as u32),
                    bk_int(
                        BkCellType::B16,
                        (if caret.format == 2 {
                            i32::from(caret.point_index)
                        } else {
                            i32::from(caret.coordiante as i16)
                        }) as u32,
                    ),
                ])),
            )],
        );
    }
    bcr
}
fn write_lig_carets(records: &LigCaretTable) -> BkBlock {
    let mut cov: Coverage = Vec::new();
    for record in records {
        push_to_coverage(&mut cov, record.glyph.clone());
    }
    let mut lct: BkBlock = bk_new_block(vec![
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(&cov))),
        ),
        bk_int(BkCellType::B16, (records.len()) as u32),
    ]);
    for record in records {
        bk_push(
            &mut lct,
            vec![bk_ptr(BkCellType::P16, Some(write_lig_caret_rec(record)))],
        );
    }
    lct
}
/// Builds the binary `GDEF` table, or `None` if `gdef` is `None`.
pub fn build_gdef(gdef: Option<&GdefTable>) -> Option<Buffer> {
    let gdef = gdef?;
    let mut b_glyph_class_def: Option<BkBlock> = None;
    let b_attach_list: Option<BkBlock> = None;
    let mut b_lig_caret_list: Option<BkBlock> = None;
    let mut b_mark_attach_class_def: Option<BkBlock> = None;
    if let Some(cd) = gdef.glyph_class_def.as_deref() {
        b_glyph_class_def = bk_new_block_from_buffer(Some(build_class_def(cd)));
    }
    if !gdef.lig_carets.is_empty() {
        b_lig_caret_list = Some(write_lig_carets(&gdef.lig_carets));
    }
    if let Some(cd) = gdef.mark_attach_class_def.as_deref() {
        b_mark_attach_class_def = bk_new_block_from_buffer(Some(build_class_def(cd)));
    }
    // Version 1.0 unless there are mark glyph sets, which need 1.2.
    let has_mark_glyph_sets = !gdef.mark_glyph_sets.is_empty();
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(
            BkCellType::B32,
            if has_mark_glyph_sets {
                GDEF_VERSION_1_2
            } else {
                0x10000
            },
        ),
        bk_ptr(BkCellType::P16, b_glyph_class_def),
        bk_ptr(BkCellType::P16, b_attach_list),
        bk_ptr(BkCellType::P16, b_lig_caret_list),
        bk_ptr(BkCellType::P16, b_mark_attach_class_def),
    ]);
    if has_mark_glyph_sets {
        let mut sets: BkBlock = bk_new_block(vec![
            bk_int(BkCellType::B16, 1),
            bk_int(BkCellType::B16, gdef.mark_glyph_sets.len() as u32),
        ]);
        for set in &gdef.mark_glyph_sets {
            bk_push(
                &mut sets,
                vec![bk_ptr(
                    BkCellType::P32,
                    bk_new_block_from_buffer(Some(build_coverage(set))),
                )],
            );
        }
        bk_push(&mut root, vec![bk_ptr(BkCellType::P16, Some(sets))]);
    }
    Some(bk_build_block(root))
}

#[cfg(test)]
mod read_gdef_tests {
    use super::*;
    use crate::font::sfnt::PacketPiece;

    fn packet_with_gdef(data: Vec<u8>) -> Packet {
        Packet {
            sfnt_version: 0,
            num_tables: 1,
            search_range: 0,
            entry_selector: 0,
            range_shift: 0,
            pieces: vec![PacketPiece {
                tag: crate::tag::TAG_GDEF,
                check_sum: 0,
                offset: 0,
                length: data.len() as u32,
                data,
            }],
        }
    }

    // header(12) + LigCaretList(4 + one 2-byte LigGlyphOffset) + one-glyph
    // Coverage(6) + one LigGlyphTable(4) + one CaretValue(4)
    fn well_formed_gdef_table() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&1u16.to_be_bytes()); // majorVersion
        b.extend_from_slice(&0u16.to_be_bytes()); // minorVersion
        b.extend_from_slice(&0u16.to_be_bytes()); // GlyphClassDefOffset
        b.extend_from_slice(&0u16.to_be_bytes()); // AttachListOffset
        b.extend_from_slice(&12u16.to_be_bytes()); // LigCaretListOffset
        b.extend_from_slice(&0u16.to_be_bytes()); // MarkAttachClassDefOffset
        // LigCaretList @12
        b.extend_from_slice(&6u16.to_be_bytes()); // CoverageOffset (rel to 12)
        b.extend_from_slice(&1u16.to_be_bytes()); // LigGlyphCount
        b.extend_from_slice(&12u16.to_be_bytes()); // LigGlyphOffset[0] (rel to 12)
        // Coverage @18
        b.extend_from_slice(&1u16.to_be_bytes()); // format
        b.extend_from_slice(&1u16.to_be_bytes()); // glyphCount
        b.extend_from_slice(&7u16.to_be_bytes()); // glyph[0]
        // LigGlyphTable @24
        b.extend_from_slice(&1u16.to_be_bytes()); // CaretCount
        b.extend_from_slice(&4u16.to_be_bytes()); // CaretValueOffset[0] (rel to 24)
        // CaretValue (format 1) @28
        b.extend_from_slice(&1u16.to_be_bytes()); // CaretValueFormat
        b.extend_from_slice(&100i16.to_be_bytes()); // Coordinate
        b
    }

    #[test]
    fn well_formed_table_reads_the_lig_caret() {
        let packet = packet_with_gdef(well_formed_gdef_table());
        let gdef = read_gdef(&packet).unwrap();
        assert_eq!(gdef.lig_carets.len(), 1);
        assert_eq!(gdef.lig_carets[0].glyph.index, 7);
        assert_eq!(gdef.lig_carets[0].carets.len(), 1);
        assert_eq!(gdef.lig_carets[0].carets[0].format, 1);
        assert_eq!(gdef.lig_carets[0].carets[0].coordiante, 100.0);
    }

    // A CaretValue coordinate is an int16. It was read unsigned, so -50
    // was dumped as 65486 and written back as 32767.
    #[test]
    fn a_negative_caret_coordinate_is_read_as_negative() {
        let mut data = well_formed_gdef_table();
        data[30..32].copy_from_slice(&(-50i16).to_be_bytes());
        let gdef = read_gdef(&packet_with_gdef(data)).unwrap();
        assert_eq!(gdef.lig_carets[0].carets[0].coordiante, -50.0);
    }

    #[test]
    fn truncated_header_is_rejected() {
        let mut data = well_formed_gdef_table();
        data.truncate(10);
        let packet = packet_with_gdef(data);
        assert!(read_gdef(&packet).is_none());
    }

    #[test]
    fn zero_lig_caret_offset_leaves_lig_carets_empty() {
        let mut data = well_formed_gdef_table();
        data[8..10].copy_from_slice(&0u16.to_be_bytes()); // LigCaretListOffset = 0
        let packet = packet_with_gdef(data);
        let gdef = read_gdef(&packet).unwrap();
        assert!(gdef.lig_carets.is_empty());
    }

    #[test]
    fn lig_glyph_count_mismatched_with_coverage_aborts_the_whole_table() {
        // LigGlyphCount says 1 but the Coverage table only has 1 glyph too
        // in the fixture -- flip it to 2 so they disagree. The original
        // treated this as fatal for the entire GDEF table, not just the
        // LigCaretList, so `read_gdef` must return `None` even
        // though `GlyphClassDefOffset`/`MarkAttachClassDefOffset` are both
        // absent (0) and would otherwise be trivially fine.
        let mut data = well_formed_gdef_table();
        data[14..16].copy_from_slice(&2u16.to_be_bytes()); // LigGlyphCount = 2
        let packet = packet_with_gdef(data);
        assert!(read_gdef(&packet).is_none());
    }

    #[test]
    fn lig_caret_list_offset_past_the_table_end_aborts_the_whole_table() {
        let mut data = well_formed_gdef_table();
        data[8..10].copy_from_slice(&1000u16.to_be_bytes()); // LigCaretListOffset
        let packet = packet_with_gdef(data);
        assert!(read_gdef(&packet).is_none());
    }
}

#[cfg(test)]
mod mark_glyph_sets_tests {
    use super::*;
    use crate::font::sfnt::PacketPiece;

    fn packet(data: Vec<u8>) -> Packet {
        Packet {
            sfnt_version: 0,
            num_tables: 1,
            search_range: 0,
            entry_selector: 0,
            range_shift: 0,
            pieces: vec![PacketPiece {
                tag: crate::tag::TAG_GDEF,
                check_sum: 0,
                offset: 0,
                length: data.len() as u32,
                data,
            }],
        }
    }

    fn glyphs(set: &Coverage) -> Vec<u16> {
        set.iter().map(|g| g.index).collect()
    }

    /// GDEF 1.2 with two mark glyph sets, {3, 5} and {9}.
    fn gdef_1_2() -> Vec<u8> {
        let mut b = Vec::new();
        let put = |b: &mut Vec<u8>, v: u16| b.extend_from_slice(&v.to_be_bytes());
        put(&mut b, 1);
        put(&mut b, 2); // version 1.2
        for _ in 0..4 {
            put(&mut b, 0); // no class defs, attach list or carets
        }
        put(&mut b, 14); // markGlyphSetsDefOffset
        // MarkGlyphSets @14: format, count, two Offset32 from @14.
        put(&mut b, 1);
        put(&mut b, 2);
        b.extend_from_slice(&12u32.to_be_bytes());
        b.extend_from_slice(&20u32.to_be_bytes());
        // Coverage @26: {3, 5}; @34: {9}.
        for v in [1, 2, 3, 5, 1, 1, 9] {
            put(&mut b, v);
        }
        b
    }

    #[test]
    fn mark_glyph_sets_are_read_from_gdef_1_2() {
        let gdef = read_gdef(&packet(gdef_1_2())).unwrap();
        let sets: Vec<Vec<u16>> = gdef.mark_glyph_sets.iter().map(glyphs).collect();
        assert_eq!(sets, [vec![3, 5], vec![9]]);
    }

    #[test]
    fn gdef_1_0_has_no_mark_glyph_sets() {
        // The same bytes labelled 1.0: offset 12 is not a field there.
        let mut data = gdef_1_2();
        data[2..4].copy_from_slice(&0u16.to_be_bytes());
        assert!(read_gdef(&packet(data)).unwrap().mark_glyph_sets.is_empty());
    }

    #[test]
    fn mark_glyph_sets_are_written_back_as_gdef_1_2() {
        let gdef = read_gdef(&packet(gdef_1_2())).unwrap();
        let built = build_gdef(Some(&gdef)).unwrap();
        let bytes = built.data;
        assert_eq!(&bytes[0..4], &0x0001_0002u32.to_be_bytes());
        let again = read_gdef(&packet(bytes)).unwrap();
        let sets: Vec<Vec<u16>> = again.mark_glyph_sets.iter().map(glyphs).collect();
        assert_eq!(sets, [vec![3, 5], vec![9]]);
    }

    #[test]
    fn a_gdef_without_mark_glyph_sets_is_still_written_as_1_0() {
        let mut gdef = read_gdef(&packet(gdef_1_2())).unwrap();
        gdef.mark_glyph_sets.clear();
        let built = build_gdef(Some(&gdef)).unwrap();
        assert_eq!(built.data[0..4], 0x0001_0000u32.to_be_bytes());
        assert_eq!(built.data.len(), 12);
    }
}
