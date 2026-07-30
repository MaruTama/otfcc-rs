#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};

use crate::support::json_funcs::{json_obj_get_type, json_obj_getint_fallback, preserialize};
use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_copy, otfcc_Handle_dispose, otfcc_Handle_init, otfcc_Handle_move, Handle, GlyphHandle, HandleState};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{ColorId, GlyphId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_Block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::support::{ComparFn};
use crate::bk::bkgraph::{bk_build_Block};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_string_new};
use crate::vendor::sds::{sdsempty, sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColrLayer {
    pub glyph: GlyphHandle,
    pub paletteIndex: ColorId,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColrLayerElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut ColrLayer) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut ColrLayer, *const ColrLayer) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut ColrLayer, *mut ColrLayer) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ColrLayer) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut ColrLayer, ColrLayer) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut ColrLayer, ColrLayer) -> ()>,
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
    pub move_0: Option<unsafe extern "C" fn(*mut ColrLayerList, *mut ColrLayerList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ColrLayerList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut ColrLayerList, ColrLayerList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut ColrLayerList, ColrLayerList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut ColrLayerList>,
    pub free: Option<unsafe extern "C" fn(*mut ColrLayerList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut ColrLayerList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut ColrLayerList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut ColrLayerList>,
    pub fill: Option<unsafe extern "C" fn(*mut ColrLayerList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut ColrLayerList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut ColrLayerList, ColrLayer) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut ColrLayerList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut ColrLayerList) -> ColrLayer>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut ColrLayerList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut ColrLayerList,
            Option<unsafe extern "C" fn(*const ColrLayer, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut ColrLayerList,
            Option<unsafe extern "C" fn(*const ColrLayer, *const ColrLayer) -> ::core::ffi::c_int>,
        ) -> (),
    >,
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
    pub move_0: Option<unsafe extern "C" fn(*mut ColrMapping, *mut ColrMapping) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ColrMapping) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut ColrMapping, ColrMapping) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut ColrMapping, ColrMapping) -> ()>,
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
    pub move_0: Option<unsafe extern "C" fn(*mut ColrTable, *mut ColrTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ColrTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut ColrTable, ColrTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut ColrTable, ColrTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut ColrTable>,
    pub free: Option<unsafe extern "C" fn(*mut ColrTable) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut ColrTable, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut ColrTable, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut ColrTable>,
    pub fill: Option<unsafe extern "C" fn(*mut ColrTable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut ColrTable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut ColrTable, ColrMapping) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut ColrTable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut ColrTable) -> ColrMapping>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut ColrTable, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut ColrTable,
            Option<unsafe extern "C" fn(*const ColrMapping, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
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
unsafe extern "C" fn initLayer(mut layer: *mut ColrLayer) {
    otfcc_Handle_init(&raw mut (*layer).glyph);
}
#[inline]
unsafe extern "C" fn copyLayer(mut dst: *mut ColrLayer, mut src: *const ColrLayer) {
    otfcc_Handle_copy(
        &raw mut (*dst).glyph,
        &raw const (*src).glyph,
    );
    (*dst).paletteIndex = (*src).paletteIndex;
}
#[inline]
unsafe extern "C" fn disposeLayer(mut layer: *mut ColrLayer) {
    otfcc_Handle_dispose(&raw mut (*layer).glyph);
}
pub static COLR_I_LAYER: ColrLayerElementInterface = {
    ColrLayerElementInterface {
        init: Some(colr_Layer_init as unsafe extern "C" fn(*mut ColrLayer) -> ()),
        copy: Some(
            colr_Layer_copy as unsafe extern "C" fn(*mut ColrLayer, *const ColrLayer) -> (),
        ),
        move_0: Some(
            colr_Layer_move as unsafe extern "C" fn(*mut ColrLayer, *mut ColrLayer) -> (),
        ),
        dispose: Some(colr_Layer_dispose as unsafe extern "C" fn(*mut ColrLayer) -> ()),
        replace: Some(
            colr_Layer_replace as unsafe extern "C" fn(*mut ColrLayer, ColrLayer) -> (),
        ),
        copyReplace: Some(
            colr_Layer_copyReplace as unsafe extern "C" fn(*mut ColrLayer, ColrLayer) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn colr_Layer_copyReplace(mut dst: *mut ColrLayer, src: ColrLayer) {
    colr_Layer_dispose(dst);
    colr_Layer_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn colr_Layer_dispose(mut x: *mut ColrLayer) {
    disposeLayer(x);
}
#[inline]
unsafe extern "C" fn colr_Layer_init(mut x: *mut ColrLayer) {
    initLayer(x);
}
#[inline]
unsafe extern "C" fn colr_Layer_copy(mut dst: *mut ColrLayer, mut src: *const ColrLayer) {
    copyLayer(dst, src);
}
#[inline]
unsafe extern "C" fn colr_Layer_move(mut dst: *mut ColrLayer, mut src: *mut ColrLayer) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ColrLayer>() as usize,
    );
    colr_Layer_init(src);
}
#[inline]
unsafe extern "C" fn colr_Layer_replace(mut dst: *mut ColrLayer, src: ColrLayer) {
    colr_Layer_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ColrLayer>() as usize,
    );
}
#[inline]
unsafe extern "C" fn colr_LayerList_growTo(arr: *mut ColrLayerList, target: usize) {
    cvec_grow_to(colr_LayerList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn colr_LayerList_sort(
    mut arr: *mut ColrLayerList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const ColrLayer, *const ColrLayer) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<ColrLayer>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const ColrLayer, *const ColrLayer) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn colr_LayerList_fill(mut arr: *mut ColrLayerList, mut n: usize) {
    while (*arr).length < n {
        let mut x: ColrLayer = ColrLayer {
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            paletteIndex: 0,
        };
        if COLR_I_LAYER.init.is_some() {
            COLR_I_LAYER.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<ColrLayer>() as usize,
            );
        }
        colr_LayerList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn colr_LayerList_push(arr: *mut ColrLayerList, elem: ColrLayer) {
    cvec_push(colr_LayerList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn colr_LayerList_grow(arr: *mut ColrLayerList) {
    cvec_grow(colr_LayerList_as_cvec(arr));
}
pub static COLR_I_LAYER_LIST: ColrLayerListVectorInterface = {
    ColrLayerListVectorInterface {
        init: Some(colr_LayerList_init as unsafe extern "C" fn(*mut ColrLayerList) -> ()),
        copy: Some(
            colr_LayerList_copy
                as unsafe extern "C" fn(*mut ColrLayerList, *const ColrLayerList) -> (),
        ),
        move_0: Some(
            colr_LayerList_move
                as unsafe extern "C" fn(*mut ColrLayerList, *mut ColrLayerList) -> (),
        ),
        dispose: Some(colr_LayerList_dispose as unsafe extern "C" fn(*mut ColrLayerList) -> ()),
        replace: Some(
            colr_LayerList_replace
                as unsafe extern "C" fn(*mut ColrLayerList, ColrLayerList) -> (),
        ),
        copyReplace: Some(
            colr_LayerList_copyReplace
                as unsafe extern "C" fn(*mut ColrLayerList, ColrLayerList) -> (),
        ),
        create: Some(colr_LayerList_create),
        free: Some(colr_LayerList_free as unsafe extern "C" fn(*mut ColrLayerList) -> ()),
        initN: Some(
            colr_LayerList_initN as unsafe extern "C" fn(*mut ColrLayerList, usize) -> (),
        ),
        initCapN: Some(
            colr_LayerList_initCapN as unsafe extern "C" fn(*mut ColrLayerList, usize) -> (),
        ),
        createN: Some(
            colr_LayerList_createN as unsafe extern "C" fn(usize) -> *mut ColrLayerList,
        ),
        fill: Some(colr_LayerList_fill as unsafe extern "C" fn(*mut ColrLayerList, usize) -> ()),
        clear: Some(colr_LayerList_dispose as unsafe extern "C" fn(*mut ColrLayerList) -> ()),
        push: Some(
            colr_LayerList_push as unsafe extern "C" fn(*mut ColrLayerList, ColrLayer) -> (),
        ),
        shrinkToFit: Some(
            colr_LayerList_shrinkToFit as unsafe extern "C" fn(*mut ColrLayerList) -> (),
        ),
        pop: Some(colr_LayerList_pop as unsafe extern "C" fn(*mut ColrLayerList) -> ColrLayer),
        disposeItem: Some(
            colr_LayerList_disposeItem as unsafe extern "C" fn(*mut ColrLayerList, usize) -> (),
        ),
        filterEnv: Some(
            colr_LayerList_filterEnv
                as unsafe extern "C" fn(
                    *mut ColrLayerList,
                    Option<
                        unsafe extern "C" fn(*const ColrLayer, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            colr_LayerList_sort
                as unsafe extern "C" fn(
                    *mut ColrLayerList,
                    Option<
                        unsafe extern "C" fn(
                            *const ColrLayer,
                            *const ColrLayer,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn colr_LayerList_pop(arr: *mut ColrLayerList) -> ColrLayer {
    cvec_pop(colr_LayerList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn colr_LayerList_copyReplace(mut dst: *mut ColrLayerList, src: ColrLayerList) {
    colr_LayerList_dispose(dst);
    colr_LayerList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn colr_LayerList_copy(
    mut dst: *mut ColrLayerList,
    mut src: *const ColrLayerList,
) {
    colr_LayerList_init(dst);
    colr_LayerList_growTo(dst, (*src).length);
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
unsafe extern "C" fn colr_LayerList_dispose(mut arr: *mut ColrLayerList) {
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
unsafe extern "C" fn colr_LayerList_replace(mut dst: *mut ColrLayerList, src: ColrLayerList) {
    colr_LayerList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ColrLayerList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn colr_LayerList_initCapN(mut arr: *mut ColrLayerList, mut n: usize) {
    colr_LayerList_init(arr);
    colr_LayerList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn colr_LayerList_growToN(arr: *mut ColrLayerList, target: usize) {
    cvec_grow_to_n(colr_LayerList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn colr_LayerList_initN(mut arr: *mut ColrLayerList, mut n: usize) {
    colr_LayerList_init(arr);
    colr_LayerList_growToN(arr, n);
    colr_LayerList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn colr_LayerList_free(mut x: *mut ColrLayerList) {
    if x.is_null() {
        return;
    }
    colr_LayerList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn colr_LayerList_createN(mut n: usize) -> *mut ColrLayerList {
    let mut t: *mut ColrLayerList =
        malloc(::core::mem::size_of::<ColrLayerList>() as usize) as *mut ColrLayerList;
    colr_LayerList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn colr_LayerList_create() -> *mut ColrLayerList {
    let mut x: *mut ColrLayerList =
        malloc(::core::mem::size_of::<ColrLayerList>() as usize) as *mut ColrLayerList;
    colr_LayerList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn colr_LayerList_shrinkToFit(mut arr: *mut ColrLayerList) {
    colr_LayerList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn colr_LayerList_resizeTo(arr: *mut ColrLayerList, target: usize) {
    cvec_resize_to(colr_LayerList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn colr_LayerList_move(dst: *mut ColrLayerList, src: *mut ColrLayerList) {
    cvec_move(colr_LayerList_as_cvec(dst), colr_LayerList_as_cvec(src));
}
#[inline]
unsafe fn colr_LayerList_as_cvec(arr: *mut ColrLayerList) -> *mut CVecRaw<ColrLayer> {
    arr as *mut CVecRaw<ColrLayer>
}
#[inline]
unsafe extern "C" fn colr_LayerList_init(arr: *mut ColrLayerList) {
    cvec_init(colr_LayerList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn colr_LayerList_disposeItem(mut arr: *mut ColrLayerList, mut n: usize) {
    if COLR_I_LAYER.dispose.is_some() {
        COLR_I_LAYER.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut ColrLayer
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn colr_LayerList_filterEnv(
    mut arr: *mut ColrLayerList,
    mut fn_0: Option<unsafe extern "C" fn(*const ColrLayer, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut ColrLayer,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if COLR_I_LAYER.dispose.is_some() {
                COLR_I_LAYER.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut ColrLayer,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn initMapping(mut mapping: *mut ColrMapping) {
    otfcc_Handle_init(&raw mut (*mapping).glyph);
    COLR_I_LAYER_LIST.init.expect("non-null function pointer")(&raw mut (*mapping).layers);
}
#[inline]
unsafe extern "C" fn copyMapping(mut dst: *mut ColrMapping, mut src: *const ColrMapping) {
    otfcc_Handle_copy(
        &raw mut (*dst).glyph,
        &raw const (*src).glyph,
    );
    COLR_I_LAYER_LIST.copy.expect("non-null function pointer")(
        &raw mut (*dst).layers,
        &raw const (*src).layers,
    );
}
#[inline]
unsafe extern "C" fn disposeMapping(mut mapping: *mut ColrMapping) {
    otfcc_Handle_dispose(&raw mut (*mapping).glyph);
    COLR_I_LAYER_LIST.dispose.expect("non-null function pointer")(&raw mut (*mapping).layers);
}
#[inline]
unsafe extern "C" fn colr_Mapping_dispose(mut x: *mut ColrMapping) {
    disposeMapping(x);
}
#[inline]
unsafe extern "C" fn colr_Mapping_copyReplace(mut dst: *mut ColrMapping, src: ColrMapping) {
    colr_Mapping_dispose(dst);
    colr_Mapping_copy(dst, &raw const src);
}
pub static COLR_I_MAPPING: ColrMappingElementInterface = {
    ColrMappingElementInterface {
        init: Some(colr_Mapping_init as unsafe extern "C" fn(*mut ColrMapping) -> ()),
        copy: Some(
            colr_Mapping_copy as unsafe extern "C" fn(*mut ColrMapping, *const ColrMapping) -> (),
        ),
        move_0: Some(
            colr_Mapping_move as unsafe extern "C" fn(*mut ColrMapping, *mut ColrMapping) -> (),
        ),
        dispose: Some(colr_Mapping_dispose as unsafe extern "C" fn(*mut ColrMapping) -> ()),
        replace: Some(
            colr_Mapping_replace as unsafe extern "C" fn(*mut ColrMapping, ColrMapping) -> (),
        ),
        copyReplace: Some(
            colr_Mapping_copyReplace as unsafe extern "C" fn(*mut ColrMapping, ColrMapping) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn colr_Mapping_move(mut dst: *mut ColrMapping, mut src: *mut ColrMapping) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ColrMapping>() as usize,
    );
    colr_Mapping_init(src);
}
#[inline]
unsafe extern "C" fn colr_Mapping_init(mut x: *mut ColrMapping) {
    initMapping(x);
}
#[inline]
unsafe extern "C" fn colr_Mapping_replace(mut dst: *mut ColrMapping, src: ColrMapping) {
    colr_Mapping_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ColrMapping>() as usize,
    );
}
#[inline]
unsafe extern "C" fn colr_Mapping_copy(mut dst: *mut ColrMapping, mut src: *const ColrMapping) {
    copyMapping(dst, src);
}
#[inline]
unsafe extern "C" fn table_COLR_replace(mut dst: *mut ColrTable, src: ColrTable) {
    table_COLR_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ColrTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_COLR_growTo(arr: *mut ColrTable, target: usize) {
    cvec_grow_to(table_COLR_as_cvec(arr), target);
}
pub static TABLE_I_COLR: ColrTableVectorInterface = {
    ColrTableVectorInterface {
        init: Some(table_COLR_init as unsafe extern "C" fn(*mut ColrTable) -> ()),
        copy: Some(
            table_COLR_copy as unsafe extern "C" fn(*mut ColrTable, *const ColrTable) -> (),
        ),
        move_0: Some(
            table_COLR_move as unsafe extern "C" fn(*mut ColrTable, *mut ColrTable) -> (),
        ),
        dispose: Some(table_COLR_dispose as unsafe extern "C" fn(*mut ColrTable) -> ()),
        replace: Some(
            table_COLR_replace as unsafe extern "C" fn(*mut ColrTable, ColrTable) -> (),
        ),
        copyReplace: Some(
            table_COLR_copyReplace as unsafe extern "C" fn(*mut ColrTable, ColrTable) -> (),
        ),
        create: Some(table_COLR_create),
        free: Some(table_COLR_free as unsafe extern "C" fn(*mut ColrTable) -> ()),
        initN: Some(table_COLR_initN as unsafe extern "C" fn(*mut ColrTable, usize) -> ()),
        initCapN: Some(table_COLR_initCapN as unsafe extern "C" fn(*mut ColrTable, usize) -> ()),
        createN: Some(table_COLR_createN as unsafe extern "C" fn(usize) -> *mut ColrTable),
        fill: Some(table_COLR_fill as unsafe extern "C" fn(*mut ColrTable, usize) -> ()),
        clear: Some(table_COLR_dispose as unsafe extern "C" fn(*mut ColrTable) -> ()),
        push: Some(table_COLR_push as unsafe extern "C" fn(*mut ColrTable, ColrMapping) -> ()),
        shrinkToFit: Some(table_COLR_shrinkToFit as unsafe extern "C" fn(*mut ColrTable) -> ()),
        pop: Some(table_COLR_pop as unsafe extern "C" fn(*mut ColrTable) -> ColrMapping),
        disposeItem: Some(
            table_COLR_disposeItem as unsafe extern "C" fn(*mut ColrTable, usize) -> (),
        ),
        filterEnv: Some(
            table_COLR_filterEnv
                as unsafe extern "C" fn(
                    *mut ColrTable,
                    Option<
                        unsafe extern "C" fn(*const ColrMapping, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            table_COLR_sort
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
unsafe extern "C" fn table_COLR_shrinkToFit(mut arr: *mut ColrTable) {
    table_COLR_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn table_COLR_resizeTo(arr: *mut ColrTable, target: usize) {
    cvec_resize_to(table_COLR_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_COLR_move(dst: *mut ColrTable, src: *mut ColrTable) {
    cvec_move(table_COLR_as_cvec(dst), table_COLR_as_cvec(src));
}
#[inline]
unsafe fn table_COLR_as_cvec(arr: *mut ColrTable) -> *mut CVecRaw<ColrMapping> {
    arr as *mut CVecRaw<ColrMapping>
}
#[inline]
unsafe extern "C" fn table_COLR_init(arr: *mut ColrTable) {
    cvec_init(table_COLR_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_COLR_filterEnv(
    mut arr: *mut ColrTable,
    mut fn_0: Option<unsafe extern "C" fn(*const ColrMapping, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut ColrMapping,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if COLR_I_MAPPING.dispose.is_some() {
                COLR_I_MAPPING.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut ColrMapping,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn table_COLR_disposeItem(mut arr: *mut ColrTable, mut n: usize) {
    if COLR_I_MAPPING.dispose.is_some() {
        COLR_I_MAPPING.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut ColrMapping
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn table_COLR_sort(
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
unsafe extern "C" fn table_COLR_fill(mut arr: *mut ColrTable, mut n: usize) {
    while (*arr).length < n {
        let mut x: ColrMapping = ColrMapping {
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
        if COLR_I_MAPPING.init.is_some() {
            COLR_I_MAPPING.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<ColrMapping>() as usize,
            );
        }
        table_COLR_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn table_COLR_push(arr: *mut ColrTable, elem: ColrMapping) {
    cvec_push(table_COLR_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_COLR_grow(arr: *mut ColrTable) {
    cvec_grow(table_COLR_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_COLR_pop(arr: *mut ColrTable) -> ColrMapping {
    cvec_pop(table_COLR_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn table_COLR_copyReplace(mut dst: *mut ColrTable, src: ColrTable) {
    table_COLR_dispose(dst);
    table_COLR_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_COLR_copy(mut dst: *mut ColrTable, mut src: *const ColrTable) {
    table_COLR_init(dst);
    table_COLR_growTo(dst, (*src).length);
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
unsafe extern "C" fn table_COLR_dispose(mut arr: *mut ColrTable) {
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
unsafe extern "C" fn table_COLR_initCapN(mut arr: *mut ColrTable, mut n: usize) {
    table_COLR_init(arr);
    table_COLR_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn table_COLR_growToN(arr: *mut ColrTable, target: usize) {
    cvec_grow_to_n(table_COLR_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_COLR_initN(mut arr: *mut ColrTable, mut n: usize) {
    table_COLR_init(arr);
    table_COLR_growToN(arr, n);
    table_COLR_fill(arr, n);
}
#[inline]
unsafe extern "C" fn table_COLR_free(mut x: *mut ColrTable) {
    if x.is_null() {
        return;
    }
    table_COLR_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_COLR_createN(mut n: usize) -> *mut ColrTable {
    let mut t: *mut ColrTable =
        malloc(::core::mem::size_of::<ColrTable>() as usize) as *mut ColrTable;
    table_COLR_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn table_COLR_create() -> *mut ColrTable {
    let mut x: *mut ColrTable =
        malloc(::core::mem::size_of::<ColrTable>() as usize) as *mut ColrTable;
    table_COLR_init(x);
    return x;
}
static BASE_GLYPH_REC_LENGTH: usize = 6 as usize;
static LAYER_REC_LENGTH: usize = 4 as usize;
pub unsafe extern "C" fn otfcc_readCOLR(
    packet: Packet,
    mut options: *const Options,
) -> *mut ColrTable {
    let mut numBaseGlyphRecords: u16 = 0;
    let mut numLayerRecords: u16 = 0;
    let mut offsetBaseGlyphRecord: u32 = 0;
    let mut offsetLayerRecord: u32 = 0;
    let mut gids: *mut GlyphId = ::core::ptr::null_mut::<GlyphId>();
    let mut colors: *mut ColorId = ::core::ptr::null_mut::<ColorId>();
    let mut colr: *mut ColrTable = ::core::ptr::null_mut::<ColrTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1129270354i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    if !(table.length < 14 as u32) {
                        numBaseGlyphRecords =
                            read_16u(table.data.offset(2 as ::core::ffi::c_int as isize));
                        numLayerRecords =
                            read_16u(table.data.offset(12 as ::core::ffi::c_int as isize));
                        offsetBaseGlyphRecord =
                            read_32u(table.data.offset(4 as ::core::ffi::c_int as isize));
                        offsetLayerRecord =
                            read_32u(table.data.offset(8 as ::core::ffi::c_int as isize));
                        if !((table.length as usize)
                            < (offsetBaseGlyphRecord as usize).wrapping_add(
                                BASE_GLYPH_REC_LENGTH.wrapping_mul(numBaseGlyphRecords as usize),
                            ))
                        {
                            if !((table.length as usize)
                                < (offsetLayerRecord as usize).wrapping_add(
                                    LAYER_REC_LENGTH.wrapping_mul(numLayerRecords as usize),
                                ))
                            {
                                gids = ::core::ptr::null_mut::<GlyphId>();
                                colors = ::core::ptr::null_mut::<ColorId>();
                                gids = __caryll_allocate_clean(
                                    (::core::mem::size_of::<GlyphId>() as usize)
                                        .wrapping_mul(numLayerRecords as usize),
                                    52 as ::core::ffi::c_ulong,
                                ) as *mut GlyphId;
                                colors = __caryll_allocate_clean(
                                    (::core::mem::size_of::<ColorId>() as usize)
                                        .wrapping_mul(numLayerRecords as usize),
                                    53 as ::core::ffi::c_ulong,
                                ) as *mut ColorId;
                                let mut j: GlyphId = 0 as GlyphId;
                                while (j as ::core::ffi::c_int)
                                    < numLayerRecords as ::core::ffi::c_int
                                {
                                    *gids.offset(j as isize) = read_16u(
                                        table.data.offset(offsetLayerRecord as isize).offset(
                                            LAYER_REC_LENGTH.wrapping_mul(j as usize) as isize,
                                        ),
                                    )
                                        as GlyphId;
                                    *colors.offset(j as isize) =
                                        read_16u(
                                            table
                                                .data
                                                .offset(offsetLayerRecord as isize)
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
                                    < numBaseGlyphRecords as ::core::ffi::c_int
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
                                            .offset(offsetBaseGlyphRecord as isize)
                                            .offset(BASE_GLYPH_REC_LENGTH.wrapping_mul(j_0 as usize)
                                                as isize),
                                    );
                                    let mut firstLayerIndex: u16 = read_16u(
                                        table
                                            .data
                                            .offset(offsetBaseGlyphRecord as isize)
                                            .offset(BASE_GLYPH_REC_LENGTH.wrapping_mul(j_0 as usize)
                                                as isize)
                                            .offset(2 as ::core::ffi::c_int as isize),
                                    );
                                    let mut numLayers: u16 = read_16u(
                                        table
                                            .data
                                            .offset(offsetBaseGlyphRecord as isize)
                                            .offset(BASE_GLYPH_REC_LENGTH.wrapping_mul(j_0 as usize)
                                                as isize)
                                            .offset(4 as ::core::ffi::c_int as isize),
                                    );
                                    let mut baseGlyph: GlyphHandle = handle_fromIndex(
                                        gid as GlyphId
                                    )
                                        as GlyphHandle;
                                    otfcc_Handle_move(
                                        &raw mut mapping.glyph,
                                        &raw mut baseGlyph,
                                    );
                                    let mut k: GlyphId = 0 as GlyphId;
                                    while (k as ::core::ffi::c_int)
                                        < numLayers as ::core::ffi::c_int
                                    {
                                        if (k as ::core::ffi::c_int
                                            + firstLayerIndex as ::core::ffi::c_int)
                                            < numLayerRecords as ::core::ffi::c_int
                                        {
                                            COLR_I_LAYER_LIST
                                                .push
                                                .expect("non-null function pointer")(
                                                &raw mut mapping.layers,
                                                ColrLayer {
                                                    glyph: handle_fromIndex(
                                                        *gids.offset(
                                                            (k as ::core::ffi::c_int
                                                                + firstLayerIndex
                                                                    as ::core::ffi::c_int)
                                                                as isize,
                                                        ),
                                                    )
                                                        as GlyphHandle,
                                                    paletteIndex: *colors.offset(
                                                        (k as ::core::ffi::c_int
                                                            + firstLayerIndex as ::core::ffi::c_int)
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
                        .logSDS
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
pub unsafe extern "C" fn otfcc_dumpCOLR(
    mut colr: *const ColrTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if colr.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
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
                            json_integer_new((*layer).paletteIndex as i64),
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
pub unsafe extern "C" fn otfcc_parseCOLR(
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
        .startSDS
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
                    m.glyph = handle_fromName(sdsnewlen(
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
                                        glyph: handle_fromName(
                                            sdsnewlen(
                                                (*_layerglyph).u.string.ptr
                                                    as *const ::core::ffi::c_void,
                                                (*_layerglyph).u.string.length as usize,
                                            ),
                                        )
                                            as GlyphHandle,
                                        paletteIndex: json_obj_getint_fallback(
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
unsafe extern "C" fn byGID(
    mut a: *const ColrMapping,
    mut b: *const ColrMapping,
) -> ::core::ffi::c_int {
    return (*a).glyph.index as ::core::ffi::c_int - (*b).glyph.index as ::core::ffi::c_int;
}
pub unsafe extern "C" fn otfcc_buildCOLR(
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
            byGID
                as unsafe extern "C" fn(
                    *const ColrMapping,
                    *const ColrMapping,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut currentLayerIndex: GlyphId = 0 as GlyphId;
    let mut layerRecords: *mut BkBlock = bk_new_Block(&[]);
    let mut baseRecords: *mut BkBlock = bk_new_Block(&[]);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < colr.length {
        let mut mapping: *mut ColrMapping = colr.items.offset(__caryll_index as isize);
        while keep != 0 {
            bk_push(baseRecords, &[bk_int(BkCellType::B16, ((*mapping).glyph.index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (currentLayerIndex as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*mapping).layers.length) as u32)]);
            let mut __caryll_index_0: usize = 0 as usize;
            let mut keep_0: usize = 1 as usize;
            while keep_0 != 0 && __caryll_index_0 < (*mapping).layers.length {
                let mut layer: *mut ColrLayer =
                    (*mapping).layers.items.offset(__caryll_index_0 as isize);
                while keep_0 != 0 {
                    bk_push(layerRecords, &[bk_int(BkCellType::B16, ((*layer).glyph.index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*layer).paletteIndex as ::core::ffi::c_int) as u32)]);
                    currentLayerIndex = (currentLayerIndex as ::core::ffi::c_int
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
    let mut root: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, 0 as u32), bk_int(BkCellType::B16, (colr.length) as u32), bk_ptr(BkCellType::P32, baseRecords), bk_ptr(BkCellType::P32, layerRecords), bk_int(BkCellType::B16, (currentLayerIndex as ::core::ffi::c_int) as u32)]);
    TABLE_I_COLR.dispose.expect("non-null function pointer")(&raw mut colr);
    return bk_build_Block(root);
}
