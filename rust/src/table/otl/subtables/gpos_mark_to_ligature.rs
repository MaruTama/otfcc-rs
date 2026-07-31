#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcmp, memcpy, memset, qsort, strlen};


use crate::support::json_funcs::{json_obj_get_type, preserialize};
use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_name, otfcc_handle_dispose, otfcc_handle_dup, Handle, GlyphHandle, HandleState};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphClass, GlyphId};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::support::{NULL, ComparFn};
use crate::table::otl::{GposMarkToLigatureSubtableElementInterface, LigatureArrayVectorInterface, Anchor, LigatureArray, LigatureBaseRecord, Subtable, GposMarkToLigatureSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::table::otl::subtables::gpos_common::{ClassNameHash};
use crate::vendor::uthash::{UtHashBucket, UtHashHandle};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::table::otl::subtables::gpos_common::{bk_from_anchor, otl_anchor_absent, OTL_I_MARK_ARRAY, otl_parse_mark_array, otl_parse_anchor, otl_read_mark_array, otl_read_anchor};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_object_push_length, json_string_new_length};
use crate::vendor::sds::{sdsempty, sdsfree, sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LigatureBaseRecordElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut LigatureBaseRecord) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut LigatureBaseRecord, *const LigatureBaseRecord) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut LigatureBaseRecord) -> ()>,
}
#[inline]
unsafe extern "C" fn sdslen(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
unsafe extern "C" fn delete_lig_array_item(mut entry: *mut LigatureBaseRecord) {
    otfcc_handle_dispose(&raw mut (*entry).glyph);
    if !(*entry).anchors.is_null() {
        let mut k: GlyphId = 0 as GlyphId;
        while (k as ::core::ffi::c_int) < (*entry).component_count as ::core::ffi::c_int {
            free(*(*entry).anchors.offset(k as isize) as *mut ::core::ffi::c_void);
            let ref mut fresh0 = *(*entry).anchors.offset(k as isize);
            *fresh0 = ::core::ptr::null_mut::<Anchor>();
            k = k.wrapping_add(1);
        }
        free((*entry).anchors as *mut ::core::ffi::c_void);
        (*entry).anchors = ::core::ptr::null_mut::<*mut Anchor>();
    }
}
static LA_TYPEINFO: LigatureBaseRecordElementInterface = {
    LigatureBaseRecordElementInterface {
        init: None,
        copy: None,
        dispose: Some(
            delete_lig_array_item as unsafe extern "C" fn(*mut LigatureBaseRecord) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_ligature_array_grow_to_n(arr: *mut LigatureArray, target: usize) {
    cvec_grow_to_n(otl_ligature_array_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_ligature_array_push(arr: *mut LigatureArray, elem: LigatureBaseRecord) {
    cvec_push(otl_ligature_array_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_ligature_array_grow_to(arr: *mut LigatureArray, target: usize) {
    cvec_grow_to(otl_ligature_array_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_ligature_array_pop(arr: *mut LigatureArray) -> LigatureBaseRecord {
    cvec_pop(otl_ligature_array_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_ligature_array_copy(
    mut dst: *mut LigatureArray,
    mut src: *const LigatureArray,
) {
    otl_ligature_array_init(dst);
    otl_ligature_array_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if LA_TYPEINFO.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            LA_TYPEINFO.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut LigatureBaseRecord,
                (*src).items.offset(j as isize) as *mut LigatureBaseRecord
                    as *const LigatureBaseRecord,
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
unsafe extern "C" fn otl_ligature_array_dispose(mut arr: *mut LigatureArray) {
    if arr.is_null() {
        return;
    }
    if LA_TYPEINFO.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh2 = j;
            j = j.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            LA_TYPEINFO.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut LigatureBaseRecord,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<LigatureBaseRecord>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_ligature_array_init_cap_n(mut arr: *mut LigatureArray, mut n: usize) {
    otl_ligature_array_init(arr);
    otl_ligature_array_grow_to_n(arr, n);
}
#[inline]
unsafe fn otl_ligature_array_as_cvec(arr: *mut LigatureArray) -> *mut CVecRaw<LigatureBaseRecord> {
    arr as *mut CVecRaw<LigatureBaseRecord>
}
#[inline]
unsafe extern "C" fn otl_ligature_array_init(arr: *mut LigatureArray) {
    cvec_init(otl_ligature_array_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_ligature_array_init_n(mut arr: *mut LigatureArray, mut n: usize) {
    otl_ligature_array_init(arr);
    otl_ligature_array_grow_to_n(arr, n);
    otl_ligature_array_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_ligature_array_free(mut x: *mut LigatureArray) {
    if x.is_null() {
        return;
    }
    otl_ligature_array_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_ligature_array_create_n(mut n: usize) -> *mut LigatureArray {
    let mut t: *mut LigatureArray =
        malloc(::core::mem::size_of::<LigatureArray>() as usize) as *mut LigatureArray;
    otl_ligature_array_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_ligature_array_create() -> *mut LigatureArray {
    let mut x: *mut LigatureArray =
        malloc(::core::mem::size_of::<LigatureArray>() as usize) as *mut LigatureArray;
    otl_ligature_array_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_ligature_array_filter_env(
    mut arr: *mut LigatureArray,
    mut fn_0: Option<
        unsafe extern "C" fn(*const LigatureBaseRecord, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut LigatureBaseRecord,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if LA_TYPEINFO.dispose.is_some() {
                LA_TYPEINFO.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut LigatureBaseRecord,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
pub static OTL_I_LIGATURE_ARRAY: LigatureArrayVectorInterface = {
    LigatureArrayVectorInterface {
        init: Some(otl_ligature_array_init as unsafe extern "C" fn(*mut LigatureArray) -> ()),
        copy: Some(
            otl_ligature_array_copy
                as unsafe extern "C" fn(*mut LigatureArray, *const LigatureArray) -> (),
        ),
        dispose: Some(
            otl_ligature_array_dispose as unsafe extern "C" fn(*mut LigatureArray) -> (),
        ),
        create: Some(otl_ligature_array_create),
        free: Some(otl_ligature_array_free as unsafe extern "C" fn(*mut LigatureArray) -> ()),
        init_n: Some(
            otl_ligature_array_init_n as unsafe extern "C" fn(*mut LigatureArray, usize) -> (),
        ),
        init_cap_n: Some(
            otl_ligature_array_init_cap_n
                as unsafe extern "C" fn(*mut LigatureArray, usize) -> (),
        ),
        create_n: Some(
            otl_ligature_array_create_n as unsafe extern "C" fn(usize) -> *mut LigatureArray,
        ),
        fill: Some(
            otl_ligature_array_fill as unsafe extern "C" fn(*mut LigatureArray, usize) -> (),
        ),
        clear: Some(
            otl_ligature_array_dispose as unsafe extern "C" fn(*mut LigatureArray) -> (),
        ),
        push: Some(
            otl_ligature_array_push
                as unsafe extern "C" fn(*mut LigatureArray, LigatureBaseRecord) -> (),
        ),
        shrink_to_fit: Some(
            otl_ligature_array_shrink_to_fit as unsafe extern "C" fn(*mut LigatureArray) -> (),
        ),
        pop: Some(
            otl_ligature_array_pop
                as unsafe extern "C" fn(*mut LigatureArray) -> LigatureBaseRecord,
        ),
        dispose_item: Some(
            otl_ligature_array_dispose_item
                as unsafe extern "C" fn(*mut LigatureArray, usize) -> (),
        ),
        filter_env: Some(
            otl_ligature_array_filter_env
                as unsafe extern "C" fn(
                    *mut LigatureArray,
                    Option<
                        unsafe extern "C" fn(
                            *const LigatureBaseRecord,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_ligature_array_sort
                as unsafe extern "C" fn(
                    *mut LigatureArray,
                    Option<
                        unsafe extern "C" fn(
                            *const LigatureBaseRecord,
                            *const LigatureBaseRecord,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_ligature_array_shrink_to_fit(mut arr: *mut LigatureArray) {
    otl_ligature_array_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_ligature_array_resize_to(arr: *mut LigatureArray, target: usize) {
    cvec_resize_to(otl_ligature_array_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_ligature_array_dispose_item(mut arr: *mut LigatureArray, mut n: usize) {
    if LA_TYPEINFO.dispose.is_some() {
        LA_TYPEINFO.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut LigatureBaseRecord
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_ligature_array_sort(
    mut arr: *mut LigatureArray,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const LigatureBaseRecord,
            *const LigatureBaseRecord,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<LigatureBaseRecord>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const LigatureBaseRecord,
                    *const LigatureBaseRecord,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_ligature_array_fill(mut arr: *mut LigatureArray, mut n: usize) {
    while (*arr).length < n {
        let mut x: LigatureBaseRecord = LigatureBaseRecord {
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            component_count: 0,
            anchors: ::core::ptr::null_mut::<*mut Anchor>(),
        };
        if LA_TYPEINFO.init.is_some() {
            LA_TYPEINFO.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<LigatureBaseRecord>() as usize,
            );
        }
        otl_ligature_array_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn init_mark_to_ligature(mut subtable: *mut GposMarkToLigatureSubtable) {
    OTL_I_MARK_ARRAY.init.expect("non-null function pointer")(&raw mut (*subtable).mark_array);
    OTL_I_LIGATURE_ARRAY.init.expect("non-null function pointer")(&raw mut (*subtable).lig_array);
}
#[inline]
unsafe extern "C" fn dispose_mark_to_ligature(mut subtable: *mut GposMarkToLigatureSubtable) {
    OTL_I_MARK_ARRAY.dispose.expect("non-null function pointer")(&raw mut (*subtable).mark_array);
    OTL_I_LIGATURE_ARRAY
        .dispose
        .expect("non-null function pointer")(&raw mut (*subtable).lig_array);
}
#[inline]
unsafe extern "C" fn subtable_gpos_mark_to_ligature_init(mut x: *mut GposMarkToLigatureSubtable) {
    init_mark_to_ligature(x);
}
#[inline]
unsafe extern "C" fn subtable_gpos_mark_to_ligature_copy(
    mut dst: *mut GposMarkToLigatureSubtable,
    mut src: *const GposMarkToLigatureSubtable,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GposMarkToLigatureSubtable>() as usize,
    );
}
pub static I_SUBTABLE_GPOS_MARK_TO_LIGATURE:
    GposMarkToLigatureSubtableElementInterface = {
    GposMarkToLigatureSubtableElementInterface {
        init: Some(
            subtable_gpos_mark_to_ligature_init
                as unsafe extern "C" fn(*mut GposMarkToLigatureSubtable) -> (),
        ),
        copy: Some(
            subtable_gpos_mark_to_ligature_copy
                as unsafe extern "C" fn(
                    *mut GposMarkToLigatureSubtable,
                    *const GposMarkToLigatureSubtable,
                ) -> (),
        ),
        dispose: Some(
            subtable_gpos_mark_to_ligature_dispose
                as unsafe extern "C" fn(*mut GposMarkToLigatureSubtable) -> (),
        ),
        create: Some(subtable_gpos_mark_to_ligature_create),
        free: Some(
            subtable_gpos_mark_to_ligature_free
                as unsafe extern "C" fn(*mut GposMarkToLigatureSubtable) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_mark_to_ligature_create() -> *mut GposMarkToLigatureSubtable {
    let mut x: *mut GposMarkToLigatureSubtable =
        malloc(::core::mem::size_of::<GposMarkToLigatureSubtable>() as usize)
            as *mut GposMarkToLigatureSubtable;
    subtable_gpos_mark_to_ligature_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gpos_mark_to_ligature_free(mut x: *mut GposMarkToLigatureSubtable) {
    if x.is_null() {
        return;
    }
    subtable_gpos_mark_to_ligature_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gpos_mark_to_ligature_dispose(
    mut x: *mut GposMarkToLigatureSubtable,
) {
    dispose_mark_to_ligature(x);
}
pub unsafe extern "C" fn otl_read_gpos_mark_to_ligature(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    _max_glyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut mark_array_offset: u32 = 0;
    let mut lig_array_offset: u32 = 0;
    let mut current_block: u64;
    let mut subtable: *mut GposMarkToLigatureSubtable =
        (
            I_SUBTABLE_GPOS_MARK_TO_LIGATURE
                .create
                .expect("non-null function pointer"))();
    let mut marks: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut bases: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    if !(table_length < offset.wrapping_add(12 as u32)) {
        marks = read_coverage(
            data as *const u8,
            table_length,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        bases = read_coverage(
            data as *const u8,
            table_length,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !(marks.is_null()
            || (*marks).num_glyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || bases.is_null()
            || (*bases).num_glyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            (*subtable).class_count = read_16u(
                data.offset(offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize) as *const u8,
            ) as GlyphClass;
            mark_array_offset = offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32);
            otl_read_mark_array(
                &raw mut (*subtable).mark_array,
                marks,
                data,
                table_length,
                mark_array_offset,
            );
            lig_array_offset = offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(10 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32);
            if !(table_length
                < lig_array_offset.wrapping_add(2 as u32).wrapping_add(
                    (2 as ::core::ffi::c_int * (*bases).num_glyphs as ::core::ffi::c_int)
                        as u32,
                ))
            {
                if !(read_16u(data.offset(lig_array_offset as isize) as *const u8)
                    as ::core::ffi::c_int
                    != (*bases).num_glyphs as ::core::ffi::c_int)
                {
                    let mut j: GlyphId = 0 as GlyphId;
                    loop {
                        if !((j as ::core::ffi::c_int) < (*bases).num_glyphs as ::core::ffi::c_int) {
                            current_block = 17788412896529399552;
                            break;
                        }
                        let mut lig: LigatureBaseRecord = LigatureBaseRecord {
                            glyph: Handle {
                                state: HandleState::Empty,
                                index: 0,
                                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                            component_count: 0,
                            anchors: ::core::ptr::null_mut::<*mut Anchor>(),
                        };
                        lig.glyph = otfcc_handle_dup(
                            *(*bases).glyphs.offset(j as isize) as Handle,
                        ) as GlyphHandle;
                        let mut lig_attach_offset: u32 = lig_array_offset.wrapping_add(read_16u(
                            data.offset(lig_array_offset as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(
                                    (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        )
                            as u32);
                        if table_length < lig_attach_offset.wrapping_add(2 as u32) {
                            current_block = 14470250473917821325;
                            break;
                        }
                        lig.component_count =
                            read_16u(data.offset(lig_attach_offset as isize) as *const u8)
                                as GlyphId;
                        if table_length
                            < lig_attach_offset.wrapping_add(2 as u32).wrapping_add(
                                (2 as ::core::ffi::c_int
                                    * lig.component_count as ::core::ffi::c_int
                                    * (*subtable).class_count as ::core::ffi::c_int)
                                    as u32,
                            )
                        {
                            current_block = 14470250473917821325;
                            break;
                        }
                        lig.anchors = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut Anchor>() as usize)
                                .wrapping_mul(lig.component_count as usize),
                            58 as ::core::ffi::c_ulong,
                        ) as *mut *mut Anchor;
                        let mut _offset: u32 = lig_attach_offset.wrapping_add(2 as u32);
                        let mut k: GlyphId = 0 as GlyphId;
                        while (k as ::core::ffi::c_int) < lig.component_count as ::core::ffi::c_int {
                            let ref mut fresh3 = *lig.anchors.offset(k as isize);
                            *fresh3 = __caryll_allocate_clean(
                                (::core::mem::size_of::<Anchor>() as usize)
                                    .wrapping_mul((*subtable).class_count as usize),
                                62 as ::core::ffi::c_ulong,
                            ) as *mut Anchor;
                            let mut m: GlyphClass = 0 as GlyphClass;
                            while (m as ::core::ffi::c_int)
                                < (*subtable).class_count as ::core::ffi::c_int
                            {
                                let mut anchor_offset: u32 =
                                    read_16u(data.offset(_offset as isize) as *const u8)
                                        as u32;
                                if anchor_offset != 0 {
                                    *(*lig.anchors.offset(k as isize)).offset(m as isize) =
                                        otl_read_anchor(
                                            data,
                                            table_length,
                                            lig_attach_offset.wrapping_add(anchor_offset),
                                        );
                                } else {
                                    *(*lig.anchors.offset(k as isize)).offset(m as isize) =
                                        otl_anchor_absent();
                                }
                                _offset = _offset.wrapping_add(2 as u32);
                                m = m.wrapping_add(1);
                            }
                            k = k.wrapping_add(1);
                        }
                        OTL_I_LIGATURE_ARRAY.push.expect("non-null function pointer")(
                            &raw mut (*subtable).lig_array,
                            lig,
                        );
                        j = j.wrapping_add(1);
                    }
                    match current_block {
                        14470250473917821325 => {}
                        _ => {
                            if !marks.is_null() {
                                otl_coverage_free(marks);
                            }
                            if !bases.is_null() {
                                otl_coverage_free(bases);
                            }
                            return subtable as *mut Subtable;
                        }
                    }
                }
            }
        }
    }
    if !marks.is_null() {
        otl_coverage_free(marks);
    }
    if !bases.is_null() {
        otl_coverage_free(bases);
    }
    I_SUBTABLE_GPOS_MARK_TO_LIGATURE
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gpos_dump_mark_to_ligature(
    mut st: *const Subtable,
) -> *mut JsonValue {
    let mut subtable: *const GposMarkToLigatureSubtable = &raw const (*st).gpos_mark_to_ligature;
    let mut _subtable: *mut JsonValue = json_object_new(3 as usize);
    let mut _marks: *mut JsonValue = json_object_new((*subtable).mark_array.length);
    let mut _bases: *mut JsonValue = json_object_new((*subtable).lig_array.length);
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).mark_array.length {
        let mut _mark: *mut JsonValue = json_object_new(3 as usize);
        let mut mark_class_name: SdsRaw = crate::sdsbuild!(
            sdsempty(),
            b"ac_",
            (*(*subtable).mark_array.items.offset(j as isize)).mark_class as ::core::ffi::c_int,
        );
        json_object_push(
            _mark,
            b"class\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new_length(
                sdslen(mark_class_name) as ::core::ffi::c_uint,
                mark_class_name as *const ::core::ffi::c_char,
            ),
        );
        sdsfree(mark_class_name);
        json_object_push(
            _mark,
            b"x\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*(*subtable).mark_array.items.offset(j as isize)).anchor.x as i64),
        );
        json_object_push(
            _mark,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*(*subtable).mark_array.items.offset(j as isize)).anchor.y as i64),
        );
        json_object_push(
            _marks,
            (*(*subtable).mark_array.items.offset(j as isize)).glyph.name
                as *const ::core::ffi::c_char,
            preserialize(_mark),
        );
        j = j.wrapping_add(1);
    }
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).lig_array.length {
        let mut base: *mut LigatureBaseRecord =
            (*subtable).lig_array.items.offset(j_0 as isize) as *mut LigatureBaseRecord;
        let mut _base: *mut JsonValue = json_array_new((*base).component_count as usize);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as ::core::ffi::c_int) < (*base).component_count as ::core::ffi::c_int {
            let mut _bk: *mut JsonValue = json_object_new((*subtable).class_count as usize);
            let mut m: GlyphClass = 0 as GlyphClass;
            while (m as ::core::ffi::c_int) < (*subtable).class_count as ::core::ffi::c_int {
                if (*(*(*base).anchors.offset(k as isize)).offset(m as isize)).present {
                    let mut _anchor: *mut JsonValue = json_object_new(2 as usize);
                    json_object_push(
                        _anchor,
                        b"x\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer_new(
                            (*(*(*base).anchors.offset(k as isize)).offset(m as isize)).x
                                as i64,
                        ),
                    );
                    json_object_push(
                        _anchor,
                        b"y\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer_new(
                            (*(*(*base).anchors.offset(k as isize)).offset(m as isize)).y
                                as i64,
                        ),
                    );
                    let mut mark_class_name_0: SdsRaw = crate::sdsbuild!(sdsempty(), b"ac_", m as ::core::ffi::c_int);
                    json_object_push_length(
                        _bk,
                        sdslen(mark_class_name_0) as ::core::ffi::c_uint,
                        mark_class_name_0 as *const ::core::ffi::c_char,
                        _anchor,
                    );
                    sdsfree(mark_class_name_0);
                }
                m = m.wrapping_add(1);
            }
            json_array_push(_base, _bk);
            k = k.wrapping_add(1);
        }
        json_object_push(
            _bases,
            (*base).glyph.name as *const ::core::ffi::c_char,
            preserialize(_base),
        );
        j_0 = j_0.wrapping_add(1);
    }
    json_object_push(
        _subtable,
        b"classCount\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*subtable).class_count as i64),
    );
    json_object_push(
        _subtable,
        b"marks\0" as *const u8 as *const ::core::ffi::c_char,
        _marks,
    );
    json_object_push(
        _subtable,
        b"bases\0" as *const u8 as *const ::core::ffi::c_char,
        _bases,
    );
    return _subtable;
}
unsafe extern "C" fn parse_bases(
    mut _bases: *mut JsonValue,
    mut subtable: *mut GposMarkToLigatureSubtable,
    mut h: *mut *mut ClassNameHash,
    mut options: *const Options,
) {
    let mut class_count: GlyphClass = (if !(*h).is_null() {
        (*(**h).hh.tbl).num_items
    } else {
        0 as ::core::ffi::c_uint
    }) as GlyphClass;
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < (*_bases).u.object.length {
        let mut gname: *mut ::core::ffi::c_char =
            (*(*_bases).u.object.values.offset(j as isize)).name;
        let mut lig: LigatureBaseRecord = LigatureBaseRecord {
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            component_count: 0,
            anchors: ::core::ptr::null_mut::<*mut Anchor>(),
        };
        lig.component_count = 0 as GlyphId;
        lig.anchors = ::core::ptr::null_mut::<*mut Anchor>();
        lig.glyph = handle_from_name(sdsnewlen(
            (*(*_bases).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
            (*(*_bases).u.object.values.offset(j as isize)).name_length as usize,
        )) as GlyphHandle;
        let mut base_record: *mut JsonValue =
            (*(*_bases).u.object.values.offset(j as isize)).value as *mut JsonValue;
        if base_record.is_null()
            || (*base_record).type_0 != JsonType::Array
        {
            OTL_I_LIGATURE_ARRAY.push.expect("non-null function pointer")(
                &raw mut (*subtable).lig_array,
                lig,
            );
        } else {
            lig.component_count = (*base_record).u.array.length as GlyphId;
            lig.anchors = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut Anchor>() as usize)
                    .wrapping_mul(lig.component_count as usize),
                146 as ::core::ffi::c_ulong,
            ) as *mut *mut Anchor;
            let mut k: GlyphId = 0 as GlyphId;
            while (k as ::core::ffi::c_int) < lig.component_count as ::core::ffi::c_int {
                let mut _component_record: *mut JsonValue =
                    *(*base_record).u.array.values.offset(k as isize) as *mut JsonValue;
                let ref mut fresh6 = *lig.anchors.offset(k as isize);
                *fresh6 = __caryll_allocate_clean(
                    (::core::mem::size_of::<Anchor>() as usize)
                        .wrapping_mul(class_count as usize),
                    150 as ::core::ffi::c_ulong,
                ) as *mut Anchor;
                let mut m: GlyphClass = 0 as GlyphClass;
                while (m as ::core::ffi::c_int) < class_count as ::core::ffi::c_int {
                    *(*lig.anchors.offset(k as isize)).offset(m as isize) = otl_anchor_absent();
                    m = m.wrapping_add(1);
                }
                if !(_component_record.is_null()
                    || (*_component_record).type_0 != JsonType::Object)
                {
                    let mut m_0: GlyphClass = 0 as GlyphClass;
                    while (m_0 as ::core::ffi::c_uint) < (*_component_record).u.object.length {
                        let mut class_name: SdsRaw = sdsnewlen(
                            (*(*_component_record).u.object.values.offset(m_0 as isize)).name
                                as *const ::core::ffi::c_void,
                            (*(*_component_record).u.object.values.offset(m_0 as isize)).name_length
                                as usize,
                        );
                        let mut s: *mut ClassNameHash =
                            ::core::ptr::null_mut::<ClassNameHash>();
                        let mut _hf_hashv: ::core::ffi::c_uint = 0;
                        let mut _hj_i: ::core::ffi::c_uint = 0;
                        let mut _hj_j: ::core::ffi::c_uint = 0;
                        let mut _hj_k: ::core::ffi::c_uint = 0;
                        let mut _hj_key: *const ::core::ffi::c_uchar =
                            class_name as *const ::core::ffi::c_uchar;
                        _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
                        _hj_i = _hj_j;
                        _hj_k =
                            strlen(class_name as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
                        while _hj_k >= 12 as ::core::ffi::c_uint {
                            _hj_i = _hj_i.wrapping_add(
                                (*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 8 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(
                                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 16 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(
                                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int,
                                    ),
                            );
                            _hj_j = _hj_j.wrapping_add(
                                (*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 8 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(
                                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 16 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(
                                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int,
                                    ),
                            );
                            _hf_hashv = _hf_hashv.wrapping_add(
                                (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 8 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(
                                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 16 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(
                                        (*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int,
                                    ),
                            );
                            _hj_i = _hj_i.wrapping_sub(_hj_j);
                            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                            _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
                            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                            _hj_j = _hj_j.wrapping_sub(_hj_i);
                            _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
                            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                            _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
                            _hj_i = _hj_i.wrapping_sub(_hj_j);
                            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                            _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
                            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                            _hj_j = _hj_j.wrapping_sub(_hj_i);
                            _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
                            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                            _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
                            _hj_i = _hj_i.wrapping_sub(_hj_j);
                            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                            _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
                            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                            _hj_j = _hj_j.wrapping_sub(_hj_i);
                            _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
                            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                            _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
                            _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
                            _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
                        }
                        _hf_hashv = _hf_hashv
                            .wrapping_add(strlen(class_name as *const ::core::ffi::c_char)
                                as ::core::ffi::c_uint);
                        let mut current_block_60: u64;
                        match _hj_k {
                            11 => {
                                _hf_hashv = _hf_hashv.wrapping_add(
                                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_60 = 9227261782747844496;
                            }
                            10 => {
                                current_block_60 = 9227261782747844496;
                            }
                            9 => {
                                current_block_60 = 18202155370509119360;
                            }
                            8 => {
                                current_block_60 = 5681848287071205093;
                            }
                            7 => {
                                current_block_60 = 4599947766850985381;
                            }
                            6 => {
                                current_block_60 = 1884041102650695646;
                            }
                            5 => {
                                current_block_60 = 4244705422846740112;
                            }
                            4 => {
                                current_block_60 = 12409020096634314305;
                            }
                            3 => {
                                current_block_60 = 12224275105439652028;
                            }
                            2 => {
                                current_block_60 = 16847718851714741986;
                            }
                            1 => {
                                current_block_60 = 17727222704389703247;
                            }
                            _ => {
                                current_block_60 = 2116367355679836638;
                            }
                        }
                        match current_block_60 {
                            9227261782747844496 => {
                                _hf_hashv = _hf_hashv.wrapping_add(
                                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_60 = 18202155370509119360;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            18202155370509119360 => {
                                _hf_hashv = _hf_hashv.wrapping_add(
                                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_60 = 5681848287071205093;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            5681848287071205093 => {
                                _hj_j = _hj_j.wrapping_add(
                                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_60 = 4599947766850985381;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            4599947766850985381 => {
                                _hj_j = _hj_j.wrapping_add(
                                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_60 = 1884041102650695646;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            1884041102650695646 => {
                                _hj_j = _hj_j.wrapping_add(
                                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_60 = 4244705422846740112;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            4244705422846740112 => {
                                _hj_j = _hj_j.wrapping_add(
                                    *_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint,
                                );
                                current_block_60 = 12409020096634314305;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            12409020096634314305 => {
                                _hj_i = _hj_i.wrapping_add(
                                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_60 = 12224275105439652028;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            12224275105439652028 => {
                                _hj_i = _hj_i.wrapping_add(
                                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_60 = 16847718851714741986;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            16847718851714741986 => {
                                _hj_i = _hj_i.wrapping_add(
                                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_60 = 17727222704389703247;
                            }
                            _ => {}
                        }
                        match current_block_60 {
                            17727222704389703247 => {
                                _hj_i = _hj_i.wrapping_add(
                                    *_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint,
                                );
                            }
                            _ => {}
                        }
                        _hj_i = _hj_i.wrapping_sub(_hj_j);
                        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                        _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
                        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                        _hj_j = _hj_j.wrapping_sub(_hj_i);
                        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
                        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                        _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
                        _hj_i = _hj_i.wrapping_sub(_hj_j);
                        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                        _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
                        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                        _hj_j = _hj_j.wrapping_sub(_hj_i);
                        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
                        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                        _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
                        _hj_i = _hj_i.wrapping_sub(_hj_j);
                        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                        _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
                        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                        _hj_j = _hj_j.wrapping_sub(_hj_i);
                        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
                        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                        _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
                        s = ::core::ptr::null_mut::<ClassNameHash>();
                        if !(*h).is_null() {
                            let mut _hf_bkt: ::core::ffi::c_uint = 0;
                            _hf_bkt = _hf_hashv
                                & (*(**h).hh.tbl)
                                    .num_buckets
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                if !(*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize))
                                    .hh_head
                                    .is_null()
                                {
                                    s = ((*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                                        as *mut ::core::ffi::c_char)
                                        .offset(-(*(**h).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut ClassNameHash
                                        as *mut ClassNameHash;
                                } else {
                                    s = ::core::ptr::null_mut::<ClassNameHash>();
                                }
                                while !s.is_null() {
                                    if (*s).hh.hashv == _hf_hashv
                                        && (*s).hh.keylen
                                            == strlen(class_name as *const ::core::ffi::c_char)
                                                as ::core::ffi::c_uint
                                    {
                                        if memcmp(
                                            (*s).hh.key,
                                            class_name as *const ::core::ffi::c_void,
                                            strlen(class_name as *const ::core::ffi::c_char)
                                                as ::core::ffi::c_uint
                                                as usize,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            break;
                                        }
                                    }
                                    if !(*s).hh.hh_next.is_null() {
                                        s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                            .offset(-(*(**h).hh.tbl).hho)
                                            as *mut ::core::ffi::c_void
                                            as *mut ClassNameHash
                                            as *mut ClassNameHash;
                                    } else {
                                        s = ::core::ptr::null_mut::<ClassNameHash>();
                                    }
                                }
                            }
                        }
                        if s.is_null() {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[OTFCC-fea] Invalid anchor class name <",
                                    class_name,
                                    b"> for /",
                                    gname,
                                    b". This base anchor is ignored.\n",
                                ),
                            );
                        } else {
                            *(*lig.anchors.offset(k as isize)).offset((*s).class_id as isize) =
                                otl_parse_anchor(
                                    (*(*_component_record).u.object.values.offset(m_0 as isize))
                                        .value
                                        as *mut JsonValue,
                                );
                        }
                        sdsfree(class_name);
                        m_0 = m_0.wrapping_add(1);
                    }
                }
                k = k.wrapping_add(1);
            }
            OTL_I_LIGATURE_ARRAY.push.expect("non-null function pointer")(
                &raw mut (*subtable).lig_array,
                lig,
            );
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe extern "C" fn otl_gpos_parse_mark_to_ligature(
    mut _subtable: *const JsonValue,
    mut options: *const Options,
) -> *mut Subtable {
    let mut _marks: *mut JsonValue = json_obj_get_type(
        _subtable,
        b"marks\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    let mut _bases: *mut JsonValue = json_obj_get_type(
        _subtable,
        b"bases\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if _marks.is_null() || _bases.is_null() {
        return ::core::ptr::null_mut::<Subtable>();
    }
    let mut st: *mut GposMarkToLigatureSubtable =
        (
            I_SUBTABLE_GPOS_MARK_TO_LIGATURE
                .create
                .expect("non-null function pointer"))();
    let mut h: *mut ClassNameHash = ::core::ptr::null_mut::<ClassNameHash>();
    otl_parse_mark_array(_marks, &raw mut (*st).mark_array, &raw mut h, options);
    (*st).class_count = (if !h.is_null() {
        (*(*h).hh.tbl).num_items
    } else {
        0 as ::core::ffi::c_uint
    }) as GlyphClass;
    parse_bases(_bases, st, &raw mut h, options);
    let mut s: *mut ClassNameHash = ::core::ptr::null_mut::<ClassNameHash>();
    let mut tmp: *mut ClassNameHash = ::core::ptr::null_mut::<ClassNameHash>();
    s = h;
    tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut ClassNameHash
        as *mut ClassNameHash;
    while !s.is_null() {
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*h).hh.tbl as *mut ::core::ffi::c_void);
            h = ::core::ptr::null_mut::<ClassNameHash>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*h).hh.tbl).tail {
                (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle
                    as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh4 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh4 = (*_hd_hh_del).next;
            } else {
                h = (*_hd_hh_del).next as *mut ClassNameHash as *mut ClassNameHash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh5 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh5 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket =
                (*(*h).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
            (*_hd_head).count = (*_hd_head).count.wrapping_sub(1);
            if (*_hd_head).hh_head == _hd_hh_del {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_sub(1);
        }
        sdsfree((*s).class_name);
        free(s as *mut ::core::ffi::c_void);
        s = ::core::ptr::null_mut::<ClassNameHash>();
        s = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut ClassNameHash
            as *mut ClassNameHash;
    }
    return st as *mut Subtable;
}
pub unsafe extern "C" fn otfcc_build_gpos_mark_to_ligature(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let mut subtable: *const GposMarkToLigatureSubtable =
        &raw const (*_subtable).gpos_mark_to_ligature;
    let mut marks: *mut Coverage = otl_coverage_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).mark_array.length {
        push_to_coverage(
            marks,
            otfcc_handle_dup(
                (*(*subtable).mark_array.items.offset(j as isize)).glyph as Handle,
            ) as GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let mut bases: *mut Coverage = otl_coverage_create();
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).lig_array.length {
        push_to_coverage(
            bases,
            otfcc_handle_dup(
                (*(*subtable).lig_array.items.offset(j_0 as isize)).glyph as Handle,
            ) as GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
            marks,
        ))), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
            bases,
        ))), bk_int(BkCellType::B16, ((*subtable).class_count as ::core::ffi::c_int) as u32)]);
    let mut mark_array: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*subtable).mark_array.length) as u32)]);
    let mut j_1: GlyphId = 0 as GlyphId;
    while (j_1 as usize) < (*subtable).mark_array.length {
        bk_push(mark_array, &[bk_int(BkCellType::B16, ((*(*subtable).mark_array.items.offset(j_1 as isize)).mark_class as ::core::ffi::c_int) as u32), bk_ptr(BkCellType::P16, bk_from_anchor((*(*subtable).mark_array.items.offset(j_1 as isize)).anchor))]);
        j_1 = j_1.wrapping_add(1);
    }
    let mut ligature_array: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*subtable).lig_array.length) as u32)]);
    let mut j_2: GlyphId = 0 as GlyphId;
    while (j_2 as usize) < (*subtable).lig_array.length {
        let mut attach: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*(*subtable).lig_array.items.offset(j_2 as isize)).component_count as ::core::ffi::c_int) as u32)]);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as ::core::ffi::c_int)
            < (*(*subtable).lig_array.items.offset(j_2 as isize)).component_count
                as ::core::ffi::c_int
        {
            let mut m: GlyphClass = 0 as GlyphClass;
            while (m as ::core::ffi::c_int) < (*subtable).class_count as ::core::ffi::c_int {
                bk_push(attach, &[bk_ptr(BkCellType::P16, bk_from_anchor(
                        *(*(*(*subtable).lig_array.items.offset(j_2 as isize))
                            .anchors
                            .offset(k as isize))
                        .offset(m as isize),
                    ))]);
                m = m.wrapping_add(1);
            }
            k = k.wrapping_add(1);
        }
        bk_push(ligature_array, &[bk_ptr(BkCellType::P16, attach)]);
        j_2 = j_2.wrapping_add(1);
    }
    bk_push(root, &[bk_ptr(BkCellType::P16, mark_array), bk_ptr(BkCellType::P16, ligature_array)]);
    otl_coverage_free(marks);
    otl_coverage_free(bases);
    return bk_build_block(root);
}
