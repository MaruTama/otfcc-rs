#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, memcmp, strlen};


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
use crate::support::{NULL};
use crate::table::otl::{Anchor, LigatureArray, LigatureBaseRecord, Subtable, GposMarkToLigatureSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::table::otl::subtables::gpos_common::{ClassNameHash};
use crate::vendor::uthash::{UtHashBucket, UtHashHandle};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::table::otl::subtables::gpos_common::{bk_from_anchor, otl_anchor_absent, dispose_mark_array, otl_parse_mark_array, otl_parse_anchor, otl_read_mark_array, otl_read_anchor};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_object_push_length, json_string_new_length};
use crate::vendor::sds::{sdsempty, sdsfree, sdslen, sdsnewlen};
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
pub(crate) unsafe fn dispose_lig_array(arr: *mut LigatureArray) {
    for e in (*arr).iter_mut() {
        delete_lig_array_item(e);
    }
    *arr = Vec::new();
}
unsafe extern "C" fn init_mark_to_ligature(subtable: *mut GposMarkToLigatureSubtable) {
    (*subtable).mark_array = Vec::new();
    (*subtable).lig_array = Vec::new();
}
unsafe extern "C" fn dispose_mark_to_ligature(subtable: *mut GposMarkToLigatureSubtable) {
    dispose_mark_array(&raw mut (*subtable).mark_array);
    dispose_lig_array(&raw mut (*subtable).lig_array);
}
pub(crate) unsafe extern "C" fn subtable_gpos_mark_to_ligature_free(x: *mut GposMarkToLigatureSubtable) {
    if x.is_null() {
        return;
    }
    dispose_mark_to_ligature(x);
    free(x as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn subtable_gpos_mark_to_ligature_create() -> *mut GposMarkToLigatureSubtable {
    let x: *mut GposMarkToLigatureSubtable = __caryll_allocate_clean(
        ::core::mem::size_of::<GposMarkToLigatureSubtable>() as usize,
        0,
    ) as *mut GposMarkToLigatureSubtable;
    init_mark_to_ligature(x);
    x
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
    let mut subtable: *mut GposMarkToLigatureSubtable = subtable_gpos_mark_to_ligature_create();
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
            || (*marks).len() as GlyphId as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || bases.is_null()
            || (*bases).len() as GlyphId as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
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
                    (2 as ::core::ffi::c_int * (*bases).len() as GlyphId as ::core::ffi::c_int)
                        as u32,
                ))
            {
                if !(read_16u(data.offset(lig_array_offset as isize) as *const u8)
                    as ::core::ffi::c_int
                    != (*bases).len() as GlyphId as ::core::ffi::c_int)
                {
                    let mut j: GlyphId = 0 as GlyphId;
                    loop {
                        if !((j as ::core::ffi::c_int) < (*bases).len() as GlyphId as ::core::ffi::c_int) {
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
                            (&(*bases))[j as usize].clone() as Handle,
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
                        (*subtable).lig_array.push(lig);
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
    subtable_gpos_mark_to_ligature_free(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gpos_dump_mark_to_ligature(
    mut st: *const Subtable,
) -> *mut JsonValue {
    let mut subtable: *const GposMarkToLigatureSubtable = &raw const (*st).gpos_mark_to_ligature as *const GposMarkToLigatureSubtable;
    let mut _subtable: *mut JsonValue = json_object_new(3 as usize);
    let mut _marks: *mut JsonValue = json_object_new((*subtable).mark_array.len());
    let mut _bases: *mut JsonValue = json_object_new((*subtable).lig_array.len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).mark_array.len() {
        let mut _mark: *mut JsonValue = json_object_new(3 as usize);
        let mut mark_class_name: SdsRaw = crate::sdsbuild!(
            sdsempty(),
            b"ac_",
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
    while (j_0 as usize) < (*subtable).lig_array.len() {
        let base: *const LigatureBaseRecord = &(&(*subtable).lig_array)[j_0 as usize] as *const LigatureBaseRecord;
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
            (*subtable).lig_array.push(lig);
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
            (*subtable).lig_array.push(lig);
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
    let mut st: *mut GposMarkToLigatureSubtable = subtable_gpos_mark_to_ligature_create();
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
        &raw const (*_subtable).gpos_mark_to_ligature as *const GposMarkToLigatureSubtable;
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
    while (j_0 as usize) < (*subtable).lig_array.len() {
        push_to_coverage(
            bases,
            otfcc_handle_dup(
                (&(*subtable).lig_array)[j_0 as usize].glyph.clone() as Handle,
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
    let mut ligature_array: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*subtable).lig_array.len()) as u32)]);
    let mut j_2: GlyphId = 0 as GlyphId;
    while (j_2 as usize) < (*subtable).lig_array.len() {
        let mut attach: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((&(*subtable).lig_array)[j_2 as usize].component_count as ::core::ffi::c_int) as u32)]);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as ::core::ffi::c_int)
            < (&(*subtable).lig_array)[j_2 as usize].component_count
                as ::core::ffi::c_int
        {
            let mut m: GlyphClass = 0 as GlyphClass;
            while (m as ::core::ffi::c_int) < (*subtable).class_count as ::core::ffi::c_int {
                bk_push(attach, &[bk_ptr(BkCellType::P16, bk_from_anchor(
                        *(*(&(*subtable).lig_array)[j_2 as usize]
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
