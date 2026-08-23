#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{
    GlyphHandle, Handle, HandleState, handle_from_name, otfcc_handle_dup,
};
use crate::support::parsed_json::{
    ParsedValue, json_obj_get_type, json_obj_key_at, json_obj_key_bytes_at, json_obj_len,
    json_obj_val_at, json_type_of,
};
use crate::table::otl::coverage::{
    Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage,
};

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::bk::bkgraph::bk_build_block;
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::buffer::Buffer;
use crate::support::built_json::{
    BuiltValue, json_integer_new, json_object_new, json_object_push, json_object_push_bytes_key,
    json_string_new_from_bytes, preserialize,
};
use crate::support::font_reader::FontReader;
use crate::support::options::Options;
use crate::support::primitives::{FontFilePointer, GlyphClass, GlyphId};
use crate::table::otl::coverage::build_coverage;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::subtables::gpos_common::{
    bk_from_anchor, otl_anchor_absent, otl_parse_anchor, otl_parse_mark_array, otl_read_anchor,
    otl_read_mark_array,
};
use crate::table::otl::{
    Anchor, BaseArray, BaseRecord, GposMarkToSingleSubtable, Subtable, subtable_from_raw,
};
use crate::vendor::json::JsonType;
// `BaseRecord.anchors` is a plain `Vec<Anchor>` now and `glyph: GlyphHandle`
// already has its own `Drop`, so a `BaseArray` (`Vec<BaseRecord>`) fully
// self-drops -- clearing it (still needed: `consolidate/otl/mark.rs`'s dedup
// pass clears an in-place array mid-function, not just at end of scope) is
// exactly `*arr = Vec::new()`.
pub(crate) unsafe fn dispose_base_array(arr: *mut BaseArray) {
    *arr = Vec::new();
}
pub(crate) unsafe fn subtable_gpos_mark_to_single_free(x: *mut GposMarkToSingleSubtable) {
    if x.is_null() {
        return;
    }
    // `Box::from_raw` reclaims exactly the allocation `_create()` made below
    // and runs `mark_array`/`base_array`'s own drop glue directly -- no more
    // `ptr::read`-then-`free` shell dance (Stage 7-2-d): that idiom was only
    // ever needed to avoid mixing a `__caryll_allocate_clean`'d (`calloc`)
    // shell with a `Box`'s own drop glue, and `_create()` no longer produces
    // one. `init_mark_to_single` had no other callers, so it's gone too.
    drop(Box::from_raw(x));
}
unsafe fn subtable_gpos_mark_to_single_create() -> *mut GposMarkToSingleSubtable {
    Box::into_raw(Box::new(GposMarkToSingleSubtable {
        class_count: 0,
        mark_array: Vec::new(),
        base_array: Vec::new(),
    }))
}
// `2 * bases.len() * class_count` (the BaseArray's byte-length guard) is a
// real, previously-undocumented overflow-defeats-guard bug: `bases.len()`
// can be as large as the glyph count (bounded by `GlyphId`, up to 65535)
// and `class_count` is an independent, unbounded `u16` read straight from
// the file -- their product can exceed `i32::MAX` (65535*65535*2 is
// ~8.6 billion), the same class of bug as `cmap.rs`'s `n_groups` guard,
// just reached by two independently-large factors instead of one. Fixed
// with `checked_mul` before ever calling `require_room`.
//
// Also fixes a real (if minor) pre-existing leak: the original only freed
// `marks`/`bases` on the success path -- every failure guard after they
// were read (`read_coverage` always allocates, even for an empty result)
// fell through to `subtable_gpos_mark_to_single_free(subtable)` without
// freeing either. Restructured so cleanup runs once, after the parse
// attempt, on every path.
pub unsafe fn otl_read_gpos_mark_to_single(
    data: FontFilePointer,
    table_length: u32,
    subtable_offset: u32,
    _max_glyphs: GlyphId,
) -> *mut Subtable {
    let subtable: *mut GposMarkToSingleSubtable = subtable_gpos_mark_to_single_create();
    let mut marks: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut bases: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);

    let result: Option<*mut Subtable> = 'parse: {
        let mut header = match FontReader::new(slice).at(subtable_offset as usize) {
            Ok(r) => r,
            Err(_) => break 'parse None,
        };
        if header.skip(2).is_err() {
            break 'parse None; // format, unused
        }
        let Ok(marks_rel) = header.u16() else {
            break 'parse None;
        };
        let Ok(bases_rel) = header.u16() else {
            break 'parse None;
        };
        let Ok(class_count) = header.u16() else {
            break 'parse None;
        };
        let Ok(mark_array_rel) = header.u16() else {
            break 'parse None;
        };
        let Ok(base_array_rel) = header.u16() else {
            break 'parse None;
        };

        marks = read_coverage(
            data,
            table_length,
            subtable_offset.wrapping_add(marks_rel as u32),
        );
        bases = read_coverage(
            data,
            table_length,
            subtable_offset.wrapping_add(bases_rel as u32),
        );
        if marks.is_null() || (*marks).is_empty() || bases.is_null() || (*bases).is_empty() {
            break 'parse None;
        }

        (*subtable).class_count = class_count as GlyphClass;
        let mark_array_offset = subtable_offset.wrapping_add(mark_array_rel as u32);
        otl_read_mark_array(
            &raw mut (*subtable).mark_array,
            marks,
            data,
            table_length,
            mark_array_offset,
        );

        let base_array_offset = subtable_offset.wrapping_add(base_array_rel as u32);
        let Ok(mut base_reader) = FontReader::new(slice).at(base_array_offset as usize) else {
            break 'parse None;
        };
        let Ok(base_count) = base_reader.u16() else {
            break 'parse None;
        };
        if base_count as usize != (*bases).len() {
            break 'parse None;
        }
        let Some(total_anchors) = (*bases).len().checked_mul(class_count as usize) else {
            break 'parse None;
        };
        if base_reader.require_room(total_anchors, 2).is_err() {
            break 'parse None;
        }

        for j in 0..(*bases).len() {
            let mut base_anchors: Vec<Anchor> = Vec::with_capacity(class_count as usize);
            for _ in 0..class_count {
                let anchor_rel = base_reader.u16().unwrap();
                if anchor_rel != 0 {
                    base_anchors.push(otl_read_anchor(
                        data,
                        table_length,
                        base_array_offset.wrapping_add(anchor_rel as u32),
                    ));
                } else {
                    base_anchors.push(otl_anchor_absent());
                }
            }
            (*subtable).base_array.push(BaseRecord {
                glyph: otfcc_handle_dup((&(*bases))[j].clone() as Handle) as GlyphHandle,
                anchors: base_anchors,
            });
        }
        break 'parse Some(subtable_from_raw(subtable, Subtable::GposMarkToSingle));
    };

    if !marks.is_null() {
        otl_coverage_free(marks);
    }
    if !bases.is_null() {
        otl_coverage_free(bases);
    }
    match result {
        Some(s) => s,
        None => {
            subtable_gpos_mark_to_single_free(subtable);
            ::core::ptr::null_mut::<Subtable>()
        }
    }
}
pub unsafe extern "C" fn otl_gpos_dump_mark_to_single(mut st: *const Subtable) -> *mut BuiltValue {
    let Subtable::GposMarkToSingle(mut_subtable) = &*st else {
        unreachable!()
    };
    let subtable: *const GposMarkToSingleSubtable = mut_subtable;
    let mut _subtable: *mut BuiltValue = json_object_new(3 as usize);
    let mut _marks: *mut BuiltValue = json_object_new((*subtable).mark_array.len());
    let mut _bases: *mut BuiltValue = json_object_new((*subtable).base_array.len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).mark_array.len() {
        let mut _mark: *mut BuiltValue = json_object_new(3 as usize);
        let mark_class_name: Vec<u8> = crate::bytesbuild!(
            b"anchor",
            (&(*subtable).mark_array)[j as usize].mark_class as ::core::ffi::c_int,
        );
        json_object_push(
            _mark,
            b"class\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new_from_bytes(&mark_class_name),
        );
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
    while (j_0 as usize) < (*subtable).base_array.len() {
        let mut _base: *mut BuiltValue = json_object_new((*subtable).class_count as usize);
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < (*subtable).class_count as ::core::ffi::c_int {
            if (&(*subtable).base_array)[j_0 as usize].anchors[k as usize].present {
                let mut _anchor: *mut BuiltValue = json_object_new(2 as usize);
                json_object_push(
                    _anchor,
                    b"x\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(
                        (&(*subtable).base_array)[j_0 as usize].anchors[k as usize].x as i64,
                    ),
                );
                json_object_push(
                    _anchor,
                    b"y\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(
                        (&(*subtable).base_array)[j_0 as usize].anchors[k as usize].y as i64,
                    ),
                );
                let mark_class_name_0: Vec<u8> =
                    crate::bytesbuild!(b"anchor", k as ::core::ffi::c_int);
                json_object_push_bytes_key(_base, &mark_class_name_0, _anchor);
            }
            k = k.wrapping_add(1);
        }
        json_object_push_bytes_key(
            _bases,
            &(&(*subtable).base_array)[j_0 as usize].glyph.name,
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
unsafe fn parse_bases(
    mut _bases: *const ParsedValue,
    mut subtable: *mut GposMarkToSingleSubtable,
    mut h: *mut std::collections::BTreeMap<Vec<u8>, GlyphClass>,
    mut options: &Options,
) {
    let class_count: GlyphClass = (*h).len() as GlyphClass;
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < json_obj_len(_bases) {
        let mut gname: *mut ::core::ffi::c_char = json_obj_key_at(_bases, j as u32);
        let mut base: BaseRecord = BaseRecord {
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: Vec::new(),
            },
            anchors: Vec::new(),
        };
        base.glyph = handle_from_name(Some(json_obj_key_bytes_at(_bases, j as u32))) as GlyphHandle;
        // Indexed by `class_id` below, out of JSON key order -- pre-sized
        // and filled with "absent" rather than built with `.push()`.
        base.anchors = vec![otl_anchor_absent(); class_count as usize];
        let mut base_record: *const ParsedValue = json_obj_val_at(_bases, j as u32);
        if base_record.is_null() || json_type_of(base_record) != JsonType::Object {
            (*subtable).base_array.push(base);
        } else {
            let mut k_0: GlyphClass = 0 as GlyphClass;
            while (k_0 as ::core::ffi::c_uint) < json_obj_len(base_record) {
                let name_ptr: *mut ::core::ffi::c_char = json_obj_key_at(base_record, k_0 as u32);
                // `strlen`-bounded, matching `otl_parse_mark_array`'s
                // registration key exactly.
                let class_name: Vec<u8> = ::core::ffi::CStr::from_ptr(name_ptr).to_bytes().to_vec();
                match (*h).get(&class_name) {
                    None => {
                        logger_log_sds(
                            &mut *options.logger.borrow_mut(),
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(
                                b"[OTFCC-fea] Invalid anchor class name <",
                                name_ptr,
                                b"> for /",
                                gname,
                                b". This base anchor is ignored.\n",
                            ),
                        );
                    }
                    Some(&class_id) => {
                        base.anchors[class_id as usize] =
                            otl_parse_anchor(json_obj_val_at(base_record, k_0 as u32));
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
    mut _subtable: *const ParsedValue,
    mut options: *const Options,
) -> *mut Subtable {
    let mut _marks: *const ParsedValue = json_obj_get_type(
        _subtable,
        b"marks\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    let mut _bases: *const ParsedValue = json_obj_get_type(
        _subtable,
        b"bases\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if _marks.is_null() || _bases.is_null() {
        return ::core::ptr::null_mut::<Subtable>();
    }
    let mut st: *mut GposMarkToSingleSubtable = subtable_gpos_mark_to_single_create();
    let mut h: std::collections::BTreeMap<Vec<u8>, GlyphClass> = std::collections::BTreeMap::new();
    otl_parse_mark_array(_marks, &raw mut (*st).mark_array, &raw mut h);
    (*st).class_count = h.len() as GlyphClass;
    parse_bases(_bases, st, &raw mut h, &*options);
    return subtable_from_raw(st, Subtable::GposMarkToSingle);
}
pub unsafe extern "C" fn otfcc_build_gpos_mark_to_single(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let Subtable::GposMarkToSingle(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GposMarkToSingleSubtable = mut_subtable;
    let mut marks: *mut Coverage = otl_coverage_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).mark_array.len() {
        push_to_coverage(
            marks,
            otfcc_handle_dup((&(*subtable).mark_array)[j as usize].glyph.clone() as Handle)
                as GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let mut bases: *mut Coverage = otl_coverage_create();
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).base_array.len() {
        push_to_coverage(
            bases,
            otfcc_handle_dup((&(*subtable).base_array)[j_0 as usize].glyph.clone() as Handle)
                as GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let mut root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B16, 1 as u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(build_coverage(marks)),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(build_coverage(bases)),
        ),
        bk_int(
            BkCellType::B16,
            ((*subtable).class_count as ::core::ffi::c_int) as u32,
        ),
    ]);
    let mut mark_array: *mut BkBlock = bk_new_block(&[bk_int(
        BkCellType::B16,
        ((*subtable).mark_array.len()) as u32,
    )]);
    let mut j_1: GlyphId = 0 as GlyphId;
    while (j_1 as usize) < (*subtable).mark_array.len() {
        bk_push(
            mark_array,
            &[
                bk_int(
                    BkCellType::B16,
                    ((&(*subtable).mark_array)[j_1 as usize].mark_class as ::core::ffi::c_int)
                        as u32,
                ),
                bk_ptr(
                    BkCellType::P16,
                    bk_from_anchor((&(*subtable).mark_array)[j_1 as usize].anchor),
                ),
            ],
        );
        j_1 = j_1.wrapping_add(1);
    }
    let mut base_array: *mut BkBlock = bk_new_block(&[bk_int(
        BkCellType::B16,
        ((*subtable).base_array.len()) as u32,
    )]);
    let mut j_2: GlyphId = 0 as GlyphId;
    while (j_2 as usize) < (*subtable).base_array.len() {
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < (*subtable).class_count as ::core::ffi::c_int {
            bk_push(
                base_array,
                &[bk_ptr(
                    BkCellType::P16,
                    bk_from_anchor((&(*subtable).base_array)[j_2 as usize].anchors[k as usize]),
                )],
            );
            k = k.wrapping_add(1);
        }
        j_2 = j_2.wrapping_add(1);
    }
    bk_push(
        root,
        &[
            bk_ptr(BkCellType::P16, mark_array),
            bk_ptr(BkCellType::P16, base_array),
        ],
    );
    otl_coverage_free(marks);
    otl_coverage_free(bases);
    return bk_build_block(root);
}

#[cfg(test)]
mod otl_read_gpos_mark_to_single_tests {
    use super::*;

    // format(2)@0, marksOffset(2)@2 -> 12, basesOffset(2)@4 -> 18,
    // classCount(2)@6, markArrayOffset(2)@8 -> 24, baseArrayOffset(2)@10
    // -> 26; marks coverage @12 (glyph 5); bases coverage @18 (glyph 6);
    // mark array @24 (markCount=0, avoiding any dependency on
    // `otl_read_anchor`/a real Anchor subtable); base array @26
    // (baseCount + baseCount*classCount anchor offsets, all absent).
    fn well_formed_data(class_count: u16) -> Vec<u8> {
        let mut data = vec![0u8; 30];
        data[2..4].copy_from_slice(&12u16.to_be_bytes());
        data[4..6].copy_from_slice(&18u16.to_be_bytes());
        data[6..8].copy_from_slice(&class_count.to_be_bytes());
        data[8..10].copy_from_slice(&24u16.to_be_bytes());
        data[10..12].copy_from_slice(&26u16.to_be_bytes());
        data[12..14].copy_from_slice(&1u16.to_be_bytes());
        data[14..16].copy_from_slice(&1u16.to_be_bytes());
        data[16..18].copy_from_slice(&5u16.to_be_bytes());
        data[18..20].copy_from_slice(&1u16.to_be_bytes());
        data[20..22].copy_from_slice(&1u16.to_be_bytes());
        data[22..24].copy_from_slice(&6u16.to_be_bytes());
        data[24..26].copy_from_slice(&0u16.to_be_bytes()); // markCount = 0
        data[26..28].copy_from_slice(&1u16.to_be_bytes()); // baseCount
        data[28..30].copy_from_slice(&0u16.to_be_bytes()); // anchorOffset[0][0] = absent
        data
    }

    #[test]
    fn well_formed_table_reads_the_base_array() {
        let data = well_formed_data(1);
        unsafe {
            let raw = otl_read_gpos_mark_to_single(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                0,
            );
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::GposMarkToSingle(subtable) = &*boxed else {
                unreachable!()
            };
            assert_eq!(subtable.class_count, 1);
            assert_eq!(subtable.base_array.len(), 1);
            assert_eq!(subtable.base_array[0].glyph.index, 6);
            assert!(!subtable.base_array[0].anchors[0].present);
        }
    }

    #[test]
    fn base_count_mismatch_with_coverage_is_rejected() {
        let mut data = well_formed_data(1);
        data[26..28].copy_from_slice(&2u16.to_be_bytes()); // baseCount claims 2, coverage has only 1
        unsafe {
            let raw = otl_read_gpos_mark_to_single(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                0,
            );
            assert!(raw.is_null());
        }
    }

    #[test]
    fn anchor_array_shorter_than_class_count_times_base_count_is_rejected() {
        // The guard here (`bases.len() * class_count`, checked via
        // `checked_mul` on `usize`) is what the original computed as
        // unchecked `i32` arithmetic -- for a large enough `class_count`
        // and `bases.len()`, that product can exceed `i32::MAX` and wrap.
        // Demonstrating the exact overflow needs an impractically large
        // base coverage; this instead confirms the guard rejects a
        // shortfall at an ordinary scale (class_count raised from 1 to 5,
        // but the base array still has room for only 1 anchor slot).
        let data = well_formed_data(5); // baseCount=1, but only 1 anchor slot is present, not 5
        unsafe {
            let raw = otl_read_gpos_mark_to_single(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                0,
            );
            assert!(raw.is_null());
        }
    }
}
