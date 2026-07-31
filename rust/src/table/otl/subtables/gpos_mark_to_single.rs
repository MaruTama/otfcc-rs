#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcmp, memcpy, strlen};


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
use crate::support::cvec::{CVecRaw, cvec_grow_to, cvec_init, cvec_push};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::support::{NULL};
use crate::table::otl::{GposMarkToSingleSubtableElementInterface, BaseArrayVectorInterface, Anchor, BaseArray, BaseRecord, Subtable, GposMarkToSingleSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::table::otl::subtables::gpos_common::{ClassNameHash};
use crate::vendor::uthash::{UtHashBucket, UtHashHandle};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::table::otl::subtables::gpos_common::{bk_from_anchor, otl_anchor_absent, OTL_I_MARK_ARRAY, otl_parse_mark_array, otl_parse_anchor, otl_read_mark_array, otl_read_anchor};
use crate::vendor::json_builder::{json_integer_new, json_object_new, json_object_push, json_object_push_length, json_string_new_length};
use crate::vendor::sds::{sdsempty, sdsfree, sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseRecordElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut BaseRecord) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut BaseRecord, *const BaseRecord) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut BaseRecord) -> ()>,
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
unsafe extern "C" fn delete_base_array_item(mut entry: *mut BaseRecord) {
    otfcc_handle_dispose(&raw mut (*entry).glyph);
    free((*entry).anchors as *mut ::core::ffi::c_void);
    (*entry).anchors = ::core::ptr::null_mut::<Anchor>();
}
static BA_TYPEINFO: BaseRecordElementInterface = {
    BaseRecordElementInterface {
        init: None,
        copy: None,
        dispose: Some(delete_base_array_item as unsafe extern "C" fn(*mut BaseRecord) -> ()),
    }
};
#[inline]
unsafe extern "C" fn otl_base_array_push(arr: *mut BaseArray, elem: BaseRecord) {
    cvec_push(otl_base_array_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_base_array_grow_to(arr: *mut BaseArray, target: usize) {
    cvec_grow_to(otl_base_array_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_base_array_copy(
    mut dst: *mut BaseArray,
    mut src: *const BaseArray,
) {
    otl_base_array_init(dst);
    otl_base_array_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if BA_TYPEINFO.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            BA_TYPEINFO.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut BaseRecord,
                (*src).items.offset(j as isize) as *mut BaseRecord as *const BaseRecord,
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
unsafe extern "C" fn otl_base_array_dispose(mut arr: *mut BaseArray) {
    if arr.is_null() {
        return;
    }
    if BA_TYPEINFO.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            BA_TYPEINFO.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut BaseRecord,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<BaseRecord>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_base_array_free(mut x: *mut BaseArray) {
    if x.is_null() {
        return;
    }
    otl_base_array_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe fn otl_base_array_as_cvec(arr: *mut BaseArray) -> *mut CVecRaw<BaseRecord> {
    arr as *mut CVecRaw<BaseRecord>
}
#[inline]
unsafe extern "C" fn otl_base_array_init(arr: *mut BaseArray) {
    cvec_init(otl_base_array_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_base_array_create() -> *mut BaseArray {
    let mut x: *mut BaseArray =
        malloc(::core::mem::size_of::<BaseArray>() as usize) as *mut BaseArray;
    otl_base_array_init(x);
    return x;
}
pub static OTL_I_BASE_ARRAY: BaseArrayVectorInterface = {
    BaseArrayVectorInterface {
        init: Some(otl_base_array_init as unsafe extern "C" fn(*mut BaseArray) -> ()),
        copy: Some(
            otl_base_array_copy
                as unsafe extern "C" fn(*mut BaseArray, *const BaseArray) -> (),
        ),
        dispose: Some(otl_base_array_dispose as unsafe extern "C" fn(*mut BaseArray) -> ()),
        create: Some(otl_base_array_create),
        free: Some(otl_base_array_free as unsafe extern "C" fn(*mut BaseArray) -> ()),
        clear: Some(otl_base_array_dispose as unsafe extern "C" fn(*mut BaseArray) -> ()),
        push: Some(
            otl_base_array_push as unsafe extern "C" fn(*mut BaseArray, BaseRecord) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn init_mark_to_single(mut subtable: *mut GposMarkToSingleSubtable) {
    OTL_I_MARK_ARRAY.init.expect("non-null function pointer")(&raw mut (*subtable).mark_array);
    OTL_I_BASE_ARRAY.init.expect("non-null function pointer")(&raw mut (*subtable).base_array);
}
#[inline]
unsafe extern "C" fn dispose_mark_to_single(mut subtable: *mut GposMarkToSingleSubtable) {
    OTL_I_MARK_ARRAY.dispose.expect("non-null function pointer")(&raw mut (*subtable).mark_array);
    OTL_I_BASE_ARRAY.dispose.expect("non-null function pointer")(&raw mut (*subtable).base_array);
}
#[inline]
unsafe extern "C" fn subtable_gpos_mark_to_single_copy(
    mut dst: *mut GposMarkToSingleSubtable,
    mut src: *const GposMarkToSingleSubtable,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GposMarkToSingleSubtable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_mark_to_single_free(mut x: *mut GposMarkToSingleSubtable) {
    if x.is_null() {
        return;
    }
    subtable_gpos_mark_to_single_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gpos_mark_to_single_init(mut x: *mut GposMarkToSingleSubtable) {
    init_mark_to_single(x);
}
#[inline]
unsafe extern "C" fn subtable_gpos_mark_to_single_dispose(mut x: *mut GposMarkToSingleSubtable) {
    dispose_mark_to_single(x);
}
pub static I_SUBTABLE_GPOS_MARK_TO_SINGLE: GposMarkToSingleSubtableElementInterface = {
    GposMarkToSingleSubtableElementInterface {
        init: Some(
            subtable_gpos_mark_to_single_init
                as unsafe extern "C" fn(*mut GposMarkToSingleSubtable) -> (),
        ),
        copy: Some(
            subtable_gpos_mark_to_single_copy
                as unsafe extern "C" fn(
                    *mut GposMarkToSingleSubtable,
                    *const GposMarkToSingleSubtable,
                ) -> (),
        ),
        dispose: Some(
            subtable_gpos_mark_to_single_dispose
                as unsafe extern "C" fn(*mut GposMarkToSingleSubtable) -> (),
        ),
        create: Some(subtable_gpos_mark_to_single_create),
        free: Some(
            subtable_gpos_mark_to_single_free
                as unsafe extern "C" fn(*mut GposMarkToSingleSubtable) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_mark_to_single_create() -> *mut GposMarkToSingleSubtable {
    let mut x: *mut GposMarkToSingleSubtable =
        malloc(::core::mem::size_of::<GposMarkToSingleSubtable>() as usize)
            as *mut GposMarkToSingleSubtable;
    subtable_gpos_mark_to_single_init(x);
    return x;
}
pub unsafe extern "C" fn otl_read_gpos_mark_to_single(
    data: FontFilePointer,
    mut table_length: u32,
    mut subtable_offset: u32,
    _max_glyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut mark_array_offset: u32 = 0;
    let mut base_array_offset: u32 = 0;
    let mut _offset: u32 = 0;
    let mut subtable: *mut GposMarkToSingleSubtable =
        (
            I_SUBTABLE_GPOS_MARK_TO_SINGLE
                .create
                .expect("non-null function pointer"))();
    let mut marks: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut bases: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    if !(table_length < subtable_offset.wrapping_add(12 as u32)) {
        marks = read_coverage(
            data as *const u8,
            table_length,
            subtable_offset.wrapping_add(read_16u(
                data.offset(subtable_offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        bases = read_coverage(
            data as *const u8,
            table_length,
            subtable_offset.wrapping_add(read_16u(
                data.offset(subtable_offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !(marks.is_null()
            || (*marks).num_glyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || bases.is_null()
            || (*bases).num_glyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            (*subtable).class_count = read_16u(
                data.offset(subtable_offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize) as *const u8,
            ) as GlyphClass;
            mark_array_offset = subtable_offset.wrapping_add(read_16u(
                data.offset(subtable_offset as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32);
            otl_read_mark_array(
                &raw mut (*subtable).mark_array,
                marks,
                data,
                table_length,
                mark_array_offset,
            );
            base_array_offset = subtable_offset.wrapping_add(read_16u(
                data.offset(subtable_offset as isize)
                    .offset(10 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32);
            if !(table_length
                < base_array_offset.wrapping_add(2 as u32).wrapping_add(
                    (2 as ::core::ffi::c_int
                        * (*bases).num_glyphs as ::core::ffi::c_int
                        * (*subtable).class_count as ::core::ffi::c_int)
                        as u32,
                ))
            {
                if !(read_16u(data.offset(base_array_offset as isize) as *const u8)
                    as ::core::ffi::c_int
                    != (*bases).num_glyphs as ::core::ffi::c_int)
                {
                    _offset = base_array_offset.wrapping_add(2 as u32);
                    let mut j: GlyphId = 0 as GlyphId;
                    while (j as ::core::ffi::c_int) < (*bases).num_glyphs as ::core::ffi::c_int {
                        let mut base_anchors: *mut Anchor =
                            ::core::ptr::null_mut::<Anchor>();
                        base_anchors = __caryll_allocate_clean(
                            (::core::mem::size_of::<Anchor>() as usize)
                                .wrapping_mul((*subtable).class_count as usize),
                            49 as ::core::ffi::c_ulong,
                        ) as *mut Anchor;
                        let mut k: GlyphClass = 0 as GlyphClass;
                        while (k as ::core::ffi::c_int)
                            < (*subtable).class_count as ::core::ffi::c_int
                        {
                            if read_16u(data.offset(_offset as isize) as *const u8) != 0 {
                                *base_anchors.offset(k as isize) = otl_read_anchor(
                                    data,
                                    table_length,
                                    base_array_offset.wrapping_add(read_16u(
                                        data.offset(_offset as isize) as *const u8,
                                    )
                                        as u32),
                                );
                            } else {
                                *base_anchors.offset(k as isize) = otl_anchor_absent();
                            }
                            _offset = _offset.wrapping_add(2 as u32);
                            k = k.wrapping_add(1);
                        }
                        OTL_I_BASE_ARRAY.push.expect("non-null function pointer")(
                            &raw mut (*subtable).base_array,
                            BaseRecord {
                                glyph: otfcc_handle_dup(
                                    *(*bases).glyphs.offset(j as isize) as Handle,
                                ) as GlyphHandle,
                                anchors: base_anchors,
                            },
                        );
                        j = j.wrapping_add(1);
                    }
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
    I_SUBTABLE_GPOS_MARK_TO_SINGLE
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gpos_dump_mark_to_single(
    mut st: *const Subtable,
) -> *mut JsonValue {
    let mut subtable: *const GposMarkToSingleSubtable = &raw const (*st).gpos_mark_to_single;
    let mut _subtable: *mut JsonValue = json_object_new(3 as usize);
    let mut _marks: *mut JsonValue = json_object_new((*subtable).mark_array.length);
    let mut _bases: *mut JsonValue = json_object_new((*subtable).base_array.length);
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).mark_array.length {
        let mut _mark: *mut JsonValue = json_object_new(3 as usize);
        let mut mark_class_name: SdsRaw = crate::sdsbuild!(
            sdsempty(),
            b"anchor",
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
    while (j_0 as usize) < (*subtable).base_array.length {
        let mut _base: *mut JsonValue = json_object_new((*subtable).class_count as usize);
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < (*subtable).class_count as ::core::ffi::c_int {
            if (*(*(*subtable).base_array.items.offset(j_0 as isize))
                .anchors
                .offset(k as isize))
            .present
            {
                let mut _anchor: *mut JsonValue = json_object_new(2 as usize);
                json_object_push(
                    _anchor,
                    b"x\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(
                        (*(*(*subtable).base_array.items.offset(j_0 as isize))
                            .anchors
                            .offset(k as isize))
                        .x as i64,
                    ),
                );
                json_object_push(
                    _anchor,
                    b"y\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(
                        (*(*(*subtable).base_array.items.offset(j_0 as isize))
                            .anchors
                            .offset(k as isize))
                        .y as i64,
                    ),
                );
                let mut mark_class_name_0: SdsRaw = crate::sdsbuild!(sdsempty(), b"anchor", k as ::core::ffi::c_int);
                json_object_push_length(
                    _base,
                    sdslen(mark_class_name_0) as ::core::ffi::c_uint,
                    mark_class_name_0 as *const ::core::ffi::c_char,
                    _anchor,
                );
                sdsfree(mark_class_name_0);
            }
            k = k.wrapping_add(1);
        }
        json_object_push(
            _bases,
            (*(*subtable).base_array.items.offset(j_0 as isize))
                .glyph
                .name as *const ::core::ffi::c_char,
            preserialize(_base),
        );
        j_0 = j_0.wrapping_add(1);
    }
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
    mut subtable: *mut GposMarkToSingleSubtable,
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
        let mut base: BaseRecord = BaseRecord {
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            anchors: ::core::ptr::null_mut::<Anchor>(),
        };
        base.glyph = handle_from_name(sdsnewlen(
            gname as *const ::core::ffi::c_void,
            (*(*_bases).u.object.values.offset(j as isize)).name_length as usize,
        )) as GlyphHandle;
        base.anchors = __caryll_allocate_clean(
            (::core::mem::size_of::<Anchor>() as usize).wrapping_mul(class_count as usize),
            116 as ::core::ffi::c_ulong,
        ) as *mut Anchor;
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < class_count as ::core::ffi::c_int {
            *base.anchors.offset(k as isize) = otl_anchor_absent();
            k = k.wrapping_add(1);
        }
        let mut base_record: *mut JsonValue =
            (*(*_bases).u.object.values.offset(j as isize)).value as *mut JsonValue;
        if base_record.is_null()
            || (*base_record).type_0 != JsonType::Object
        {
            OTL_I_BASE_ARRAY.push.expect("non-null function pointer")(
                &raw mut (*subtable).base_array,
                base,
            );
        } else {
            let mut k_0: GlyphClass = 0 as GlyphClass;
            while (k_0 as ::core::ffi::c_uint) < (*base_record).u.object.length {
                let mut class_name: SdsRaw = sdsnewlen(
                    (*(*base_record).u.object.values.offset(k_0 as isize)).name
                        as *const ::core::ffi::c_void,
                    (*(*base_record).u.object.values.offset(k_0 as isize)).name_length as usize,
                );
                let mut s: *mut ClassNameHash = ::core::ptr::null_mut::<ClassNameHash>();
                let mut _hf_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i: ::core::ffi::c_uint = 0;
                let mut _hj_j: ::core::ffi::c_uint = 0;
                let mut _hj_k: ::core::ffi::c_uint = 0;
                let mut _hj_key: *const ::core::ffi::c_uchar =
                    class_name as *const ::core::ffi::c_uchar;
                _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i = _hj_j;
                _hj_k = strlen(class_name as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
                while _hj_k >= 12 as ::core::ffi::c_uint {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
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
                        (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
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
                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
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
                _hf_hashv = _hf_hashv.wrapping_add(
                    strlen(class_name as *const ::core::ffi::c_char) as ::core::ffi::c_uint
                );
                let mut current_block_56: u64;
                match _hj_k {
                    11 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_56 = 14536411282452634839;
                    }
                    10 => {
                        current_block_56 = 14536411282452634839;
                    }
                    9 => {
                        current_block_56 = 9913348930486913067;
                    }
                    8 => {
                        current_block_56 = 1505195771936801158;
                    }
                    7 => {
                        current_block_56 = 15021600489117130768;
                    }
                    6 => {
                        current_block_56 = 8233865231112875104;
                    }
                    5 => {
                        current_block_56 = 3771526520438017190;
                    }
                    4 => {
                        current_block_56 = 6788034837040873263;
                    }
                    3 => {
                        current_block_56 = 17257476062468164659;
                    }
                    2 => {
                        current_block_56 = 16976244951184097103;
                    }
                    1 => {
                        current_block_56 = 14519719227392997025;
                    }
                    _ => {
                        current_block_56 = 8151474771948790331;
                    }
                }
                match current_block_56 {
                    14536411282452634839 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_56 = 9913348930486913067;
                    }
                    _ => {}
                }
                match current_block_56 {
                    9913348930486913067 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_56 = 1505195771936801158;
                    }
                    _ => {}
                }
                match current_block_56 {
                    1505195771936801158 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_56 = 15021600489117130768;
                    }
                    _ => {}
                }
                match current_block_56 {
                    15021600489117130768 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_56 = 8233865231112875104;
                    }
                    _ => {}
                }
                match current_block_56 {
                    8233865231112875104 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_56 = 3771526520438017190;
                    }
                    _ => {}
                }
                match current_block_56 {
                    3771526520438017190 => {
                        _hj_j = _hj_j
                            .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_56 = 6788034837040873263;
                    }
                    _ => {}
                }
                match current_block_56 {
                    6788034837040873263 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_56 = 17257476062468164659;
                    }
                    _ => {}
                }
                match current_block_56 {
                    17257476062468164659 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_56 = 16976244951184097103;
                    }
                    _ => {}
                }
                match current_block_56 {
                    16976244951184097103 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_56 = 14519719227392997025;
                    }
                    _ => {}
                }
                match current_block_56 {
                    14519719227392997025 => {
                        _hj_i = _hj_i
                            .wrapping_add(*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
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
                    *base.anchors.offset((*s).class_id as isize) = otl_parse_anchor(
                        (*(*base_record).u.object.values.offset(k_0 as isize)).value
                            as *mut JsonValue,
                    );
                }
                sdsfree(class_name);
                k_0 = k_0.wrapping_add(1);
            }
            OTL_I_BASE_ARRAY.push.expect("non-null function pointer")(
                &raw mut (*subtable).base_array,
                base,
            );
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe extern "C" fn otl_gpos_parse_mark_to_single(
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
    let mut st: *mut GposMarkToSingleSubtable =
        (
            I_SUBTABLE_GPOS_MARK_TO_SINGLE
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
                let ref mut fresh2 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh2 = (*_hd_hh_del).next;
            } else {
                h = (*_hd_hh_del).next as *mut ClassNameHash as *mut ClassNameHash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh3 = (*_hd_hh_del).prev;
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
pub unsafe extern "C" fn otfcc_build_gpos_mark_to_single(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let mut subtable: *const GposMarkToSingleSubtable = &raw const (*_subtable).gpos_mark_to_single;
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
    while (j_0 as usize) < (*subtable).base_array.length {
        push_to_coverage(
            bases,
            otfcc_handle_dup(
                (*(*subtable).base_array.items.offset(j_0 as isize)).glyph as Handle,
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
    let mut base_array: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*subtable).base_array.length) as u32)]);
    let mut j_2: GlyphId = 0 as GlyphId;
    while (j_2 as usize) < (*subtable).base_array.length {
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < (*subtable).class_count as ::core::ffi::c_int {
            bk_push(base_array, &[bk_ptr(BkCellType::P16, bk_from_anchor(
                    *(*(*subtable).base_array.items.offset(j_2 as isize))
                        .anchors
                        .offset(k as isize),
                ))]);
            k = k.wrapping_add(1);
        }
        j_2 = j_2.wrapping_add(1);
    }
    bk_push(root, &[bk_ptr(BkCellType::P16, mark_array), bk_ptr(BkCellType::P16, base_array)]);
    otl_coverage_free(marks);
    otl_coverage_free(bases);
    return bk_build_block(root);
}
