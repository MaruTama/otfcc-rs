#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};


use crate::support::parsed_json::{ParsedValue, json_obj_get_type, json_obj_key_at, json_obj_key_len_at, json_obj_len, json_obj_val_at, json_type_of};
use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_name, otfcc_handle_dup, Handle, GlyphHandle, HandleState};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphClass, GlyphId};
use crate::vendor::json::{JsonType};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::table::otl::{Anchor, BaseArray, BaseRecord, Subtable, GposMarkToSingleSubtable, subtable_from_raw};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::table::otl::subtables::gpos_common::{bk_from_anchor, otl_anchor_absent, otl_parse_mark_array, otl_parse_anchor, otl_read_mark_array, otl_read_anchor};
use crate::support::built_json::{BuiltValue, json_integer_new, json_object_new, json_object_push, json_object_push_bytes_key, json_string_new_from_bytes, preserialize};
use crate::vendor::sds::{sdsnewlen};
// `BaseRecord.anchors` is a plain `Vec<Anchor>` now and `glyph: GlyphHandle`
// already has its own `Drop`, so a `BaseArray` (`Vec<BaseRecord>`) fully
// self-drops -- clearing it (still needed: `consolidate/otl/mark.rs`'s dedup
// pass clears an in-place array mid-function, not just at end of scope) is
// exactly `*arr = Vec::new()`.
pub(crate) unsafe fn dispose_base_array(arr: *mut BaseArray) {
    *arr = Vec::new();
}
unsafe extern "C" fn init_mark_to_single(subtable: *mut GposMarkToSingleSubtable) {
    (*subtable).mark_array = Vec::new();
    (*subtable).base_array = Vec::new();
}
pub(crate) unsafe extern "C" fn subtable_gpos_mark_to_single_free(x: *mut GposMarkToSingleSubtable) {
    if x.is_null() {
        return;
    }
    // `mark_array`/`base_array` both self-drop; `ptr::read` moves the whole
    // value out of the malloc'd shell so `drop` can run that field-by-field
    // teardown, then `free` releases the now-empty shell -- the same
    // "unwrap_X_table" idiom used throughout Stage 6-4, minus the
    // `Box::new` at the end since nothing adopts this value.
    drop(::core::ptr::read(x));
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
                        let mut base_anchors: Vec<Anchor> =
                            Vec::with_capacity((*subtable).class_count as usize);
                        let mut k: GlyphClass = 0 as GlyphClass;
                        while (k as ::core::ffi::c_int)
                            < (*subtable).class_count as ::core::ffi::c_int
                        {
                            if read_16u(data.offset(_offset as isize) as *const u8) != 0 {
                                base_anchors.push(otl_read_anchor(
                                    data,
                                    table_length,
                                    base_array_offset.wrapping_add(read_16u(
                                        data.offset(_offset as isize) as *const u8,
                                    )
                                        as u32),
                                ));
                            } else {
                                base_anchors.push(otl_anchor_absent());
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
                    return subtable_from_raw(subtable, Subtable::GposMarkToSingle);
                }
            }
        }
    }
    subtable_gpos_mark_to_single_free(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gpos_dump_mark_to_single(
    mut st: *const Subtable,
) -> *mut BuiltValue {
    let Subtable::GposMarkToSingle(mut_subtable) = &*st else { unreachable!() };
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
            if (&(*subtable).base_array)[j_0 as usize].anchors[k as usize].present
            {
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
                let mark_class_name_0: Vec<u8> = crate::bytesbuild!(b"anchor", k as ::core::ffi::c_int);
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
unsafe extern "C" fn parse_bases(
    mut _bases: *const ParsedValue,
    mut subtable: *mut GposMarkToSingleSubtable,
    mut h: *mut std::collections::BTreeMap<Vec<u8>, GlyphClass>,
    mut options: *const Options,
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
        base.glyph = handle_from_name(sdsnewlen(
            gname as *const ::core::ffi::c_void,
            json_obj_key_len_at(_bases, j as u32) as usize,
        )) as GlyphHandle;
        // Indexed by `class_id` below, out of JSON key order -- pre-sized
        // and filled with "absent" rather than built with `.push()`.
        base.anchors = vec![otl_anchor_absent(); class_count as usize];
        let mut base_record: *const ParsedValue = json_obj_val_at(_bases, j as u32);
        if base_record.is_null()
            || json_type_of(base_record) != JsonType::Object
        {
            (*subtable).base_array.push(base);
        } else {
            let mut k_0: GlyphClass = 0 as GlyphClass;
            while (k_0 as ::core::ffi::c_uint) < json_obj_len(base_record) {
                let name_ptr: *mut ::core::ffi::c_char = json_obj_key_at(base_record, k_0 as u32);
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
                            crate::bytesbuild!(b"[OTFCC-fea] Invalid anchor class name <",
                                name_ptr,
                                b"> for /",
                                gname,
                                b". This base anchor is ignored.\n",
                            ),
                        );
                    }
                    Some(&class_id) => {
                        base.anchors[class_id as usize] = otl_parse_anchor(
                            json_obj_val_at(base_record, k_0 as u32),
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
    otl_parse_mark_array(_marks, &raw mut (*st).mark_array, &raw mut h, options);
    (*st).class_count = h.len() as GlyphClass;
    parse_bases(_bases, st, &raw mut h, options);
    return subtable_from_raw(st, Subtable::GposMarkToSingle);
}
pub unsafe extern "C" fn otfcc_build_gpos_mark_to_single(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let Subtable::GposMarkToSingle(mut_subtable) = &*_subtable else { unreachable!() };
    let subtable: *const GposMarkToSingleSubtable = mut_subtable;
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
                    (&(*subtable).base_array)[j_2 as usize]
                        .anchors[k as usize],
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
