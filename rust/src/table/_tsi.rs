#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{strcmp};
use crate::support::parsed_json::{ParsedValue, json_obj_get_type, json_obj_key_at, json_obj_key_bytes_at, json_obj_len, json_obj_val_at, json_str_len, json_str_ptr, json_type_of};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dup, otfcc_handle_empty, otfcc_handle_init, Handle, GlyphHandle, HandleState};
use crate::support::font_reader::{FontReader, ReadError};
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};
use crate::vendor::json::{JsonType};
use crate::font::caryll_sfnt::{Packet};
use crate::support::buffer::{bufnew, bufnwrite8, bufwrite16b, bufwrite32b};
use crate::support::built_json::{BuiltValue, json_object_new, json_object_push, json_object_push_bytes_key, json_string_new_length};

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
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TsiBuildTarget {
    pub index_part: *mut Buffer,
    pub text_part: *mut Buffer,
}
// Stage 6-4 "Box化": `Font.tsi_01`/`Font.tsi_23` become `Option<Vec<TsiEntry>>`
// (not `Option<Box<Vec<...>>>` -- `Vec` already owns its own heap buffer).
// `.glyph` (a `Handle`) and `.content` (a `Vec<u8>`) both have real drop
// glue on their own, so a `TsiEntry` (and therefore a `TsiTable`) tears
// itself down correctly with no manual per-element walk needed.
#[inline]
unsafe fn is_valid_gid(mut gid: u16, mut tag_index: u32) -> bool {
    if tag_index == crate::tag::TAG_TSI0 {
        return gid as ::core::ffi::c_int != 0xfffe as ::core::ffi::c_int
            && gid as ::core::ffi::c_int != 0xfffc as ::core::ffi::c_int;
    } else {
        return (gid as ::core::ffi::c_int) < 0xfffa as ::core::ffi::c_int;
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
    mut _options: *const Options,
    mut tag_index: u32,
    mut tag_text: u32,
) -> Option<TsiTable> {
    let index_part = packet.pieces.iter().find(|p| p.tag == tag_index)?;
    let text_part = packet.pieces.iter().find(|p| p.tag == tag_text)?;
    let text_len = text_part.data.len() as u32;
    let mut tsi: TsiTable = Vec::new();
    let mut j: u32 = 0 as u32;
    while let Ok(entry) = read_tsi_index_entry(&index_part.data, j) {
        if is_valid_gid(entry.gid, tag_index) && entry.text_offset < text_len && entry.text_length != 0 {
            let mut predicted_text_length: u32 = text_len.wrapping_sub(entry.text_offset);
            let mut k: u32 = j.wrapping_add(1 as u32);
            while let Ok(entry_k) = read_tsi_index_entry(&index_part.data, k) {
                if entry_k.gid as ::core::ffi::c_int != 0xfffe as ::core::ffi::c_int
                    && entry_k.text_offset < text_len
                    && entry_k.text_offset > entry.text_offset
                {
                    predicted_text_length = entry_k.text_offset.wrapping_sub(entry.text_offset);
                    break;
                } else {
                    k = k.wrapping_add(1);
                }
            }
            let text_length = if entry.text_length >= 0x8000 as u32 {
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
            match entry.gid as ::core::ffi::c_int {
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
                    tsi_entry.glyph = handle_from_index(
                        entry.gid as GlyphId,
                    ) as GlyphHandle;
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
    mut root: *mut BuiltValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) {
    let tsi = match tsi {
        Some(t) => t,
        None => return,
    };
    logger_start_sds(
        (*options).logger,
        crate::bytesbuild!(tag),
    );
    let entries: &Vec<TsiEntry> = tsi;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _tsi: *mut BuiltValue = json_object_new(2 as usize);
        let mut _glyphs: *mut BuiltValue = json_object_new(entries.len());
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < entries.len() {
            let entry: *const TsiEntry = &entries[__caryll_index];
            while keep != 0 {
                if !((*entry).type_0 as ::core::ffi::c_uint
                    != TsiEntryType::Glyph as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    json_object_push_bytes_key(
                        _glyphs,
                        &(*entry).glyph.name,
                        json_string_new_length(
                            (*entry).content.len() as ::core::ffi::c_uint,
                            (*entry).content.as_ptr() as *const ::core::ffi::c_char,
                        ),
                    );
                }
                keep = (keep == 0) as ::core::ffi::c_int as usize;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        let mut _extra: *mut BuiltValue = json_object_new(entries.len());
        let mut __caryll_index_0: usize = 0 as usize;
        let mut keep_0: usize = 1 as usize;
        while keep_0 != 0 && __caryll_index_0 < entries.len() {
            let entry_0: *const TsiEntry = &entries[__caryll_index_0];
            while keep_0 != 0 {
                if !((*entry_0).type_0 as ::core::ffi::c_uint
                    == TsiEntryType::Glyph as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    let mut extra_key: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    match (*entry_0).type_0 as ::core::ffi::c_uint {
                        3 => {
                            extra_key = b"cvt\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        1 => {
                            extra_key = b"fpgm\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        2 => {
                            extra_key = b"prep\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        _ => {
                            extra_key = b"reserved\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                    }
                    json_object_push(
                        _extra,
                        extra_key,
                        json_string_new_length(
                            (*entry_0).content.len() as ::core::ffi::c_uint,
                            (*entry_0).content.as_ptr() as *const ::core::ffi::c_char,
                        ),
                    );
                }
                keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
            }
            keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
            __caryll_index_0 = __caryll_index_0.wrapping_add(1);
        }
        json_object_push(
            _tsi,
            b"glyphs\0" as *const u8 as *const ::core::ffi::c_char,
            _glyphs,
        );
        json_object_push(
            _tsi,
            b"extra\0" as *const u8 as *const ::core::ffi::c_char,
            _extra,
        );
        json_object_push(root, tag, _tsi);
        ___loggedstep_v = false;
        logger_finish((*options).logger);
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_parse_tsi(
    mut root: *const ParsedValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> Option<TsiTable> {
    let mut _tsi: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    _tsi = json_obj_get_type(root, tag, JsonType::Object);
    if _tsi.is_null() {
        return None;
    }
    let mut tsi: TsiTable = Vec::new();
    logger_start_sds(
        (*options).logger,
        crate::bytesbuild!(tag),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _glyphs: *const ParsedValue = json_obj_get_type(
            _tsi,
            b"glyphs\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Object,
        );
        if !_glyphs.is_null() {
            let mut j: u32 = 0 as u32;
            while j < json_obj_len(_glyphs) {
                let mut _content: *const ParsedValue =
                    json_obj_val_at(_glyphs, j as u32);
                if !(_content.is_null()
                    || json_type_of(_content) != JsonType::String)
                {
                    tsi.push(TsiEntry {
                            type_0: TsiEntryType::Glyph,
                            glyph: handle_from_name(
                                Some(json_obj_key_bytes_at(_glyphs, j as u32)),
                            ) as GlyphHandle,
                            content: ::core::slice::from_raw_parts(
                                json_str_ptr(_content) as *const u8,
                                json_str_len(_content) as usize,
                            )
                            .to_vec(),
                        });
                }
                j = j.wrapping_add(1);
            }
        }
        let mut _extra: *const ParsedValue = json_obj_get_type(
            _tsi,
            b"extra\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Object,
        );
        if !_extra.is_null() {
            let mut j_0: u32 = 0 as u32;
            while j_0 < json_obj_len(_extra) {
                let mut _key: *mut ::core::ffi::c_char =
                    json_obj_key_at(_extra, j_0 as u32);
                let mut _content_0: *const ParsedValue =
                    json_obj_val_at(_extra, j_0 as u32);
                if !(_content_0.is_null()
                    || json_type_of(_content_0) != JsonType::String)
                {
                    if strcmp(_key, b"cvt\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        tsi.push(TsiEntry {
                                type_0: TsiEntryType::Cvt,
                                glyph: otfcc_handle_empty() as GlyphHandle,
                                content: ::core::slice::from_raw_parts(
                                    json_str_ptr(_content_0) as *const u8,
                                    json_str_len(_content_0) as usize,
                                )
                                .to_vec(),
                            });
                    } else if strcmp(_key, b"fpgm\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        tsi.push(TsiEntry {
                                type_0: TsiEntryType::Fpgm,
                                glyph: otfcc_handle_empty() as GlyphHandle,
                                content: ::core::slice::from_raw_parts(
                                    json_str_ptr(_content_0) as *const u8,
                                    json_str_len(_content_0) as usize,
                                )
                                .to_vec(),
                            });
                    } else if strcmp(_key, b"prep\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        tsi.push(TsiEntry {
                                type_0: TsiEntryType::Prep,
                                glyph: otfcc_handle_empty() as GlyphHandle,
                                content: ::core::slice::from_raw_parts(
                                    json_str_ptr(_content_0) as *const u8,
                                    json_str_len(_content_0) as usize,
                                )
                                .to_vec(),
                            });
                    }
                }
                j_0 = j_0.wrapping_add(1);
            }
        }
        ___loggedstep_v = false;
        logger_finish((*options).logger);
    }
    return Some(tsi);
}
unsafe fn propergid(mut entry: *mut TsiEntry, type_0: TsiEntryType) -> GlyphId {
    match type_0 as ::core::ffi::c_uint {
        3 => return 0xfffb as GlyphId,
        1 => return 0xfffd as GlyphId,
        2 => return 0xfffa as GlyphId,
        4 => return 0xfffc as GlyphId,
        0 => return (*entry).glyph.index,
        _ => {}
    }
    panic!("Reached end of non-void function without returning");
}
unsafe fn push_tsi_entries(
    mut target: *mut TsiBuildTarget,
    mut tsi: *const TsiTable,
    type_0: TsiEntryType,
    min_n: GlyphId,
) {
    let entries: &Vec<TsiEntry> = &*tsi;
    let mut items_pushed: GlyphId = 0 as GlyphId;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < entries.len() {
        let entry: *mut TsiEntry = &entries[__caryll_index] as *const TsiEntry as *mut TsiEntry;
        while keep != 0 {
            if !((*entry).type_0 as ::core::ffi::c_uint != type_0 as ::core::ffi::c_uint) {
                let mut length_sofar: usize = (*(*target).text_part).cursor;
                bufnwrite8((*target).text_part, &(*entry).content);
                let mut length_after: usize = (*(*target).text_part).cursor;
                bufwrite16b((*target).index_part, propergid(entry, type_0) as u16);
                if length_after.wrapping_sub(length_sofar) < 0x8000 as usize {
                    bufwrite16b(
                        (*target).index_part,
                        length_after.wrapping_sub(length_sofar) as u16,
                    );
                } else {
                    bufwrite16b((*target).index_part, 0x8000 as u16);
                }
                bufwrite32b((*target).index_part, length_sofar as u32);
                items_pushed =
                    (items_pushed as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    while (items_pushed as ::core::ffi::c_int) < min_n as ::core::ffi::c_int {
        bufwrite16b(
            (*target).index_part,
            propergid(::core::ptr::null_mut::<TsiEntry>(), type_0) as u16,
        );
        bufwrite16b((*target).index_part, 0 as u16);
        bufwrite32b(
            (*target).index_part,
            (*(*target).text_part).cursor as u32,
        );
        items_pushed = (items_pushed as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_tsi(
    tsi: Option<&TsiTable>,
    mut _options: *const Options,
) -> TsiBuildTarget {
    let tsi: *const TsiTable = tsi.map_or(::core::ptr::null(), |t| t as *const TsiTable);
    let mut target: TsiBuildTarget = TsiBuildTarget {
        index_part: ::core::ptr::null_mut::<Buffer>(),
        text_part: ::core::ptr::null_mut::<Buffer>(),
    };
    if tsi.is_null() {
        target.text_part = ::core::ptr::null_mut::<Buffer>();
        target.index_part = ::core::ptr::null_mut::<Buffer>();
    } else {
        target.text_part = bufnew();
        target.index_part = bufnew();
        push_tsi_entries(&raw mut target, tsi, TsiEntryType::Glyph, 0 as GlyphId);
        bufwrite16b(target.index_part, 0xfffe as u16);
        bufwrite16b(target.index_part, 0 as u16);
        bufwrite32b(target.index_part, 0xabfc1f34 as u32);
        push_tsi_entries(&raw mut target, tsi, TsiEntryType::Prep, 1 as GlyphId);
        push_tsi_entries(&raw mut target, tsi, TsiEntryType::Cvt, 1 as GlyphId);
        push_tsi_entries(&raw mut target, tsi, TsiEntryType::ReservedFffc, 1 as GlyphId);
        push_tsi_entries(&raw mut target, tsi, TsiEntryType::Fpgm, 1 as GlyphId);
    }
    return target;
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
        otfcc_read_tsi(&p, ::core::ptr::null(), crate::tag::TAG_TSI0, crate::tag::TAG_TSI1).unwrap()
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
