#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc};

use crate::support::json_funcs::{json_obj_get, preserialize};
use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_name, otfcc_handle_dispose, otfcc_handle_dup, Handle, GlyphHandle};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::table::otl::{Anchor, GposCursiveEntry, Subtable, GposCursiveSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::table::otl::subtables::gpos_common::{bk_from_anchor, otl_anchor_absent, otl_dump_anchor, otl_parse_anchor, otl_read_anchor};
use crate::vendor::json_builder::{json_object_new, json_object_push};
use crate::vendor::sds::{sdsnewlen};
unsafe extern "C" fn delete_gpos_cursive_entry(mut entry: *mut GposCursiveEntry) {
    otfcc_handle_dispose(&raw mut (*entry).target);
}
pub(crate) unsafe fn dispose_gpos_cursive_subtable(arr: *mut GposCursiveSubtable) {
    for e in (*arr).iter_mut() {
        delete_gpos_cursive_entry(e);
    }
    *arr = Vec::new();
}
pub(crate) unsafe extern "C" fn subtable_gpos_cursive_free(x: *mut GposCursiveSubtable) {
    if x.is_null() {
        return;
    }
    dispose_gpos_cursive_subtable(x);
    free(x as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn subtable_gpos_cursive_create() -> *mut GposCursiveSubtable {
    let x: *mut GposCursiveSubtable =
        malloc(::core::mem::size_of::<GposCursiveSubtable>() as usize)
            as *mut GposCursiveSubtable;
    x.write(Vec::new());
    x
}
pub unsafe extern "C" fn otl_read_gpos_cursive(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    _max_glyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut value_count: GlyphId = 0;
    let subtable: *mut GposCursiveSubtable = subtable_gpos_cursive_create();
    let mut targets: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    if !(table_length < offset.wrapping_add(6 as u32)) {
        targets = read_coverage(
            data as *const u8,
            table_length,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !(targets.is_null()
            || (*targets).len() as GlyphId as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            value_count = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as GlyphId;
            if !(table_length
                < offset.wrapping_add(6 as u32).wrapping_add(
                    (4 as ::core::ffi::c_int * value_count as ::core::ffi::c_int) as u32,
                ))
            {
                if !(value_count as ::core::ffi::c_int != (*targets).len() as GlyphId as ::core::ffi::c_int)
                {
                    let mut j: GlyphId = 0 as GlyphId;
                    while (j as ::core::ffi::c_int) < value_count as ::core::ffi::c_int {
                        let mut enter_offset: u16 = read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (4 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        );
                        let mut exit_offset: u16 = read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (4 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                                )
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const u8,
                        );
                        let mut enter: Anchor = otl_anchor_absent();
                        let mut exit: Anchor = otl_anchor_absent();
                        if enter_offset != 0 {
                            enter = otl_read_anchor(
                                data,
                                table_length,
                                offset.wrapping_add(enter_offset as u32),
                            );
                        }
                        if exit_offset != 0 {
                            exit = otl_read_anchor(
                                data,
                                table_length,
                                offset.wrapping_add(exit_offset as u32),
                            );
                        }
                        (*subtable).push(GposCursiveEntry {
                            target: otfcc_handle_dup(
                                (&(*targets))[j as usize].clone() as Handle,
                            ) as GlyphHandle,
                            enter: enter,
                            exit: exit,
                        });
                        j = j.wrapping_add(1);
                    }
                    if !targets.is_null() {
                        otl_coverage_free(targets);
                    }
                    return subtable as *mut Subtable;
                }
            }
        }
    }
    if !targets.is_null() {
        otl_coverage_free(targets);
    }
    subtable_gpos_cursive_free(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gpos_dump_cursive(
    mut _subtable: *const Subtable,
) -> *mut JsonValue {
    let subtable: *const GposCursiveSubtable = &raw const (*_subtable).gpos_cursive as *const GposCursiveSubtable;
    let mut st: *mut JsonValue = json_object_new((*subtable).len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).len() {
        let mut rec: *mut JsonValue = json_object_new(2 as usize);
        json_object_push(
            rec,
            b"enter\0" as *const u8 as *const ::core::ffi::c_char,
            otl_dump_anchor((&(*subtable))[j as usize].enter),
        );
        json_object_push(
            rec,
            b"exit\0" as *const u8 as *const ::core::ffi::c_char,
            otl_dump_anchor((&(*subtable))[j as usize].exit),
        );
        json_object_push(
            st,
            (&(*subtable))[j as usize].target.name as *const ::core::ffi::c_char,
            preserialize(rec),
        );
        j = j.wrapping_add(1);
    }
    return st;
}
pub unsafe extern "C" fn otl_gpos_parse_cursive(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    let subtable: *mut GposCursiveSubtable = subtable_gpos_cursive_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < (*_subtable).u.object.length {
        if !(*(*_subtable).u.object.values.offset(j as isize))
            .value
            .is_null()
            && (*(*(*_subtable).u.object.values.offset(j as isize)).value).type_0
                as ::core::ffi::c_uint
                == JsonType::Object as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut gname: SdsRaw = sdsnewlen(
                (*(*_subtable).u.object.values.offset(j as isize)).name
                    as *const ::core::ffi::c_void,
                (*(*_subtable).u.object.values.offset(j as isize)).name_length as usize,
            );
            (*subtable).push(GposCursiveEntry {
                target: handle_from_name(gname)
                    as GlyphHandle,
                enter: otl_parse_anchor(json_obj_get(
                    (*(*_subtable).u.object.values.offset(j as isize)).value,
                    b"enter\0" as *const u8 as *const ::core::ffi::c_char,
                )),
                exit: otl_parse_anchor(json_obj_get(
                    (*(*_subtable).u.object.values.offset(j as isize)).value,
                    b"exit\0" as *const u8 as *const ::core::ffi::c_char,
                )),
            });
        }
        j = j.wrapping_add(1);
    }
    return subtable as *mut Subtable;
}
pub unsafe extern "C" fn otfcc_build_gpos_cursive(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let subtable: *const GposCursiveSubtable = &raw const (*_subtable).gpos_cursive as *const GposCursiveSubtable;
    let mut cov: *mut Coverage = otl_coverage_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).len() {
        push_to_coverage(
            cov,
            otfcc_handle_dup(
                (&(*subtable))[j as usize].target.clone() as Handle,
            ) as GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(cov))), bk_int(BkCellType::B16, ((*subtable).len()) as u32)]);
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*subtable).len() {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_from_anchor((&(*subtable))[j_0 as usize].enter)), bk_ptr(BkCellType::P16, bk_from_anchor((&(*subtable))[j_0 as usize].exit))]);
        j_0 = j_0.wrapping_add(1);
    }
    otl_coverage_free(cov);
    return bk_build_block(root);
}
