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
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::binio::{read_16u, read_32u};
use crate::support::buffer::Buffer;
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
static BASE_GLYPH_REC_LENGTH: usize = 6 as usize;
static LAYER_REC_LENGTH: usize = 4 as usize;
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_read_colr(packet: &Packet, options: &Options) -> Option<ColrTable> {
    let mut num_base_glyph_records: u16;
    let mut num_layer_records: u16;
    let mut offset_base_glyph_record: u32;
    let mut offset_layer_record: u32;
    let mut gids: Vec<GlyphId>;
    let mut colors: Vec<ColorId>;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_COLR {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    if !(table.length < 14 as u32) {
                        num_base_glyph_records =
                            read_16u(table.data.as_ptr().offset(2 as ::core::ffi::c_int as isize));
                        num_layer_records = read_16u(
                            table
                                .data
                                .as_ptr()
                                .offset(12 as ::core::ffi::c_int as isize),
                        );
                        offset_base_glyph_record =
                            read_32u(table.data.as_ptr().offset(4 as ::core::ffi::c_int as isize));
                        offset_layer_record =
                            read_32u(table.data.as_ptr().offset(8 as ::core::ffi::c_int as isize));
                        if !((table.length as usize)
                            < (offset_base_glyph_record as usize).wrapping_add(
                                BASE_GLYPH_REC_LENGTH.wrapping_mul(num_base_glyph_records as usize),
                            ))
                        {
                            if !((table.length as usize)
                                < (offset_layer_record as usize).wrapping_add(
                                    LAYER_REC_LENGTH.wrapping_mul(num_layer_records as usize),
                                ))
                            {
                                gids = Vec::with_capacity(num_layer_records as usize);
                                colors = Vec::with_capacity(num_layer_records as usize);
                                let mut j: GlyphId = 0 as GlyphId;
                                while (j as ::core::ffi::c_int)
                                    < num_layer_records as ::core::ffi::c_int
                                {
                                    gids.push(read_16u(
                                        table
                                            .data
                                            .as_ptr()
                                            .offset(offset_layer_record as isize)
                                            .offset(
                                                LAYER_REC_LENGTH.wrapping_mul(j as usize) as isize
                                            ),
                                    ) as GlyphId);
                                    colors.push(read_16u(
                                        table
                                            .data
                                            .as_ptr()
                                            .offset(offset_layer_record as isize)
                                            .offset(
                                                LAYER_REC_LENGTH.wrapping_mul(j as usize) as isize
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize),
                                    ) as ColorId);
                                    j = j.wrapping_add(1);
                                }
                                let mut colr: ColrTable = Vec::new();
                                let mut j_0: GlyphId = 0 as GlyphId;
                                while (j_0 as ::core::ffi::c_int)
                                    < num_base_glyph_records as ::core::ffi::c_int
                                {
                                    let mut mapping: ColrMapping = ColrMapping {
                                        glyph: Handle {
                                            state: HandleState::Empty,
                                            index: 0,
                                            name: Vec::new(),
                                        },
                                        layers: Vec::new(),
                                    };
                                    let gid: u16 = read_16u(
                                        table
                                            .data
                                            .as_ptr()
                                            .offset(offset_base_glyph_record as isize)
                                            .offset(
                                                BASE_GLYPH_REC_LENGTH.wrapping_mul(j_0 as usize)
                                                    as isize,
                                            ),
                                    );
                                    let first_layer_index: u16 = read_16u(
                                        table
                                            .data
                                            .as_ptr()
                                            .offset(offset_base_glyph_record as isize)
                                            .offset(
                                                BASE_GLYPH_REC_LENGTH.wrapping_mul(j_0 as usize)
                                                    as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize),
                                    );
                                    let num_layers: u16 = read_16u(
                                        table
                                            .data
                                            .as_ptr()
                                            .offset(offset_base_glyph_record as isize)
                                            .offset(
                                                BASE_GLYPH_REC_LENGTH.wrapping_mul(j_0 as usize)
                                                    as isize,
                                            )
                                            .offset(4 as ::core::ffi::c_int as isize),
                                    );
                                    let mut base_glyph: GlyphHandle =
                                        handle_from_index(gid as GlyphId) as GlyphHandle;
                                    otfcc_handle_move(&raw mut mapping.glyph, &raw mut base_glyph);
                                    let mut k: GlyphId = 0 as GlyphId;
                                    while (k as ::core::ffi::c_int)
                                        < num_layers as ::core::ffi::c_int
                                    {
                                        if (k as ::core::ffi::c_int
                                            + first_layer_index as ::core::ffi::c_int)
                                            < num_layer_records as ::core::ffi::c_int
                                        {
                                            mapping.layers.push(ColrLayer {
                                                glyph: handle_from_index(
                                                    gids[(k as ::core::ffi::c_int
                                                        + first_layer_index as ::core::ffi::c_int)
                                                        as usize],
                                                )
                                                    as GlyphHandle,
                                                palette_index: colors[(k as ::core::ffi::c_int
                                                    + first_layer_index as ::core::ffi::c_int)
                                                    as usize],
                                            });
                                        }
                                        k = k.wrapping_add(1);
                                    }
                                    colr.push(mapping);
                                    j_0 = j_0.wrapping_add(1);
                                }
                                return Some(colr);
                            }
                        }
                    }
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(b"Table 'COLR' corrupted.\n"),
                    );
                    // No `colr` to free here: every path that constructs
                    // one (deep inside the nested guards above) returns
                    // immediately afterward, so this branch is only ever
                    // reached before any allocation happens.
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return None;
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
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < mappings.len() {
            let mapping: &ColrMapping = &mappings[__caryll_index];
            while keep != 0 {
                let mut _map: *mut BuiltValue = json_object_new(2 as usize);
                json_object_push(
                    _map,
                    b"from\0" as *const u8 as *const ::core::ffi::c_char,
                    json_string_new_from_bytes(&mapping.glyph.name),
                );
                let mut _layers: *mut BuiltValue = json_array_new(mapping.layers.len());
                let mut __caryll_index_0: usize = 0 as usize;
                let mut keep_0: usize = 1 as usize;
                while keep_0 != 0 && __caryll_index_0 < mapping.layers.len() {
                    let layer: &ColrLayer = &mapping.layers[__caryll_index_0];
                    while keep_0 != 0 {
                        let mut _layer: *mut BuiltValue = json_object_new(2 as usize);
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
                        keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    }
                    keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    __caryll_index_0 = __caryll_index_0.wrapping_add(1);
                }
                json_object_push(
                    _map,
                    b"to\0" as *const u8 as *const ::core::ffi::c_char,
                    preserialize(_layers),
                );
                json_array_push(_colr, _map);
                keep = (keep == 0) as ::core::ffi::c_int as usize;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
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
                                        0xffff as i32,
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
pub unsafe fn otfcc_build_colr(_colr: Option<&ColrTable>) -> *mut Buffer {
    let src = match _colr {
        Some(c) if !c.is_empty() => c,
        _ => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut colr: ColrTable = src.iter().map(colr_mapping_dup).collect();
    colr.sort_by(|a, b| a.glyph.index.cmp(&b.glyph.index));
    let mut current_layer_index: GlyphId = 0 as GlyphId;
    let layer_records: *mut BkBlock = bk_new_block(&[]);
    let base_records: *mut BkBlock = bk_new_block(&[]);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < colr.len() {
        let mapping: &ColrMapping = &colr[__caryll_index];
        while keep != 0 {
            bk_push(
                base_records,
                &[
                    bk_int(
                        BkCellType::B16,
                        (mapping.glyph.index as ::core::ffi::c_int) as u32,
                    ),
                    bk_int(
                        BkCellType::B16,
                        (current_layer_index as ::core::ffi::c_int) as u32,
                    ),
                    bk_int(BkCellType::B16, (mapping.layers.len()) as u32),
                ],
            );
            let mut __caryll_index_0: usize = 0 as usize;
            let mut keep_0: usize = 1 as usize;
            while keep_0 != 0 && __caryll_index_0 < mapping.layers.len() {
                let layer: &ColrLayer = &mapping.layers[__caryll_index_0];
                while keep_0 != 0 {
                    bk_push(
                        layer_records,
                        &[
                            bk_int(
                                BkCellType::B16,
                                (layer.glyph.index as ::core::ffi::c_int) as u32,
                            ),
                            bk_int(
                                BkCellType::B16,
                                (layer.palette_index as ::core::ffi::c_int) as u32,
                            ),
                        ],
                    );
                    current_layer_index = (current_layer_index as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as GlyphId;
                    keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                }
                keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                __caryll_index_0 = __caryll_index_0.wrapping_add(1);
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B16, 0 as u32),
        bk_int(BkCellType::B16, (colr.len()) as u32),
        bk_ptr(BkCellType::P32, base_records),
        bk_ptr(BkCellType::P32, layer_records),
        bk_int(
            BkCellType::B16,
            (current_layer_index as ::core::ffi::c_int) as u32,
        ),
    ]);
    // `colr` drops naturally at the end of this scope -- no explicit
    // dispose call needed (`ColrMapping`'s `Handle` fields already free
    // themselves via their own `Drop`).
    return bk_build_block(root);
}
