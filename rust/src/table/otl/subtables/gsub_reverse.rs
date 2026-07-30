#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy};


use crate::support::json_funcs::{json_obj_get_type, json_obj_getnum_fallback};
use crate::table::otl::coverage::{Coverage, otl_Coverage_free, readCoverage};
use crate::support::handle::{handle_fromIndex, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_Block, bk_ptr, bk_push};

use crate::table::otl::{GsubReverseSubtableElementInterface, Subtable, GsubReverseSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::bk::bkblock::{bk_newBlockFromBuffer};
use crate::bk::bkgraph::{bk_build_Block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push};

#[inline]
unsafe extern "C" fn initGsubReverse(mut subtable: *mut GsubReverseSubtable) {
    (*subtable).match_0 = ::core::ptr::null_mut::<*mut Coverage>();
    (*subtable).to = ::core::ptr::null_mut::<Coverage>();
}
#[inline]
unsafe extern "C" fn disposeGsubReverse(mut subtable: *mut GsubReverseSubtable) {
    if !(*subtable).match_0.is_null() {
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
            otl_Coverage_free(
                *(*subtable).match_0.offset(j as isize),
            );
            j = j.wrapping_add(1);
        }
    }
    if !(*subtable).to.is_null() {
        otl_Coverage_free((*subtable).to);
    }
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_dispose(mut x: *mut GsubReverseSubtable) {
    disposeGsubReverse(x);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_free(mut x: *mut GsubReverseSubtable) {
    if x.is_null() {
        return;
    }
    subtable_gsub_reverse_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub static I_SUBTABLE_GSUB_REVERSE: GsubReverseSubtableElementInterface = {
    GsubReverseSubtableElementInterface {
        init: Some(
            subtable_gsub_reverse_init as unsafe extern "C" fn(*mut GsubReverseSubtable) -> (),
        ),
        copy: Some(
            subtable_gsub_reverse_copy
                as unsafe extern "C" fn(
                    *mut GsubReverseSubtable,
                    *const GsubReverseSubtable,
                ) -> (),
        ),
        move_0: Some(
            subtable_gsub_reverse_move
                as unsafe extern "C" fn(
                    *mut GsubReverseSubtable,
                    *mut GsubReverseSubtable,
                ) -> (),
        ),
        dispose: Some(
            subtable_gsub_reverse_dispose as unsafe extern "C" fn(*mut GsubReverseSubtable) -> (),
        ),
        replace: Some(
            subtable_gsub_reverse_replace
                as unsafe extern "C" fn(*mut GsubReverseSubtable, GsubReverseSubtable) -> (),
        ),
        copyReplace: Some(
            subtable_gsub_reverse_copyReplace
                as unsafe extern "C" fn(*mut GsubReverseSubtable, GsubReverseSubtable) -> (),
        ),
        create: Some(subtable_gsub_reverse_create),
        free: Some(
            subtable_gsub_reverse_free as unsafe extern "C" fn(*mut GsubReverseSubtable) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_create() -> *mut GsubReverseSubtable {
    let mut x: *mut GsubReverseSubtable =
        malloc(::core::mem::size_of::<GsubReverseSubtable>() as usize)
            as *mut GsubReverseSubtable;
    subtable_gsub_reverse_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_init(mut x: *mut GsubReverseSubtable) {
    initGsubReverse(x);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_copy(
    mut dst: *mut GsubReverseSubtable,
    mut src: *const GsubReverseSubtable,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GsubReverseSubtable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_copyReplace(
    mut dst: *mut GsubReverseSubtable,
    src: GsubReverseSubtable,
) {
    subtable_gsub_reverse_dispose(dst);
    subtable_gsub_reverse_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_move(
    mut dst: *mut GsubReverseSubtable,
    mut src: *mut GsubReverseSubtable,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GsubReverseSubtable>() as usize,
    );
    subtable_gsub_reverse_init(src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_replace(
    mut dst: *mut GsubReverseSubtable,
    src: GsubReverseSubtable,
) {
    subtable_gsub_reverse_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GsubReverseSubtable>() as usize,
    );
}
unsafe extern "C" fn reverseBacktracks(
    mut match_0: *mut *mut Coverage,
    mut inputIndex: TableId,
) {
    if inputIndex as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut start: TableId = 0 as TableId;
        let mut end: TableId =
            (inputIndex as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as TableId;
        while end as ::core::ffi::c_int > start as ::core::ffi::c_int {
            let mut tmp: *mut Coverage = *match_0.offset(start as isize);
            let ref mut fresh3 = *match_0.offset(start as isize);
            *fresh3 = *match_0.offset(end as isize);
            let ref mut fresh4 = *match_0.offset(end as isize);
            *fresh4 = tmp;
            end = end.wrapping_sub(1);
            start = start.wrapping_add(1);
        }
    }
}
pub unsafe extern "C" fn otl_read_gsub_reverse(
    data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
    _maxGlyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut nBacktrack: TableId = 0;
    let mut nForward: TableId = 0;
    let mut nReplacement: TableId = 0;
    let mut subtable: *mut GsubReverseSubtable =
        (
            I_SUBTABLE_GSUB_REVERSE
                .create
                .expect("non-null function pointer"))();
    if !(tableLength < offset.wrapping_add(6 as u32)) {
        nBacktrack = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        if !(tableLength
            < offset.wrapping_add(6 as u32).wrapping_add(
                (nBacktrack as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
            ))
        {
            nForward = read_16u(
                data.offset(offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize)
                    .offset((nBacktrack as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                    as *const u8,
            ) as TableId;
            if !(tableLength
                < offset.wrapping_add(8 as u32).wrapping_add(
                    ((nBacktrack as ::core::ffi::c_int + nForward as ::core::ffi::c_int)
                        * 2 as ::core::ffi::c_int) as u32,
                ))
            {
                nReplacement = read_16u(
                    data.offset(offset as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset(
                            ((nBacktrack as ::core::ffi::c_int + nForward as ::core::ffi::c_int)
                                * 2 as ::core::ffi::c_int) as isize,
                        ) as *const u8,
                ) as TableId;
                if !(tableLength
                    < offset.wrapping_add(10 as u32).wrapping_add(
                        ((nBacktrack as ::core::ffi::c_int
                            + nForward as ::core::ffi::c_int
                            + nReplacement as ::core::ffi::c_int)
                            * 2 as ::core::ffi::c_int) as u32,
                    ))
                {
                    (*subtable).matchCount = (nBacktrack as ::core::ffi::c_int
                        + nForward as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as TableId;
                    (*subtable).match_0 = __caryll_allocate_clean(
                        (::core::mem::size_of::<*mut Coverage>() as usize)
                            .wrapping_mul((*subtable).matchCount as usize),
                        47 as ::core::ffi::c_ulong,
                    ) as *mut *mut Coverage;
                    (*subtable).inputIndex = nBacktrack;
                    let mut j: TableId = 0 as TableId;
                    while (j as ::core::ffi::c_int) < nBacktrack as ::core::ffi::c_int {
                        let mut covOffset: u32 = offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        )
                            as u32);
                        let ref mut fresh0 = *(*subtable).match_0.offset(j as isize);
                        *fresh0 = readCoverage(
                            data as *const u8,
                            tableLength,
                            covOffset,
                        );
                        j = j.wrapping_add(1);
                    }
                    let mut covOffset_0: u32 = offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *const u8,
                    )
                        as u32);
                    let ref mut fresh1 =
                        *(*subtable).match_0.offset((*subtable).inputIndex as isize);
                    *fresh1 = readCoverage(
                        data as *const u8,
                        tableLength,
                        covOffset_0,
                    );
                    if !(nReplacement as ::core::ffi::c_int
                        != (**(*subtable).match_0.offset((*subtable).inputIndex as isize)).numGlyphs
                            as ::core::ffi::c_int)
                    {
                        let mut j_0: TableId = 0 as TableId;
                        while (j_0 as ::core::ffi::c_int) < nForward as ::core::ffi::c_int {
                            let mut covOffset_1: u32 = offset.wrapping_add(read_16u(
                                data.offset(offset as isize)
                                    .offset(8 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (nBacktrack as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    )
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as u32);
                            let ref mut fresh2 = *(*subtable).match_0.offset(
                                (nBacktrack as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                    + j_0 as ::core::ffi::c_int)
                                    as isize,
                            );
                            *fresh2 = readCoverage(
                                data as *const u8,
                                tableLength,
                                covOffset_1,
                            );
                            j_0 = j_0.wrapping_add(1);
                        }
                        (*subtable).to = __caryll_allocate_clean(
                            ::core::mem::size_of::<Coverage>() as usize,
                            64 as ::core::ffi::c_ulong,
                        ) as *mut Coverage;
                        (*(*subtable).to).numGlyphs = nReplacement as GlyphId;
                        (*(*subtable).to).glyphs = __caryll_allocate_clean(
                            (::core::mem::size_of::<GlyphHandle>() as usize)
                                .wrapping_mul(nReplacement as usize),
                            66 as ::core::ffi::c_ulong,
                        )
                            as *mut GlyphHandle;
                        let mut j_1: TableId = 0 as TableId;
                        while (j_1 as ::core::ffi::c_int) < nReplacement as ::core::ffi::c_int {
                            *(*(*subtable).to).glyphs.offset(j_1 as isize) =
                                handle_fromIndex(
                                    read_16u(
                                        data.offset(offset as isize)
                                            .offset(10 as ::core::ffi::c_int as isize)
                                            .offset(
                                                ((nBacktrack as ::core::ffi::c_int
                                                    + nForward as ::core::ffi::c_int
                                                    + j_1 as ::core::ffi::c_int)
                                                    * 2 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            as *const u8,
                                    ) as GlyphId,
                                ) as GlyphHandle;
                            j_1 = j_1.wrapping_add(1);
                        }
                        reverseBacktracks((*subtable).match_0, (*subtable).inputIndex);
                        return subtable as *mut Subtable;
                    }
                }
            }
        }
    }
    I_SUBTABLE_GSUB_REVERSE
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gsub_dump_reverse(
    mut _subtable: *const Subtable,
) -> *mut JsonValue {
    let mut subtable: *const GsubReverseSubtable = &raw const (*_subtable).gsub_reverse;
    let mut _st: *mut JsonValue = json_object_new(3 as usize);
    let mut _match: *mut JsonValue = json_array_new((*subtable).matchCount as usize);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
        json_array_push(
            _match,
            OTL_I_COVERAGE.dump.expect("non-null function pointer")(
                *(*subtable).match_0.offset(j as isize),
            ),
        );
        j = j.wrapping_add(1);
    }
    json_object_push(
        _st,
        b"match\0" as *const u8 as *const ::core::ffi::c_char,
        _match,
    );
    json_object_push(
        _st,
        b"to\0" as *const u8 as *const ::core::ffi::c_char,
        OTL_I_COVERAGE.dump.expect("non-null function pointer")((*subtable).to),
    );
    json_object_push(
        _st,
        b"inputIndex\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*subtable).inputIndex as i64),
    );
    return _st;
}
pub unsafe extern "C" fn otl_gsub_parse_reverse(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut _match: *mut JsonValue = json_obj_get_type(
        _subtable,
        b"match\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    let mut _to: *mut JsonValue = json_obj_get_type(
        _subtable,
        b"to\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _match.is_null() || _to.is_null() {
        return ::core::ptr::null_mut::<Subtable>();
    }
    let mut subtable: *mut GsubReverseSubtable =
        (
            I_SUBTABLE_GSUB_REVERSE
                .create
                .expect("non-null function pointer"))();
    (*subtable).matchCount = (*_match).u.array.length as TableId;
    (*subtable).match_0 = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut Coverage>() as usize)
            .wrapping_mul((*subtable).matchCount as usize),
        100 as ::core::ffi::c_ulong,
    ) as *mut *mut Coverage;
    (*subtable).inputIndex = json_obj_getnum_fallback(
        _subtable,
        b"inputIndex\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as TableId;
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
        let ref mut fresh5 = *(*subtable).match_0.offset(j as isize);
        *fresh5 = OTL_I_COVERAGE.parse.expect("non-null function pointer")(
            *(*_match).u.array.values.offset(j as isize),
        );
        j = j.wrapping_add(1);
    }
    (*subtable).to = OTL_I_COVERAGE.parse.expect("non-null function pointer")(_to);
    return subtable as *mut Subtable;
}
pub unsafe extern "C" fn otfcc_build_gsub_reverse(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let mut subtable: *const GsubReverseSubtable = &raw const (*_subtable).gsub_reverse;
    reverseBacktracks((*subtable).match_0, (*subtable).inputIndex);
    let mut root: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
            *(*subtable).match_0.offset((*subtable).inputIndex as isize),
        )))]);
    bk_push(root, &[bk_int(BkCellType::B16, ((*subtable).inputIndex as ::core::ffi::c_int) as u32)]);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*subtable).inputIndex as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
                *(*subtable).match_0.offset(j as isize),
            )))]);
        j = j.wrapping_add(1);
    }
    bk_push(root, &[bk_int(BkCellType::B16, ((*subtable).matchCount as ::core::ffi::c_int
            - (*subtable).inputIndex as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as u32)]);
    let mut j_0: TableId =
        ((*subtable).inputIndex as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
    while (j_0 as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
                *(*subtable).match_0.offset(j_0 as isize),
            )))]);
        j_0 = j_0.wrapping_add(1);
    }
    bk_push(root, &[bk_int(BkCellType::B16, ((*(*subtable).to).numGlyphs as ::core::ffi::c_int) as u32)]);
    let mut j_1: TableId = 0 as TableId;
    while (j_1 as ::core::ffi::c_int) < (*(*subtable).to).numGlyphs as ::core::ffi::c_int {
        bk_push(root, &[bk_int(BkCellType::B16, ((*(*(*subtable).to).glyphs.offset(j_1 as isize)).index as ::core::ffi::c_int) as u32)]);
        j_1 = j_1.wrapping_add(1);
    }
    return bk_build_Block(root);
}
