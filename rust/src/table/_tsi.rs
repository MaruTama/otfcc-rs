#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::font::caryll_sfnt::Packet;
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::handle::{
    GlyphHandle, Handle, HandleState, handle_from_index, handle_from_name, otfcc_handle_dup,
    otfcc_handle_empty, otfcc_handle_init,
};
use crate::support::options::Options;
use crate::support::parsed_json::ParsedValue;
use crate::support::primitives::GlyphId;
use crate::vendor::json::JsonType;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TsiEntryType {
    Glyph = 0,
    Fpgm = 1,
    Prep = 2,
    Cvt = 3,
    ReservedFffc = 4,
}
#[derive(Clone)]
pub struct TsiEntry {
    pub type_0: TsiEntryType,
    pub glyph: GlyphHandle,
    pub content: Vec<u8>,
}
pub type TsiTable = Vec<TsiEntry>;
// `TsiEntry` derives only `Clone`, not `Copy` (it embeds `GlyphHandle`,
// which owns its `sds` name for real -- `Handle`'s `Drop`/`Clone`, Stage
// 6-4's `Handle` pilot -- and `content` is now a real `Vec<u8>`, also not
// `Copy`). `TABLE_I_TSI.copy` (whole-table clone) was dead before this
// conversion and is deleted below, not ported -- the one real duplicate
// this file needs is per-element (`tsi_entry_dup`, used once from
// `consolidate.rs`), not a `Vec::clone()`.
pub(crate) unsafe fn tsi_entry_dup(e: &TsiEntry) -> TsiEntry {
    TsiEntry {
        type_0: e.type_0,
        glyph: otfcc_handle_dup(e.glyph.clone()),
        content: e.content.clone(),
    }
}
pub struct TsiBuildTarget {
    pub index_part: Option<Buffer>,
    pub text_part: Option<Buffer>,
}
// Stage 6-4 "Box化": `Font.tsi_01`/`Font.tsi_23` become `Option<Vec<TsiEntry>>`
// (not `Option<Box<Vec<...>>>` -- `Vec` already owns its own heap buffer).
// `.glyph` (a `Handle`) and `.content` (a `Vec<u8>`) both have real drop
// glue on their own, so a `TsiEntry` (and therefore a `TsiTable`) tears
// itself down correctly with no manual per-element walk needed.
#[inline]
unsafe fn is_valid_gid(gid: u16, tag_index: u32) -> bool {
    if tag_index == crate::tag::TAG_TSI0 {
        return gid as i32 != 0xfffe_i32
            && gid as i32 != 0xfffc_i32;
    } else {
        return (gid as i32) < 0xfffa_i32;
    };
}
// One 8-byte record: gid(u16) + text_length(u16, widened) + text_offset(u32).
// `FontReader::at` + the three field reads only succeed together when the
// full 8 bytes are actually present -- unlike the original's `j * 8 <
// index_part.length` loop guard, which admits a final *partial* record
// whenever `index_part.length` isn't a multiple of 8 (the same off-by-one
// class `table/tsi5.rs::otfcc_read_tsi5` had, fixed two PRs ago).
struct TsiIndexEntry {
    gid: u16,
    text_length: u32,
    text_offset: u32,
}
fn read_tsi_index_entry(index_data: &[u8], idx: u32) -> Result<TsiIndexEntry, ReadError> {
    let mut r = FontReader::new(index_data).at((idx as usize).wrapping_mul(8))?;
    Ok(TsiIndexEntry {
        gid: r.u16()?,
        text_length: r.u16()? as u32,
        text_offset: r.u32()?,
    })
}

