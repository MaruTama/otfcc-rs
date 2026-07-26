#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy};
unsafe extern "C" {
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    static otl_iCoverage: __otfcc_ICoverage;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}


use crate::support::json_funcs::{json_obj_get_type, json_obj_getnum_fallback};
use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage, otl_Coverage_free, readCoverage};
use crate::support::handle::{handle_fromIndex, otfcc_GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphid_t, tableid_t};
use crate::vendor::json::{json_array, json_value};
use crate::bk::bkblock::{b16, bk_Block, bk_int, bk_new_Block, bk_ptr, bk_push, p16};

use crate::table::otl::{__caryll_elementinterface_subtable_gsub_reverse, otl_Subtable, subtable_gsub_reverse};
use crate::table::otl::subtables::{otl_BuildHeuristics};

#[inline]
unsafe extern "C" fn initGsubReverse(mut subtable: *mut subtable_gsub_reverse) {
    (*subtable).match_0 = ::core::ptr::null_mut::<*mut otl_Coverage>();
    (*subtable).to = ::core::ptr::null_mut::<otl_Coverage>();
}
#[inline]
unsafe extern "C" fn disposeGsubReverse(mut subtable: *mut subtable_gsub_reverse) {
    if !(*subtable).match_0.is_null() {
        let mut j: tableid_t = 0 as tableid_t;
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
unsafe extern "C" fn subtable_gsub_reverse_dispose(mut x: *mut subtable_gsub_reverse) {
    disposeGsubReverse(x);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_free(mut x: *mut subtable_gsub_reverse) {
    if x.is_null() {
        return;
    }
    subtable_gsub_reverse_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub static iSubtable_gsub_reverse: __caryll_elementinterface_subtable_gsub_reverse = {
    __caryll_elementinterface_subtable_gsub_reverse {
        init: Some(
            subtable_gsub_reverse_init as unsafe extern "C" fn(*mut subtable_gsub_reverse) -> (),
        ),
        copy: Some(
            subtable_gsub_reverse_copy
                as unsafe extern "C" fn(
                    *mut subtable_gsub_reverse,
                    *const subtable_gsub_reverse,
                ) -> (),
        ),
        move_0: Some(
            subtable_gsub_reverse_move
                as unsafe extern "C" fn(
                    *mut subtable_gsub_reverse,
                    *mut subtable_gsub_reverse,
                ) -> (),
        ),
        dispose: Some(
            subtable_gsub_reverse_dispose as unsafe extern "C" fn(*mut subtable_gsub_reverse) -> (),
        ),
        replace: Some(
            subtable_gsub_reverse_replace
                as unsafe extern "C" fn(*mut subtable_gsub_reverse, subtable_gsub_reverse) -> (),
        ),
        copyReplace: Some(
            subtable_gsub_reverse_copyReplace
                as unsafe extern "C" fn(*mut subtable_gsub_reverse, subtable_gsub_reverse) -> (),
        ),
        create: Some(subtable_gsub_reverse_create),
        free: Some(
            subtable_gsub_reverse_free as unsafe extern "C" fn(*mut subtable_gsub_reverse) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_create() -> *mut subtable_gsub_reverse {
    let mut x: *mut subtable_gsub_reverse =
        malloc(::core::mem::size_of::<subtable_gsub_reverse>() as usize)
            as *mut subtable_gsub_reverse;
    subtable_gsub_reverse_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_init(mut x: *mut subtable_gsub_reverse) {
    initGsubReverse(x);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_copy(
    mut dst: *mut subtable_gsub_reverse,
    mut src: *const subtable_gsub_reverse,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gsub_reverse>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_copyReplace(
    mut dst: *mut subtable_gsub_reverse,
    src: subtable_gsub_reverse,
) {
    subtable_gsub_reverse_dispose(dst);
    subtable_gsub_reverse_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_move(
    mut dst: *mut subtable_gsub_reverse,
    mut src: *mut subtable_gsub_reverse,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gsub_reverse>() as usize,
    );
    subtable_gsub_reverse_init(src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_replace(
    mut dst: *mut subtable_gsub_reverse,
    src: subtable_gsub_reverse,
) {
    subtable_gsub_reverse_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gsub_reverse>() as usize,
    );
}
unsafe extern "C" fn reverseBacktracks(
    mut match_0: *mut *mut otl_Coverage,
    mut inputIndex: tableid_t,
) {
    if inputIndex as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut start: tableid_t = 0 as tableid_t;
        let mut end: tableid_t =
            (inputIndex as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as tableid_t;
        while end as ::core::ffi::c_int > start as ::core::ffi::c_int {
            let mut tmp: *mut otl_Coverage = *match_0.offset(start as isize);
            let ref mut fresh3 = *match_0.offset(start as isize);
            *fresh3 = *match_0.offset(end as isize);
            let ref mut fresh4 = *match_0.offset(end as isize);
            *fresh4 = tmp;
            end = end.wrapping_sub(1);
            start = start.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otl_read_gsub_reverse(
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut nBacktrack: tableid_t = 0;
    let mut nForward: tableid_t = 0;
    let mut nReplacement: tableid_t = 0;
    let mut subtable: *mut subtable_gsub_reverse =
        (
            iSubtable_gsub_reverse
                .create
                .expect("non-null function pointer"))();
    if !(tableLength < offset.wrapping_add(6 as u32)) {
        nBacktrack = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as tableid_t;
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
            ) as tableid_t;
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
                ) as tableid_t;
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
                        as tableid_t;
                    (*subtable).match_0 = __caryll_allocate_clean(
                        (::core::mem::size_of::<*mut otl_Coverage>() as usize)
                            .wrapping_mul((*subtable).matchCount as usize),
                        47 as ::core::ffi::c_ulong,
                    ) as *mut *mut otl_Coverage;
                    (*subtable).inputIndex = nBacktrack;
                    let mut j: tableid_t = 0 as tableid_t;
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
                        let mut j_0: tableid_t = 0 as tableid_t;
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
                            ::core::mem::size_of::<otl_Coverage>() as usize,
                            64 as ::core::ffi::c_ulong,
                        ) as *mut otl_Coverage;
                        (*(*subtable).to).numGlyphs = nReplacement as glyphid_t;
                        (*(*subtable).to).glyphs = __caryll_allocate_clean(
                            (::core::mem::size_of::<otfcc_GlyphHandle>() as usize)
                                .wrapping_mul(nReplacement as usize),
                            66 as ::core::ffi::c_ulong,
                        )
                            as *mut otfcc_GlyphHandle;
                        let mut j_1: tableid_t = 0 as tableid_t;
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
                                    ) as glyphid_t,
                                ) as otfcc_GlyphHandle;
                            j_1 = j_1.wrapping_add(1);
                        }
                        reverseBacktracks((*subtable).match_0, (*subtable).inputIndex);
                        return subtable as *mut otl_Subtable;
                    }
                }
            }
        }
    }
    iSubtable_gsub_reverse
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otl_gsub_dump_reverse(
    mut _subtable: *const otl_Subtable,
) -> *mut json_value {
    let mut subtable: *const subtable_gsub_reverse = &raw const (*_subtable).gsub_reverse;
    let mut _st: *mut json_value = json_object_new(3 as usize);
    let mut _match: *mut json_value = json_array_new((*subtable).matchCount as usize);
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
        json_array_push(
            _match,
            otl_iCoverage.dump.expect("non-null function pointer")(
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
        otl_iCoverage.dump.expect("non-null function pointer")((*subtable).to),
    );
    json_object_push(
        _st,
        b"inputIndex\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*subtable).inputIndex as i64),
    );
    return _st;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otl_gsub_parse_reverse(
    mut _subtable: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut _match: *mut json_value = json_obj_get_type(
        _subtable,
        b"match\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    let mut _to: *mut json_value = json_obj_get_type(
        _subtable,
        b"to\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    if _match.is_null() || _to.is_null() {
        return ::core::ptr::null_mut::<otl_Subtable>();
    }
    let mut subtable: *mut subtable_gsub_reverse =
        (
            iSubtable_gsub_reverse
                .create
                .expect("non-null function pointer"))();
    (*subtable).matchCount = (*_match).u.array.length as tableid_t;
    (*subtable).match_0 = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut otl_Coverage>() as usize)
            .wrapping_mul((*subtable).matchCount as usize),
        100 as ::core::ffi::c_ulong,
    ) as *mut *mut otl_Coverage;
    (*subtable).inputIndex = json_obj_getnum_fallback(
        _subtable,
        b"inputIndex\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as tableid_t;
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
        let ref mut fresh5 = *(*subtable).match_0.offset(j as isize);
        *fresh5 = otl_iCoverage.parse.expect("non-null function pointer")(
            *(*_match).u.array.values.offset(j as isize),
        );
        j = j.wrapping_add(1);
    }
    (*subtable).to = otl_iCoverage.parse.expect("non-null function pointer")(_to);
    return subtable as *mut otl_Subtable;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_build_gsub_reverse(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gsub_reverse = &raw const (*_subtable).gsub_reverse;
    reverseBacktracks((*subtable).match_0, (*subtable).inputIndex);
    let mut root: *mut bk_Block = bk_new_Block(&[bk_int(b16, 1 as u32), bk_ptr(p16, bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
            *(*subtable).match_0.offset((*subtable).inputIndex as isize),
        )))]);
    bk_push(root, &[bk_int(b16, ((*subtable).inputIndex as ::core::ffi::c_int) as u32)]);
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*subtable).inputIndex as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(p16, bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
                *(*subtable).match_0.offset(j as isize),
            )))]);
        j = j.wrapping_add(1);
    }
    bk_push(root, &[bk_int(b16, ((*subtable).matchCount as ::core::ffi::c_int
            - (*subtable).inputIndex as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as u32)]);
    let mut j_0: tableid_t =
        ((*subtable).inputIndex as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as tableid_t;
    while (j_0 as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(p16, bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
                *(*subtable).match_0.offset(j_0 as isize),
            )))]);
        j_0 = j_0.wrapping_add(1);
    }
    bk_push(root, &[bk_int(b16, ((*(*subtable).to).numGlyphs as ::core::ffi::c_int) as u32)]);
    let mut j_1: tableid_t = 0 as tableid_t;
    while (j_1 as ::core::ffi::c_int) < (*(*subtable).to).numGlyphs as ::core::ffi::c_int {
        bk_push(root, &[bk_int(b16, ((*(*(*subtable).to).glyphs.offset(j_1 as isize)).index as ::core::ffi::c_int) as u32)]);
        j_1 = j_1.wrapping_add(1);
    }
    return bk_build_Block(root);
}
