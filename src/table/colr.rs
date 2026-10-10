
use crate::support::handle::{GlyphHandle, Handle, HandleState, handle_from_index, handle_from_name};
use otfcc_json::ParsedValue;

use otfcc_binary::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::sfnt::Packet;
use otfcc_binary::Buffer;
use otfcc_binary::{FontReader, ReadError};
use crate::support::primitives::{ColorId, GlyphId};
use otfcc_json::JsonType;

use otfcc_binary::bk::graph::bk_build_block;
use otfcc_json::BuiltValue;
#[derive(Clone, Debug)]
pub struct ColrLayer {
    pub glyph: GlyphHandle,
    pub palette_index: ColorId,
}
#[derive(Clone, Debug)]
pub struct ColrMapping {
    pub glyph: GlyphHandle,
    pub layers: Vec<ColrLayer>,
}
pub type ColrTable = Vec<ColrMapping>;

static BASE_GLYPH_REC_LENGTH: usize = 6_usize;
static LAYER_REC_LENGTH: usize = 4_usize;
/// `offset_base_glyph_record`/`offset_layer_record` are raw `u32`s straight
/// from the file, so every position is computed through `FontReader`'s
/// `checked_add`/`checked_mul` rather than by plain addition.
fn decode_colr(data: &[u8]) -> Result<ColrTable, ReadError> {
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
            glyph: Handle::new(HandleState::Empty, 0, Vec::new()),
            layers: Vec::new(),
        };
        mapping.glyph = handle_from_index(gid as GlyphId);
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
pub fn read_colr(packet: &Packet) -> Option<ColrTable> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_COLR)?;
    match decode_colr(&table.data) {
        Ok(colr) => Some(colr),
        Err(_) => {
            tracing::warn!("Table 'COLR' corrupted.\n");
            None
        }
    }
}
pub fn dump_colr(colr: Option<&ColrTable>, root: &mut BuiltValue) {
    let Some(mappings) = colr else {
        return;
    };
    let stage = crate::logger::stage("COLR");
    let mut _colr = BuiltValue::new_array(mappings.len());
    for mapping in mappings.iter() {
        let mut _map = BuiltValue::new_object(2);
        _map.push_field(b"from", BuiltValue::str_truncated_at_nul(&mapping.glyph.name));
        let mut _layers = BuiltValue::new_array(mapping.layers.len());
        for layer in mapping.layers.iter() {
            let mut _layer = BuiltValue::new_object(2);
            _layer.push_field(b"layer", BuiltValue::str_truncated_at_nul(&layer.glyph.name));
            _layer.push_field(b"paletteIndex", BuiltValue::Int(layer.palette_index as i64));
            _layers.push_item(_layer);
        }
        _map.push_field(b"to", _layers.preserialize());
        _colr.push_item(_map);
    }
    root.push_field(b"COLR", _colr);
    stage.finish();
}
pub fn parse_colr(root: &ParsedValue) -> Option<ColrTable> {
    let colr_val = root.get_typed(b"COLR", JsonType::Array)?;
    let mut colr: ColrTable = Vec::new();
    let stage = crate::logger::stage("COLR");
    if let Some(mappings) = colr_val.as_array() {
        for mapping in mappings {
            if mapping.as_object().is_none() {
                continue;
            }
            let baseglyph = mapping.get_typed(b"from", JsonType::String);
            let layers_val = mapping.get_typed(b"to", JsonType::Array);
            if let (Some(baseglyph), Some(layers_val)) = (baseglyph, layers_val) {
                let mut m: ColrMapping = ColrMapping {
                    glyph: Handle::new(HandleState::Empty, 0, Vec::new()),
                    layers: Vec::new(),
                };
                m.glyph = handle_from_name(baseglyph.as_str_bytes().map(|b| b.to_vec()));
                if let Some(layer_items) = layers_val.as_array() {
                    for layer in layer_items {
                        if layer.as_object().is_none() {
                            continue;
                        }
                        if let Some(layerglyph) = layer.get_typed(b"layer", JsonType::String) {
                            m.layers.push(ColrLayer {
                                glyph: handle_from_name(
                                    layerglyph.as_str_bytes().map(|b| b.to_vec()),
                                ),
                                palette_index: layer.get_int_or(b"paletteIndex", 0xffff_i32)
                                    as ColorId,
                            });
                        }
                    }
                }
                colr.push(m);
            }
        }
    }
    stage.finish();
    Some(colr)
}
pub fn build_colr(_colr: Option<&ColrTable>) -> Option<Buffer> {
    let src = match _colr {
        Some(c) if !c.is_empty() => c,
        _ => return None,
    };
    let mut colr: ColrTable = src.clone();
    colr.sort_by_key(|a| a.glyph.index);
    let mut current_layer_index: GlyphId = 0 as GlyphId;
    let mut layer_records: BkBlock = bk_new_block(Vec::new());
    let mut base_records: BkBlock = bk_new_block(Vec::new());
    for mapping in colr.iter() {
        bk_push(
            &mut base_records,
            vec![
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
        for layer in mapping.layers.iter() {
            bk_push(
                &mut layer_records,
                vec![
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
        }
    }
    let root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, 0_u32),
        bk_int(BkCellType::B16, (colr.len()) as u32),
        bk_ptr(BkCellType::P32, Some(base_records)),
        bk_ptr(BkCellType::P32, Some(layer_records)),
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
        let colr = decode_colr(&data).unwrap();
        assert_eq!(colr.len(), 1);
        assert_eq!(colr[0].glyph.index, 5);
        assert_eq!(colr[0].layers.len(), 1);
        assert_eq!(colr[0].layers[0].glyph.index, 9);
        assert_eq!(colr[0].layers[0].palette_index, 3);
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        let data = well_formed_colr_table();
        assert!(decode_colr(&data[..10]).is_err());
    }

    #[test]
    fn base_glyph_record_offset_past_the_table_end_errs_instead_of_reading_oob() {
        let mut data = well_formed_colr_table();
        data[4..8].copy_from_slice(&1000u32.to_be_bytes());
        assert!(decode_colr(&data).is_err());
    }

    #[test]
    fn layer_index_past_num_layer_records_is_skipped_not_read_oob() {
        // numLayers/firstLayerIndex say this base glyph covers layer index
        // 5, but only one layer record actually exists -- layers that fail
        // this bound are dropped (the base glyph still appears, just with
        // no layers) rather than read past `gids`/`colors`.
        let mut data = well_formed_colr_table();
        data[16..18].copy_from_slice(&5u16.to_be_bytes()); // firstLayerIndex = 5
        let colr = decode_colr(&data).unwrap();
        assert_eq!(colr[0].layers.len(), 0);
    }
}
