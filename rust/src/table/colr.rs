#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, qsort};

use crate::support::json_funcs::{json_obj_get_type, json_obj_getint_fallback, preserialize};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_copy, otfcc_handle_dispose, otfcc_handle_init, otfcc_handle_move, Handle, GlyphHandle, HandleState};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{ColorId, GlyphId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow_to, cvec_init, cvec_push};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::support::{ComparFn};
use crate::bk::bkgraph::{bk_build_block};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_string_new};
use crate::vendor::sds::{sdsempty, sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColrLayer {
    pub glyph: GlyphHandle,
    pub palette_index: ColorId,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColrLayerElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut ColrLayer) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut ColrLayer, *const ColrLayer) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ColrLayer) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColrLayerList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut ColrLayer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColrLayerListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut ColrLayerList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut ColrLayerList, *const ColrLayerList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ColrLayerList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut ColrLayerList>,
    pub free: Option<unsafe extern "C" fn(*mut ColrLayerList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut ColrLayerList, ColrLayer) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColrMapping {
    pub glyph: GlyphHandle,
    pub layers: ColrLayerList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColrMappingElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut ColrMapping) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut ColrMapping, *const ColrMapping) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ColrMapping) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColrTable {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut ColrMapping,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColrTableVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut ColrTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut ColrTable, *const ColrTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ColrTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut ColrTable>,
    pub free: Option<unsafe extern "C" fn(*mut ColrTable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut ColrTable, ColrMapping) -> ()>,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut ColrTable,
            Option<
                unsafe extern "C" fn(
                    *const ColrMapping,
                    *const ColrMapping,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[inline]
unsafe extern "C" fn init_layer(mut layer: *mut ColrLayer) {
    otfcc_handle_init(&raw mut (*layer).glyph);
}
#[inline]
unsafe extern "C" fn copy_layer(mut dst: *mut ColrLayer, mut src: *const ColrLayer) {
    otfcc_handle_copy(
        &raw mut (*dst).glyph,
        &raw const (*src).glyph,
    );
    (*dst).palette_index = (*src).palette_index;
}
#[inline]
unsafe extern "C" fn dispose_layer(mut layer: *mut ColrLayer) {
    otfcc_handle_dispose(&raw mut (*layer).glyph);
}
pub static COLR_I_LAYER: ColrLayerElementInterface = {
    ColrLayerElementInterface {
        init: Some(colr_layer_init as unsafe extern "C" fn(*mut ColrLayer) -> ()),
        copy: Some(
            colr_layer_copy as unsafe extern "C" fn(*mut ColrLayer, *const ColrLayer) -> (),
        ),
        dispose: Some(colr_layer_dispose as unsafe extern "C" fn(*mut ColrLayer) -> ()),
    }
};
#[inline]
unsafe extern "C" fn colr_layer_dispose(mut x: *mut ColrLayer) {
    dispose_layer(x);
}
#[inline]
unsafe extern "C" fn colr_layer_init(mut x: *mut ColrLayer) {
    init_layer(x);
}
#[inline]
unsafe extern "C" fn colr_layer_copy(mut dst: *mut ColrLayer, mut src: *const ColrLayer) {
    copy_layer(dst, src);
}
#[inline]
unsafe extern "C" fn colr_layer_list_grow_to(arr: *mut ColrLayerList, target: usize) {
    cvec_grow_to(colr_layer_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn colr_layer_list_push(arr: *mut ColrLayerList, elem: ColrLayer) {
    cvec_push(colr_layer_list_as_cvec(arr), elem);
}
pub static COLR_I_LAYER_LIST: ColrLayerListVectorInterface = {
    ColrLayerListVectorInterface {
        init: Some(colr_layer_list_init as unsafe extern "C" fn(*mut ColrLayerList) -> ()),
        copy: Some(
            colr_layer_list_copy
                as unsafe extern "C" fn(*mut ColrLayerList, *const ColrLayerList) -> (),
        ),
        dispose: Some(colr_layer_list_dispose as unsafe extern "C" fn(*mut ColrLayerList) -> ()),
        create: Some(colr_layer_list_create),
        free: Some(colr_layer_list_free as unsafe extern "C" fn(*mut ColrLayerList) -> ()),
        push: Some(
            colr_layer_list_push as unsafe extern "C" fn(*mut ColrLayerList, ColrLayer) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn colr_layer_list_copy(
    mut dst: *mut ColrLayerList,
    mut src: *const ColrLayerList,
) {
    colr_layer_list_init(dst);
    colr_layer_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if COLR_I_LAYER.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            COLR_I_LAYER.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut ColrLayer,
                (*src).items.offset(j as isize) as *mut ColrLayer as *const ColrLayer,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn colr_layer_list_dispose(mut arr: *mut ColrLayerList) {
    if arr.is_null() {
        return;
    }
    if COLR_I_LAYER.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            COLR_I_LAYER.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut ColrLayer,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<ColrLayer>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn colr_layer_list_free(mut x: *mut ColrLayerList) {
    if x.is_null() {
        return;
    }
    colr_layer_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn colr_layer_list_create() -> *mut ColrLayerList {
    let mut x: *mut ColrLayerList =
        malloc(::core::mem::size_of::<ColrLayerList>() as usize) as *mut ColrLayerList;
    colr_layer_list_init(x);
    return x;
}
#[inline]
unsafe fn colr_layer_list_as_cvec(arr: *mut ColrLayerList) -> *mut CVecRaw<ColrLayer> {
    arr as *mut CVecRaw<ColrLayer>
}
#[inline]
unsafe extern "C" fn colr_layer_list_init(arr: *mut ColrLayerList) {
    cvec_init(colr_layer_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn init_mapping(mut mapping: *mut ColrMapping) {
    otfcc_handle_init(&raw mut (*mapping).glyph);
    COLR_I_LAYER_LIST.init.expect("non-null function pointer")(&raw mut (*mapping).layers);
}
#[inline]
unsafe extern "C" fn copy_mapping(mut dst: *mut ColrMapping, mut src: *const ColrMapping) {
    otfcc_handle_copy(
        &raw mut (*dst).glyph,
        &raw const (*src).glyph,
    );
    COLR_I_LAYER_LIST.copy.expect("non-null function pointer")(
        &raw mut (*dst).layers,
        &raw const (*src).layers,
    );
}
#[inline]
unsafe extern "C" fn dispose_mapping(mut mapping: *mut ColrMapping) {
    otfcc_handle_dispose(&raw mut (*mapping).glyph);
    COLR_I_LAYER_LIST.dispose.expect("non-null function pointer")(&raw mut (*mapping).layers);
}
#[inline]
unsafe extern "C" fn colr_mapping_dispose(mut x: *mut ColrMapping) {
    dispose_mapping(x);
}
pub static COLR_I_MAPPING: ColrMappingElementInterface = {
    ColrMappingElementInterface {
        init: Some(colr_mapping_init as unsafe extern "C" fn(*mut ColrMapping) -> ()),
        copy: Some(
            colr_mapping_copy as unsafe extern "C" fn(*mut ColrMapping, *const ColrMapping) -> (),
        ),
        dispose: Some(colr_mapping_dispose as unsafe extern "C" fn(*mut ColrMapping) -> ()),
    }
};
#[inline]
unsafe extern "C" fn colr_mapping_init(mut x: *mut ColrMapping) {
    init_mapping(x);
}
#[inline]
unsafe extern "C" fn colr_mapping_copy(mut dst: *mut ColrMapping, mut src: *const ColrMapping) {
    copy_mapping(dst, src);
}
#[inline]
unsafe extern "C" fn table_colr_grow_to(arr: *mut ColrTable, target: usize) {
    cvec_grow_to(table_colr_as_cvec(arr), target);
}
pub static TABLE_I_COLR: ColrTableVectorInterface = {
    ColrTableVectorInterface {
        init: Some(table_colr_init as unsafe extern "C" fn(*mut ColrTable) -> ()),
        copy: Some(
            table_colr_copy as unsafe extern "C" fn(*mut ColrTable, *const ColrTable) -> (),
        ),
        dispose: Some(table_colr_dispose as unsafe extern "C" fn(*mut ColrTable) -> ()),
        create: Some(table_colr_create),
        free: Some(table_colr_free as unsafe extern "C" fn(*mut ColrTable) -> ()),
        push: Some(table_colr_push as unsafe extern "C" fn(*mut ColrTable, ColrMapping) -> ()),
        sort: Some(
            table_colr_sort
                as unsafe extern "C" fn(
                    *mut ColrTable,
                    Option<
                        unsafe extern "C" fn(
                            *const ColrMapping,
                            *const ColrMapping,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe fn table_colr_as_cvec(arr: *mut ColrTable) -> *mut CVecRaw<ColrMapping> {
    arr as *mut CVecRaw<ColrMapping>
}
#[inline]
unsafe extern "C" fn table_colr_init(arr: *mut ColrTable) {
    cvec_init(table_colr_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_colr_sort(
    mut arr: *mut ColrTable,
    mut fn_0: Option<
        unsafe extern "C" fn(*const ColrMapping, *const ColrMapping) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<ColrMapping>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const ColrMapping,
                    *const ColrMapping,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn table_colr_push(arr: *mut ColrTable, elem: ColrMapping) {
    cvec_push(table_colr_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_colr_copy(mut dst: *mut ColrTable, mut src: *const ColrTable) {
    table_colr_init(dst);
    table_colr_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if COLR_I_MAPPING.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            COLR_I_MAPPING.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut ColrMapping,
                (*src).items.offset(j as isize) as *mut ColrMapping as *const ColrMapping,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn table_colr_dispose(mut arr: *mut ColrTable) {
    if arr.is_null() {
        return;
    }
    if COLR_I_MAPPING.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh3 = j;
            j = j.wrapping_sub(1);
            if !(fresh3 != 0) {
                break;
            }
            COLR_I_MAPPING.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut ColrMapping,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<ColrMapping>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn table_colr_free(mut x: *mut ColrTable) {
    if x.is_null() {
        return;
    }
    table_colr_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_colr_create() -> *mut ColrTable {
    let mut x: *mut ColrTable =
        malloc(::core::mem::size_of::<ColrTable>() as usize) as *mut ColrTable;
    table_colr_init(x);
    return x;
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
                                colr = (
                                    TABLE_I_COLR.create.expect("non-null function pointer"))();
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
                                        layers: ColrLayerList {
                                            length: 0,
                                            capacity: 0,
                                            items: ::core::ptr::null_mut::<ColrLayer>(),
                                        },
                                    };
                                    COLR_I_MAPPING.init.expect("non-null function pointer")(
                                        &raw mut mapping,
                                    );
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
                                            COLR_I_LAYER_LIST
                                                .push
                                                .expect("non-null function pointer")(
                                                &raw mut mapping.layers,
                                                ColrLayer {
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
                                                },
                                            );
                                        }
                                        k = k.wrapping_add(1);
                                    }
                                    TABLE_I_COLR.push.expect("non-null function pointer")(
                                        colr, mapping,
                                    );
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
                    TABLE_I_COLR.free.expect("non-null function pointer")(colr);
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
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _colr: *mut JsonValue = json_array_new((*colr).length);
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < (*colr).length {
            let mut mapping: *mut ColrMapping = (*colr).items.offset(__caryll_index as isize);
            while keep != 0 {
                let mut _map: *mut JsonValue = json_object_new(2 as usize);
                json_object_push(
                    _map,
                    b"from\0" as *const u8 as *const ::core::ffi::c_char,
                    json_string_new((*mapping).glyph.name as *const ::core::ffi::c_char),
                );
                let mut _layers: *mut JsonValue = json_array_new((*mapping).layers.length);
                let mut __caryll_index_0: usize = 0 as usize;
                let mut keep_0: usize = 1 as usize;
                while keep_0 != 0 && __caryll_index_0 < (*mapping).layers.length {
                    let mut layer: *mut ColrLayer =
                        (*mapping).layers.items.offset(__caryll_index_0 as isize);
                    while keep_0 != 0 {
                        let mut _layer: *mut JsonValue = json_object_new(2 as usize);
                        json_object_push(
                            _layer,
                            b"layer\0" as *const u8 as *const ::core::ffi::c_char,
                            json_string_new((*layer).glyph.name as *const ::core::ffi::c_char),
                        );
                        json_object_push(
                            _layer,
                            b"paletteIndex\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*layer).palette_index as i64),
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
    let mut colr: *mut ColrTable = (
        TABLE_I_COLR.create.expect("non-null function pointer"))();
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
                        layers: ColrLayerList {
                            length: 0,
                            capacity: 0,
                            items: ::core::ptr::null_mut::<ColrLayer>(),
                        },
                    };
                    COLR_I_MAPPING.init.expect("non-null function pointer")(&raw mut m);
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
                                COLR_I_LAYER_LIST.push.expect("non-null function pointer")(
                                    &raw mut m.layers,
                                    ColrLayer {
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
                                    },
                                );
                            }
                        }
                        k = k.wrapping_add(1);
                    }
                    TABLE_I_COLR.push.expect("non-null function pointer")(colr, m);
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
unsafe extern "C" fn by_gid(
    mut a: *const ColrMapping,
    mut b: *const ColrMapping,
) -> ::core::ffi::c_int {
    return (*a).glyph.index as ::core::ffi::c_int - (*b).glyph.index as ::core::ffi::c_int;
}
pub unsafe extern "C" fn otfcc_build_colr(
    mut _colr: *const ColrTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if _colr.is_null() || (*_colr).length == 0 {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut colr: ColrTable = ColrTable {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<ColrMapping>(),
    };
    TABLE_I_COLR.copy.expect("non-null function pointer")(&raw mut colr, _colr);
    TABLE_I_COLR.sort.expect("non-null function pointer")(
        &raw mut colr,
        Some(
            by_gid
                as unsafe extern "C" fn(
                    *const ColrMapping,
                    *const ColrMapping,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut current_layer_index: GlyphId = 0 as GlyphId;
    let mut layer_records: *mut BkBlock = bk_new_block(&[]);
    let mut base_records: *mut BkBlock = bk_new_block(&[]);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < colr.length {
        let mut mapping: *mut ColrMapping = colr.items.offset(__caryll_index as isize);
        while keep != 0 {
            bk_push(base_records, &[bk_int(BkCellType::B16, ((*mapping).glyph.index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (current_layer_index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*mapping).layers.length) as u32)]);
            let mut __caryll_index_0: usize = 0 as usize;
            let mut keep_0: usize = 1 as usize;
            while keep_0 != 0 && __caryll_index_0 < (*mapping).layers.length {
                let mut layer: *mut ColrLayer =
                    (*mapping).layers.items.offset(__caryll_index_0 as isize);
                while keep_0 != 0 {
                    bk_push(layer_records, &[bk_int(BkCellType::B16, ((*layer).glyph.index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*layer).palette_index as ::core::ffi::c_int) as u32)]);
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
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 0 as u32), bk_int(BkCellType::B16, (colr.length) as u32), bk_ptr(BkCellType::P32, base_records), bk_ptr(BkCellType::P32, layer_records), bk_int(BkCellType::B16, (current_layer_index as ::core::ffi::c_int) as u32)]);
    TABLE_I_COLR.dispose.expect("non-null function pointer")(&raw mut colr);
    return bk_build_block(root);
}
