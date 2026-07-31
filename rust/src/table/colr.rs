#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc};

use crate::support::json_funcs::{json_obj_get_type, json_obj_getint_fallback, preserialize};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dispose, otfcc_handle_dup, otfcc_handle_move, Handle, GlyphHandle, HandleState};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{ColorId, GlyphId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::bk::bkgraph::{bk_build_block};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_string_new};
use crate::vendor::sds::{sdsempty, sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColrLayer {
    pub glyph: GlyphHandle,
    pub palette_index: ColorId,
}
pub struct ColrMapping {
    pub glyph: GlyphHandle,
    pub layers: Vec<ColrLayer>,
}
pub type ColrTable = Vec<ColrMapping>;

// `ColrLayer`/`ColrMapping` stay Copy-at-the-leaf (`Handle` does, crate-wide --
// Stage 6-4's job to change), which means a bitwise `Clone`/`Copy` of either
// only *aliases* the `Handle.name` sds string rather than duplicating it. The
// crate's own convention, unchanged here, is that a REAL duplicate always
// goes through an explicit dup/copy function -- never an implicit derive --
// so these two are written the same way, not derived.
pub(crate) fn colr_layer_dup(l: &ColrLayer) -> ColrLayer {
    unsafe {
        ColrLayer {
            glyph: otfcc_handle_dup(l.glyph),
            palette_index: l.palette_index,
        }
    }
}
unsafe fn dispose_colr_layer(l: *mut ColrLayer) {
    otfcc_handle_dispose(&raw mut (*l).glyph);
}
fn colr_mapping_dup(m: &ColrMapping) -> ColrMapping {
    ColrMapping {
        glyph: unsafe { otfcc_handle_dup(m.glyph) },
        layers: m.layers.iter().map(colr_layer_dup).collect(),
    }
}
pub(crate) unsafe fn dispose_colr_mapping(m: *mut ColrMapping) {
    otfcc_handle_dispose(&raw mut (*m).glyph);
    for l in (*m).layers.iter_mut() {
        dispose_colr_layer(l as *mut ColrLayer);
    }
    (*m).layers = Vec::new();
}
unsafe fn dispose_colr_table(t: *mut ColrTable) {
    for m in (*t).iter_mut() {
        dispose_colr_mapping(m as *mut ColrMapping);
    }
    // Drops the (now Handle-less) `Vec<ColrMapping>` -- which, via each
    // `ColrMapping`'s compiler-generated drop glue, also frees every
    // mapping's `layers: Vec<ColrLayer>` backing array. `t` itself is *not*
    // freed here (see `table_colr_free`, which calls this first).
    *t = Vec::new();
}
pub(crate) unsafe fn table_colr_free(x: *mut ColrTable) {
    if x.is_null() {
        return;
    }
    dispose_colr_table(x);
    free(x as *mut ::core::ffi::c_void);
}
pub(crate) unsafe fn table_colr_create() -> *mut ColrTable {
    let x: *mut ColrTable = malloc(::core::mem::size_of::<ColrTable>() as usize) as *mut ColrTable;
    // `.write()`, not a field/deref assignment: this is placement-constructing
    // the whole value into fresh (possibly uninitialized) memory, so nothing
    // must be read or dropped first -- unlike `GaspTable`'s `calloc` fix
    // (rust/README.md), which had other already-initialized fields around
    // the `Vec` one. Correct regardless of what `malloc` left behind.
    x.write(Vec::new());
    x
}
static BASE_GLYPH_REC_LENGTH: usize = 6 as usize;
static LAYER_REC_LENGTH: usize = 4 as usize;
pub unsafe extern "C" fn otfcc_read_colr(
    packet: Packet,
    mut options: *const Options,
) -> *mut ColrTable {
    let mut num_base_glyph_records: u16 = 0;
    let mut num_layer_records: u16 = 0;
    let mut offset_base_glyph_record: u32 = 0;
    let mut offset_layer_record: u32 = 0;
    let mut gids: *mut GlyphId = ::core::ptr::null_mut::<GlyphId>();
    let mut colors: *mut ColorId = ::core::ptr::null_mut::<ColorId>();
    let mut colr: *mut ColrTable = ::core::ptr::null_mut::<ColrTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1129270354i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    if !(table.length < 14 as u32) {
                        num_base_glyph_records =
                            read_16u(table.data.offset(2 as ::core::ffi::c_int as isize));
                        num_layer_records =
                            read_16u(table.data.offset(12 as ::core::ffi::c_int as isize));
                        offset_base_glyph_record =
                            read_32u(table.data.offset(4 as ::core::ffi::c_int as isize));
                        offset_layer_record =
                            read_32u(table.data.offset(8 as ::core::ffi::c_int as isize));
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
                                gids = ::core::ptr::null_mut::<GlyphId>();
                                colors = ::core::ptr::null_mut::<ColorId>();
                                gids = __caryll_allocate_clean(
                                    (::core::mem::size_of::<GlyphId>() as usize)
                                        .wrapping_mul(num_layer_records as usize),
                                    52 as ::core::ffi::c_ulong,
                                ) as *mut GlyphId;
                                colors = __caryll_allocate_clean(
                                    (::core::mem::size_of::<ColorId>() as usize)
                                        .wrapping_mul(num_layer_records as usize),
                                    53 as ::core::ffi::c_ulong,
                                ) as *mut ColorId;
                                let mut j: GlyphId = 0 as GlyphId;
                                while (j as ::core::ffi::c_int)
                                    < num_layer_records as ::core::ffi::c_int
                                {
                                    *gids.offset(j as isize) = read_16u(
                                        table.data.offset(offset_layer_record as isize).offset(
                                            LAYER_REC_LENGTH.wrapping_mul(j as usize) as isize,
                                        ),
                                    )
                                        as GlyphId;
                                    *colors.offset(j as isize) =
                                        read_16u(
                                            table
                                                .data
                                                .offset(offset_layer_record as isize)
                                                .offset(LAYER_REC_LENGTH.wrapping_mul(j as usize)
                                                    as isize)
                                                .offset(2 as ::core::ffi::c_int as isize),
                                        ) as ColorId;
                                    j = j.wrapping_add(1);
                                }
                                colr = table_colr_create();
                                let mut j_0: GlyphId = 0 as GlyphId;
                                while (j_0 as ::core::ffi::c_int)
                                    < num_base_glyph_records as ::core::ffi::c_int
                                {
                                    let mut mapping: ColrMapping = ColrMapping {
                                        glyph: Handle {
                                            state: HandleState::Empty,
                                            index: 0,
                                            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                        },
                                        layers: Vec::new(),
                                    };
                                    let mut gid: u16 = read_16u(
                                        table
                                            .data
                                            .offset(offset_base_glyph_record as isize)
                                            .offset(BASE_GLYPH_REC_LENGTH.wrapping_mul(j_0 as usize)
                                                as isize),
                                    );
                                    let mut first_layer_index: u16 = read_16u(
                                        table
                                            .data
                                            .offset(offset_base_glyph_record as isize)
                                            .offset(BASE_GLYPH_REC_LENGTH.wrapping_mul(j_0 as usize)
                                                as isize)
                                            .offset(2 as ::core::ffi::c_int as isize),
                                    );
                                    let mut num_layers: u16 = read_16u(
                                        table
                                            .data
                                            .offset(offset_base_glyph_record as isize)
                                            .offset(BASE_GLYPH_REC_LENGTH.wrapping_mul(j_0 as usize)
                                                as isize)
                                            .offset(4 as ::core::ffi::c_int as isize),
                                    );
                                    let mut base_glyph: GlyphHandle = handle_from_index(
                                        gid as GlyphId
                                    )
                                        as GlyphHandle;
                                    otfcc_handle_move(
                                        &raw mut mapping.glyph,
                                        &raw mut base_glyph,
                                    );
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
                                                    *gids.offset(
                                                        (k as ::core::ffi::c_int
                                                            + first_layer_index
                                                                as ::core::ffi::c_int)
                                                            as isize,
                                                    ),
                                                )
                                                    as GlyphHandle,
                                                palette_index: *colors.offset(
                                                    (k as ::core::ffi::c_int
                                                        + first_layer_index as ::core::ffi::c_int)
                                                        as isize,
                                                ),
                                            });
                                        }
                                        k = k.wrapping_add(1);
                                    }
                                    (*colr).push(mapping);
                                    j_0 = j_0.wrapping_add(1);
                                }
                                return colr;
                            }
                        }
                    }
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(sdsempty(), b"Table 'COLR' corrupted.\n"),
                    );
                    table_colr_free(colr);
                    colr = ::core::ptr::null_mut::<ColrTable>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return colr;
}
pub unsafe extern "C" fn otfcc_dump_colr(
    mut colr: *const ColrTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if colr.is_null() {
        return;
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"COLR"),
    );
    let mappings: &Vec<ColrMapping> = &*colr;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _colr: *mut JsonValue = json_array_new(mappings.len());
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < mappings.len() {
            let mapping: &ColrMapping = &mappings[__caryll_index];
            while keep != 0 {
                let mut _map: *mut JsonValue = json_object_new(2 as usize);
                json_object_push(
                    _map,
                    b"from\0" as *const u8 as *const ::core::ffi::c_char,
                    json_string_new(mapping.glyph.name as *const ::core::ffi::c_char),
                );
                let mut _layers: *mut JsonValue = json_array_new(mapping.layers.len());
                let mut __caryll_index_0: usize = 0 as usize;
                let mut keep_0: usize = 1 as usize;
                while keep_0 != 0 && __caryll_index_0 < mapping.layers.len() {
                    let layer: &ColrLayer = &mapping.layers[__caryll_index_0];
                    while keep_0 != 0 {
                        let mut _layer: *mut JsonValue = json_object_new(2 as usize);
                        json_object_push(
                            _layer,
                            b"layer\0" as *const u8 as *const ::core::ffi::c_char,
                            json_string_new(layer.glyph.name as *const ::core::ffi::c_char),
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
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_colr(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut ColrTable {
    let mut _colr: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    _colr = json_obj_get_type(
        root,
        b"COLR\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _colr.is_null() {
        return ::core::ptr::null_mut::<ColrTable>();
    }
    let mut colr: *mut ColrTable = table_colr_create();
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"COLR"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: GlyphId = 0 as GlyphId;
        while (j as ::core::ffi::c_uint) < (*_colr).u.array.length {
            let mut _mapping: *mut JsonValue =
                *(*_colr).u.array.values.offset(j as isize) as *mut JsonValue;
            if !(_mapping.is_null()
                || (*_mapping).type_0 != JsonType::Object)
            {
                let mut _baseglyph: *mut JsonValue = json_obj_get_type(
                    _mapping,
                    b"from\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::String,
                );
                let mut _layers: *mut JsonValue = json_obj_get_type(
                    _mapping,
                    b"to\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                );
                if !(_baseglyph.is_null() || _layers.is_null()) {
                    let mut m: ColrMapping = ColrMapping {
                        glyph: Handle {
                            state: HandleState::Empty,
                            index: 0,
                            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        },
                        layers: Vec::new(),
                    };
                    m.glyph = handle_from_name(sdsnewlen(
                        (*_baseglyph).u.string.ptr as *const ::core::ffi::c_void,
                        (*_baseglyph).u.string.length as usize,
                    )) as GlyphHandle;
                    let mut k: GlyphId = 0 as GlyphId;
                    while (k as ::core::ffi::c_uint) < (*_layers).u.array.length {
                        let mut _layer: *mut JsonValue =
                            *(*_layers).u.array.values.offset(k as isize) as *mut JsonValue;
                        if !(_layer.is_null()
                            || (*_layer).type_0 != JsonType::Object)
                        {
                            let mut _layerglyph: *mut JsonValue = json_obj_get_type(
                                _layer,
                                b"layer\0" as *const u8 as *const ::core::ffi::c_char,
                                JsonType::String,
                            );
                            if !_layerglyph.is_null() {
                                m.layers.push(ColrLayer {
                                    glyph: handle_from_name(
                                        sdsnewlen(
                                            (*_layerglyph).u.string.ptr
                                                as *const ::core::ffi::c_void,
                                            (*_layerglyph).u.string.length as usize,
                                        ),
                                    )
                                        as GlyphHandle,
                                    palette_index: json_obj_getint_fallback(
                                        _layer,
                                        b"paletteIndex\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        0xffff as i32,
                                    )
                                        as ColorId,
                                });
                            }
                        }
                        k = k.wrapping_add(1);
                    }
                    (*colr).push(m);
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    return colr;
}
pub unsafe extern "C" fn otfcc_build_colr(
    mut _colr: *const ColrTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if _colr.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let src: &Vec<ColrMapping> = &*_colr;
    if src.is_empty() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut colr: ColrTable = src.iter().map(colr_mapping_dup).collect();
    colr.sort_by(|a, b| a.glyph.index.cmp(&b.glyph.index));
    let mut current_layer_index: GlyphId = 0 as GlyphId;
    let mut layer_records: *mut BkBlock = bk_new_block(&[]);
    let mut base_records: *mut BkBlock = bk_new_block(&[]);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < colr.len() {
        let mapping: &ColrMapping = &colr[__caryll_index];
        while keep != 0 {
            bk_push(base_records, &[bk_int(BkCellType::B16, (mapping.glyph.index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (current_layer_index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (mapping.layers.len()) as u32)]);
            let mut __caryll_index_0: usize = 0 as usize;
            let mut keep_0: usize = 1 as usize;
            while keep_0 != 0 && __caryll_index_0 < mapping.layers.len() {
                let layer: &ColrLayer = &mapping.layers[__caryll_index_0];
                while keep_0 != 0 {
                    bk_push(layer_records, &[bk_int(BkCellType::B16, (layer.glyph.index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (layer.palette_index as ::core::ffi::c_int) as u32)]);
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
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 0 as u32), bk_int(BkCellType::B16, (colr.len()) as u32), bk_ptr(BkCellType::P32, base_records), bk_ptr(BkCellType::P32, layer_records), bk_int(BkCellType::B16, (current_layer_index as ::core::ffi::c_int) as u32)]);
    dispose_colr_table(&raw mut colr);
    return bk_build_block(root);
}