#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_read_tsi(
    packet: &Packet,
    tag_index: u32,
    tag_text: u32,
) -> Option<TsiTable> {
    let index_part = packet.pieces.iter().find(|p| p.tag == tag_index)?;
    let text_part = packet.pieces.iter().find(|p| p.tag == tag_text)?;
    let text_len = text_part.data.len() as u32;
    let mut tsi: TsiTable = Vec::new();
    let mut j: u32 = 0_u32;
    while let Ok(entry) = read_tsi_index_entry(&index_part.data, j) {
        if is_valid_gid(entry.gid, tag_index)
            && entry.text_offset < text_len
            && entry.text_length != 0
        {
            let mut predicted_text_length: u32 = text_len.wrapping_sub(entry.text_offset);
            let mut k: u32 = j.wrapping_add(1_u32);
            while let Ok(entry_k) = read_tsi_index_entry(&index_part.data, k) {
                if entry_k.gid as i32 != 0xfffe_i32
                    && entry_k.text_offset < text_len
                    && entry_k.text_offset > entry.text_offset
                {
                    predicted_text_length = entry_k.text_offset.wrapping_sub(entry.text_offset);
                    break;
                } else {
                    k = k.wrapping_add(1);
                }
            }
            let text_length = if entry.text_length >= 0x8000_u32 {
                predicted_text_length
            } else {
                entry.text_length
            };
            // The original read `text_length` bytes from `text_offset`
            // unconditionally, trusting the declared length even when it
            // wasn't the `>= 0x8000` "compute it instead" sentinel --
            // `text_offset < text_len` alone does not imply `text_offset +
            // text_length <= text_len`. `at` + `peek_bytes` check that
            // full span actually fits `text_part` before any bytes are
            // read; a declared length that doesn't fit drops this index
            // entry entirely; matches the "corrupted piece, skip it"
            // pattern the rest of this migration uses.
            let content = match FontReader::new(&text_part.data)
                .at(entry.text_offset as usize)
                .and_then(|r| r.peek_bytes(text_length as usize))
            {
                Ok(bytes) => bytes.to_vec(),
                Err(_) => {
                    j = j.wrapping_add(1);
                    continue;
                }
            };
            let mut tsi_entry: TsiEntry = TsiEntry {
                type_0: TsiEntryType::Glyph,
                glyph: Handle {
                    state: HandleState::Empty,
                    index: 0,
                    name: Vec::new(),
                },
                content,
            };
            match entry.gid as i32 {
                65530 => {
                    tsi_entry.type_0 = TsiEntryType::Prep;
                    otfcc_handle_init(&raw mut tsi_entry.glyph);
                }
                65531 => {
                    tsi_entry.type_0 = TsiEntryType::Cvt;
                    otfcc_handle_init(&raw mut tsi_entry.glyph);
                }
                65533 => {
                    tsi_entry.type_0 = TsiEntryType::Fpgm;
                    otfcc_handle_init(&raw mut tsi_entry.glyph);
                }
                _ => {
                    tsi_entry.type_0 = TsiEntryType::Glyph;
                    tsi_entry.glyph = handle_from_index(entry.gid as GlyphId) as GlyphHandle;
                }
            }
            tsi.push(tsi_entry);
        }
        j = j.wrapping_add(1);
    }
    return Some(tsi);
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_tsi(
    tsi: Option<&TsiTable>,
    root: &mut BuiltValue,
    options: &Options,
    tag: *const ::core::ffi::c_char,
) {
    let tsi = match tsi {
        Some(t) => t,
        None => return,
    };
    logger_start_sds(&mut *options.logger.borrow_mut(), crate::bytesbuild!(tag));
    let entries: &Vec<TsiEntry> = tsi;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _tsi = BuiltValue::new_object(2);
        let mut _glyphs = BuiltValue::new_object(entries.len());
        for entry in entries.iter() {
            if entry.type_0 == TsiEntryType::Glyph {
                _glyphs.push_field_bytes_key(&entry.glyph.name, BuiltValue::Str(entry.content.clone()));
            }
        }
        let mut _extra = BuiltValue::new_object(entries.len());
        for entry in entries.iter() {
            if entry.type_0 != TsiEntryType::Glyph {
                let extra_key: &[u8] = match entry.type_0 as ::core::ffi::c_uint {
                    3 => b"cvt",
                    1 => b"fpgm",
                    2 => b"prep",
                    _ => b"reserved",
                };
                _extra.push_field(extra_key, BuiltValue::Str(entry.content.clone()));
            }
        }
        _tsi.push_field(b"glyphs", _glyphs);
        _tsi.push_field(b"extra", _extra);
        root.push_field(::core::ffi::CStr::from_ptr(tag).to_bytes(), _tsi);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_parse_tsi(
    root: *const ParsedValue,
    options: &Options,
    tag: *const ::core::ffi::c_char,
) -> Option<TsiTable> {
    let tag_key = ::core::ffi::CStr::from_ptr(tag).to_bytes();
    let _tsi = root.as_ref().and_then(|r| r.get_typed(tag_key, JsonType::Object))?;
    let mut tsi: TsiTable = Vec::new();
    logger_start_sds(&mut *options.logger.borrow_mut(), crate::bytesbuild!(tag));
    if let Some(fields) = _tsi
        .get_typed(b"glyphs", JsonType::Object)
        .and_then(ParsedValue::as_object)
    {
        for (key, _content) in fields {
            let Some(bytes) = _content.as_str_bytes() else {
                continue;
            };
            tsi.push(TsiEntry {
                type_0: TsiEntryType::Glyph,
                glyph: handle_from_name(Some(key[..key.len() - 1].to_vec())) as GlyphHandle,
                content: bytes.to_vec(),
            });
        }
    }
    if let Some(fields) = _tsi
        .get_typed(b"extra", JsonType::Object)
        .and_then(ParsedValue::as_object)
    {
        for (key, _content_0) in fields {
            let Some(bytes) = _content_0.as_str_bytes() else {
                continue;
            };
            let type_0 = match &key[..key.len() - 1] {
                b"cvt" => TsiEntryType::Cvt,
                b"fpgm" => TsiEntryType::Fpgm,
                b"prep" => TsiEntryType::Prep,
                _ => continue,
            };
            tsi.push(TsiEntry {
                type_0,
                glyph: otfcc_handle_empty() as GlyphHandle,
                content: bytes.to_vec(),
            });
        }
    }
    logger_finish(&mut *options.logger.borrow_mut());
    Some(tsi)
}
// c2rust residue: the original had this as a numeric `switch` over
// `TsiEntryType as c_uint` with a fallthrough `panic!` for "no case
// matched" -- but `TsiEntryType` is a closed 5-variant enum (`Glyph=0`,
// `Fpgm=1`, `Prep=2`, `Cvt=3`, `ReservedFffc=4`) and the switch already
// covered all five, so that arm was unreachable, not a real error path.
// Matching on the enum directly instead of its numeric cast makes that
// exhaustiveness compiler-checked rather than asserted at runtime, and
// the panic falls away with it.
//
// `entry` is only actually dereferenced in the `Glyph` arm --
// `push_tsi_entries` (below) passes a null `entry` from its own
// `min_n`-padding loop, but only ever calls this with `type_0 ==
// TsiEntryType::Glyph` when `min_n` is `0`, which keeps that loop from
// running at all for `Glyph` (see `otfcc_build_tsi`'s call sites), so the
// null never actually reaches this arm.
fn propergid(entry: Option<&TsiEntry>, type_0: TsiEntryType) -> GlyphId {
    match type_0 {
        TsiEntryType::Cvt => 0xfffb as GlyphId,
        TsiEntryType::Fpgm => 0xfffd as GlyphId,
        TsiEntryType::Prep => 0xfffa as GlyphId,
        TsiEntryType::ReservedFffc => 0xfffc as GlyphId,
        TsiEntryType::Glyph => entry.unwrap().glyph.index,
    }
}
fn push_tsi_entries(target: &mut TsiBuildTarget, tsi: &TsiTable, type_0: TsiEntryType, min_n: GlyphId) {
    let mut items_pushed: GlyphId = 0 as GlyphId;
    for entry in tsi.iter() {
        if entry.type_0 != type_0 {
            continue;
        }
        let length_sofar = target.text_part.as_ref().unwrap().pos();
        target.text_part.as_mut().unwrap().write_bytes(&entry.content);
        let length_after = target.text_part.as_ref().unwrap().pos();
        let index_part = target.index_part.as_mut().unwrap();
        index_part.write_u16be(propergid(Some(entry), type_0) as u16);
        if length_after.wrapping_sub(length_sofar) < 0x8000_usize {
            index_part.write_u16be(length_after.wrapping_sub(length_sofar) as u16);
        } else {
            index_part.write_u16be(0x8000_u16);
        }
        index_part.write_u32be(length_sofar as u32);
        items_pushed = (items_pushed as i32 + 1_i32) as GlyphId;
    }
    while (items_pushed as i32) < min_n as i32 {
        let text_pos = target.text_part.as_ref().unwrap().pos();
        let index_part = target.index_part.as_mut().unwrap();
        index_part.write_u16be(propergid(None, type_0) as u16);
        index_part.write_u16be(0_u16);
        index_part.write_u32be(text_pos as u32);
        items_pushed = (items_pushed as i32 + 1_i32) as GlyphId;
    }
}
#[allow(improper_ctypes_definitions)]
pub fn otfcc_build_tsi(tsi: Option<&TsiTable>) -> TsiBuildTarget {
    let Some(tsi) = tsi else {
        return TsiBuildTarget {
            index_part: None,
            text_part: None,
        };
    };
    let mut target = TsiBuildTarget {
        index_part: Some(Buffer::new()),
        text_part: Some(Buffer::new()),
    };
    push_tsi_entries(&mut target, tsi, TsiEntryType::Glyph, 0 as GlyphId);
    let index_part = target.index_part.as_mut().unwrap();
    index_part.write_u16be(0xfffe_u16);
    index_part.write_u16be(0_u16);
    index_part.write_u32be(0xabfc1f34_u32);
    push_tsi_entries(&mut target, tsi, TsiEntryType::Prep, 1 as GlyphId);
    push_tsi_entries(&mut target, tsi, TsiEntryType::Cvt, 1 as GlyphId);
    push_tsi_entries(&mut target, tsi, TsiEntryType::ReservedFffc, 1 as GlyphId);
    push_tsi_entries(&mut target, tsi, TsiEntryType::Fpgm, 1 as GlyphId);
    target
}

#[cfg(test)]
mod otfcc_read_tsi_tests {
    use super::*;
    use crate::font::caryll_sfnt::PacketPiece;

    // No committed payload has a TSI0/TSI1 (or TSI2/TSI3) pair (checked by
    // hand against every tests/payload/*.ttf), so this whole module is the
    // only coverage -- both of the fix and of the original three bugs it
    // replaces.

    fn index_record(gid: u16, text_length: u16, text_offset: u32) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&gid.to_be_bytes());
        b.extend_from_slice(&text_length.to_be_bytes());
        b.extend_from_slice(&text_offset.to_be_bytes());
        b
    }

    fn packet(index_data: Vec<u8>, text_data: Vec<u8>) -> Packet {
        Packet {
            sfnt_version: 0,
            num_tables: 2,
            search_range: 0,
            entry_selector: 0,
            range_shift: 0,
            pieces: vec![
                PacketPiece {
                    tag: crate::tag::TAG_TSI0,
                    check_sum: 0,
                    offset: 0,
                    length: index_data.len() as u32,
                    data: index_data,
                },
                PacketPiece {
                    tag: crate::tag::TAG_TSI1,
                    check_sum: 0,
                    offset: 0,
                    length: text_data.len() as u32,
                    data: text_data,
                },
            ],
        }
    }

    unsafe fn read(index_data: Vec<u8>, text_data: Vec<u8>) -> TsiTable {
        let p = packet(index_data, text_data);
        otfcc_read_tsi(&p, crate::tag::TAG_TSI0, crate::tag::TAG_TSI1).unwrap()
    }

    #[test]
    fn declared_length_resolves_content_directly() {
        let index = index_record(9, 3, 0);
        unsafe {
            let tsi = read(index, b"ABC".to_vec());
            assert_eq!(tsi.len(), 1);
            assert_eq!(tsi[0].type_0, TsiEntryType::Glyph);
            assert_eq!(tsi[0].glyph.index, 9);
            assert_eq!(tsi[0].content, b"ABC");
        }
    }

    #[test]
    fn sentinel_length_is_predicted_from_the_next_entrys_offset() {
        // Entry 0: gid 1, text_length = 0x8000 ("compute it"), offset 0.
        // Entry 1: gid 2, text_length 1, offset 5.
        // Entry 0's predicted length must come from entry 1's offset (5),
        // not from "rest of the buffer" (10).
        let mut index = index_record(1, 0x8000, 0);
        index.extend(index_record(2, 1, 5));
        unsafe {
            let tsi = read(index, b"0123456789".to_vec());
            assert_eq!(tsi.len(), 2);
            assert_eq!(tsi[0].content, b"01234");
            assert_eq!(tsi[1].content, b"5");
        }
    }

    #[test]
    fn sentinel_length_falls_back_to_the_rest_of_the_buffer_with_no_next_entry() {
        let index = index_record(1, 0x8000, 2);
        unsafe {
            let tsi = read(index, b"ABCDEF".to_vec());
            assert_eq!(tsi[0].content, b"CDEF"); // offset 2 to the end
        }
    }

    #[test]
    fn trailing_partial_index_record_is_dropped_not_read_oob() {
        // One full 8-byte record, then 3 stray bytes -- not a full second
        // record. The original `j * 8 < index_part.length` loop guard
        // would have read 5 bytes past the (8+3)-byte index buffer trying
        // to parse that partial record as real.
        let mut index = index_record(9, 2, 0);
        index.extend_from_slice(&[0xAA, 0xBB, 0xCC]);
        unsafe {
            let tsi = read(index, b"AB".to_vec());
            assert_eq!(tsi.len(), 1);
            assert_eq!(tsi[0].content, b"AB");
        }
    }

    #[test]
    fn declared_length_longer_than_the_text_part_is_dropped_not_read_oob() {
        // The actual overread this migration exists to fix: a declared
        // (non-sentinel) text_length that runs past text_part's real
        // length. text_offset(0) < text_len(2) passes the original's only
        // check, but 0 + 100 > 2.
        let index = index_record(9, 100, 0);
        unsafe {
            let tsi = read(index, b"AB".to_vec());
            assert!(tsi.is_empty());
        }
    }

    #[test]
    fn text_offset_past_the_text_part_is_skipped() {
        // Preserved from the original: an out-of-range text_offset was
        // already checked (`text_offset >= text_part.length`).
        let index = index_record(9, 1, 5);
        unsafe {
            let tsi = read(index, b"AB".to_vec());
            assert!(tsi.is_empty());
        }
    }

    #[test]
    fn zero_length_entry_is_skipped() {
        let index = index_record(9, 0, 0);
        unsafe {
            let tsi = read(index, b"AB".to_vec());
            assert!(tsi.is_empty());
        }
    }

    #[test]
    fn special_gids_map_to_their_reserved_entry_types() {
        for (gid, expected) in [
            (65530u16, TsiEntryType::Prep),
            (65531u16, TsiEntryType::Cvt),
            (65533u16, TsiEntryType::Fpgm),
        ] {
            let index = index_record(gid, 1, 0);
            unsafe {
                let tsi = read(index, b"X".to_vec());
                assert_eq!(tsi[0].type_0, expected, "gid {gid}");
            }
        }
    }

    #[test]
    fn empty_index_table_produces_no_entries() {
        unsafe {
            let tsi = read(Vec::new(), Vec::new());
            assert!(tsi.is_empty());
        }
    }
}
