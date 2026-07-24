use libc::{free, malloc, memcpy, memset, qsort, strcmp};
extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new(_: *const ::core::ffi::c_char) -> *mut json_value;
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}

use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_copy, otfcc_Handle_dispose, otfcc_Handle_init, otfcc_Handle_move, otfcc_Handle, otfcc_GlyphHandle, HANDLE_STATE_EMPTY};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32u};
use crate::support::cvec::{
    cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push,
    cvec_resize_to, CVecRaw,
};
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type json_type = ::core::ffi::c_uint;
pub const json_pre_serialized: json_type = 8;
pub const json_null: json_type = 7;
pub const json_boolean: json_type = 6;
pub const json_string: json_type = 5;
pub const json_double: json_type = 4;
pub const json_integer: json_type = 3;
pub const json_array: json_type = 2;
pub const json_object: json_type = 1;
pub const json_none: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_value {
    pub parent: *mut _json_value,
    pub type_0: json_type,
    pub u: C2RustUnnamed_0,
    pub _reserved: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub next_alloc: *mut _json_value,
    pub object_mem: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub boolean: ::core::ffi::c_int,
    pub integer: i64,
    pub dbl: ::core::ffi::c_double,
    pub string: C2RustUnnamed_3,
    pub object: C2RustUnnamed_2,
    pub array: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut json_object_entry,
}
pub type json_object_entry = _json_object_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_object_entry {
    pub name: *mut ::core::ffi::c_char,
    pub name_length: ::core::ffi::c_uint,
    pub value: *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub length: ::core::ffi::c_uint,
    pub ptr: *mut ::core::ffi::c_char,
}
pub type json_value = _json_value;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: usize,
    pub size: usize,
    pub free: usize,
    pub data: *mut u8,
}
pub type glyphid_t = u16;
pub type colorid_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILoggerTarget {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()>,
}
pub type otfcc_LoggerType = ::core::ffi::c_uint;
pub const log_type_progress: otfcc_LoggerType = 3;
pub const log_type_info: otfcc_LoggerType = 2;
pub const log_type_warning: otfcc_LoggerType = 1;
pub const log_type_error: otfcc_LoggerType = 0;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed_4 = 10;
pub const log_vl_info: C2RustUnnamed_4 = 5;
pub const log_vl_notice: C2RustUnnamed_4 = 2;
pub const log_vl_important: C2RustUnnamed_4 = 1;
pub const log_vl_critical: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILogger {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub indent: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub indentSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub start: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub startSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub log: Option<
        unsafe extern "C" fn(
            *mut otfcc_ILogger,
            u8,
            otfcc_LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub logSDS:
        Option<unsafe extern "C" fn(*mut otfcc_ILogger, u8, otfcc_LoggerType, sds) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub setVerbosity: Option<unsafe extern "C" fn(*mut otfcc_ILogger, u8) -> ()>,
    pub getTarget: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> *mut otfcc_ILoggerTarget>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Options {
    pub debug_wait_on_start: bool,
    pub ignore_glyph_order: bool,
    pub ignore_hints: bool,
    pub has_vertical_metrics: bool,
    pub export_fdselect: bool,
    pub keep_average_char_width: bool,
    pub keep_unicode_ranges: bool,
    pub short_post: bool,
    pub dummy_DSIG: bool,
    pub keep_modified_time: bool,
    pub instr_as_bytes: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub cff_short_vmtx: bool,
    pub merge_lookups: bool,
    pub merge_features: bool,
    pub force_cid: bool,
    pub cff_rollCharString: bool,
    pub cff_doSubroutinize: bool,
    pub stub_cmap4: bool,
    pub decimal_cmap: bool,
    pub name_glyphs_by_hash: bool,
    pub name_glyphs_by_gid: bool,
    pub glyph_name_prefix: *mut ::core::ffi::c_char,
    pub logger: *mut otfcc_ILogger,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_PacketPiece {
    pub tag: u32,
    pub checkSum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Packet {
    pub sfnt_version: u32,
    pub numTables: u16,
    pub searchRange: u16,
    pub entrySelector: u16,
    pub rangeShift: u16,
    pub pieces: *mut otfcc_PacketPiece,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colr_Layer {
    pub glyph: otfcc_GlyphHandle,
    pub paletteIndex: colorid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_colr_Layer {
    pub init: Option<unsafe extern "C" fn(*mut colr_Layer) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut colr_Layer, *const colr_Layer) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut colr_Layer, *mut colr_Layer) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut colr_Layer) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut colr_Layer, colr_Layer) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut colr_Layer, colr_Layer) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colr_LayerList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut colr_Layer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_colr_LayerList {
    pub init: Option<unsafe extern "C" fn(*mut colr_LayerList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut colr_LayerList, *const colr_LayerList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut colr_LayerList, *mut colr_LayerList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut colr_LayerList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut colr_LayerList, colr_LayerList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut colr_LayerList, colr_LayerList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut colr_LayerList>,
    pub free: Option<unsafe extern "C" fn(*mut colr_LayerList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut colr_LayerList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut colr_LayerList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut colr_LayerList>,
    pub fill: Option<unsafe extern "C" fn(*mut colr_LayerList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut colr_LayerList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut colr_LayerList, colr_Layer) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut colr_LayerList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut colr_LayerList) -> colr_Layer>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut colr_LayerList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut colr_LayerList,
            Option<unsafe extern "C" fn(*const colr_Layer, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut colr_LayerList,
            Option<unsafe extern "C" fn(*const colr_Layer, *const colr_Layer) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colr_Mapping {
    pub glyph: otfcc_GlyphHandle,
    pub layers: colr_LayerList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_colr_Mapping {
    pub init: Option<unsafe extern "C" fn(*mut colr_Mapping) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut colr_Mapping, *const colr_Mapping) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut colr_Mapping, *mut colr_Mapping) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut colr_Mapping) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut colr_Mapping, colr_Mapping) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut colr_Mapping, colr_Mapping) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_COLR {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut colr_Mapping,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_table_COLR {
    pub init: Option<unsafe extern "C" fn(*mut table_COLR) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_COLR, *const table_COLR) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_COLR, *mut table_COLR) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_COLR) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_COLR, table_COLR) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_COLR, table_COLR) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_COLR>,
    pub free: Option<unsafe extern "C" fn(*mut table_COLR) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut table_COLR, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut table_COLR, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut table_COLR>,
    pub fill: Option<unsafe extern "C" fn(*mut table_COLR, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut table_COLR) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut table_COLR, colr_Mapping) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut table_COLR) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut table_COLR) -> colr_Mapping>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut table_COLR, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut table_COLR,
            Option<unsafe extern "C" fn(*const colr_Mapping, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut table_COLR,
            Option<
                unsafe extern "C" fn(
                    *const colr_Mapping,
                    *const colr_Mapping,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
pub type glyph_handle = otfcc_GlyphHandle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_serialize_opts {
    pub mode: ::core::ffi::c_int,
    pub opts: ::core::ffi::c_int,
    pub indent_size: ::core::ffi::c_int,
}
pub type bk_Block = __caryll_bkblock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_bkblock {
    pub _visitstate: bk_cell_visit_state,
    pub _index: u32,
    pub _height: u32,
    pub _depth: u32,
    pub length: u32,
    pub free: u32,
    pub cells: *mut bk_Cell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bk_Cell {
    pub t: bk_CellType,
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub z: u32,
    pub p: *mut __caryll_bkblock,
}
pub type bk_CellType = ::core::ffi::c_uint;
pub const bkembed: bk_CellType = 255;
pub const bkcopy: bk_CellType = 254;
pub const sp32: bk_CellType = 129;
pub const sp16: bk_CellType = 128;
pub const p32: bk_CellType = 17;
pub const p16: bk_CellType = 16;
pub const b32: bk_CellType = 3;
pub const b16: bk_CellType = 2;
pub const b8: bk_CellType = 1;
pub const bkover: bk_CellType = 0;
pub type bk_cell_visit_state = ::core::ffi::c_uint;
pub const VISIT_BLACK: bk_cell_visit_state = 2;
pub const VISIT_GRAY: bk_cell_visit_state = 1;
pub const VISIT_WHITE: bk_cell_visit_state = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn initLayer(mut layer: *mut colr_Layer) {
    otfcc_Handle_init(&raw mut (*layer).glyph);
}
#[inline]
unsafe extern "C" fn copyLayer(mut dst: *mut colr_Layer, mut src: *const colr_Layer) {
    otfcc_Handle_copy(
        &raw mut (*dst).glyph,
        &raw const (*src).glyph,
    );
    (*dst).paletteIndex = (*src).paletteIndex;
}
#[inline]
unsafe extern "C" fn disposeLayer(mut layer: *mut colr_Layer) {
    otfcc_Handle_dispose(&raw mut (*layer).glyph);
}
#[no_mangle]
pub static mut colr_iLayer: __caryll_elementinterface_colr_Layer = {
    __caryll_elementinterface_colr_Layer {
        init: Some(colr_Layer_init as unsafe extern "C" fn(*mut colr_Layer) -> ()),
        copy: Some(
            colr_Layer_copy as unsafe extern "C" fn(*mut colr_Layer, *const colr_Layer) -> (),
        ),
        move_0: Some(
            colr_Layer_move as unsafe extern "C" fn(*mut colr_Layer, *mut colr_Layer) -> (),
        ),
        dispose: Some(colr_Layer_dispose as unsafe extern "C" fn(*mut colr_Layer) -> ()),
        replace: Some(
            colr_Layer_replace as unsafe extern "C" fn(*mut colr_Layer, colr_Layer) -> (),
        ),
        copyReplace: Some(
            colr_Layer_copyReplace as unsafe extern "C" fn(*mut colr_Layer, colr_Layer) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn colr_Layer_copyReplace(mut dst: *mut colr_Layer, src: colr_Layer) {
    colr_Layer_dispose(dst);
    colr_Layer_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn colr_Layer_dispose(mut x: *mut colr_Layer) {
    disposeLayer(x);
}
#[inline]
unsafe extern "C" fn colr_Layer_init(mut x: *mut colr_Layer) {
    initLayer(x);
}
#[inline]
unsafe extern "C" fn colr_Layer_copy(mut dst: *mut colr_Layer, mut src: *const colr_Layer) {
    copyLayer(dst, src);
}
#[inline]
unsafe extern "C" fn colr_Layer_move(mut dst: *mut colr_Layer, mut src: *mut colr_Layer) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<colr_Layer>() as usize,
    );
    colr_Layer_init(src);
}
#[inline]
unsafe extern "C" fn colr_Layer_replace(mut dst: *mut colr_Layer, src: colr_Layer) {
    colr_Layer_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<colr_Layer>() as usize,
    );
}
#[inline]
unsafe extern "C" fn colr_LayerList_growTo(arr: *mut colr_LayerList, target: usize) {
    cvec_grow_to(colr_LayerList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn colr_LayerList_sort(
    mut arr: *mut colr_LayerList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const colr_Layer, *const colr_Layer) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<colr_Layer>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const colr_Layer, *const colr_Layer) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn colr_LayerList_fill(mut arr: *mut colr_LayerList, mut n: usize) {
    while (*arr).length < n {
        let mut x: colr_Layer = colr_Layer {
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            paletteIndex: 0,
        };
        if colr_iLayer.init.is_some() {
            colr_iLayer.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<colr_Layer>() as usize,
            );
        }
        colr_LayerList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn colr_LayerList_push(arr: *mut colr_LayerList, elem: colr_Layer) {
    cvec_push(colr_LayerList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn colr_LayerList_grow(arr: *mut colr_LayerList) {
    cvec_grow(colr_LayerList_as_cvec(arr));
}
#[no_mangle]
pub static mut colr_iLayerList: __caryll_vectorinterface_colr_LayerList = {
    __caryll_vectorinterface_colr_LayerList {
        init: Some(colr_LayerList_init as unsafe extern "C" fn(*mut colr_LayerList) -> ()),
        copy: Some(
            colr_LayerList_copy
                as unsafe extern "C" fn(*mut colr_LayerList, *const colr_LayerList) -> (),
        ),
        move_0: Some(
            colr_LayerList_move
                as unsafe extern "C" fn(*mut colr_LayerList, *mut colr_LayerList) -> (),
        ),
        dispose: Some(colr_LayerList_dispose as unsafe extern "C" fn(*mut colr_LayerList) -> ()),
        replace: Some(
            colr_LayerList_replace
                as unsafe extern "C" fn(*mut colr_LayerList, colr_LayerList) -> (),
        ),
        copyReplace: Some(
            colr_LayerList_copyReplace
                as unsafe extern "C" fn(*mut colr_LayerList, colr_LayerList) -> (),
        ),
        create: Some(colr_LayerList_create),
        free: Some(colr_LayerList_free as unsafe extern "C" fn(*mut colr_LayerList) -> ()),
        initN: Some(
            colr_LayerList_initN as unsafe extern "C" fn(*mut colr_LayerList, usize) -> (),
        ),
        initCapN: Some(
            colr_LayerList_initCapN as unsafe extern "C" fn(*mut colr_LayerList, usize) -> (),
        ),
        createN: Some(
            colr_LayerList_createN as unsafe extern "C" fn(usize) -> *mut colr_LayerList,
        ),
        fill: Some(colr_LayerList_fill as unsafe extern "C" fn(*mut colr_LayerList, usize) -> ()),
        clear: Some(colr_LayerList_dispose as unsafe extern "C" fn(*mut colr_LayerList) -> ()),
        push: Some(
            colr_LayerList_push as unsafe extern "C" fn(*mut colr_LayerList, colr_Layer) -> (),
        ),
        shrinkToFit: Some(
            colr_LayerList_shrinkToFit as unsafe extern "C" fn(*mut colr_LayerList) -> (),
        ),
        pop: Some(colr_LayerList_pop as unsafe extern "C" fn(*mut colr_LayerList) -> colr_Layer),
        disposeItem: Some(
            colr_LayerList_disposeItem as unsafe extern "C" fn(*mut colr_LayerList, usize) -> (),
        ),
        filterEnv: Some(
            colr_LayerList_filterEnv
                as unsafe extern "C" fn(
                    *mut colr_LayerList,
                    Option<
                        unsafe extern "C" fn(*const colr_Layer, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            colr_LayerList_sort
                as unsafe extern "C" fn(
                    *mut colr_LayerList,
                    Option<
                        unsafe extern "C" fn(
                            *const colr_Layer,
                            *const colr_Layer,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn colr_LayerList_pop(arr: *mut colr_LayerList) -> colr_Layer {
    cvec_pop(colr_LayerList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn colr_LayerList_copyReplace(mut dst: *mut colr_LayerList, src: colr_LayerList) {
    colr_LayerList_dispose(dst);
    colr_LayerList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn colr_LayerList_copy(
    mut dst: *mut colr_LayerList,
    mut src: *const colr_LayerList,
) {
    colr_LayerList_init(dst);
    colr_LayerList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if colr_iLayer.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            colr_iLayer.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut colr_Layer,
                (*src).items.offset(j as isize) as *mut colr_Layer as *const colr_Layer,
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
unsafe extern "C" fn colr_LayerList_dispose(mut arr: *mut colr_LayerList) {
    if arr.is_null() {
        return;
    }
    if colr_iLayer.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            colr_iLayer.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut colr_Layer,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<colr_Layer>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn colr_LayerList_replace(mut dst: *mut colr_LayerList, src: colr_LayerList) {
    colr_LayerList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<colr_LayerList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn colr_LayerList_initCapN(mut arr: *mut colr_LayerList, mut n: usize) {
    colr_LayerList_init(arr);
    colr_LayerList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn colr_LayerList_growToN(arr: *mut colr_LayerList, target: usize) {
    cvec_grow_to_n(colr_LayerList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn colr_LayerList_initN(mut arr: *mut colr_LayerList, mut n: usize) {
    colr_LayerList_init(arr);
    colr_LayerList_growToN(arr, n);
    colr_LayerList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn colr_LayerList_free(mut x: *mut colr_LayerList) {
    if x.is_null() {
        return;
    }
    colr_LayerList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn colr_LayerList_createN(mut n: usize) -> *mut colr_LayerList {
    let mut t: *mut colr_LayerList =
        malloc(::core::mem::size_of::<colr_LayerList>() as usize) as *mut colr_LayerList;
    colr_LayerList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn colr_LayerList_create() -> *mut colr_LayerList {
    let mut x: *mut colr_LayerList =
        malloc(::core::mem::size_of::<colr_LayerList>() as usize) as *mut colr_LayerList;
    colr_LayerList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn colr_LayerList_shrinkToFit(mut arr: *mut colr_LayerList) {
    colr_LayerList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn colr_LayerList_resizeTo(arr: *mut colr_LayerList, target: usize) {
    cvec_resize_to(colr_LayerList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn colr_LayerList_move(dst: *mut colr_LayerList, src: *mut colr_LayerList) {
    cvec_move(colr_LayerList_as_cvec(dst), colr_LayerList_as_cvec(src));
}
#[inline]
unsafe fn colr_LayerList_as_cvec(arr: *mut colr_LayerList) -> *mut CVecRaw<colr_Layer> {
    arr as *mut CVecRaw<colr_Layer>
}
#[inline]
unsafe extern "C" fn colr_LayerList_init(arr: *mut colr_LayerList) {
    cvec_init(colr_LayerList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn colr_LayerList_disposeItem(mut arr: *mut colr_LayerList, mut n: usize) {
    if colr_iLayer.dispose.is_some() {
        colr_iLayer.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut colr_Layer
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn colr_LayerList_filterEnv(
    mut arr: *mut colr_LayerList,
    mut fn_0: Option<unsafe extern "C" fn(*const colr_Layer, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut colr_Layer,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if colr_iLayer.dispose.is_some() {
                colr_iLayer.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut colr_Layer,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn initMapping(mut mapping: *mut colr_Mapping) {
    otfcc_Handle_init(&raw mut (*mapping).glyph);
    colr_iLayerList.init.expect("non-null function pointer")(&raw mut (*mapping).layers);
}
#[inline]
unsafe extern "C" fn copyMapping(mut dst: *mut colr_Mapping, mut src: *const colr_Mapping) {
    otfcc_Handle_copy(
        &raw mut (*dst).glyph,
        &raw const (*src).glyph,
    );
    colr_iLayerList.copy.expect("non-null function pointer")(
        &raw mut (*dst).layers,
        &raw const (*src).layers,
    );
}
#[inline]
unsafe extern "C" fn disposeMapping(mut mapping: *mut colr_Mapping) {
    otfcc_Handle_dispose(&raw mut (*mapping).glyph);
    colr_iLayerList.dispose.expect("non-null function pointer")(&raw mut (*mapping).layers);
}
#[inline]
unsafe extern "C" fn colr_Mapping_dispose(mut x: *mut colr_Mapping) {
    disposeMapping(x);
}
#[inline]
unsafe extern "C" fn colr_Mapping_copyReplace(mut dst: *mut colr_Mapping, src: colr_Mapping) {
    colr_Mapping_dispose(dst);
    colr_Mapping_copy(dst, &raw const src);
}
#[no_mangle]
pub static mut colr_iMapping: __caryll_elementinterface_colr_Mapping = {
    __caryll_elementinterface_colr_Mapping {
        init: Some(colr_Mapping_init as unsafe extern "C" fn(*mut colr_Mapping) -> ()),
        copy: Some(
            colr_Mapping_copy as unsafe extern "C" fn(*mut colr_Mapping, *const colr_Mapping) -> (),
        ),
        move_0: Some(
            colr_Mapping_move as unsafe extern "C" fn(*mut colr_Mapping, *mut colr_Mapping) -> (),
        ),
        dispose: Some(colr_Mapping_dispose as unsafe extern "C" fn(*mut colr_Mapping) -> ()),
        replace: Some(
            colr_Mapping_replace as unsafe extern "C" fn(*mut colr_Mapping, colr_Mapping) -> (),
        ),
        copyReplace: Some(
            colr_Mapping_copyReplace as unsafe extern "C" fn(*mut colr_Mapping, colr_Mapping) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn colr_Mapping_move(mut dst: *mut colr_Mapping, mut src: *mut colr_Mapping) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<colr_Mapping>() as usize,
    );
    colr_Mapping_init(src);
}
#[inline]
unsafe extern "C" fn colr_Mapping_init(mut x: *mut colr_Mapping) {
    initMapping(x);
}
#[inline]
unsafe extern "C" fn colr_Mapping_replace(mut dst: *mut colr_Mapping, src: colr_Mapping) {
    colr_Mapping_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<colr_Mapping>() as usize,
    );
}
#[inline]
unsafe extern "C" fn colr_Mapping_copy(mut dst: *mut colr_Mapping, mut src: *const colr_Mapping) {
    copyMapping(dst, src);
}
#[inline]
unsafe extern "C" fn table_COLR_replace(mut dst: *mut table_COLR, src: table_COLR) {
    table_COLR_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_COLR>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_COLR_growTo(arr: *mut table_COLR, target: usize) {
    cvec_grow_to(table_COLR_as_cvec(arr), target);
}
#[no_mangle]
pub static mut table_iCOLR: __caryll_vectorinterface_table_COLR = {
    __caryll_vectorinterface_table_COLR {
        init: Some(table_COLR_init as unsafe extern "C" fn(*mut table_COLR) -> ()),
        copy: Some(
            table_COLR_copy as unsafe extern "C" fn(*mut table_COLR, *const table_COLR) -> (),
        ),
        move_0: Some(
            table_COLR_move as unsafe extern "C" fn(*mut table_COLR, *mut table_COLR) -> (),
        ),
        dispose: Some(table_COLR_dispose as unsafe extern "C" fn(*mut table_COLR) -> ()),
        replace: Some(
            table_COLR_replace as unsafe extern "C" fn(*mut table_COLR, table_COLR) -> (),
        ),
        copyReplace: Some(
            table_COLR_copyReplace as unsafe extern "C" fn(*mut table_COLR, table_COLR) -> (),
        ),
        create: Some(table_COLR_create),
        free: Some(table_COLR_free as unsafe extern "C" fn(*mut table_COLR) -> ()),
        initN: Some(table_COLR_initN as unsafe extern "C" fn(*mut table_COLR, usize) -> ()),
        initCapN: Some(table_COLR_initCapN as unsafe extern "C" fn(*mut table_COLR, usize) -> ()),
        createN: Some(table_COLR_createN as unsafe extern "C" fn(usize) -> *mut table_COLR),
        fill: Some(table_COLR_fill as unsafe extern "C" fn(*mut table_COLR, usize) -> ()),
        clear: Some(table_COLR_dispose as unsafe extern "C" fn(*mut table_COLR) -> ()),
        push: Some(table_COLR_push as unsafe extern "C" fn(*mut table_COLR, colr_Mapping) -> ()),
        shrinkToFit: Some(table_COLR_shrinkToFit as unsafe extern "C" fn(*mut table_COLR) -> ()),
        pop: Some(table_COLR_pop as unsafe extern "C" fn(*mut table_COLR) -> colr_Mapping),
        disposeItem: Some(
            table_COLR_disposeItem as unsafe extern "C" fn(*mut table_COLR, usize) -> (),
        ),
        filterEnv: Some(
            table_COLR_filterEnv
                as unsafe extern "C" fn(
                    *mut table_COLR,
                    Option<
                        unsafe extern "C" fn(*const colr_Mapping, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            table_COLR_sort
                as unsafe extern "C" fn(
                    *mut table_COLR,
                    Option<
                        unsafe extern "C" fn(
                            *const colr_Mapping,
                            *const colr_Mapping,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn table_COLR_shrinkToFit(mut arr: *mut table_COLR) {
    table_COLR_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn table_COLR_resizeTo(arr: *mut table_COLR, target: usize) {
    cvec_resize_to(table_COLR_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_COLR_move(dst: *mut table_COLR, src: *mut table_COLR) {
    cvec_move(table_COLR_as_cvec(dst), table_COLR_as_cvec(src));
}
#[inline]
unsafe fn table_COLR_as_cvec(arr: *mut table_COLR) -> *mut CVecRaw<colr_Mapping> {
    arr as *mut CVecRaw<colr_Mapping>
}
#[inline]
unsafe extern "C" fn table_COLR_init(arr: *mut table_COLR) {
    cvec_init(table_COLR_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_COLR_filterEnv(
    mut arr: *mut table_COLR,
    mut fn_0: Option<unsafe extern "C" fn(*const colr_Mapping, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut colr_Mapping,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if colr_iMapping.dispose.is_some() {
                colr_iMapping.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut colr_Mapping,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn table_COLR_disposeItem(mut arr: *mut table_COLR, mut n: usize) {
    if colr_iMapping.dispose.is_some() {
        colr_iMapping.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut colr_Mapping
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn table_COLR_sort(
    mut arr: *mut table_COLR,
    mut fn_0: Option<
        unsafe extern "C" fn(*const colr_Mapping, *const colr_Mapping) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<colr_Mapping>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const colr_Mapping,
                    *const colr_Mapping,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn table_COLR_fill(mut arr: *mut table_COLR, mut n: usize) {
    while (*arr).length < n {
        let mut x: colr_Mapping = colr_Mapping {
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            layers: colr_LayerList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<colr_Layer>(),
            },
        };
        if colr_iMapping.init.is_some() {
            colr_iMapping.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<colr_Mapping>() as usize,
            );
        }
        table_COLR_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn table_COLR_push(arr: *mut table_COLR, elem: colr_Mapping) {
    cvec_push(table_COLR_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_COLR_grow(arr: *mut table_COLR) {
    cvec_grow(table_COLR_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_COLR_pop(arr: *mut table_COLR) -> colr_Mapping {
    cvec_pop(table_COLR_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn table_COLR_copyReplace(mut dst: *mut table_COLR, src: table_COLR) {
    table_COLR_dispose(dst);
    table_COLR_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_COLR_copy(mut dst: *mut table_COLR, mut src: *const table_COLR) {
    table_COLR_init(dst);
    table_COLR_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if colr_iMapping.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            colr_iMapping.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut colr_Mapping,
                (*src).items.offset(j as isize) as *mut colr_Mapping as *const colr_Mapping,
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
unsafe extern "C" fn table_COLR_dispose(mut arr: *mut table_COLR) {
    if arr.is_null() {
        return;
    }
    if colr_iMapping.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh3 = j;
            j = j.wrapping_sub(1);
            if !(fresh3 != 0) {
                break;
            }
            colr_iMapping.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut colr_Mapping,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<colr_Mapping>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn table_COLR_initCapN(mut arr: *mut table_COLR, mut n: usize) {
    table_COLR_init(arr);
    table_COLR_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn table_COLR_growToN(arr: *mut table_COLR, target: usize) {
    cvec_grow_to_n(table_COLR_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_COLR_initN(mut arr: *mut table_COLR, mut n: usize) {
    table_COLR_init(arr);
    table_COLR_growToN(arr, n);
    table_COLR_fill(arr, n);
}
#[inline]
unsafe extern "C" fn table_COLR_free(mut x: *mut table_COLR) {
    if x.is_null() {
        return;
    }
    table_COLR_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_COLR_createN(mut n: usize) -> *mut table_COLR {
    let mut t: *mut table_COLR =
        malloc(::core::mem::size_of::<table_COLR>() as usize) as *mut table_COLR;
    table_COLR_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn table_COLR_create() -> *mut table_COLR {
    let mut x: *mut table_COLR =
        malloc(::core::mem::size_of::<table_COLR>() as usize) as *mut table_COLR;
    table_COLR_init(x);
    return x;
}
static mut baseGlyphRecLength: usize = 6 as usize;
static mut layerRecLength: usize = 4 as usize;
#[no_mangle]
pub unsafe extern "C" fn otfcc_readCOLR(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_COLR {
    let mut numBaseGlyphRecords: u16 = 0;
    let mut numLayerRecords: u16 = 0;
    let mut offsetBaseGlyphRecord: u32 = 0;
    let mut offsetLayerRecord: u32 = 0;
    let mut gids: *mut glyphid_t = ::core::ptr::null_mut::<glyphid_t>();
    let mut colors: *mut colorid_t = ::core::ptr::null_mut::<colorid_t>();
    let mut colr: *mut table_COLR = ::core::ptr::null_mut::<table_COLR>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
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
                                baseGlyphRecLength.wrapping_mul(numBaseGlyphRecords as usize),
                            ))
                        {
                            if !((table.length as usize)
                                < (offsetLayerRecord as usize).wrapping_add(
                                    layerRecLength.wrapping_mul(numLayerRecords as usize),
                                ))
                            {
                                gids = ::core::ptr::null_mut::<glyphid_t>();
                                colors = ::core::ptr::null_mut::<colorid_t>();
                                gids = __caryll_allocate_clean(
                                    (::core::mem::size_of::<glyphid_t>() as usize)
                                        .wrapping_mul(numLayerRecords as usize),
                                    52 as ::core::ffi::c_ulong,
                                ) as *mut glyphid_t;
                                colors = __caryll_allocate_clean(
                                    (::core::mem::size_of::<colorid_t>() as usize)
                                        .wrapping_mul(numLayerRecords as usize),
                                    53 as ::core::ffi::c_ulong,
                                ) as *mut colorid_t;
                                let mut j: glyphid_t = 0 as glyphid_t;
                                while (j as ::core::ffi::c_int)
                                    < numLayerRecords as ::core::ffi::c_int
                                {
                                    *gids.offset(j as isize) = read_16u(
                                        table.data.offset(offsetLayerRecord as isize).offset(
                                            layerRecLength.wrapping_mul(j as usize) as isize,
                                        ),
                                    )
                                        as glyphid_t;
                                    *colors.offset(j as isize) =
                                        read_16u(
                                            table
                                                .data
                                                .offset(offsetLayerRecord as isize)
                                                .offset(layerRecLength.wrapping_mul(j as usize)
                                                    as isize)
                                                .offset(2 as ::core::ffi::c_int as isize),
                                        ) as colorid_t;
                                    j = j.wrapping_add(1);
                                }
                                colr = (
                                    table_iCOLR.create.expect("non-null function pointer"))();
                                let mut j_0: glyphid_t = 0 as glyphid_t;
                                while (j_0 as ::core::ffi::c_int)
                                    < numBaseGlyphRecords as ::core::ffi::c_int
                                {
                                    let mut mapping: colr_Mapping = colr_Mapping {
                                        glyph: otfcc_Handle {
                                            state: HANDLE_STATE_EMPTY,
                                            index: 0,
                                            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                        },
                                        layers: colr_LayerList {
                                            length: 0,
                                            capacity: 0,
                                            items: ::core::ptr::null_mut::<colr_Layer>(),
                                        },
                                    };
                                    colr_iMapping.init.expect("non-null function pointer")(
                                        &raw mut mapping,
                                    );
                                    let mut gid: u16 = read_16u(
                                        table
                                            .data
                                            .offset(offsetBaseGlyphRecord as isize)
                                            .offset(baseGlyphRecLength.wrapping_mul(j_0 as usize)
                                                as isize),
                                    );
                                    let mut firstLayerIndex: u16 = read_16u(
                                        table
                                            .data
                                            .offset(offsetBaseGlyphRecord as isize)
                                            .offset(baseGlyphRecLength.wrapping_mul(j_0 as usize)
                                                as isize)
                                            .offset(2 as ::core::ffi::c_int as isize),
                                    );
                                    let mut numLayers: u16 = read_16u(
                                        table
                                            .data
                                            .offset(offsetBaseGlyphRecord as isize)
                                            .offset(baseGlyphRecLength.wrapping_mul(j_0 as usize)
                                                as isize)
                                            .offset(4 as ::core::ffi::c_int as isize),
                                    );
                                    let mut baseGlyph: glyph_handle = handle_fromIndex(
                                        gid as glyphid_t
                                    )
                                        as glyph_handle;
                                    otfcc_Handle_move(
                                        &raw mut mapping.glyph,
                                        &raw mut baseGlyph,
                                    );
                                    let mut k: glyphid_t = 0 as glyphid_t;
                                    while (k as ::core::ffi::c_int)
                                        < numLayers as ::core::ffi::c_int
                                    {
                                        if (k as ::core::ffi::c_int
                                            + firstLayerIndex as ::core::ffi::c_int)
                                            < numLayerRecords as ::core::ffi::c_int
                                        {
                                            colr_iLayerList
                                                .push
                                                .expect("non-null function pointer")(
                                                &raw mut mapping.layers,
                                                colr_Layer {
                                                    glyph: handle_fromIndex(
                                                        *gids.offset(
                                                            (k as ::core::ffi::c_int
                                                                + firstLayerIndex
                                                                    as ::core::ffi::c_int)
                                                                as isize,
                                                        ),
                                                    )
                                                        as otfcc_GlyphHandle,
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
                                    table_iCOLR.push.expect("non-null function pointer")(
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
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as u8,
                        log_type_warning,
                        sdscatprintf(
                            sdsempty(),
                            b"Table 'COLR' corrupted.\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        ),
                    );
                    table_iCOLR.free.expect("non-null function pointer")(colr);
                    colr = ::core::ptr::null_mut::<table_COLR>();
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
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpCOLR(
    mut colr: *const table_COLR,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if colr.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"COLR\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _colr: *mut json_value = json_array_new((*colr).length);
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < (*colr).length {
            let mut mapping: *mut colr_Mapping = (*colr).items.offset(__caryll_index as isize);
            while keep != 0 {
                let mut _map: *mut json_value = json_object_new(2 as usize);
                json_object_push(
                    _map,
                    b"from\0" as *const u8 as *const ::core::ffi::c_char,
                    json_string_new((*mapping).glyph.name as *const ::core::ffi::c_char),
                );
                let mut _layers: *mut json_value = json_array_new((*mapping).layers.length);
                let mut __caryll_index_0: usize = 0 as usize;
                let mut keep_0: usize = 1 as usize;
                while keep_0 != 0 && __caryll_index_0 < (*mapping).layers.length {
                    let mut layer: *mut colr_Layer =
                        (*mapping).layers.items.offset(__caryll_index_0 as isize);
                    while keep_0 != 0 {
                        let mut _layer: *mut json_value = json_object_new(2 as usize);
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
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseCOLR(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_COLR {
    let mut _colr: *mut json_value = ::core::ptr::null_mut::<json_value>();
    _colr = json_obj_get_type(
        root,
        b"COLR\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    if _colr.is_null() {
        return ::core::ptr::null_mut::<table_COLR>();
    }
    let mut colr: *mut table_COLR = (
        table_iCOLR.create.expect("non-null function pointer"))();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"COLR\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as ::core::ffi::c_uint) < (*_colr).u.array.length {
            let mut _mapping: *mut json_value =
                *(*_colr).u.array.values.offset(j as isize) as *mut json_value;
            if !(_mapping.is_null()
                || (*_mapping).type_0 as ::core::ffi::c_uint
                    != json_object as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                let mut _baseglyph: *mut json_value = json_obj_get_type(
                    _mapping,
                    b"from\0" as *const u8 as *const ::core::ffi::c_char,
                    json_string,
                );
                let mut _layers: *mut json_value = json_obj_get_type(
                    _mapping,
                    b"to\0" as *const u8 as *const ::core::ffi::c_char,
                    json_array,
                );
                if !(_baseglyph.is_null() || _layers.is_null()) {
                    let mut m: colr_Mapping = colr_Mapping {
                        glyph: otfcc_Handle {
                            state: HANDLE_STATE_EMPTY,
                            index: 0,
                            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        },
                        layers: colr_LayerList {
                            length: 0,
                            capacity: 0,
                            items: ::core::ptr::null_mut::<colr_Layer>(),
                        },
                    };
                    colr_iMapping.init.expect("non-null function pointer")(&raw mut m);
                    m.glyph = handle_fromName(sdsnewlen(
                        (*_baseglyph).u.string.ptr as *const ::core::ffi::c_void,
                        (*_baseglyph).u.string.length as usize,
                    )) as otfcc_GlyphHandle;
                    let mut k: glyphid_t = 0 as glyphid_t;
                    while (k as ::core::ffi::c_uint) < (*_layers).u.array.length {
                        let mut _layer: *mut json_value =
                            *(*_layers).u.array.values.offset(k as isize) as *mut json_value;
                        if !(_layer.is_null()
                            || (*_layer).type_0 as ::core::ffi::c_uint
                                != json_object as ::core::ffi::c_int as ::core::ffi::c_uint)
                        {
                            let mut _layerglyph: *mut json_value = json_obj_get_type(
                                _layer,
                                b"layer\0" as *const u8 as *const ::core::ffi::c_char,
                                json_string,
                            );
                            if !_layerglyph.is_null() {
                                colr_iLayerList.push.expect("non-null function pointer")(
                                    &raw mut m.layers,
                                    colr_Layer {
                                        glyph: handle_fromName(
                                            sdsnewlen(
                                                (*_layerglyph).u.string.ptr
                                                    as *const ::core::ffi::c_void,
                                                (*_layerglyph).u.string.length as usize,
                                            ),
                                        )
                                            as otfcc_GlyphHandle,
                                        paletteIndex: json_obj_getint_fallback(
                                            _layer,
                                            b"paletteIndex\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            0xffff as i32,
                                        )
                                            as colorid_t,
                                    },
                                );
                            }
                        }
                        k = k.wrapping_add(1);
                    }
                    table_iCOLR.push.expect("non-null function pointer")(colr, m);
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    return colr;
}
unsafe extern "C" fn byGID(
    mut a: *const colr_Mapping,
    mut b: *const colr_Mapping,
) -> ::core::ffi::c_int {
    return (*a).glyph.index as ::core::ffi::c_int - (*b).glyph.index as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildCOLR(
    mut _colr: *const table_COLR,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if _colr.is_null() || (*_colr).length == 0 {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut colr: table_COLR = table_COLR {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<colr_Mapping>(),
    };
    table_iCOLR.copy.expect("non-null function pointer")(&raw mut colr, _colr);
    table_iCOLR.sort.expect("non-null function pointer")(
        &raw mut colr,
        Some(
            byGID
                as unsafe extern "C" fn(
                    *const colr_Mapping,
                    *const colr_Mapping,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut currentLayerIndex: glyphid_t = 0 as glyphid_t;
    let mut layerRecords: *mut bk_Block = bk_new_Block(bkover as ::core::ffi::c_int);
    let mut baseRecords: *mut bk_Block = bk_new_Block(bkover as ::core::ffi::c_int);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < colr.length {
        let mut mapping: *mut colr_Mapping = colr.items.offset(__caryll_index as isize);
        while keep != 0 {
            bk_push(
                baseRecords,
                b16 as ::core::ffi::c_int,
                (*mapping).glyph.index as ::core::ffi::c_int,
                b16 as ::core::ffi::c_int,
                currentLayerIndex as ::core::ffi::c_int,
                b16 as ::core::ffi::c_int,
                (*mapping).layers.length,
                bkover as ::core::ffi::c_int,
            );
            let mut __caryll_index_0: usize = 0 as usize;
            let mut keep_0: usize = 1 as usize;
            while keep_0 != 0 && __caryll_index_0 < (*mapping).layers.length {
                let mut layer: *mut colr_Layer =
                    (*mapping).layers.items.offset(__caryll_index_0 as isize);
                while keep_0 != 0 {
                    bk_push(
                        layerRecords,
                        b16 as ::core::ffi::c_int,
                        (*layer).glyph.index as ::core::ffi::c_int,
                        b16 as ::core::ffi::c_int,
                        (*layer).paletteIndex as ::core::ffi::c_int,
                        bkover as ::core::ffi::c_int,
                    );
                    currentLayerIndex = (currentLayerIndex as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as glyphid_t;
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
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        b16 as ::core::ffi::c_int,
        colr.length,
        p32 as ::core::ffi::c_int,
        baseRecords,
        p32 as ::core::ffi::c_int,
        layerRecords,
        b16 as ::core::ffi::c_int,
        currentLayerIndex as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    table_iCOLR.dispose.expect("non-null function pointer")(&raw mut colr);
    return bk_build_Block(root);
}
pub const json_serialize_mode_packed: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<json_value>();
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        }
        _k = _k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_get_type(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    type_0: json_type,
) -> *mut json_value {
    let mut v: *mut json_value = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return v;
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_getint_fallback(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    mut fallback: i32,
) -> i32 {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return fallback;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as i32;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl as i32;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return fallback;
}
#[inline]
unsafe extern "C" fn preserialize(mut x: *mut json_value) -> *mut json_value {
    let mut opts: json_serialize_opts = json_serialize_opts {
        mode: json_serialize_mode_packed,
        opts: 0,
        indent_size: 0,
    };
    let mut preserialize_len: usize = json_measure_ex(x, opts);
    let mut buf: *mut ::core::ffi::c_char = malloc(preserialize_len) as *mut ::core::ffi::c_char;
    json_serialize_ex(buf, x, opts);
    json_builder_free(x);
    let mut xx: *mut json_value = json_string_new_nocopy(
        preserialize_len.wrapping_sub(1 as usize) as ::core::ffi::c_uint,
        buf,
    );
    (*xx).type_0 = json_pre_serialized;
    return xx;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
