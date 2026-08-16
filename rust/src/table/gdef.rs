#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::parsed_json::{ParsedValue, json_arr_at, json_arr_len, json_obj_get, json_obj_get_type, json_obj_getint, json_obj_getnum, json_obj_key_bytes_at, json_obj_len, json_obj_val_at, json_type_of};
use crate::table::otl::classdef::{ClassDef, otl_class_def_free, read_class_def};
use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_name, otfcc_handle_dup, Handle, GlyphHandle, HandleState};
use crate::support::binio::{read_16u};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, Pos, ShapeId};
use crate::vendor::json::{JsonType};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::classdef::{dump_class_def, parse_class_def, build_class_def};
use crate::table::otl::coverage::{build_coverage};
use crate::support::built_json::{BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_object_push_bytes_key, preserialize};
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub glyph_class_def: *mut ClassDef,
    pub mark_attach_class_def: *mut ClassDef,
    pub lig_carets: LigCaretTable,
}
// Stage 6-4 "Box化": `glyph_class_def`/`mark_attach_class_def` are the only
// allocations this struct owns beyond `lig_carets` (already a `Vec`, freed
// by its own drop glue) -- left as raw pointers for this PR (their own
// Box化 is a separate future task), freed the same way `dispose_gdef`
// always did. Replaces the entire `table_gdef_init`/`_dispose`/`_create`/
// `_free` quartet -- construction now goes through `Box::new` directly at
// each call site (`otfcc_read_gdef`/`otfcc_parse_gdef`), matching the
// `GaspTable`/`VorgTable` precedent.
impl Drop for GdefTable {
    fn drop(&mut self) {
        unsafe {
            if !self.glyph_class_def.is_null() {
                otl_class_def_free(self.glyph_class_def);
            }
            if !self.mark_attach_class_def.is_null() {
                otl_class_def_free(self.mark_attach_class_def);
            }
            self.lig_carets.clear();
        }
    }
}
// `table_gdef_copy`'s old `memcpy`-based body is gone outright, not
// `.clone()`-ported: it was unreachable even before this conversion (only
// ever assigned into `GdefTableElementInterface.copy`, never called through
// that field or by name -- confirmed by grep across the crate), and a bitwise
// memcpy would double-free `lig_carets` now that it owns a `Vec`.
unsafe fn read_caret_value(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
) -> CaretValue {
    let mut v: CaretValue = CaretValue {
        format: 0,
        coordiante: 0.,
        point_index: 0,
    };
    v.format = 0 as i8;
    v.coordiante = 0 as ::core::ffi::c_int as Pos;
    v.point_index = 0xffff as ::core::ffi::c_int as i16;
    if !(table_length < offset.wrapping_add(4 as u32)) {
        v.format = read_16u(data.offset(offset as isize) as *const u8) as i8;
        if v.format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            v.point_index = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as i16;
        } else {
            v.coordiante = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as Pos;
        }
    }
    return v;
}
// `extern "C"` here is c2rust's calling-convention residue, not real FFI --
// this is only ever called from `otfcc_read_gdef` in this same crate. The
// non-`repr(C)` `CaretValueRecord` return (owns a `Vec`) is fine for an
// internal call; nothing crosses the actual C ABI boundary through it.
#[allow(improper_ctypes_definitions)]
unsafe fn read_lig_caret_record(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
) -> CaretValueRecord {
    let mut caret_count: ShapeId = 0;
    let mut g: CaretValueRecord = CaretValueRecord {
        glyph: Handle {
            state: HandleState::Empty,
            index: 0,
            name: Vec::new(),
        },
        carets: Vec::new(),
    };
    if !(table_length < offset.wrapping_add(2 as u32)) {
        caret_count = read_16u(data.offset(offset as isize) as *const u8) as ShapeId;
        if !(table_length
            < offset.wrapping_add(2 as u32).wrapping_add(
                (caret_count as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
            ))
        {
            let mut j: GlyphId = 0 as GlyphId;
            while (j as ::core::ffi::c_int) < caret_count as ::core::ffi::c_int {
                g.carets.push(
                    read_caret_value(
                        data,
                        table_length,
                        offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(
                                    (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        ) as u32),
                    ),
                );
                j = j.wrapping_add(1);
            }
        }
    }
    return g;
}
pub unsafe fn otfcc_read_gdef(
    packet: &Packet,
    mut _options: *const Options,
) -> Option<Box<GdefTable>> {
    let mut classdef_offset: u16 = 0;
    let mut lig_caret_offset: u16 = 0;
    let mut mark_attach_def_offset: u16 = 0;
    let mut current_block: u64;
    let mut gdef: Option<Box<GdefTable>> = None;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_GDEF {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data.as_ptr() as FontFilePointer;
                    let mut table_length: u32 = table.length;
                    if !(table_length < 12 as u32) {
                        gdef = Some(Box::new(GdefTable {
                            glyph_class_def: ::core::ptr::null_mut::<ClassDef>(),
                            mark_attach_class_def: ::core::ptr::null_mut::<ClassDef>(),
                            lig_carets: Vec::new(),
                        }));
                        classdef_offset = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if classdef_offset != 0 {
                            gdef.as_mut().unwrap().glyph_class_def =
                                read_class_def(
                                    data as *const u8,
                                    table_length,
                                    classdef_offset as u32,
                                );
                        }
                        lig_caret_offset = read_16u(
                            data.offset(8 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if lig_caret_offset != 0 {
                            if table_length
                                < (lig_caret_offset as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                    as u32
                            {
                                current_block = 10802812094495641425;
                            } else {
                                let mut cov: *mut Coverage =
                                    read_coverage(
                                        data as *const u8,
                                        table_length,
                                        (lig_caret_offset as ::core::ffi::c_int
                                            + read_16u(data.offset(
                                                lig_caret_offset as ::core::ffi::c_int as isize,
                                            )
                                                as *const u8)
                                                as ::core::ffi::c_int)
                                            as u32,
                                    );
                                if cov.is_null()
                                    || (*cov).len() as ::core::ffi::c_int
                                        != read_16u(
                                            data.offset(
                                                lig_caret_offset as ::core::ffi::c_int as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize)
                                                as *const u8,
                                        )
                                            as ::core::ffi::c_int
                                {
                                    current_block = 10802812094495641425;
                                } else if table_length
                                    < (lig_caret_offset as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int
                                        + (*cov).len() as ::core::ffi::c_int
                                            * 2 as ::core::ffi::c_int)
                                        as u32
                                {
                                    current_block = 10802812094495641425;
                                } else {
                                    let mut j: GlyphId = 0 as GlyphId;
                                    while (j as ::core::ffi::c_int)
                                        < (*cov).len() as ::core::ffi::c_int
                                    {
                                        let mut v: CaretValueRecord = read_lig_caret_record(
                                            data,
                                            table_length,
                                            (lig_caret_offset as ::core::ffi::c_int
                                                + read_16u(
                                                    data.offset(
                                                        lig_caret_offset as ::core::ffi::c_int
                                                            as isize,
                                                    )
                                                    .offset(4 as ::core::ffi::c_int as isize)
                                                    .offset(
                                                        (j as ::core::ffi::c_int
                                                            * 2 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                        as *const u8,
                                                )
                                                    as ::core::ffi::c_int)
                                                as u32,
                                        );
                                        v.glyph =
                                            otfcc_handle_dup(
                                                (&(*cov))[j as usize].clone() as Handle,
                                            )
                                                as GlyphHandle;
                                        gdef.as_mut().unwrap().lig_carets.push(v);
                                        j = j.wrapping_add(1);
                                    }
                                    otl_coverage_free(cov);
                                    current_block = 11307063007268554308;
                                }
                            }
                        } else {
                            current_block = 11307063007268554308;
                        }
                        match current_block {
                            10802812094495641425 => {}
                            _ => {
                                mark_attach_def_offset =
                                    read_16u(data.offset(10 as ::core::ffi::c_int as isize)
                                        as *const u8);
                                if mark_attach_def_offset != 0 {
                                    gdef.as_mut().unwrap().mark_attach_class_def =
                                        read_class_def(
                                            data as *const u8,
                                            table_length,
                                            mark_attach_def_offset as u32,
                                        );
                                }
                                return gdef;
                            }
                        }
                    }
                    gdef = None;
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return gdef;
}
unsafe fn dump_gdef_lig_carets(mut gdef: *const GdefTable) -> *mut BuiltValue {
    let lig_carets: &Vec<CaretValueRecord> = &(*gdef).lig_carets;
    let mut _carets: *mut BuiltValue = json_object_new(lig_carets.len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < lig_carets.len() {
        let name: &[u8] = &lig_carets[j as usize].glyph.name;
        let carets: &Vec<CaretValue> = &lig_carets[j as usize].carets;
        let mut _record: *mut BuiltValue = json_array_new(carets.len());
        let mut k: GlyphId = 0 as GlyphId;
        while (k as usize) < carets.len() {
            let mut _cv: *mut BuiltValue = json_object_new(1 as usize);
            if carets[k as usize].format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
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
    mut root: *mut BuiltValue,
    mut options: *const Options,
) {
    let gdef = match gdef {
        Some(g) => g as *const GdefTable,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::bytesbuild!(b"GDEF"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _gdef: *mut BuiltValue = json_object_new(4 as usize);
        if !(*gdef).glyph_class_def.is_null() {
            json_object_push(
                _gdef,
                b"glyphClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                dump_class_def((*gdef).glyph_class_def),
            );
        }
        if !(*gdef).mark_attach_class_def.is_null() {
            json_object_push(
                _gdef,
                b"markAttachClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                dump_class_def((*gdef).mark_attach_class_def),
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
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
unsafe fn lig_caret_from_json(
    mut _carets: *const ParsedValue,
    mut lc: *mut LigCaretTable,
) {
    if _carets.is_null()
        || json_type_of(_carets) != JsonType::Object
    {
        return;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < json_obj_len(_carets) {
        let mut a: *const ParsedValue = json_obj_val_at(_carets, j as u32);
        if !(a.is_null()
            || json_type_of(a) != JsonType::Array)
        {
            let mut v: CaretValueRecord = CaretValueRecord {
                glyph: Handle {
                    state: HandleState::Empty,
                    index: 0,
                    name: Vec::new(),
                },
                carets: Vec::new(),
            };
            v.glyph = handle_from_name(Some(json_obj_key_bytes_at(_carets, j as u32))) as GlyphHandle;
            let mut caret_count: ShapeId = json_arr_len(a) as ShapeId;
            let mut k: GlyphId = 0 as GlyphId;
            while (k as ::core::ffi::c_int) < caret_count as ::core::ffi::c_int {
                let mut caret: CaretValue = CaretValue {
                    format: 0,
                    coordiante: 0.,
                    point_index: 0,
                };
                caret.format = 1 as i8;
                caret.coordiante = 0 as ::core::ffi::c_int as Pos;
                caret.point_index = 0xffff as ::core::ffi::c_int as i16;
                let mut _caret: *const ParsedValue = json_arr_at(a, k as u32);
                if !_caret.is_null()
                    && json_type_of(_caret) == JsonType::Object
                {
                    if !json_obj_get_type(
                        _caret,
                        b"atPoint\0" as *const u8 as *const ::core::ffi::c_char,
                        JsonType::Integer,
                    )
                    .is_null()
                    {
                        caret.format = 2 as i8;
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
    mut root: *const ParsedValue,
    mut options: *const Options,
) -> Option<Box<GdefTable>> {
    let mut gdef: Option<Box<GdefTable>> = None;
    let mut table: *const ParsedValue = ::core::ptr::null();
    table = json_obj_get_type(
        root,
        b"GDEF\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::bytesbuild!(b"GDEF"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            gdef = Some(Box::new(GdefTable {
                glyph_class_def: ::core::ptr::null_mut::<ClassDef>(),
                mark_attach_class_def: ::core::ptr::null_mut::<ClassDef>(),
                lig_carets: Vec::new(),
            }));
            gdef.as_mut().unwrap().glyph_class_def =
                parse_class_def(json_obj_get(
                    table,
                    b"glyphClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            gdef.as_mut().unwrap().mark_attach_class_def =
                parse_class_def(json_obj_get(
                    table,
                    b"markAttachClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            lig_caret_from_json(
                json_obj_get(
                    table,
                    b"ligCarets\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &raw mut gdef.as_mut().unwrap().lig_carets,
            );
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return gdef;
}
unsafe fn write_lig_caret_rec(mut cr: *mut CaretValueRecord) -> *mut BkBlock {
    let carets: &Vec<CaretValue> = &(*cr).carets;
    let mut bcr: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (carets.len()) as u32)]);
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < carets.len() {
        let caret = &carets[j as usize];
        bk_push(bcr, &[bk_ptr(BkCellType::P16, bk_new_block(&[bk_int(BkCellType::B16, (caret.format as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (if caret.format as ::core::ffi::c_int
                    == 2 as ::core::ffi::c_int
                {
                    caret.point_index as ::core::ffi::c_int
                } else {
                    caret.coordiante as i16
                        as ::core::ffi::c_int
                }) as u32)]))]);
        j = j.wrapping_add(1);
    }
    return bcr;
}
unsafe fn write_lig_carets(mut lc: *const LigCaretTable) -> *mut BkBlock {
    let records: &Vec<CaretValueRecord> = &*lc;
    let mut cov: *mut Coverage = otl_coverage_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < records.len() {
        push_to_coverage(
            cov,
            otfcc_handle_dup(records[j as usize].glyph.clone() as Handle) as GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let mut lct: *mut BkBlock = bk_new_block(&[bk_ptr(BkCellType::P16, bk_new_block_from_buffer(build_coverage(cov))), bk_int(BkCellType::B16, (records.len()) as u32)]);
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < records.len() {
        bk_push(lct, &[bk_ptr(BkCellType::P16, write_lig_caret_rec(&records[j_0 as usize] as *const CaretValueRecord as *mut CaretValueRecord))]);
        j_0 = j_0.wrapping_add(1);
    }
    otl_coverage_free(cov);
    return lct;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_gdef(
    gdef: Option<&GdefTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let gdef = match gdef {
        Some(g) => g as *const GdefTable,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut b_glyph_class_def: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    let mut b_attach_list: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    let mut b_lig_caret_list: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    let mut b_mark_attach_class_def: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    if !(*gdef).glyph_class_def.is_null() {
        b_glyph_class_def =
            bk_new_block_from_buffer(build_class_def(
                (*gdef).glyph_class_def,
            ));
    }
    if !(*gdef).lig_carets.is_empty() {
        b_lig_caret_list = write_lig_carets(&raw const (*gdef).lig_carets);
    }
    if !(*gdef).mark_attach_class_def.is_null() {
        b_mark_attach_class_def =
            bk_new_block_from_buffer(build_class_def(
                (*gdef).mark_attach_class_def,
            ));
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B32, 0x10000 as u32), bk_ptr(BkCellType::P16, b_glyph_class_def), bk_ptr(BkCellType::P16, b_attach_list), bk_ptr(BkCellType::P16, b_lig_caret_list), bk_ptr(BkCellType::P16, b_mark_attach_class_def)]);
    return bk_build_block(root);
}
