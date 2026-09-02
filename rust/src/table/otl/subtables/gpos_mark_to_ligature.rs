#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{
    GlyphHandle, Handle, HandleState, handle_from_name, otfcc_handle_dup,
};
use crate::support::parsed_json::ParsedValue;
use crate::table::otl::coverage::{
    Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage,
};

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::bk::bkgraph::bk_build_block;
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
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
    Anchor, GposMarkToLigatureSubtable, LigatureArray, LigatureBaseRecord, Subtable,
    subtable_from_raw,
};
use crate::vendor::json::JsonType;
// `LigatureBaseRecord.anchors` is a plain `Vec<Vec<Anchor>>` now and
// `glyph: GlyphHandle` already has its own `Drop`, so a `LigatureArray`
// (`Vec<LigatureBaseRecord>`) fully self-drops -- clearing it (still needed:
// `consolidate/otl/mark.rs`'s dedup pass clears an in-place array
// mid-function, not just at end of scope) is exactly `*arr = Vec::new()`.
pub(crate) unsafe fn dispose_lig_array(arr: *mut LigatureArray) {
    *arr = Vec::new();
}
pub(crate) unsafe fn subtable_gpos_mark_to_ligature_free(x: *mut GposMarkToLigatureSubtable) {
    if x.is_null() {
        return;
    }
    // `Box::from_raw` reclaims exactly the allocation `_create()` made below
    // and runs `mark_array`/`lig_array`'s own drop glue directly -- no more
    // `ptr::read`-then-`free` shell dance (Stage 7-2-d): that idiom was only
    // ever needed to avoid mixing a `__caryll_allocate_clean`'d (`calloc`)
    // shell with a `Box`'s own drop glue, and `_create()` no longer produces
    // one. `init_mark_to_ligature` had no other callers, so it's gone too.
    drop(Box::from_raw(x));
}
unsafe fn subtable_gpos_mark_to_ligature_create() -> *mut GposMarkToLigatureSubtable {
    Box::into_raw(Box::new(GposMarkToLigatureSubtable {
        class_count: 0,
        mark_array: Vec::new(),
        lig_array: Vec::new(),
    }))
}
// `2 * component_count * class_count` (the LigatureAttach's byte-length
// guard) is the same overflow-defeats-guard shape as
// `gpos_mark_to_single.rs`'s `bases.len() * class_count`, but sharper
// here: *both* factors are independently unbounded `u16` fields read
// straight from the file (unlike `bases.len()`, which is at least bounded
// by the actual glyph count), so the product can reach 65535*65535*2 --
// ~8.6 billion -- from a much smaller, more plausible crafted input.
// Fixed the same way: `checked_mul` on `usize` before `require_room`.
pub unsafe fn otl_read_gpos_mark_to_ligature(
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    _max_glyphs: GlyphId,
) -> *mut Subtable {
    let subtable: *mut GposMarkToLigatureSubtable = subtable_gpos_mark_to_ligature_create();
    let mut marks: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut bases: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);

    'parse: {
        let mut header = match FontReader::new(slice).at(offset as usize) {
            Ok(r) => r,
            Err(_) => break 'parse,
        };
        if header.skip(2).is_err() {
            break 'parse; // format, unused
        }
        let Ok(marks_rel) = header.u16() else {
            break 'parse;
        };
        let Ok(bases_rel) = header.u16() else {
            break 'parse;
        };
        let Ok(class_count) = header.u16() else {
            break 'parse;
        };
        let Ok(mark_array_rel) = header.u16() else {
            break 'parse;
        };
        let Ok(lig_array_rel) = header.u16() else {
            break 'parse;
        };

        marks = read_coverage(data, table_length, offset.wrapping_add(marks_rel as u32));
        bases = read_coverage(data, table_length, offset.wrapping_add(bases_rel as u32));
        if marks.is_null() || (*marks).is_empty() || bases.is_null() || (*bases).is_empty() {
            break 'parse;
        }

        (*subtable).class_count = class_count as GlyphClass;
        let mark_array_offset = offset.wrapping_add(mark_array_rel as u32);
        otl_read_mark_array(
            &raw mut (*subtable).mark_array,
            marks,
            data,
            table_length,
            mark_array_offset,
        );

        let lig_array_offset = offset.wrapping_add(lig_array_rel as u32);
        let Ok(mut lr) = FontReader::new(slice).at(lig_array_offset as usize) else {
            break 'parse;
        };
        let Ok(lig_count) = lr.u16() else {
            break 'parse;
        };
        if lig_count as usize != (*bases).len() {
            break 'parse;
        }
        if lr.require_room(lig_count as usize, 2).is_err() {
            break 'parse;
        }
        let mut lig_attach_offsets = Vec::with_capacity(lig_count as usize);
        for _ in 0..lig_count {
            lig_attach_offsets.push(lig_array_offset.wrapping_add(lr.u16().unwrap() as u32));
        }

        for (j, &lig_attach_offset) in lig_attach_offsets.iter().enumerate() {
            let Ok(mut ar) = FontReader::new(slice).at(lig_attach_offset as usize) else {
                break 'parse;
            };
            let Ok(component_count) = ar.u16() else {
                break 'parse;
            };
            let Some(total_anchors) = (component_count as usize).checked_mul(class_count as usize)
            else {
                break 'parse;
            };
            if ar.require_room(total_anchors, 2).is_err() {
                break 'parse;
            }
            let mut lig = LigatureBaseRecord {
                glyph: otfcc_handle_dup((&(*bases))[j].clone() as Handle) as GlyphHandle,
                component_count,
                anchors: Vec::with_capacity(component_count as usize),
            };
            for _ in 0..component_count {
                let mut component: Vec<Anchor> = Vec::with_capacity(class_count as usize);
                for _ in 0..class_count {
                    let anchor_rel = ar.u16().unwrap();
                    if anchor_rel != 0 {
                        component.push(otl_read_anchor(
                            data,
                            table_length,
                            lig_attach_offset.wrapping_add(anchor_rel as u32),
                        ));
                    } else {
                        component.push(otl_anchor_absent());
                    }
                }
                lig.anchors.push(component);
            }
            (*subtable).lig_array.push(lig);
        }

        if !marks.is_null() {
            otl_coverage_free(marks);
        }
        if !bases.is_null() {
            otl_coverage_free(bases);
        }
        return subtable_from_raw(subtable, Subtable::GposMarkToLigature);
    }
    if !marks.is_null() {
        otl_coverage_free(marks);
    }
    if !bases.is_null() {
        otl_coverage_free(bases);
    }
    subtable_gpos_mark_to_ligature_free(subtable);
    ::core::ptr::null_mut::<Subtable>()
}
pub unsafe fn otl_gpos_dump_mark_to_ligature(st: *const Subtable) -> BuiltValue {
    let Subtable::GposMarkToLigature(mut_subtable) = &*st else {
        unreachable!()
    };
    let subtable: *const GposMarkToLigatureSubtable = mut_subtable;
    let mut _subtable = BuiltValue::new_object(3);
    let mut _marks = BuiltValue::new_object((*subtable).mark_array.len());
    let mut _bases = BuiltValue::new_object((*subtable).lig_array.len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).mark_array.len() {
        let mut _mark = BuiltValue::new_object(3);
        let mark_class_name: Vec<u8> = crate::bytesbuild!(
            b"ac_",
            (&(*subtable).mark_array)[j as usize].mark_class as i32,
        );
        _mark.push_field(b"class", BuiltValue::str_truncated_at_nul(&mark_class_name));
        _mark.push_field(
            b"x",
            BuiltValue::Int((&(*subtable).mark_array)[j as usize].anchor.x as i64),
        );
        _mark.push_field(
            b"y",
            BuiltValue::Int((&(*subtable).mark_array)[j as usize].anchor.y as i64),
        );
        _marks.push_field_bytes_key(
            &(&(*subtable).mark_array)[j as usize].glyph.name,
            _mark.preserialize(),
        );
        j = j.wrapping_add(1);
    }
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).lig_array.len() {
        let base: *const LigatureBaseRecord =
            &(&(*subtable).lig_array)[j_0 as usize] as *const LigatureBaseRecord;
        let base_anchors: &Vec<Vec<Anchor>> = &(*base).anchors;
        let mut _base = BuiltValue::new_array((*base).component_count as usize);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as i32) < (*base).component_count as i32 {
            let mut _bk = BuiltValue::new_object((*subtable).class_count as usize);
            let mut m: GlyphClass = 0 as GlyphClass;
            while (m as i32) < (*subtable).class_count as i32 {
                if base_anchors[k as usize][m as usize].present {
                    let mut _anchor = BuiltValue::new_object(2);
                    _anchor.push_field(
                        b"x",
                        BuiltValue::Int(base_anchors[k as usize][m as usize].x as i64),
                    );
                    _anchor.push_field(
                        b"y",
                        BuiltValue::Int(base_anchors[k as usize][m as usize].y as i64),
                    );
                    let mark_class_name_0: Vec<u8> = crate::bytesbuild!(b"ac_", m as i32);
                    _bk.push_field_bytes_key(&mark_class_name_0, _anchor);
                }
                m = m.wrapping_add(1);
            }
            _base.push_item(_bk);
            k = k.wrapping_add(1);
        }
        _bases.push_field_bytes_key(&(*base).glyph.name, _base.preserialize());
        j_0 = j_0.wrapping_add(1);
    }
    _subtable.push_field(b"classCount", BuiltValue::Int((*subtable).class_count as i64));
    _subtable.push_field(b"marks", _marks);
    _subtable.push_field(b"bases", _bases);
    _subtable
}
unsafe fn parse_bases(
    bases: *const ParsedValue,
    subtable: *mut GposMarkToLigatureSubtable,
    h: *mut std::collections::BTreeMap<Vec<u8>, GlyphClass>,
    options: &Options,
) {
    let class_count: GlyphClass = (*h).len() as GlyphClass;
    let Some(fields) = unsafe { bases.as_ref() }.and_then(ParsedValue::as_object) else {
        return;
    };
    for (key, base_record) in fields {
        let gname = &key[..key.len() - 1];
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
        lig.glyph = handle_from_name(Some(gname.to_vec())) as GlyphHandle;
        match base_record.as_array() {
            None => {
                (*subtable).lig_array.push(lig);
            }
            Some(components) => {
                lig.component_count = components.len() as GlyphId;
                lig.anchors = Vec::with_capacity(lig.component_count as usize);
                for (k, component_record) in components.iter().enumerate() {
                    // Indexed by `class_id` below, out of JSON key order --
                    // pre-sized and filled with "absent" rather than built with
                    // `.push()`.
                    lig.anchors
                        .push(vec![otl_anchor_absent(); class_count as usize]);
                    if let Some(inner_fields) = component_record.as_object() {
                        for (name_key, val) in inner_fields {
                            let class_name = &name_key[..name_key.len() - 1];
                            // `strlen`-bounded, matching
                            // `otl_parse_mark_array`'s registration key
                            // exactly.
                            match (*h).get(class_name) {
                                None => {
                                    logger_log_sds(
                                        &mut *options.logger.borrow_mut(),
                                        LOG_VL_IMPORTANT,
                                        LoggerType::Warning,
                                        crate::bytesbuild!(
                                            b"[OTFCC-fea] Invalid anchor class name <",
                                            class_name,
                                            b"> for /",
                                            gname,
                                            b". This base anchor is ignored.\n",
                                        ),
                                    );
                                }
                                Some(&class_id) => {
                                    lig.anchors[k][class_id as usize] =
                                        otl_parse_anchor(val as *const ParsedValue);
                                }
                            }
                        }
                    }
                }
                (*subtable).lig_array.push(lig);
            }
        }
    }
}
pub unsafe fn otl_gpos_parse_mark_to_ligature(
    mut _subtable: *const ParsedValue,
    options: &Options,
) -> *mut Subtable {
    let subtable_val = unsafe { _subtable.as_ref() };
    let marks = subtable_val.and_then(|v| v.get_typed(b"marks", JsonType::Object));
    let bases = subtable_val.and_then(|v| v.get_typed(b"bases", JsonType::Object));
    let (Some(marks), Some(bases)) = (marks, bases) else {
        return ::core::ptr::null_mut::<Subtable>();
    };
    let st: *mut GposMarkToLigatureSubtable = subtable_gpos_mark_to_ligature_create();
    let mut h: std::collections::BTreeMap<Vec<u8>, GlyphClass> = std::collections::BTreeMap::new();
    otl_parse_mark_array(
        marks as *const ParsedValue,
        &raw mut (*st).mark_array,
        &raw mut h,
    );
    (*st).class_count = h.len() as GlyphClass;
    parse_bases(bases as *const ParsedValue, st, &raw mut h, options);
    return subtable_from_raw(st, Subtable::GposMarkToLigature);
}
pub unsafe fn otfcc_build_gpos_mark_to_ligature(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> Buffer {
    let Subtable::GposMarkToLigature(mut_subtable) = &*_subtable else {
        unreachable!()
    };
    let subtable: *const GposMarkToLigatureSubtable = mut_subtable;
    let marks: *mut Coverage = otl_coverage_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).mark_array.len() {
        push_to_coverage(
            marks,
            otfcc_handle_dup((&(*subtable).mark_array)[j as usize].glyph.clone() as Handle)
                as GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let bases: *mut Coverage = otl_coverage_create();
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).lig_array.len() {
        push_to_coverage(
            bases,
            otfcc_handle_dup((&(*subtable).lig_array)[j_0 as usize].glyph.clone() as Handle)
                as GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B16, 1_u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(marks))),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(bases))),
        ),
        bk_int(
            BkCellType::B16,
            ((*subtable).class_count as i32) as u32,
        ),
    ]);
    let mark_array: *mut BkBlock = bk_new_block(&[bk_int(
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
                    ((&(*subtable).mark_array)[j_1 as usize].mark_class as i32)
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
    let ligature_array: *mut BkBlock = bk_new_block(&[bk_int(
        BkCellType::B16,
        ((*subtable).lig_array.len()) as u32,
    )]);
    let mut j_2: GlyphId = 0 as GlyphId;
    while (j_2 as usize) < (*subtable).lig_array.len() {
        let attach: *mut BkBlock = bk_new_block(&[bk_int(
            BkCellType::B16,
            ((&(*subtable).lig_array)[j_2 as usize].component_count as i32) as u32,
        )]);
        let mut k: GlyphId = 0 as GlyphId;
        while (k as i32)
            < (&(*subtable).lig_array)[j_2 as usize].component_count as i32
        {
            let mut m: GlyphClass = 0 as GlyphClass;
            while (m as i32) < (*subtable).class_count as i32 {
                bk_push(
                    attach,
                    &[bk_ptr(
                        BkCellType::P16,
                        bk_from_anchor(
                            (&(*subtable).lig_array)[j_2 as usize].anchors[k as usize][m as usize],
                        ),
                    )],
                );
                m = m.wrapping_add(1);
            }
            k = k.wrapping_add(1);
        }
        bk_push(ligature_array, &[bk_ptr(BkCellType::P16, attach)]);
        j_2 = j_2.wrapping_add(1);
    }
    bk_push(
        root,
        &[
            bk_ptr(BkCellType::P16, mark_array),
            bk_ptr(BkCellType::P16, ligature_array),
        ],
    );
    otl_coverage_free(marks);
    otl_coverage_free(bases);
    return bk_build_block(root);
}

#[cfg(test)]
mod otl_read_gpos_mark_to_ligature_tests {
    use super::*;

    // format(2)@0, marksOffset(2)@2 -> 12, ligatureOffset(2)@4 -> 18,
    // classCount(2)@6, markArrayOffset(2)@8 -> 24, ligatureArrayOffset(2)
    // @10 -> 26; marks coverage @12 (glyph 5); ligature coverage @18
    // (glyph 6); mark array @24 (markCount=0, avoiding any dependency on
    // a real Anchor subtable); ligature array @26 (ligatureCount=1,
    // ligAttachOffsets[0]=4 -> 30); LigatureAttach @30
    // (componentCount=1, one absent anchor).
    fn well_formed_data() -> Vec<u8> {
        let mut data = vec![0u8; 34];
        data[2..4].copy_from_slice(&12u16.to_be_bytes());
        data[4..6].copy_from_slice(&18u16.to_be_bytes());
        data[6..8].copy_from_slice(&1u16.to_be_bytes()); // classCount
        data[8..10].copy_from_slice(&24u16.to_be_bytes());
        data[10..12].copy_from_slice(&26u16.to_be_bytes());
        data[12..14].copy_from_slice(&1u16.to_be_bytes());
        data[14..16].copy_from_slice(&1u16.to_be_bytes());
        data[16..18].copy_from_slice(&5u16.to_be_bytes());
        data[18..20].copy_from_slice(&1u16.to_be_bytes());
        data[20..22].copy_from_slice(&1u16.to_be_bytes());
        data[22..24].copy_from_slice(&6u16.to_be_bytes());
        data[24..26].copy_from_slice(&0u16.to_be_bytes()); // markCount = 0
        data[26..28].copy_from_slice(&1u16.to_be_bytes()); // ligatureCount
        data[28..30].copy_from_slice(&4u16.to_be_bytes()); // ligAttachOffsets[0] -> 30
        data[30..32].copy_from_slice(&1u16.to_be_bytes()); // componentCount
        data[32..34].copy_from_slice(&0u16.to_be_bytes()); // anchorOffset = absent
        data
    }

    #[test]
    fn well_formed_table_reads_the_ligature_array() {
        let data = well_formed_data();
        unsafe {
            let raw = otl_read_gpos_mark_to_ligature(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                0,
            );
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::GposMarkToLigature(subtable) = &*boxed else {
                unreachable!()
            };
            assert_eq!(subtable.class_count, 1);
            assert_eq!(subtable.lig_array.len(), 1);
            assert_eq!(subtable.lig_array[0].glyph.index, 6);
            assert_eq!(subtable.lig_array[0].component_count, 1);
            assert!(!subtable.lig_array[0].anchors[0][0].present);
        }
    }

    #[test]
    fn ligature_count_mismatch_with_coverage_is_rejected() {
        let mut data = well_formed_data();
        data[26..28].copy_from_slice(&2u16.to_be_bytes()); // ligatureCount claims 2, coverage has only 1
        unsafe {
            let raw = otl_read_gpos_mark_to_ligature(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                0,
            );
            assert!(raw.is_null());
        }
    }

    #[test]
    fn max_component_count_times_class_count_is_rejected_not_read_oob() {
        // `component_count * class_count` can reach ~4.3 billion with
        // both factors at `u16::MAX` -- comfortably inside a 64-bit
        // `usize` (so `checked_mul` itself never overflows for any
        // u16-bounded pair; that's the point of using `usize` here
        // instead of the original's `i32`), but `require_room` still
        // correctly rejects it against this 34-byte buffer rather than
        // reading anywhere close to that many bytes.
        let mut data = well_formed_data();
        data[6..8].copy_from_slice(&u16::MAX.to_be_bytes()); // classCount
        data[30..32].copy_from_slice(&u16::MAX.to_be_bytes()); // componentCount
        unsafe {
            let raw = otl_read_gpos_mark_to_ligature(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                0,
            );
            assert!(raw.is_null());
        }
    }
}
