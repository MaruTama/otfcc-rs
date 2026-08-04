#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};


use crate::support::json_funcs::{json_obj_get_type, preserialize};
use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_name, otfcc_handle_dispose, otfcc_handle_dup, Handle, GlyphHandle, HandleState};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphClass, GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::table::otl::{Anchor, BaseArray, BaseRecord, Subtable, GposMarkToSingleSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::table::otl::subtables::gpos_common::{bk_from_anchor, otl_anchor_absent, dispose_mark_array, otl_parse_mark_array, otl_parse_anchor, otl_read_mark_array, otl_read_anchor};
use crate::vendor::json_builder::{json_integer_new, json_object_new, json_object_push, json_object_push_length, json_string_new_length};
use crate::vendor::sds::{sdsempty, sdsfree, sdslen, sdsnewlen};
unsafe extern "C" fn delete_base_array_item(mut entry: *mut BaseRecord) {
    otfcc_handle_dispose(&raw mut (*entry).glyph);
    free((*entry).anchors as *mut ::core::ffi::c_void);
    (*entry).anchors = ::core::ptr::null_mut::<Anchor>();
}
pub(crate) unsafe fn dispose_base_array(arr: *mut BaseArray) {
    for e in (*arr).iter_mut() {
        delete_base_array_item(e);
    }
    *arr = Vec::new();
}
unsafe extern "C" fn init_mark_to_single(subtable: *mut GposMarkToSingleSubtable) {
    (*subtable).mark_array = Vec::new();
    (*subtable).base_array = Vec::new();
}
unsafe extern "C" fn dispose_mark_to_single(subtable: *mut GposMarkToSingleSubtable) {
    dispose_mark_array(&raw mut (*subtable).mark_array);
    dispose_base_array(&raw mut (*subtable).base_array);
}
pub(crate) unsafe extern "C" fn subtable_gpos_mark_to_single_free(x: *mut GposMarkToSingleSubtable) {
    if x.is_null() {
        return;
    }
    dispose_mark_to_single(x);
    free(x as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn subtable_gpos_mark_to_single_create() -> *mut GposMarkToSingleSubtable {
    let x: *mut GposMarkToSingleSubtable = __caryll_allocate_clean(
        ::core::mem::size_of::<GposMarkToSingleSubtable>() as usize,
        0,
    ) as *mut GposMarkToSingleSubtable;
    init_mark_to_single(x);
    x
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
    let mut subtable: *mut GposMarkToSingleSubtable = subtable_gpos_mark_to_single_create();
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
            || (*marks).len() as GlyphId as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || bases.is_null()
            || (*bases).len() as GlyphId as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
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
                        * (*bases).len() as GlyphId as ::core::ffi::c_int
                        * (*subtable).class_count as ::core::ffi::c_int)
                        as u32,
                ))
            {
                if !(read_16u(data.offset(base_array_offset as isize) as *const u8)
                    as ::core::ffi::c_int
                    != (*bases).len() as GlyphId as ::core::ffi::c_int)
                {
                    _offset = base_array_offset.wrapping_add(2 as u32);
                    let mut j: GlyphId = 0 as GlyphId;
                    while (j as ::core::ffi::c_int) < (*bases).len() as GlyphId as ::core::ffi::c_int {
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
                        (*subtable).base_array.push(
                            BaseRecord {
                                glyph: otfcc_handle_dup(
                                    (&(*bases))[j as usize].clone() as Handle,
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
    subtable_gpos_mark_to_single_free(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gpos_dump_mark_to_single(
    mut st: *const Subtable,
) -> *mut JsonValue {
    let mut subtable: *const GposMarkToSingleSubtable = &raw const (*st).gpos_mark_to_single as *const GposMarkToSingleSubtable;
    let mut _subtable: *mut JsonValue = json_object_new(3 as usize);
    let mut _marks: *mut JsonValue = json_object_new((*subtable).mark_array.len());
    let mut _bases: *mut JsonValue = json_object_new((*subtable).base_array.len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).mark_array.len() {
        let mut _mark: *mut JsonValue = json_object_new(3 as usize);
        let mut mark_class_name: SdsRaw = crate::sdsbuild!(
            sdsempty(),
            b"anchor",
            (&(*subtable).mark_array)[j as usize].mark_class as ::core::ffi::c_int,
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
            json_integer_new((&(*subtable).mark_array)[j as usize].anchor.x as i64),
        );
        json_object_push(
            _mark,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((&(*subtable).mark_array)[j as usize].anchor.y as i64),
        );
        json_object_push(
            _marks,
            (&(*subtable).mark_array)[j as usize].glyph.name
                as *const ::core::ffi::c_char,
            preserialize(_mark),
        );
        j = j.wrapping_add(1);
    }
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).base_array.len() {
        let mut _base: *mut JsonValue = json_object_new((*subtable).class_count as usize);
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < (*subtable).class_count as ::core::ffi::c_int {
            if (*(&(*subtable).base_array)[j_0 as usize]
                .anchors
                .offset(k as isize))
            .present
            {
                let mut _anchor: *mut JsonValue = json_object_new(2 as usize);
                json_object_push(
                    _anchor,
                    b"x\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(
                        (*(&(*subtable).base_array)[j_0 as usize]
                            .anchors
                            .offset(k as isize))
                        .x as i64,
                    ),
                );
                json_object_push(
                    _anchor,
                    b"y\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(
                        (*(&(*subtable).base_array)[j_0 as usize]
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
            (&(*subtable).base_array)[j_0 as usize]
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
    mut h: *mut std::collections::BTreeMap<Vec<u8>, GlyphClass>,
    mut options: *const Options,
) {
    let class_count: GlyphClass = (*h).len() as GlyphClass;
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
            (*subtable).base_array.push(base);
        } else {
            let mut k_0: GlyphClass = 0 as GlyphClass;
            while (k_0 as ::core::ffi::c_uint) < (*base_record).u.object.length {
                let name_ptr: *mut ::core::ffi::c_char =
                    (*(*base_record).u.object.values.offset(k_0 as isize)).name;
                // `strlen`-bounded, matching `otl_parse_mark_array`'s
                // registration key exactly.
                let class_name: Vec<u8> =
                    ::core::ffi::CStr::from_ptr(name_ptr).to_bytes().to_vec();
                match (*h).get(&class_name) {
                    None => {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::sdsbuild!(
                                sdsempty(),
                                b"[OTFCC-fea] Invalid anchor class name <",
                                name_ptr,
                                b"> for /",
                                gname,
                                b". This base anchor is ignored.\n",
                            ),
                        );
                    }
                    Some(&class_id) => {
                        *base.anchors.offset(class_id as isize) = otl_parse_anchor(
                            (*(*base_record).u.object.values.offset(k_0 as isize)).value
                                as *mut JsonValue,
                        );
                    }
                }
                k_0 = k_0.wrapping_add(1);
            }
            (*subtable).base_array.push(base);
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
    let mut st: *mut GposMarkToSingleSubtable = subtable_gpos_mark_to_single_create();
    let mut h: std::collections::BTreeMap<Vec<u8>, GlyphClass> = std::collections::BTreeMap::new();
    otl_parse_mark_array(_marks, &raw mut (*st).mark_array, &raw mut h, options);
    (*st).class_count = h.len() as GlyphClass;
    parse_bases(_bases, st, &raw mut h, options);
    return st as *mut Subtable;
}
pub unsafe extern "C" fn otfcc_build_gpos_mark_to_single(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let mut subtable: *const GposMarkToSingleSubtable = &raw const (*_subtable).gpos_mark_to_single as *const GposMarkToSingleSubtable;
    let mut marks: *mut Coverage = otl_coverage_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).mark_array.len() {
        push_to_coverage(
            marks,
            otfcc_handle_dup(
                (&(*subtable).mark_array)[j as usize].glyph.clone() as Handle,
            ) as GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let mut bases: *mut Coverage = otl_coverage_create();
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).base_array.len() {
        push_to_coverage(
            bases,
            otfcc_handle_dup(
                (&(*subtable).base_array)[j_0 as usize].glyph.clone() as Handle,
            ) as GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
            marks,
        ))), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
            bases,
        ))), bk_int(BkCellType::B16, ((*subtable).class_count as ::core::ffi::c_int) as u32)]);
    let mut mark_array: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*subtable).mark_array.len()) as u32)]);
    let mut j_1: GlyphId = 0 as GlyphId;
    while (j_1 as usize) < (*subtable).mark_array.len() {
        bk_push(mark_array, &[bk_int(BkCellType::B16, ((&(*subtable).mark_array)[j_1 as usize].mark_class as ::core::ffi::c_int) as u32), bk_ptr(BkCellType::P16, bk_from_anchor((&(*subtable).mark_array)[j_1 as usize].anchor))]);
        j_1 = j_1.wrapping_add(1);
    }
    let mut base_array: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*subtable).base_array.len()) as u32)]);
    let mut j_2: GlyphId = 0 as GlyphId;
    while (j_2 as usize) < (*subtable).base_array.len() {
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < (*subtable).class_count as ::core::ffi::c_int {
            bk_push(base_array, &[bk_ptr(BkCellType::P16, bk_from_anchor(
                    *(&(*subtable).base_array)[j_2 as usize]
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
