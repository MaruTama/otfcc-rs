#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc};


use crate::support::json_funcs::{json_obj_get_type, preserialize};
use crate::table::otl::coverage::{Coverage, coverage_from_raw, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_index, handle_from_name, GlyphHandle};

use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::table::otl::{GsubLigatureEntry, Subtable, GsubLigatureSubtable, subtable_from_raw};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_object_new, json_object_push, json_string_new_from_bytes};
use crate::vendor::sds::{sdsnewlen};
// `from: Coverage` and `to: GlyphHandle` both self-drop now, so a
// `GsubLigatureSubtable` (`Vec<GsubLigatureEntry>`) fully self-drops -- no
// per-element dtor needed anymore.
pub(crate) unsafe fn dispose_gsub_ligature_subtable(arr: *mut GsubLigatureSubtable) {
    *arr = Vec::new();
}
pub(crate) unsafe extern "C" fn subtable_gsub_ligature_free(x: *mut GsubLigatureSubtable) {
    if x.is_null() {
        return;
    }
    dispose_gsub_ligature_subtable(x);
    free(x as *mut ::core::ffi::c_void);
}
/// The one live `.replace` among all the `Subtable`-union-blocked
/// containers: `consolidate_gsub_ligature` builds a fresh, empty `nt`,
/// filters/moves entries into it, then swaps it in here. `src` is always
/// that fresh local -- never reused by the caller afterward -- so disposing
/// the old `*dst` and move-assigning `src` in is equivalent to (and safer
/// than) the original's dispose-then-`memcpy`.
#[allow(improper_ctypes_definitions)]
pub(crate) unsafe extern "C" fn subtable_gsub_ligature_replace(
    dst: *mut GsubLigatureSubtable,
    src: GsubLigatureSubtable,
) {
    dispose_gsub_ligature_subtable(dst);
    *dst = src;
}
unsafe extern "C" fn subtable_gsub_ligature_create() -> *mut GsubLigatureSubtable {
    let x: *mut GsubLigatureSubtable =
        malloc(::core::mem::size_of::<GsubLigatureSubtable>() as usize)
            as *mut GsubLigatureSubtable;
    x.write(Vec::new());
    x
}
pub unsafe extern "C" fn otl_read_gsub_ligature(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    _max_glyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut start_coverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut set_count: GlyphId = 0;
    let mut ligature_count: u32 = 0;
    let mut current_block: u64;
    let subtable: *mut GsubLigatureSubtable = subtable_gsub_ligature_create();
    if !(table_length < offset.wrapping_add(6 as u32)) {
        start_coverage = read_coverage(
            data as *const u8,
            table_length,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !start_coverage.is_null() {
            set_count = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as GlyphId;
            if !(set_count as usize != (*start_coverage).len())
            {
                if !(table_length
                    < offset.wrapping_add(6 as u32).wrapping_add(
                        (set_count as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                    ))
                {
                    ligature_count = 0 as u32;
                    let mut j: GlyphId = 0 as GlyphId;
                    loop {
                        if !((j as ::core::ffi::c_int) < set_count as ::core::ffi::c_int) {
                            current_block = 17860125682698302841;
                            break;
                        }
                        let mut set_offset: u32 = offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        )
                            as u32);
                        if table_length < set_offset.wrapping_add(2 as u32) {
                            current_block = 3443835632518673764;
                            break;
                        }
                        ligature_count = ligature_count.wrapping_add(read_16u(
                            data.offset(set_offset as isize) as *const u8,
                        )
                            as u32);
                        if table_length
                            < set_offset.wrapping_add(2 as u32).wrapping_add(
                                (read_16u(data.offset(set_offset as isize) as *const u8)
                                    as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int)
                                    as u32,
                            )
                        {
                            current_block = 3443835632518673764;
                            break;
                        }
                        j = j.wrapping_add(1);
                    }
                    match current_block {
                        3443835632518673764 => {}
                        _ => {
                            let mut j_0: GlyphId = 0 as GlyphId;
                            's_77: loop {
                                if !((j_0 as ::core::ffi::c_int) < set_count as ::core::ffi::c_int) {
                                    current_block = 11932355480408055363;
                                    break;
                                }
                                let mut set_offset_0: u32 = offset.wrapping_add(read_16u(
                                    data.offset(offset as isize)
                                        .offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let mut lc: GlyphId =
                                    read_16u(data.offset(set_offset_0 as isize) as *const u8)
                                        as GlyphId;
                                let mut k: GlyphId = 0 as GlyphId;
                                while (k as ::core::ffi::c_int) < lc as ::core::ffi::c_int {
                                    let mut lig_offset: u32 = set_offset_0.wrapping_add(
                                        read_16u(
                                            data.offset(set_offset_0 as isize)
                                                .offset(2 as ::core::ffi::c_int as isize)
                                                .offset(
                                                    (k as ::core::ffi::c_int
                                                        * 2 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                as *const u8,
                                        ) as u32,
                                    );
                                    if table_length < lig_offset.wrapping_add(4 as u32) {
                                        current_block = 3443835632518673764;
                                        break 's_77;
                                    }
                                    let mut lig_components: GlyphId = read_16u(
                                        data.offset(lig_offset as isize)
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            as *const u8,
                                    )
                                        as GlyphId;
                                    if table_length
                                        < lig_offset.wrapping_add(2 as u32).wrapping_add(
                                            (lig_components as ::core::ffi::c_int
                                                * 2 as ::core::ffi::c_int)
                                                as u32,
                                        )
                                    {
                                        current_block = 3443835632518673764;
                                        break 's_77;
                                    }
                                    let mut cov: *mut Coverage =
                                        otl_coverage_create();
                                    push_to_coverage(
                                        cov,
                                        handle_from_index(
                                            (&(*start_coverage))[j_0 as usize].index,
                                        )
                                            as GlyphHandle,
                                    );
                                    let mut m: GlyphId = 1 as GlyphId;
                                    while (m as ::core::ffi::c_int)
                                        < lig_components as ::core::ffi::c_int
                                    {
                                        push_to_coverage(
                                            cov,
                                            handle_from_index(
                                                read_16u(
                                                    data.offset(lig_offset as isize)
                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                        .offset(
                                                            (m as ::core::ffi::c_int
                                                                * 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        as *const u8,
                                                )
                                                    as GlyphId,
                                            )
                                                as GlyphHandle,
                                        );
                                        m = m.wrapping_add(1);
                                    }
                                    (*subtable).push(GsubLigatureEntry {
                                        from: coverage_from_raw(cov),
                                        to: handle_from_index(
                                            read_16u(data.offset(lig_offset as isize)
                                                as *const u8)
                                                as GlyphId,
                                        )
                                            as GlyphHandle,
                                    });
                                    k = k.wrapping_add(1);
                                }
                                j_0 = j_0.wrapping_add(1);
                            }
                            match current_block {
                                3443835632518673764 => {}
                                _ => {
                                    otl_coverage_free(
                                        start_coverage,
                                    );
                                    return subtable_from_raw(subtable, Subtable::GsubLigature);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    subtable_gsub_ligature_free(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gsub_dump_ligature(
    mut _subtable: *const Subtable,
) -> *mut JsonValue {
    let Subtable::GsubLigature(mut_subtable) = &*_subtable else { unreachable!() };
    let subtable: *const GsubLigatureSubtable = mut_subtable;
    let mut st: *mut JsonValue = json_array_new((*subtable).len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).len() {
        let mut entry: *mut JsonValue = json_object_new(2 as usize);
        json_object_push(
            entry,
            b"from\0" as *const u8 as *const ::core::ffi::c_char,
            OTL_I_COVERAGE.dump.expect("non-null function pointer")(
                &(&(*subtable))[j as usize].from as *const Coverage,
            ),
        );
        json_object_push(
            entry,
            b"to\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new_from_bytes(&(&(*subtable))[j as usize].to.name),
        );
        json_array_push(st, preserialize(entry));
        j = j.wrapping_add(1);
    }
    let mut ret: *mut JsonValue = json_object_new(1 as usize);
    json_object_push(
        ret,
        b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
        st,
    );
    return ret;
}
pub unsafe extern "C" fn otl_gsub_parse_ligature(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    if !json_obj_get_type(
        _subtable,
        b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    )
    .is_null()
    {
        _subtable = json_obj_get_type(
            _subtable,
            b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Array,
        );
        let st: *mut GsubLigatureSubtable = subtable_gsub_ligature_create();
        let mut n: GlyphId = (*_subtable).u.array.length as GlyphId;
        let mut k: GlyphId = 0 as GlyphId;
        while (k as ::core::ffi::c_int) < n as ::core::ffi::c_int {
            let mut entry: *mut JsonValue =
                *(*_subtable).u.array.values.offset(k as isize) as *mut JsonValue;
            let mut _from: *mut JsonValue = json_obj_get_type(
                entry,
                b"from\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            );
            let mut _to: *mut JsonValue = json_obj_get_type(
                entry,
                b"to\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            if !(_from.is_null() || _to.is_null()) {
                (*st).push(GsubLigatureEntry {
                    from: coverage_from_raw(
                        OTL_I_COVERAGE.parse.expect("non-null function pointer")(_from),
                    ),
                    to: handle_from_name(sdsnewlen(
                        (*_to).u.string.ptr as *const ::core::ffi::c_void,
                        (*_to).u.string.length as usize,
                    )) as GlyphHandle,
                });
            }
            k = k.wrapping_add(1);
        }
        return subtable_from_raw(st, Subtable::GsubLigature);
    } else {
        let st_0: *mut GsubLigatureSubtable = subtable_gsub_ligature_create();
        let mut n_0: GlyphId = (*_subtable).u.array.length as GlyphId;
        let mut k_0: GlyphId = 0 as GlyphId;
        while (k_0 as ::core::ffi::c_int) < n_0 as ::core::ffi::c_int {
            let mut _from_0: *mut JsonValue =
                (*(*_subtable).u.object.values.offset(k_0 as isize)).value as *mut JsonValue;
            if !(_from_0.is_null()
                || (*_from_0).type_0 != JsonType::Array)
            {
                (*st_0).push(GsubLigatureEntry {
                    from: coverage_from_raw(
                        OTL_I_COVERAGE.parse.expect("non-null function pointer")(_from_0),
                    ),
                    to: handle_from_name(sdsnewlen(
                        (*(*_subtable).u.object.values.offset(k_0 as isize)).name
                            as *const ::core::ffi::c_void,
                        (*(*_subtable).u.object.values.offset(k_0 as isize)).name_length
                            as usize,
                    )) as GlyphHandle,
                });
            }
            k_0 = k_0.wrapping_add(1);
        }
        return subtable_from_raw(st_0, Subtable::GsubLigature);
    };
}
// Deduplicates by the ligature rule's starting glyph id -- the original
// uthash `LigatureAggregator` table carried no data beyond a `gid`
// (`HASH_SORT` by `by_gid` before `HASH_ITER`, no companion payload per
// entry): it was used purely to build the sorted, deduplicated Coverage
// of "first glyphs", then the original re-scanned the whole subtable
// per distinct gid (twice: once to count, once to emit) to build each
// glyph's `LigatureSet`. A `BTreeSet<i32>` reproduces the sorted,
// deduplicated set directly -- there is no value to carry, so this isn't
// even a map the way every other uthash instance in this migration has
// been.
pub unsafe extern "C" fn otfcc_build_gsub_ligature_subtable(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let Subtable::GsubLigature(mut_subtable) = &*_subtable else { unreachable!() };
    let subtable: *const GsubLigatureSubtable = mut_subtable;
    let n_ligatures: GlyphId = (*subtable).len() as GlyphId;
    let mut start_gids: std::collections::BTreeSet<::core::ffi::c_int> = std::collections::BTreeSet::new();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_int) < n_ligatures as ::core::ffi::c_int {
        let sgid: ::core::ffi::c_int = (&(*subtable))[j as usize].from[0].index as ::core::ffi::c_int;
        start_gids.insert(sgid);
        j = j.wrapping_add(1);
    }
    let mut startcov: *mut Coverage = otl_coverage_create();
    for &gid in start_gids.iter() {
        push_to_coverage(
            startcov,
            handle_from_index(gid as GlyphId) as GlyphHandle,
        );
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
            startcov,
        ))), bk_int(BkCellType::B16, ((*startcov).len() as ::core::ffi::c_int) as u32)]);
    for &gid in start_gids.iter() {
        let mut n_ligs_here: GlyphId = 0 as GlyphId;
        let mut j_0: GlyphId = 0 as GlyphId;
        while (j_0 as ::core::ffi::c_int) < n_ligatures as ::core::ffi::c_int {
            if (&(*subtable))[j_0 as usize].from[0]
            .index as ::core::ffi::c_int
                == gid
            {
                n_ligs_here = n_ligs_here.wrapping_add(1);
            }
            j_0 = j_0.wrapping_add(1);
        }
        let mut ligset: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (n_ligs_here as ::core::ffi::c_int) as u32)]);
        let mut j_1: GlyphId = 0 as GlyphId;
        while (j_1 as ::core::ffi::c_int) < n_ligatures as ::core::ffi::c_int {
            if (&(*subtable))[j_1 as usize].from[0]
            .index as ::core::ffi::c_int
                == gid
            {
                let mut ligdef: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((&(*subtable))[j_1 as usize].to.index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((&(*subtable))[j_1 as usize].from.len()
                        as ::core::ffi::c_int) as u32)]);
                let mut m: GlyphId = 1 as GlyphId;
                while (m as ::core::ffi::c_int)
                    < (&(*subtable))[j_1 as usize].from.len()
                        as ::core::ffi::c_int
                {
                    bk_push(ligdef, &[bk_int(BkCellType::B16, ((&(*subtable))[j_1 as usize].from[m as usize]
                        .index as ::core::ffi::c_int) as u32)]);
                    m = m.wrapping_add(1);
                }
                bk_push(ligset, &[bk_ptr(BkCellType::P16, ligdef)]);
            }
            j_1 = j_1.wrapping_add(1);
        }
        bk_push(root, &[bk_ptr(BkCellType::P16, ligset)]);
    }
    otl_coverage_free(startcov);
    return bk_build_block(root);
}
