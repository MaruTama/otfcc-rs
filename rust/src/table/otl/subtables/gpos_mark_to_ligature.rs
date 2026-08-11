#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};


use crate::support::json_funcs::{json_obj_get_type, preserialize};
use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_name, otfcc_handle_dup, Handle, GlyphHandle, HandleState};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphClass, GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::table::otl::{Anchor, LigatureArray, LigatureBaseRecord, Subtable, GposMarkToLigatureSubtable, subtable_from_raw};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::table::otl::subtables::gpos_common::{bk_from_anchor, otl_anchor_absent, otl_parse_mark_array, otl_parse_anchor, otl_read_mark_array, otl_read_anchor};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_object_push_bytes_key, json_object_push_length, json_string_new_length};
use crate::vendor::sds::{sdsempty, sdsfree, sdslen, sdsnewlen};
// `LigatureBaseRecord.anchors` is a plain `Vec<Vec<Anchor>>` now and
// `glyph: GlyphHandle` already has its own `Drop`, so a `LigatureArray`
// (`Vec<LigatureBaseRecord>`) fully self-drops -- clearing it (still needed:
// `consolidate/otl/mark.rs`'s dedup pass clears an in-place array
// mid-function, not just at end of scope) is exactly `*arr = Vec::new()`.
pub(crate) unsafe fn dispose_lig_array(arr: *mut LigatureArray) {
    *arr = Vec::new();
}
unsafe extern "C" fn init_mark_to_ligature(subtable: *mut GposMarkToLigatureSubtable) {
    (*subtable).mark_array = Vec::new();
    (*subtable).lig_array = Vec::new();
}
pub(crate) unsafe extern "C" fn subtable_gpos_mark_to_ligature_free(x: *mut GposMarkToLigatureSubtable) {
    if x.is_null() {
        return;
    }
    // `mark_array`/`lig_array` both self-drop; `ptr::read` moves the whole
    // value out of the malloc'd shell so `drop` can run that field-by-field
    // teardown, then `free` releases the now-empty shell -- the same
    // "unwrap_X_table" idiom used throughout Stage 6-4.
    drop(::core::ptr::read(x));
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
                                name: Vec::new(),
                            },
                            component_count: 0,
                            anchors: Vec::new(),
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
                        lig.anchors = Vec::with_capacity(lig.component_count as usize);
                        let mut _offset: u32 = lig_attach_offset.wrapping_add(2 as u32);
                        let mut k: GlyphId = 0 as GlyphId;
                        while (k as ::core::ffi::c_int) < lig.component_count as ::core::ffi::c_int {
                            let mut component: Vec<Anchor> =
                                Vec::with_capacity((*subtable).class_count as usize);
                            let mut m: GlyphClass = 0 as GlyphClass;
                            while (m as ::core::ffi::c_int)
                                < (*subtable).class_count as ::core::ffi::c_int
                            {
                                let mut anchor_offset: u32 =
                                    read_16u(data.offset(_offset as isize) as *const u8)
                                        as u32;
                                if anchor_offset != 0 {
                                    component.push(otl_read_anchor(
                                        data,
                                        table_length,
                                        lig_attach_offset.wrapping_add(anchor_offset),
                                    ));
                                } else {
                                    component.push(otl_anchor_absent());
                                }
                                _offset = _offset.wrapping_add(2 as u32);
                                m = m.wrapping_add(1);
                            }
                            lig.anchors.push(component);
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
                            return subtable_from_raw(subtable, Subtable::GposMarkToLigature);
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
    let Subtable::GposMarkToLigature(mut_subtable) = &*st else { unreachable!() };
    let subtable: *const GposMarkToLigatureSubtable = mut_subtable;
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
        json_object_push_bytes_key(
            _marks,
            &(&(*subtable).mark_array)[j as usize].glyph.name,
            preserialize(_mark),
        );
        j = j.wrapping_add(1);
    }
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).lig_array.len() {
        let base: *const LigatureBaseRecord = &(&(*subtable).lig_array)[j_0 as usize] as *const LigatureBaseRecord;
        let base_anchors: &Vec<Vec<Anchor>> = &(*base).anchors;
        let mut _base: *mut JsonValue = json_array_new((*base).component_count as usize);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as ::core::ffi::c_int) < (*base).component_count as ::core::ffi::c_int {
            let mut _bk: *mut JsonValue = json_object_new((*subtable).class_count as usize);
            let mut m: GlyphClass = 0 as GlyphClass;
            while (m as ::core::ffi::c_int) < (*subtable).class_count as ::core::ffi::c_int {
                if base_anchors[k as usize][m as usize].present {
                    let mut _anchor: *mut JsonValue = json_object_new(2 as usize);
                    json_object_push(
                        _anchor,
                        b"x\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer_new(
                            base_anchors[k as usize][m as usize].x as i64,
                        ),
                    );
                    json_object_push(
                        _anchor,
                        b"y\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer_new(
                            base_anchors[k as usize][m as usize].y as i64,
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
        json_object_push_bytes_key(_bases, &(*base).glyph.name, preserialize(_base));
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
    mut h: *mut std::collections::BTreeMap<Vec<u8>, GlyphClass>,
    mut options: *const Options,
) {
    let class_count: GlyphClass = (*h).len() as GlyphClass;
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < (*_bases).u.object.length {
        let mut gname: *mut ::core::ffi::c_char =
            (*(*_bases).u.object.values.offset(j as isize)).name;
        let mut lig: LigatureBaseRecord = LigatureBaseRecord {
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: Vec::new(),
            },
            component_count: 0,
            anchors: Vec::new(),
        };
        lig.component_count = 0 as GlyphId;
        lig.anchors = Vec::new();
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
            lig.anchors = Vec::with_capacity(lig.component_count as usize);
            let mut k: GlyphId = 0 as GlyphId;
            while (k as ::core::ffi::c_int) < lig.component_count as ::core::ffi::c_int {
                let mut _component_record: *mut JsonValue =
                    *(*base_record).u.array.values.offset(k as isize) as *mut JsonValue;
                // Indexed by `class_id` below, out of JSON key order --
                // pre-sized and filled with "absent" rather than built with
                // `.push()`.
                lig.anchors.push(vec![otl_anchor_absent(); class_count as usize]);
                if !(_component_record.is_null()
                    || (*_component_record).type_0 != JsonType::Object)
                {
                    let mut m_0: GlyphClass = 0 as GlyphClass;
                    while (m_0 as ::core::ffi::c_uint) < (*_component_record).u.object.length {
                        let name_ptr: *mut ::core::ffi::c_char =
                            (*(*_component_record).u.object.values.offset(m_0 as isize)).name;
                        // `strlen`-bounded, matching
                        // `otl_parse_mark_array`'s registration key
                        // exactly.
                        let class_name: Vec<u8> =
                            ::core::ffi::CStr::from_ptr(name_ptr).to_bytes().to_vec();
                        match (*h).get(&class_name) {
                            None => {
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
                                        name_ptr,
                                        b"> for /",
                                        gname,
                                        b". This base anchor is ignored.\n",
                                    ),
                                );
                            }
                            Some(&class_id) => {
                                lig.anchors[k as usize][class_id as usize] =
                                    otl_parse_anchor(
                                        (*(*_component_record).u.object.values.offset(m_0 as isize))
                                            .value
                                            as *mut JsonValue,
                                    );
                            }
                        }
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
    let mut h: std::collections::BTreeMap<Vec<u8>, GlyphClass> = std::collections::BTreeMap::new();
    otl_parse_mark_array(_marks, &raw mut (*st).mark_array, &raw mut h, options);
    (*st).class_count = h.len() as GlyphClass;
    parse_bases(_bases, st, &raw mut h, options);
    return subtable_from_raw(st, Subtable::GposMarkToLigature);
}
pub unsafe extern "C" fn otfcc_build_gpos_mark_to_ligature(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let Subtable::GposMarkToLigature(mut_subtable) = &*_subtable else { unreachable!() };
    let mut subtable: *const GposMarkToLigatureSubtable = mut_subtable;
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
                        (&(*subtable).lig_array)[j_2 as usize]
                            .anchors[k as usize][m as usize],
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
