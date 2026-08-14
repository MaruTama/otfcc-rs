#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{strcmp};
use crate::support::parsed_json::{ParsedValue, json_obj_get_type, json_obj_key_at, json_obj_key_bytes_at, json_obj_len, json_obj_val_at, json_str_len, json_str_ptr, json_type_of};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dup, otfcc_handle_empty, otfcc_handle_init, Handle, GlyphHandle, HandleState};
use crate::support::binio::{read_16u, read_32u};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};
use crate::vendor::json::{JsonType};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
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
unsafe extern "C" fn is_valid_gid(mut gid: u16, mut tag_index: u32) -> bool {
    if tag_index == crate::tag::TAG_TSI0 {
        return gid as ::core::ffi::c_int != 0xfffe as ::core::ffi::c_int
            && gid as ::core::ffi::c_int != 0xfffc as ::core::ffi::c_int;
    } else {
        return (gid as ::core::ffi::c_int) < 0xfffa as ::core::ffi::c_int;
    };
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_read_tsi(
    packet: Packet,
    mut _options: *const Options,
    mut tag_index: u32,
    mut tag_text: u32,
) -> Option<TsiTable> {
    let mut text_part: PacketPiece = PacketPiece {
        tag: 0,
        check_sum: 0,
        offset: 0,
        length: 0,
        data: ::core::ptr::null_mut::<u8>(),
    };
    text_part.tag = 0 as u32;
    let mut index_part: PacketPiece = PacketPiece {
        tag: 0,
        check_sum: 0,
        offset: 0,
        length: 0,
        data: ::core::ptr::null_mut::<u8>(),
    };
    index_part.tag = 0 as u32;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table_ix: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table_ix.tag == tag_index {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    index_part = table_ix;
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    let mut __fortable_keep_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound_0 != 0
        && __fortable_keep_0 != 0
        && __fortable_count_0 < packet.num_tables as ::core::ffi::c_int
    {
        let mut table_tx: PacketPiece = *packet.pieces.offset(__fortable_count_0 as isize);
        while __fortable_keep_0 != 0 {
            if table_tx.tag == tag_text {
                let mut __fortable_k2_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2_0 != 0 {
                    text_part = table_tx;
                    __fortable_k2_0 = 0 as ::core::ffi::c_int;
                    __notfound_0 = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep_0 = (__fortable_keep_0 == 0) as ::core::ffi::c_int;
        }
        __fortable_keep_0 = (__fortable_keep_0 == 0) as ::core::ffi::c_int;
        __fortable_count_0 += 1;
    }
    if text_part.tag == 0 || index_part.tag == 0 {
        return None;
    }
    let mut tsi: TsiTable = Vec::new();
    let mut j: u32 = 0 as u32;
    while j.wrapping_mul(8 as u32) < index_part.length {
        let mut gid: u16 = read_16u(
            index_part
                .data
                .offset(j.wrapping_mul(8 as u32) as isize),
        );
        let mut text_length: u32 = read_16u(
            index_part
                .data
                .offset(j.wrapping_mul(8 as u32) as isize)
                .offset(2 as ::core::ffi::c_int as isize),
        ) as u32;
        let mut text_offset: u32 = read_32u(
            index_part
                .data
                .offset(j.wrapping_mul(8 as u32) as isize)
                .offset(4 as ::core::ffi::c_int as isize),
        );
        if !(!is_valid_gid(gid, tag_index) || text_offset >= text_part.length || text_length == 0) {
            let mut predicted_text_length: u32 = text_part.length.wrapping_sub(text_offset);
            let mut k: GlyphId = j.wrapping_add(1 as u32) as GlyphId;
            while ((k as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as u32)
                < index_part.length
            {
                let mut gid_k: u16 = read_16u(
                    index_part
                        .data
                        .offset((k as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize),
                );
                let mut text_offset_k: u32 = read_32u(
                    index_part
                        .data
                        .offset((k as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize),
                );
                if gid_k as ::core::ffi::c_int != 0xfffe as ::core::ffi::c_int
                    && text_offset_k < text_part.length
                    && text_offset_k > text_offset
                {
                    predicted_text_length = text_offset_k.wrapping_sub(text_offset);
                    break;
                } else {
                    k = k.wrapping_add(1);
                }
            }
            if text_length >= 0x8000 as u32 {
                text_length = predicted_text_length;
            }
            let mut entry: TsiEntry = TsiEntry {
                type_0: TsiEntryType::Glyph,
                glyph: Handle {
                    state: HandleState::Empty,
                    index: 0,
                    name: Vec::new(),
                },
                content: Vec::new(),
            };
            match gid as ::core::ffi::c_int {
                65530 => {
                    entry.type_0 = TsiEntryType::Prep;
                    otfcc_handle_init(&raw mut entry.glyph);
                }
                65531 => {
                    entry.type_0 = TsiEntryType::Cvt;
                    otfcc_handle_init(&raw mut entry.glyph);
                }
                65533 => {
                    entry.type_0 = TsiEntryType::Fpgm;
                    otfcc_handle_init(&raw mut entry.glyph);
                }
                _ => {
                    entry.type_0 = TsiEntryType::Glyph;
                    entry.glyph = handle_from_index(
                        gid as GlyphId,
                    ) as GlyphHandle;
                }
            }
            entry.content = ::core::slice::from_raw_parts(
                text_part.data.offset(text_offset as isize),
                text_length as usize,
            )
            .to_vec();
            tsi.push(entry);
        }
        j = j.wrapping_add(1);
    }
    return Some(tsi);
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_dump_tsi(
    tsi: Option<&TsiTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) {
    let tsi = match tsi {
        Some(t) => t,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
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
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_parse_tsi(
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
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
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
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    return Some(tsi);
}
unsafe extern "C" fn propergid(mut entry: *mut TsiEntry, type_0: TsiEntryType) -> GlyphId {
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
unsafe extern "C" fn push_tsi_entries(
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
pub unsafe extern "C" fn otfcc_build_tsi(
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
