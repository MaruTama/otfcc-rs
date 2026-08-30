#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::Packet;
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::buffer::Buffer;
use crate::support::font_reader::FontReader;
use crate::support::handle::{
    GlyphHandle, Handle, HandleState, handle_from_name, otfcc_handle_dup,
};
use crate::support::options::Options;
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_obj_get, json_obj_get_type, json_obj_getint,
    json_obj_getnum, json_obj_key_bytes_at, json_obj_len, json_obj_val_at, json_type_of,
};
use crate::support::primitives::{GlyphId, Pos, ShapeId};
use crate::table::otl::classdef::{ClassDef, classdef_from_raw, read_class_def};
use crate::table::otl::coverage::{
    Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage,
};
use crate::vendor::json::JsonType;

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkgraph::bk_build_block;
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new,
    json_object_push, json_object_push_bytes_key, preserialize,
};
use crate::table::otl::classdef::{build_class_def, dump_class_def, parse_class_def};
use crate::table::otl::coverage::build_coverage;
#[derive(Copy, Clone)]
pub struct CaretValue {
    pub format: i8,
    pub coordiante: Pos,
    pub point_index: i16,
}
pub type CaretValueList = Vec<CaretValue>;
pub struct CaretValueRecord {
    pub glyph: GlyphHandle,
    pub carets: CaretValueList,
}
// `CaretValueRecord` embeds `GlyphHandle`, which now owns its `sds` name for
// real (`Handle`'s `Drop`/`Clone`, Stage 6-4's `Handle` pilot), so a derived
// `Clone` would compose correctly here -- but no dup is written because
// nothing in this file or `consolidate/otl/gdef.rs` ever duplicates a
// `CaretValueRecord` (verified: every touch is either a move via
// `mem::take`/`Vec::push` of a freshly-built value, or a dispose), so there
// is nothing for a `Clone` impl to be used for.
pub type LigCaretTable = Vec<CaretValueRecord>;
// Shared by `dispose_gdef` (whole-table teardown) and `consolidate_gdef`
// (rebuild-in-place, formerly `OTL_I_LIG_CARET_TABLE.clear`). `Vec::clear`
// alone is enough: each record's compiler-generated drop glue frees its
// `Handle`'s name and its `Vec<CaretValue>` backing array.
pub(crate) unsafe fn clear_lig_carets(lc: *mut LigCaretTable) {
    (*lc).clear();
}
pub struct GdefTable {
    pub glyph_class_def: Option<Box<ClassDef>>,
    pub mark_attach_class_def: Option<Box<ClassDef>>,
    pub lig_carets: LigCaretTable,
}
// Stage 6-4 "Box化" Box-ified the outer `GdefTable` itself (replacing the
// entire `table_gdef_init`/`_dispose`/`_create`/`_free` quartet). Stage
// 7-2-c "inner Box化" finishes the job here: `glyph_class_def`/
// `mark_attach_class_def` become `Option<Box<ClassDef>>`, the exact same
// shape `table/otl.rs`'s `ChainingRuleSet.bc`/`.ic`/`.fc` and
// `GposPairSubtable.first`/`.second` already use for this same `ClassDef`
// type. `ClassDef` itself has no manual `Drop` impl -- it is a plain
// `Vec`-holding struct (`glyphs: Vec<GlyphHandle>`, `classes:
// Vec<GlyphClass>`) that already self-drops correctly, and
// `otl_class_def_free` (the function this used to call) is itself just
// `drop(Box::from_raw(x))`, i.e. exactly what `Option<Box<ClassDef>>`'s own
// drop glue now does directly. No manual `Drop` impl remains: both class-def
// fields and `lig_carets` (a plain `Vec`) all self-drop now.
// `table_gdef_copy`'s old `memcpy`-based body is gone outright, not
// `.clone()`-ported: it was unreachable even before this conversion (only
// ever assigned into `GdefTableElementInterface.copy`, never called through
// that field or by name -- confirmed by grep across the crate), and a bitwise
// memcpy would double-free `lig_carets` now that it owns a `Vec`.
fn read_caret_value(data: &[u8], offset: usize) -> CaretValue {
    let mut v: CaretValue = CaretValue {
        format: 0,
        coordiante: 0.,
        point_index: 0xffffu16 as i16,
    };
    if let Ok(bytes) = FontReader::new(data).at(offset).and_then(|mut r| r.bytes(4)) {
        let format = u16::from_be_bytes([bytes[0], bytes[1]]);
        let value = u16::from_be_bytes([bytes[2], bytes[3]]);
        v.format = format as i8;
        if format == 2 {
            v.point_index = value as i16;
        } else {
            v.coordiante = value as Pos;
        }
    }
    v
}
fn read_lig_caret_record(data: &[u8], offset: usize) -> CaretValueRecord {
    let mut g: CaretValueRecord = CaretValueRecord {
        glyph: Handle {
            state: HandleState::Empty,
            index: 0,
            name: Vec::new(),
        },
        carets: Vec::new(),
    };
    let Ok(mut r) = FontReader::new(data).at(offset) else {
        return g;
    };
    let Ok(caret_count) = r.u16() else { return g };
    if r.require_room(caret_count as usize, 2).is_err() {
        return g;
    }
    for _ in 0..caret_count {
        let caret_rel = r.u16().unwrap();
        g.carets.push(read_caret_value(data, offset + caret_rel as usize));
    }
    g
}
/// The LigCaretList (`CoverageOffset`/`LigGlyphCount`/`LigGlyphOffset[]`),
/// isolated out of `otfcc_read_gdef` because its three failure conditions
/// each abort the *whole* GDEF table (matching the original's `current_
/// block` goto-emulation, which skipped straight past `mark_attach_class_
/// def` and returned `None` on any of them) rather than just leaving
/// `lig_carets` empty -- `?` on this function's `None` reproduces that
/// exactly. `lig_caret_offset == 0` (no LigCaretList at all) returns
/// `Some(Vec::new())`, matching the original's `current_block` value for
/// "nothing to do, continue on to mark_attach_class_def".
///
/// Also fixes a real pre-existing leak: the original allocated `cov` via
/// `read_coverage` before checking whether it was even well-formed, and
/// never freed it on either of the two abort branches below (only the
/// success branch called `otl_coverage_free`) -- `cov` leaked on every
/// malformed LigCaretList. Every return path here frees it.
unsafe fn read_lig_carets(data: &[u8], lig_caret_offset: usize) -> Option<LigCaretTable> {
    if lig_caret_offset == 0 {
        return Some(Vec::new());
    }
    if data.len() < lig_caret_offset + 4 {
        return None;
    }
    let coverage_rel = FontReader::new(data).at(lig_caret_offset).ok()?.u16().ok()? as usize;
    let cov: *mut Coverage =
        read_coverage(data.as_ptr(), data.len() as u32, (lig_caret_offset + coverage_rel) as u32);
    if cov.is_null() {
        return None;
    }
    let lig_glyph_count = FontReader::new(data).at(lig_caret_offset + 2).ok()?.u16().ok()?;
    if (*cov).len() != lig_glyph_count as usize {
        otl_coverage_free(cov);
        return None;
    }
    if data.len() < lig_caret_offset + 4 + (*cov).len() * 2 {
        otl_coverage_free(cov);
        return None;
    }
    let Ok(mut off_reader) = FontReader::new(data).at(lig_caret_offset + 4) else {
        otl_coverage_free(cov);
        return None;
    };
    let mut result = Vec::with_capacity((*cov).len());
    for j in 0..(*cov).len() {
        let Ok(lig_glyph_rel) = off_reader.u16() else {
            otl_coverage_free(cov);
            return None;
        };
        let mut v = read_lig_caret_record(data, lig_caret_offset + lig_glyph_rel as usize);
        v.glyph = otfcc_handle_dup((&(*cov))[j].clone() as Handle) as GlyphHandle;
        result.push(v);
    }
    otl_coverage_free(cov);
    Some(result)
}
pub unsafe fn otfcc_read_gdef(packet: &Packet) -> Option<Box<GdefTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_GDEF)?;
    let data: &[u8] = &table.data;
    if data.len() < 12 {
        return None;
    }
    let classdef_offset = FontReader::new(data).at(4).ok()?.u16().ok()?;
    let glyph_class_def = if classdef_offset != 0 {
        classdef_from_raw(read_class_def(
            data.as_ptr(),
            data.len() as u32,
            classdef_offset as u32,
        ))
    } else {
        None
    };

    let lig_caret_offset = FontReader::new(data).at(8).ok()?.u16().ok()? as usize;
    let lig_carets = read_lig_carets(data, lig_caret_offset)?;

    let mark_attach_def_offset = FontReader::new(data).at(10).ok()?.u16().ok()?;
    let mark_attach_class_def = if mark_attach_def_offset != 0 {
        classdef_from_raw(read_class_def(
            data.as_ptr(),
            data.len() as u32,
            mark_attach_def_offset as u32,
        ))
    } else {
        None
    };

    Some(Box::new(GdefTable {
        glyph_class_def,
        mark_attach_class_def,
        lig_carets,
    }))
}
unsafe fn dump_gdef_lig_carets(gdef: *const GdefTable) -> *mut BuiltValue {
    let lig_carets: &Vec<CaretValueRecord> = &(*gdef).lig_carets;
    let mut _carets: *mut BuiltValue = json_object_new(lig_carets.len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < lig_carets.len() {
        let name: &[u8] = &lig_carets[j as usize].glyph.name;
        let carets: &Vec<CaretValue> = &lig_carets[j as usize].carets;
        let mut _record: *mut BuiltValue = json_array_new(carets.len());
        let mut k: GlyphId = 0 as GlyphId;
        while (k as usize) < carets.len() {
            let mut _cv: *mut BuiltValue = json_object_new(1_usize);
            if carets[k as usize].format as i32 == 2_i32 {
                json_object_push(
                    _cv,
                    b"atPoint\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(carets[k as usize].point_index as i64),
                );
            } else {
                json_object_push(
                    _cv,
                    b"at\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(carets[k as usize].coordiante as i64),
                );
            }
            json_array_push(_record, _cv);
            k = k.wrapping_add(1);
        }
        json_object_push_bytes_key(_carets, name, preserialize(_record));
        j = j.wrapping_add(1);
    }
    return _carets;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_gdef(
    gdef: Option<&GdefTable>,
    root: *mut BuiltValue,
    options: &Options,
) {
    let gdef = match gdef {
        Some(g) => g as *const GdefTable,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"GDEF"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _gdef: *mut BuiltValue = json_object_new(4_usize);
        if let Some(cd) = (*gdef).glyph_class_def.as_deref() {
            json_object_push(
                _gdef,
                b"glyphClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                dump_class_def(cd),
            );
        }
        if let Some(cd) = (*gdef).mark_attach_class_def.as_deref() {
            json_object_push(
                _gdef,
                b"markAttachClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                dump_class_def(cd),
            );
        }
        if !(*gdef).lig_carets.is_empty() {
            json_object_push(
                _gdef,
                b"ligCarets\0" as *const u8 as *const ::core::ffi::c_char,
                dump_gdef_lig_carets(gdef),
            );
        }
        json_object_push(
            root,
            b"GDEF\0" as *const u8 as *const ::core::ffi::c_char,
            _gdef,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
unsafe fn lig_caret_from_json(mut _carets: *const ParsedValue, lc: *mut LigCaretTable) {
    if _carets.is_null() || json_type_of(_carets) != JsonType::Object {
        return;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < json_obj_len(_carets) {
        let a: *const ParsedValue = json_obj_val_at(_carets, j as u32);
        if !(a.is_null() || json_type_of(a) != JsonType::Array) {
            let mut v: CaretValueRecord = CaretValueRecord {
                glyph: Handle {
                    state: HandleState::Empty,
                    index: 0,
                    name: Vec::new(),
                },
                carets: Vec::new(),
            };
            v.glyph =
                handle_from_name(Some(json_obj_key_bytes_at(_carets, j as u32))) as GlyphHandle;
            let caret_count: ShapeId = json_arr_len(a) as ShapeId;
            let mut k: GlyphId = 0 as GlyphId;
            while (k as i32) < caret_count as i32 {
                let mut caret: CaretValue = CaretValue {
                    format: 0,
                    coordiante: 0.,
                    point_index: 0,
                };
                caret.format = 1_i8;
                caret.coordiante = 0_i32 as Pos;
                caret.point_index = 0xffff_i32 as i16;
                let mut _caret: *const ParsedValue = json_arr_at(a, k as u32);
                if !_caret.is_null() && json_type_of(_caret) == JsonType::Object {
                    if !json_obj_get_type(
                        _caret,
                        b"atPoint\0" as *const u8 as *const ::core::ffi::c_char,
                        JsonType::Integer,
                    )
                    .is_null()
                    {
                        caret.format = 2_i8;
                        caret.point_index = json_obj_getint(
                            _caret,
                            b"atPoint\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as i16;
                    } else {
                        caret.coordiante = json_obj_getnum(
                            _caret,
                            b"at\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as Pos;
                    }
                }
                v.carets.push(caret);
                k = k.wrapping_add(1);
            }
            (*lc).push(v);
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe fn otfcc_parse_gdef(
    root: *const ParsedValue,
    options: &Options,
) -> Option<Box<GdefTable>> {
    let mut gdef: Option<Box<GdefTable>> = None;
    let table: *const ParsedValue;
    table = json_obj_get_type(
        root,
        b"GDEF\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"GDEF"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            gdef = Some(Box::new(GdefTable {
                glyph_class_def: None,
                mark_attach_class_def: None,
                lig_carets: Vec::new(),
            }));
            gdef.as_mut().unwrap().glyph_class_def =
                classdef_from_raw(parse_class_def(json_obj_get(
                    table,
                    b"glyphClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                )));
            gdef.as_mut().unwrap().mark_attach_class_def =
                classdef_from_raw(parse_class_def(json_obj_get(
                    table,
                    b"markAttachClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                )));
            lig_caret_from_json(
                json_obj_get(
                    table,
                    b"ligCarets\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &raw mut gdef.as_mut().unwrap().lig_carets,
            );
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
    }
    return gdef;
}
unsafe fn write_lig_caret_rec(cr: *mut CaretValueRecord) -> *mut BkBlock {
    let carets: &Vec<CaretValue> = &(*cr).carets;
    let bcr: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (carets.len()) as u32)]);
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < carets.len() {
        let caret = &carets[j as usize];
        bk_push(
            bcr,
            &[bk_ptr(
                BkCellType::P16,
                bk_new_block(&[
                    bk_int(BkCellType::B16, (caret.format as i32) as u32),
                    bk_int(
                        BkCellType::B16,
                        (if caret.format as i32 == 2_i32 {
                            caret.point_index as i32
                        } else {
                            caret.coordiante as i16 as i32
                        }) as u32,
                    ),
                ]),
            )],
        );
        j = j.wrapping_add(1);
    }
    return bcr;
}
unsafe fn write_lig_carets(lc: *const LigCaretTable) -> *mut BkBlock {
    let records: &Vec<CaretValueRecord> = &*lc;
    let cov: *mut Coverage = otl_coverage_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < records.len() {
        push_to_coverage(
            cov,
            otfcc_handle_dup(records[j as usize].glyph.clone() as Handle) as GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let lct: *mut BkBlock = bk_new_block(&[
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(build_coverage(cov).into_raw()),
        ),
        bk_int(BkCellType::B16, (records.len()) as u32),
    ]);
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < records.len() {
        bk_push(
            lct,
            &[bk_ptr(
                BkCellType::P16,
                write_lig_caret_rec(
                    &records[j_0 as usize] as *const CaretValueRecord as *mut CaretValueRecord,
                ),
            )],
        );
        j_0 = j_0.wrapping_add(1);
    }
    otl_coverage_free(cov);
    return lct;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_gdef(gdef: Option<&GdefTable>) -> *mut Buffer {
    let gdef = match gdef {
        Some(g) => g as *const GdefTable,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut b_glyph_class_def: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    let b_attach_list: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    let mut b_lig_caret_list: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    let mut b_mark_attach_class_def: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    if let Some(cd) = (*gdef).glyph_class_def.as_deref() {
        b_glyph_class_def = bk_new_block_from_buffer(build_class_def(cd).into_raw());
    }
    if !(*gdef).lig_carets.is_empty() {
        b_lig_caret_list = write_lig_carets(&raw const (*gdef).lig_carets);
    }
    if let Some(cd) = (*gdef).mark_attach_class_def.as_deref() {
        b_mark_attach_class_def = bk_new_block_from_buffer(build_class_def(cd).into_raw());
    }
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B32, 0x10000_u32),
        bk_ptr(BkCellType::P16, b_glyph_class_def),
        bk_ptr(BkCellType::P16, b_attach_list),
        bk_ptr(BkCellType::P16, b_lig_caret_list),
        bk_ptr(BkCellType::P16, b_mark_attach_class_def),
    ]);
    return bk_build_block(root).into_raw();
}

#[cfg(test)]
mod otfcc_read_gdef_tests {
    use super::*;
    use crate::font::caryll_sfnt::PacketPiece;

    fn packet_with_gdef(data: Vec<u8>) -> Packet {
        Packet {
            sfnt_version: 0,
            num_tables: 1,
            search_range: 0,
            entry_selector: 0,
            range_shift: 0,
            pieces: vec![PacketPiece {
                tag: crate::tag::TAG_GDEF,
                check_sum: 0,
                offset: 0,
                length: data.len() as u32,
                data,
            }],
        }
    }

    // header(12) + LigCaretList(4 + one 2-byte LigGlyphOffset) + one-glyph
    // Coverage(6) + one LigGlyphTable(4) + one CaretValue(4)
    fn well_formed_gdef_table() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&1u16.to_be_bytes()); // majorVersion
        b.extend_from_slice(&0u16.to_be_bytes()); // minorVersion
        b.extend_from_slice(&0u16.to_be_bytes()); // GlyphClassDefOffset
        b.extend_from_slice(&0u16.to_be_bytes()); // AttachListOffset
        b.extend_from_slice(&12u16.to_be_bytes()); // LigCaretListOffset
        b.extend_from_slice(&0u16.to_be_bytes()); // MarkAttachClassDefOffset
        // LigCaretList @12
        b.extend_from_slice(&6u16.to_be_bytes()); // CoverageOffset (rel to 12)
        b.extend_from_slice(&1u16.to_be_bytes()); // LigGlyphCount
        b.extend_from_slice(&12u16.to_be_bytes()); // LigGlyphOffset[0] (rel to 12)
        // Coverage @18
        b.extend_from_slice(&1u16.to_be_bytes()); // format
        b.extend_from_slice(&1u16.to_be_bytes()); // glyphCount
        b.extend_from_slice(&7u16.to_be_bytes()); // glyph[0]
        // LigGlyphTable @24
        b.extend_from_slice(&1u16.to_be_bytes()); // CaretCount
        b.extend_from_slice(&4u16.to_be_bytes()); // CaretValueOffset[0] (rel to 24)
        // CaretValue (format 1) @28
        b.extend_from_slice(&1u16.to_be_bytes()); // CaretValueFormat
        b.extend_from_slice(&100i16.to_be_bytes()); // Coordinate
        b
    }

    #[test]
    fn well_formed_table_reads_the_lig_caret() {
        let packet = packet_with_gdef(well_formed_gdef_table());
        let gdef = unsafe { otfcc_read_gdef(&packet).unwrap() };
        assert_eq!(gdef.lig_carets.len(), 1);
        assert_eq!(gdef.lig_carets[0].glyph.index, 7);
        assert_eq!(gdef.lig_carets[0].carets.len(), 1);
        assert_eq!(gdef.lig_carets[0].carets[0].format, 1);
        assert_eq!(gdef.lig_carets[0].carets[0].coordiante, 100.0);
    }

    #[test]
    fn truncated_header_is_rejected() {
        let mut data = well_formed_gdef_table();
        data.truncate(10);
        let packet = packet_with_gdef(data);
        assert!(unsafe { otfcc_read_gdef(&packet) }.is_none());
    }

    #[test]
    fn zero_lig_caret_offset_leaves_lig_carets_empty() {
        let mut data = well_formed_gdef_table();
        data[8..10].copy_from_slice(&0u16.to_be_bytes()); // LigCaretListOffset = 0
        let packet = packet_with_gdef(data);
        let gdef = unsafe { otfcc_read_gdef(&packet).unwrap() };
        assert!(gdef.lig_carets.is_empty());
    }

    #[test]
    fn lig_glyph_count_mismatched_with_coverage_aborts_the_whole_table() {
        // LigGlyphCount says 1 but the Coverage table only has 1 glyph too
        // in the fixture -- flip it to 2 so they disagree. The original
        // treated this as fatal for the entire GDEF table, not just the
        // LigCaretList, so `otfcc_read_gdef` must return `None` even
        // though `GlyphClassDefOffset`/`MarkAttachClassDefOffset` are both
        // absent (0) and would otherwise be trivially fine.
        let mut data = well_formed_gdef_table();
        data[14..16].copy_from_slice(&2u16.to_be_bytes()); // LigGlyphCount = 2
        let packet = packet_with_gdef(data);
        assert!(unsafe { otfcc_read_gdef(&packet) }.is_none());
    }

    #[test]
    fn lig_caret_list_offset_past_the_table_end_aborts_the_whole_table() {
        let mut data = well_formed_gdef_table();
        data[8..10].copy_from_slice(&1000u16.to_be_bytes()); // LigCaretListOffset
        let packet = packet_with_gdef(data);
        assert!(unsafe { otfcc_read_gdef(&packet) }.is_none());
    }
}
