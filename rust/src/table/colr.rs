#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{
    GlyphHandle, Handle, HandleState, handle_from_index, handle_from_name, otfcc_handle_dup,
    otfcc_handle_move,
};
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_getint_fallback,
    json_str_bytes, json_type_of,
};

use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::primitives::{ColorId, GlyphId};
use crate::vendor::json::JsonType;

use crate::bk::bkgraph::bk_build_block;
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new,
    json_object_push, json_string_new_from_bytes, preserialize,
};
#[derive(Clone)]
pub struct ColrLayer {
    pub glyph: GlyphHandle,
    pub palette_index: ColorId,
}
pub struct ColrMapping {
    pub glyph: GlyphHandle,
    pub layers: Vec<ColrLayer>,
}
pub type ColrTable = Vec<ColrMapping>;
// Stage 6-4 "Box化": `Font.colr` becomes `Option<Vec<ColrMapping>>` (not
// `Option<Box<Vec<...>>>` -- `Vec` already owns its own heap buffer).
// `ColrMapping`/`ColrLayer` own only a `GlyphHandle`, which already has a
// real `Drop` (Stage 6-4's `Handle` pilot), so a plain `Vec<ColrMapping>`'s
// own `Drop` already frees everything recursively.

// `ColrLayer`/`ColrMapping` embed `GlyphHandle`, which owns its `sds` name
// for real (`Handle`'s `Drop`/`Clone`, Stage 6-4's `Handle` pilot), so a
// bitwise `Clone`/`Copy` of either would still only alias it -- a REAL
// duplicate always goes through an explicit dup/copy function here, never an
// implicit derive, so these two are written the same way, not derived.
pub(crate) fn colr_layer_dup(l: &ColrLayer) -> ColrLayer {
    unsafe {
        ColrLayer {
            glyph: otfcc_handle_dup(l.glyph.clone()),
            palette_index: l.palette_index,
        }
    }
}
fn colr_mapping_dup(m: &ColrMapping) -> ColrMapping {
    ColrMapping {
        glyph: unsafe { otfcc_handle_dup(m.glyph.clone()) },
        layers: m.layers.iter().map(colr_layer_dup).collect(),
    }
}
static BASE_GLYPH_REC_LENGTH: usize = 6_usize;
static LAYER_REC_LENGTH: usize = 4_usize;
/// `offset_base_glyph_record`/`offset_layer_record` are each a raw `u32`
/// read straight from the file (full attacker control); unlike
/// `table/cpal.rs`'s equivalent fields, the original's own guards here
/// already cast to `usize` *before* `wrapping_add`, so on this crate's
/// actual 64-bit CI targets neither guard can wrap the way `cpal.rs`'s
/// 32-bit `u32::wrapping_add` did -- `FontReader`'s `checked_add`/
/// `checked_mul` still replace them, for the same "true on every pointer
/// width, not just the ones this crate happens to test on" reason
/// `require_room` exists at all.
unsafe fn parse_colr(data: &[u8]) -> Result<ColrTable, ReadError> {
    if data.len() < 14 {
        return Err(ReadError { needed: 14, available: data.len() });
    }
    let num_base_glyph_records = FontReader::new(data).at(2)?.u16()?;
    let num_layer_records = FontReader::new(data).at(12)?.u16()?;
    let offset_base_glyph_record = FontReader::new(data).at(4)?.u32()? as usize;
    let offset_layer_record = FontReader::new(data).at(8)?.u32()? as usize;

    let mut br = FontReader::new(data).at(offset_base_glyph_record)?;
    br.require_room(num_base_glyph_records as usize, BASE_GLYPH_REC_LENGTH)?;
    let mut lr = FontReader::new(data).at(offset_layer_record)?;
    lr.require_room(num_layer_records as usize, LAYER_REC_LENGTH)?;

    let mut gids: Vec<GlyphId> = Vec::with_capacity(num_layer_records as usize);
    let mut colors: Vec<ColorId> = Vec::with_capacity(num_layer_records as usize);
    for _ in 0..num_layer_records {
        gids.push(lr.u16()? as GlyphId);
        colors.push(lr.u16()? as ColorId);
    }

    let mut colr: ColrTable = Vec::new();
    for _ in 0..num_base_glyph_records {
        let gid = br.u16()?;
        let first_layer_index = br.u16()?;
        let num_layers = br.u16()?;
        let mut mapping: ColrMapping = ColrMapping {
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: Vec::new(),
            },
            layers: Vec::new(),
        };
        let mut base_glyph: GlyphHandle = handle_from_index(gid as GlyphId) as GlyphHandle;
        otfcc_handle_move(&raw mut mapping.glyph, &raw mut base_glyph);
        for k in 0..num_layers {
            let idx = k as usize + first_layer_index as usize;
            if idx < num_layer_records as usize {
                mapping.layers.push(ColrLayer {
                    glyph: handle_from_index(gids[idx]) as GlyphHandle,
                    palette_index: colors[idx],
                });
            }
        }
        colr.push(mapping);
    }
    Ok(colr)
}
#[allow(improper_ctypes_definitions)]
pub fn otfcc_read_colr(packet: &Packet, options: &Options) -> Option<ColrTable> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_COLR)?;
    match unsafe { parse_colr(&table.data) } {
        Ok(colr) => Some(colr),
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"Table 'COLR' corrupted.\n"),
            );
            None
        }
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_colr(
    colr: Option<&ColrTable>,
    root: *mut BuiltValue,
    options: &Options,
) {
    let colr = match colr {
        Some(c) => c,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"COLR"),
    );
    let mappings: &Vec<ColrMapping> = colr;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _colr: *mut BuiltValue = json_array_new(mappings.len());
        let mut __caryll_index: usize = 0_usize;
        let mut keep: usize = 1_usize;
        while keep != 0 && __caryll_index < mappings.len() {
            let mapping: &ColrMapping = &mappings[__caryll_index];
            while keep != 0 {
                let mut _map: *mut BuiltValue = json_object_new(2_usize);
                json_object_push(
                    _map,
                    b"from\0" as *const u8 as *const ::core::ffi::c_char,
                    json_string_new_from_bytes(&mapping.glyph.name),
                );
                let mut _layers: *mut BuiltValue = json_array_new(mapping.layers.len());
                let mut __caryll_index_0: usize = 0_usize;
                let mut keep_0: usize = 1_usize;
                while keep_0 != 0 && __caryll_index_0 < mapping.layers.len() {
                    let layer: &ColrLayer = &mapping.layers[__caryll_index_0];
                    while keep_0 != 0 {
                        let mut _layer: *mut BuiltValue = json_object_new(2_usize);
                        json_object_push(
                            _layer,
                            b"layer\0" as *const u8 as *const ::core::ffi::c_char,
                            json_string_new_from_bytes(&layer.glyph.name),
                        );
                        json_object_push(
                            _layer,
                            b"paletteIndex\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new(layer.palette_index as i64),
                        );
                        json_array_push(_layers, _layer);
                        keep_0 = (keep_0 == 0) as i32 as usize;
                    }
                    keep_0 = (keep_0 == 0) as i32 as usize;
                    __caryll_index_0 = __caryll_index_0.wrapping_add(1);
                }
                json_object_push(
                    _map,
                    b"to\0" as *const u8 as *const ::core::ffi::c_char,
                    preserialize(_layers),
                );
                json_array_push(_colr, _map);
                keep = (keep == 0) as i32 as usize;
            }
            keep = (keep == 0) as i32 as usize;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        json_object_push(
            root,
            b"COLR\0" as *const u8 as *const ::core::ffi::c_char,
            _colr,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_parse_colr(
    root: *const ParsedValue,
    options: &Options,
) -> Option<ColrTable> {
    let mut _colr: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    _colr = json_obj_get_type(
        root,
        b"COLR\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _colr.is_null() {
        return None;
    }
    let mut colr: ColrTable = Vec::new();
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"COLR"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: GlyphId = 0 as GlyphId;
        while (j as ::core::ffi::c_uint) < json_arr_len(_colr) {
            let mut _mapping: *const ParsedValue = json_arr_at(_colr, j as u32);
            if !(_mapping.is_null() || json_type_of(_mapping) != JsonType::Object) {
                let mut _baseglyph: *const ParsedValue = json_obj_get_type(
                    _mapping,
                    b"from\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::String,
                );
                let mut _layers: *const ParsedValue = json_obj_get_type(
                    _mapping,
                    b"to\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                );
                if !(_baseglyph.is_null() || _layers.is_null()) {
                    let mut m: ColrMapping = ColrMapping {
                        glyph: Handle {
                            state: HandleState::Empty,
                            index: 0,
                            name: Vec::new(),
                        },
                        layers: Vec::new(),
                    };
                    m.glyph = handle_from_name(Some(json_str_bytes(_baseglyph))) as GlyphHandle;
                    let mut k: GlyphId = 0 as GlyphId;
                    while (k as ::core::ffi::c_uint) < json_arr_len(_layers) {
                        let mut _layer: *const ParsedValue = json_arr_at(_layers, k as u32);
                        if !(_layer.is_null() || json_type_of(_layer) != JsonType::Object) {
                            let mut _layerglyph: *const ParsedValue = json_obj_get_type(
                                _layer,
                                b"layer\0" as *const u8 as *const ::core::ffi::c_char,
                                JsonType::String,
                            );
                            if !_layerglyph.is_null() {
                                m.layers.push(ColrLayer {
                                    glyph: handle_from_name(Some(json_str_bytes(_layerglyph)))
                                        as GlyphHandle,
                                    palette_index: json_obj_getint_fallback(
                                        _layer,
                                        b"paletteIndex\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        0xffff_i32,
                                    ) as ColorId,
                                });
                            }
                        }
                        k = k.wrapping_add(1);
                    }
                    colr.push(m);
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    return Some(colr);
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_colr(_colr: Option<&ColrTable>) -> Option<Buffer> {
    let src = match _colr {
        Some(c) if !c.is_empty() => c,
        _ => return None,
    };
    let mut colr: ColrTable = src.iter().map(colr_mapping_dup).collect();
    colr.sort_by(|a, b| a.glyph.index.cmp(&b.glyph.index));
    let mut current_layer_index: GlyphId = 0 as GlyphId;
    let layer_records: *mut BkBlock = bk_new_block(&[]);
    let base_records: *mut BkBlock = bk_new_block(&[]);
    let mut __caryll_index: usize = 0_usize;
    let mut keep: usize = 1_usize;
    while keep != 0 && __caryll_index < colr.len() {
        let mapping: &ColrMapping = &colr[__caryll_index];
        while keep != 0 {
            bk_push(
                base_records,
                &[
                    bk_int(
                        BkCellType::B16,
                        (mapping.glyph.index as i32) as u32,
                    ),
                    bk_int(
                        BkCellType::B16,
                        (current_layer_index as i32) as u32,
                    ),
                    bk_int(BkCellType::B16, (mapping.layers.len()) as u32),
                ],
            );
            let mut __caryll_index_0: usize = 0_usize;
            let mut keep_0: usize = 1_usize;
            while keep_0 != 0 && __caryll_index_0 < mapping.layers.len() {
                let layer: &ColrLayer = &mapping.layers[__caryll_index_0];
                while keep_0 != 0 {
                    bk_push(
                        layer_records,
                        &[
                            bk_int(
                                BkCellType::B16,
                                (layer.glyph.index as i32) as u32,
                            ),
                            bk_int(
                                BkCellType::B16,
                                (layer.palette_index as i32) as u32,
                            ),
                        ],
                    );
                    current_layer_index = (current_layer_index as i32
                        + 1_i32)
                        as GlyphId;
                    keep_0 = (keep_0 == 0) as i32 as usize;
                }
                keep_0 = (keep_0 == 0) as i32 as usize;
                __caryll_index_0 = __caryll_index_0.wrapping_add(1);
            }
            keep = (keep == 0) as i32 as usize;
        }
        keep = (keep == 0) as i32 as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B16, 0_u32),
        bk_int(BkCellType::B16, (colr.len()) as u32),
        bk_ptr(BkCellType::P32, base_records),
        bk_ptr(BkCellType::P32, layer_records),
        bk_int(
            BkCellType::B16,
            (current_layer_index as i32) as u32,
        ),
    ]);
    // `colr` drops naturally at the end of this scope -- no explicit
    // dispose call needed (`ColrMapping`'s `Handle` fields already free
    // themselves via their own `Drop`).
    Some(bk_build_block(root))
}

#[cfg(test)]
mod parse_colr_tests {
    use super::*;

    // header(14) + one base glyph record(6) + one layer record(4)
    fn well_formed_colr_table() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&0u16.to_be_bytes()); // version
        b.extend_from_slice(&1u16.to_be_bytes()); // numBaseGlyphRecords
        b.extend_from_slice(&14u32.to_be_bytes()); // offsetBaseGlyphRecord
        b.extend_from_slice(&20u32.to_be_bytes()); // offsetLayerRecord
        b.extend_from_slice(&1u16.to_be_bytes()); // numLayerRecords
        // BaseGlyphRecord @14
        b.extend_from_slice(&5u16.to_be_bytes()); // gid
        b.extend_from_slice(&0u16.to_be_bytes()); // firstLayerIndex
        b.extend_from_slice(&1u16.to_be_bytes()); // numLayers
        // LayerRecord @20
        b.extend_from_slice(&9u16.to_be_bytes()); // gid
        b.extend_from_slice(&3u16.to_be_bytes()); // paletteIndex
        b
    }

    #[test]
    fn well_formed_table_reads_one_base_glyph_and_its_layer() {
        let data = well_formed_colr_table();
        let colr = unsafe { parse_colr(&data).unwrap() };
        assert_eq!(colr.len(), 1);
        assert_eq!(colr[0].glyph.index, 5);
        assert_eq!(colr[0].layers.len(), 1);
        assert_eq!(colr[0].layers[0].glyph.index, 9);
        assert_eq!(colr[0].layers[0].palette_index, 3);
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        let data = well_formed_colr_table();
        assert!(unsafe { parse_colr(&data[..10]) }.is_err());
    }

    #[test]
    fn base_glyph_record_offset_past_the_table_end_errs_instead_of_reading_oob() {
        let mut data = well_formed_colr_table();
        data[4..8].copy_from_slice(&1000u32.to_be_bytes());
        assert!(unsafe { parse_colr(&data) }.is_err());
    }

    #[test]
    fn layer_index_past_num_layer_records_is_skipped_not_read_oob() {
        // numLayers/firstLayerIndex say this base glyph covers layer index
        // 5, but only one layer record actually exists -- the original
        // silently dropped layers that failed this bound, and this
        // preserves that (the base glyph still appears, just with no
        // layers), rather than reading past `gids`/`colors`.
        let mut data = well_formed_colr_table();
        data[16..18].copy_from_slice(&5u16.to_be_bytes()); // firstLayerIndex = 5
        let colr = unsafe { parse_colr(&data).unwrap() };
        assert_eq!(colr[0].layers.len(), 0);
    }
}
