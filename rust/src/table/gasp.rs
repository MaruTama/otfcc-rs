#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};
use crate::support::binio::{read_16u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphSize, TableId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::{ComparFn};
use crate::support::json_funcs::{json_obj_get_type, json_obj_getbool, json_obj_getint_fallback};
use crate::support::buffer::{bufnew, bufwrite16b};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_boolean_new, json_integer_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsempty};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GaspRecord {
    pub rangeMaxPPEM: GlyphSize,
    pub dogray: bool,
    pub gridfit: bool,
    pub symmetric_smoothing: bool,
    pub symmetric_gridfit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GaspRecordElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GaspRecord) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut GaspRecord, *const GaspRecord) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut GaspRecord, *mut GaspRecord) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GaspRecord) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut GaspRecord, GaspRecord) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut GaspRecord, GaspRecord) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GaspRecordList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut GaspRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GaspRecordListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut GaspRecordList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut GaspRecordList, *const GaspRecordList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut GaspRecordList, *mut GaspRecordList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GaspRecordList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut GaspRecordList, GaspRecordList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut GaspRecordList, GaspRecordList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GaspRecordList>,
    pub free: Option<unsafe extern "C" fn(*mut GaspRecordList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut GaspRecordList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut GaspRecordList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut GaspRecordList>,
    pub fill: Option<unsafe extern "C" fn(*mut GaspRecordList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut GaspRecordList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut GaspRecordList, GaspRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut GaspRecordList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut GaspRecordList) -> GaspRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut GaspRecordList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut GaspRecordList,
            Option<unsafe extern "C" fn(*const GaspRecord, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut GaspRecordList,
            Option<
                unsafe extern "C" fn(*const GaspRecord, *const GaspRecord) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GaspTable {
    pub version: u16,
    pub records: GaspRecordList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GaspTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GaspTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut GaspTable, *const GaspTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut GaspTable, *mut GaspTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GaspTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut GaspTable, GaspTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut GaspTable, GaspTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GaspTable>,
    pub free: Option<unsafe extern "C" fn(*mut GaspTable) -> ()>,
}
pub const GASP_DOGRAY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const GASP_GRIDFIT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const GASP_SYMMETRIC_GRIDFIT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const GASP_SYMMETRIC_SMOOTHING: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn gasp_record_copy(mut dst: *mut GaspRecord, mut src: *const GaspRecord) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GaspRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn gasp_record_dispose(mut _x: *mut GaspRecord) {}
#[inline]
unsafe extern "C" fn gasp_record_replace(mut dst: *mut GaspRecord, src: GaspRecord) {
    gasp_record_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GaspRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn gasp_record_move(mut dst: *mut GaspRecord, mut src: *mut GaspRecord) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GaspRecord>() as usize,
    );
    gasp_record_init(src);
}
#[inline]
unsafe extern "C" fn gasp_record_init(mut x: *mut GaspRecord) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GaspRecord>() as usize,
    );
}
pub static GASP_I_RECORD: GaspRecordElementInterface = {
    GaspRecordElementInterface {
        init: Some(gasp_record_init as unsafe extern "C" fn(*mut GaspRecord) -> ()),
        copy: Some(
            gasp_record_copy as unsafe extern "C" fn(*mut GaspRecord, *const GaspRecord) -> (),
        ),
        move_0: Some(
            gasp_record_move as unsafe extern "C" fn(*mut GaspRecord, *mut GaspRecord) -> (),
        ),
        dispose: Some(gasp_record_dispose as unsafe extern "C" fn(*mut GaspRecord) -> ()),
        replace: Some(
            gasp_record_replace as unsafe extern "C" fn(*mut GaspRecord, GaspRecord) -> (),
        ),
        copyReplace: Some(
            gasp_record_copy_replace as unsafe extern "C" fn(*mut GaspRecord, GaspRecord) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn gasp_record_copy_replace(mut dst: *mut GaspRecord, src: GaspRecord) {
    gasp_record_dispose(dst);
    gasp_record_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn gasp_record_list_shrink_to_fit(mut arr: *mut GaspRecordList) {
    gasp_record_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn gasp_record_list_resize_to(arr: *mut GaspRecordList, target: usize) {
    cvec_resize_to(gasp_record_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn gasp_record_list_grow(arr: *mut GaspRecordList) {
    cvec_grow(gasp_record_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn gasp_record_list_grow_to(arr: *mut GaspRecordList, target: usize) {
    cvec_grow_to(gasp_record_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn gasp_record_list_pop(arr: *mut GaspRecordList) -> GaspRecord {
    cvec_pop(gasp_record_list_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn gasp_record_list_copy_replace(
    mut dst: *mut GaspRecordList,
    src: GaspRecordList,
) {
    gasp_record_list_dispose(dst);
    gasp_record_list_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn gasp_record_list_copy(
    mut dst: *mut GaspRecordList,
    mut src: *const GaspRecordList,
) {
    gasp_record_list_init(dst);
    gasp_record_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if GASP_I_RECORD.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            GASP_I_RECORD.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut GaspRecord,
                (*src).items.offset(j as isize) as *mut GaspRecord as *const GaspRecord,
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
unsafe extern "C" fn gasp_record_list_dispose(mut arr: *mut GaspRecordList) {
    if arr.is_null() {
        return;
    }
    if GASP_I_RECORD.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            GASP_I_RECORD.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut GaspRecord,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<GaspRecord>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn gasp_record_list_replace(mut dst: *mut GaspRecordList, src: GaspRecordList) {
    gasp_record_list_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GaspRecordList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn gasp_record_list_init_cap_n(mut arr: *mut GaspRecordList, mut n: usize) {
    gasp_record_list_init(arr);
    gasp_record_list_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn gasp_record_list_grow_to_n(arr: *mut GaspRecordList, target: usize) {
    cvec_grow_to_n(gasp_record_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn gasp_record_list_init_n(mut arr: *mut GaspRecordList, mut n: usize) {
    gasp_record_list_init(arr);
    gasp_record_list_grow_to_n(arr, n);
    gasp_record_list_fill(arr, n);
}
#[inline]
unsafe extern "C" fn gasp_record_list_free(mut x: *mut GaspRecordList) {
    if x.is_null() {
        return;
    }
    gasp_record_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn gasp_record_list_create_n(mut n: usize) -> *mut GaspRecordList {
    let mut t: *mut GaspRecordList =
        malloc(::core::mem::size_of::<GaspRecordList>() as usize) as *mut GaspRecordList;
    gasp_record_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn gasp_record_list_create() -> *mut GaspRecordList {
    let mut x: *mut GaspRecordList =
        malloc(::core::mem::size_of::<GaspRecordList>() as usize) as *mut GaspRecordList;
    gasp_record_list_init(x);
    return x;
}
#[inline]
unsafe fn gasp_record_list_as_cvec(arr: *mut GaspRecordList) -> *mut CVecRaw<GaspRecord> {
    arr as *mut CVecRaw<GaspRecord>
}
#[inline]
unsafe extern "C" fn gasp_record_list_init(arr: *mut GaspRecordList) {
    cvec_init(gasp_record_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn gasp_record_list_move(dst: *mut GaspRecordList, src: *mut GaspRecordList) {
    cvec_move(gasp_record_list_as_cvec(dst), gasp_record_list_as_cvec(src));
}
#[inline]
unsafe extern "C" fn gasp_record_list_filter_env(
    mut arr: *mut GaspRecordList,
    mut fn_0: Option<unsafe extern "C" fn(*const GaspRecord, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut GaspRecord,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if GASP_I_RECORD.dispose.is_some() {
                GASP_I_RECORD.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut GaspRecord,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
pub static GASP_I_RECORD_LIST: GaspRecordListVectorInterface = {
    GaspRecordListVectorInterface {
        init: Some(gasp_record_list_init as unsafe extern "C" fn(*mut GaspRecordList) -> ()),
        copy: Some(
            gasp_record_list_copy
                as unsafe extern "C" fn(*mut GaspRecordList, *const GaspRecordList) -> (),
        ),
        move_0: Some(
            gasp_record_list_move
                as unsafe extern "C" fn(*mut GaspRecordList, *mut GaspRecordList) -> (),
        ),
        dispose: Some(gasp_record_list_dispose as unsafe extern "C" fn(*mut GaspRecordList) -> ()),
        replace: Some(
            gasp_record_list_replace
                as unsafe extern "C" fn(*mut GaspRecordList, GaspRecordList) -> (),
        ),
        copyReplace: Some(
            gasp_record_list_copy_replace
                as unsafe extern "C" fn(*mut GaspRecordList, GaspRecordList) -> (),
        ),
        create: Some(gasp_record_list_create),
        free: Some(gasp_record_list_free as unsafe extern "C" fn(*mut GaspRecordList) -> ()),
        initN: Some(
            gasp_record_list_init_n as unsafe extern "C" fn(*mut GaspRecordList, usize) -> (),
        ),
        initCapN: Some(
            gasp_record_list_init_cap_n as unsafe extern "C" fn(*mut GaspRecordList, usize) -> (),
        ),
        createN: Some(
            gasp_record_list_create_n as unsafe extern "C" fn(usize) -> *mut GaspRecordList,
        ),
        fill: Some(
            gasp_record_list_fill as unsafe extern "C" fn(*mut GaspRecordList, usize) -> (),
        ),
        clear: Some(gasp_record_list_dispose as unsafe extern "C" fn(*mut GaspRecordList) -> ()),
        push: Some(
            gasp_record_list_push as unsafe extern "C" fn(*mut GaspRecordList, GaspRecord) -> (),
        ),
        shrinkToFit: Some(
            gasp_record_list_shrink_to_fit as unsafe extern "C" fn(*mut GaspRecordList) -> (),
        ),
        pop: Some(gasp_record_list_pop as unsafe extern "C" fn(*mut GaspRecordList) -> GaspRecord),
        disposeItem: Some(
            gasp_record_list_dispose_item as unsafe extern "C" fn(*mut GaspRecordList, usize) -> (),
        ),
        filterEnv: Some(
            gasp_record_list_filter_env
                as unsafe extern "C" fn(
                    *mut GaspRecordList,
                    Option<
                        unsafe extern "C" fn(*const GaspRecord, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            gasp_record_list_sort
                as unsafe extern "C" fn(
                    *mut GaspRecordList,
                    Option<
                        unsafe extern "C" fn(
                            *const GaspRecord,
                            *const GaspRecord,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn gasp_record_list_dispose_item(mut arr: *mut GaspRecordList, mut n: usize) {
    if GASP_I_RECORD.dispose.is_some() {
        GASP_I_RECORD.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut GaspRecord
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn gasp_record_list_sort(
    mut arr: *mut GaspRecordList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const GaspRecord, *const GaspRecord) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<GaspRecord>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const GaspRecord, *const GaspRecord) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn gasp_record_list_fill(mut arr: *mut GaspRecordList, mut n: usize) {
    while (*arr).length < n {
        let mut x: GaspRecord = GaspRecord {
            rangeMaxPPEM: 0,
            dogray: false,
            gridfit: false,
            symmetric_smoothing: false,
            symmetric_gridfit: false,
        };
        if GASP_I_RECORD.init.is_some() {
            GASP_I_RECORD.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<GaspRecord>() as usize,
            );
        }
        gasp_record_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn gasp_record_list_push(arr: *mut GaspRecordList, elem: GaspRecord) {
    cvec_push(gasp_record_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn init_gasp(mut gasp: *mut GaspTable) {
    (*gasp).version = 1 as u16;
    GASP_I_RECORD_LIST.init.expect("non-null function pointer")(&raw mut (*gasp).records);
}
#[inline]
unsafe extern "C" fn dispose_gasp(mut gasp: *mut GaspTable) {
    GASP_I_RECORD_LIST.dispose.expect("non-null function pointer")(&raw mut (*gasp).records);
}
#[inline]
unsafe extern "C" fn table_gasp_create() -> *mut GaspTable {
    let mut x: *mut GaspTable =
        malloc(::core::mem::size_of::<GaspTable>() as usize) as *mut GaspTable;
    table_gasp_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_gasp_dispose(mut x: *mut GaspTable) {
    dispose_gasp(x);
}
#[inline]
unsafe extern "C" fn table_gasp_copy(mut dst: *mut GaspTable, mut src: *const GaspTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GaspTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_gasp_init(mut x: *mut GaspTable) {
    init_gasp(x);
}
#[inline]
unsafe extern "C" fn table_gasp_move(mut dst: *mut GaspTable, mut src: *mut GaspTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GaspTable>() as usize,
    );
    table_gasp_init(src);
}
#[inline]
unsafe extern "C" fn table_gasp_replace(mut dst: *mut GaspTable, src: GaspTable) {
    table_gasp_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GaspTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_gasp_copy_replace(mut dst: *mut GaspTable, src: GaspTable) {
    table_gasp_dispose(dst);
    table_gasp_copy(dst, &raw const src);
}
pub static TABLE_I_GASP: GaspTableElementInterface = {
    GaspTableElementInterface {
        init: Some(table_gasp_init as unsafe extern "C" fn(*mut GaspTable) -> ()),
        copy: Some(
            table_gasp_copy as unsafe extern "C" fn(*mut GaspTable, *const GaspTable) -> (),
        ),
        move_0: Some(
            table_gasp_move as unsafe extern "C" fn(*mut GaspTable, *mut GaspTable) -> (),
        ),
        dispose: Some(table_gasp_dispose as unsafe extern "C" fn(*mut GaspTable) -> ()),
        replace: Some(
            table_gasp_replace as unsafe extern "C" fn(*mut GaspTable, GaspTable) -> (),
        ),
        copyReplace: Some(
            table_gasp_copy_replace as unsafe extern "C" fn(*mut GaspTable, GaspTable) -> (),
        ),
        create: Some(table_gasp_create),
        free: Some(table_gasp_free as unsafe extern "C" fn(*mut GaspTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_gasp_free(mut x: *mut GaspTable) {
    if x.is_null() {
        return;
    }
    table_gasp_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_read_gasp(
    packet: Packet,
    mut options: *const Options,
) -> *mut GaspTable {
    let mut num_ranges: TableId = 0;
    let mut gasp: *mut GaspTable = ::core::ptr::null_mut::<GaspTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1734439792i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if !(length < 4 as u32) {
                        gasp = (
                            TABLE_I_GASP.create.expect("non-null function pointer"))();
                        (*gasp).version = read_16u(data as *const u8);
                        num_ranges = read_16u(
                            data.offset(2 as ::core::ffi::c_int as isize) as *const u8
                        ) as TableId;
                        if !(length
                            < (4 as ::core::ffi::c_int
                                + num_ranges as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as u32)
                        {
                            let mut j: u32 = 0 as u32;
                            while j < num_ranges as u32 {
                                let mut record: GaspRecord = GaspRecord {
                                    rangeMaxPPEM: 0,
                                    dogray: false,
                                    gridfit: false,
                                    symmetric_smoothing: false,
                                    symmetric_gridfit: false,
                                };
                                record.rangeMaxPPEM = read_16u(
                                    data.offset(4 as ::core::ffi::c_int as isize)
                                        .offset(j.wrapping_mul(4 as u32) as isize)
                                        as *const u8,
                                )
                                    as GlyphSize;
                                let mut range_gasp_behavior: u16 = read_16u(
                                    data.offset(4 as ::core::ffi::c_int as isize)
                                        .offset(j.wrapping_mul(4 as u32) as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                record.dogray =
                                    range_gasp_behavior as ::core::ffi::c_int & GASP_DOGRAY != 0;
                                record.gridfit =
                                    range_gasp_behavior as ::core::ffi::c_int & GASP_GRIDFIT != 0;
                                record.symmetric_smoothing = range_gasp_behavior
                                    as ::core::ffi::c_int
                                    & GASP_SYMMETRIC_SMOOTHING
                                    != 0;
                                record.symmetric_gridfit = range_gasp_behavior as ::core::ffi::c_int
                                    & GASP_SYMMETRIC_GRIDFIT
                                    != 0;
                                GASP_I_RECORD_LIST.push.expect("non-null function pointer")(
                                    &raw mut (*gasp).records,
                                    record,
                                );
                                j = j.wrapping_add(1);
                            }
                            return gasp;
                        }
                    }
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(sdsempty(), b"table 'gasp' corrupted.\n"),
                    );
                    TABLE_I_GASP.free.expect("non-null function pointer")(gasp);
                    gasp = ::core::ptr::null_mut::<GaspTable>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<GaspTable>();
}
pub unsafe extern "C" fn otfcc_dump_gasp(
    mut table: *const GaspTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"gasp"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut t: *mut JsonValue = json_array_new((*table).records.length);
        let mut j: u16 = 0 as u16;
        while (j as usize) < (*table).records.length {
            let mut rec: *mut JsonValue = json_object_new(5 as usize);
            json_object_push(
                rec,
                b"rangeMaxPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new(
                    (*(*table).records.items.offset(j as isize)).rangeMaxPPEM as i64,
                ),
            );
            json_object_push(
                rec,
                b"dogray\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    (*(*table).records.items.offset(j as isize)).dogray as ::core::ffi::c_int,
                ),
            );
            json_object_push(
                rec,
                b"gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    (*(*table).records.items.offset(j as isize)).gridfit as ::core::ffi::c_int,
                ),
            );
            json_object_push(
                rec,
                b"symmetric_smoothing\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    (*(*table).records.items.offset(j as isize)).symmetric_smoothing
                        as ::core::ffi::c_int,
                ),
            );
            json_object_push(
                rec,
                b"symmetric_gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    (*(*table).records.items.offset(j as isize)).symmetric_gridfit
                        as ::core::ffi::c_int,
                ),
            );
            json_array_push(t, rec);
            j = j.wrapping_add(1);
        }
        json_object_push(
            root,
            b"gasp\0" as *const u8 as *const ::core::ffi::c_char,
            t,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_gasp(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut GaspTable {
    let mut gasp: *mut GaspTable = ::core::ptr::null_mut::<GaspTable>();
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"gasp\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"gasp"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            gasp = (
                TABLE_I_GASP.create.expect("non-null function pointer"))();
            let mut j: u16 = 0 as u16;
            while (j as ::core::ffi::c_uint) < (*table).u.array.length {
                let mut r: *mut JsonValue =
                    *(*table).u.array.values.offset(j as isize) as *mut JsonValue;
                if !(r.is_null()
                    || (*r).type_0 != JsonType::Object)
                {
                    let mut record: GaspRecord = GaspRecord {
                        rangeMaxPPEM: 0,
                        dogray: false,
                        gridfit: false,
                        symmetric_smoothing: false,
                        symmetric_gridfit: false,
                    };
                    record.rangeMaxPPEM = json_obj_getint_fallback(
                        r,
                        b"rangeMaxPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                        0xffff as i32,
                    ) as GlyphSize;
                    record.dogray =
                        json_obj_getbool(r, b"dogray\0" as *const u8 as *const ::core::ffi::c_char);
                    record.gridfit = json_obj_getbool(
                        r,
                        b"gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    record.symmetric_smoothing = json_obj_getbool(
                        r,
                        b"symmetric_smoothing\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    record.symmetric_gridfit = json_obj_getbool(
                        r,
                        b"symmetric_gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    GASP_I_RECORD_LIST.push.expect("non-null function pointer")(
                        &raw mut (*gasp).records,
                        record,
                    );
                }
                j = j.wrapping_add(1);
            }
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return gasp;
}
pub unsafe extern "C" fn otfcc_build_gasp(
    mut gasp: *const GaspTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if gasp.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 1 as u16);
    bufwrite16b(buf, (*gasp).records.length as u16);
    let mut j: u16 = 0 as u16;
    while (j as usize) < (*gasp).records.length {
        let mut r: *mut GaspRecord = (*gasp).records.items.offset(j as isize) as *mut GaspRecord;
        bufwrite16b(buf, (*r).rangeMaxPPEM as u16);
        bufwrite16b(
            buf,
            ((if (*r).dogray as ::core::ffi::c_int != 0 {
                GASP_DOGRAY
            } else {
                0 as ::core::ffi::c_int
            }) | (if (*r).gridfit as ::core::ffi::c_int != 0 {
                GASP_GRIDFIT
            } else {
                0 as ::core::ffi::c_int
            }) | (if (*r).symmetric_gridfit as ::core::ffi::c_int != 0 {
                GASP_SYMMETRIC_GRIDFIT
            } else {
                0 as ::core::ffi::c_int
            }) | (if (*r).symmetric_smoothing as ::core::ffi::c_int != 0 {
                GASP_SYMMETRIC_SMOOTHING
            } else {
                0 as ::core::ffi::c_int
            })) as u16,
        );
        j = j.wrapping_add(1);
    }
    return buf;
}
