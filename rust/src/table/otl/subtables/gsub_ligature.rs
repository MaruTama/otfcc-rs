#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc};


use crate::support::parsed_json::{ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_key_bytes_at, json_obj_len, json_obj_val_at, json_str_bytes, json_type_of};
use crate::table::otl::coverage::{Coverage, coverage_from_raw, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_index, handle_from_name, GlyphHandle};

use crate::support::font_reader::{FontReader};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::json::{JsonType};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::table::otl::{GsubLigatureEntry, Subtable, GsubLigatureSubtable, subtable_from_raw};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{dump_coverage, parse_coverage, build_coverage};
use crate::support::built_json::{BuiltValue, json_array_new, json_array_push, json_object_new, json_object_push, json_string_new_from_bytes, preserialize};
// `from: Coverage` and `to: GlyphHandle` both self-drop now, so a
// `GsubLigatureSubtable` (`Vec<GsubLigatureEntry>`) fully self-drops -- no
// per-element dtor needed anymore.
pub(crate) unsafe fn dispose_gsub_ligature_subtable(arr: *mut GsubLigatureSubtable) {
    *arr = Vec::new();
}
pub(crate) unsafe fn subtable_gsub_ligature_free(x: *mut GsubLigatureSubtable) {
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
pub(crate) unsafe fn subtable_gsub_ligature_replace(
    dst: *mut GsubLigatureSubtable,
    src: GsubLigatureSubtable,
) {
    dispose_gsub_ligature_subtable(dst);
    *dst = src;
}
unsafe fn subtable_gsub_ligature_create() -> *mut GsubLigatureSubtable {
    let x: *mut GsubLigatureSubtable =
        malloc(::core::mem::size_of::<GsubLigatureSubtable>() as usize)
            as *mut GsubLigatureSubtable;
    x.write(Vec::new());
    x
}
// Also fixes a real (if minor) pre-existing leak, same shape as
// `gpos_mark_to_single.rs`'s: the original only freed `start_coverage`
// (always-allocated by `read_coverage`, even for an empty result) on the
// success path -- every failure guard reached after it was read fell
// through to `subtable_gsub_ligature_free(subtable)` without freeing it.
pub unsafe fn otl_read_gsub_ligature(
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    _max_glyphs: GlyphId,
) -> *mut Subtable {
    let subtable: *mut GsubLigatureSubtable = subtable_gsub_ligature_create();
    let mut start_coverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);

    'parse: {
        let mut header = match FontReader::new(slice).at(offset as usize) {
            Ok(r) => r,
            Err(_) => break 'parse,
        };
        if header.skip(2).is_err() {
            break 'parse; // format, unused
        }
        let Ok(cov_rel) = header.u16() else { break 'parse };
        let Ok(set_count) = header.u16() else { break 'parse };

        start_coverage = read_coverage(data, table_length, offset.wrapping_add(cov_rel as u32));
        if start_coverage.is_null() {
            break 'parse;
        }
        if set_count as usize != (*start_coverage).len() {
            break 'parse;
        }
        if header.require_room(set_count as usize, 2).is_err() {
            break 'parse;
        }
        let mut set_offsets = Vec::with_capacity(set_count as usize);
        for _ in 0..set_count {
            set_offsets.push(offset.wrapping_add(header.u16().unwrap() as u32));
        }

        for (j, &set_offset) in set_offsets.iter().enumerate() {
            let Ok(mut sr) = FontReader::new(slice).at(set_offset as usize) else { break 'parse };
            let Ok(lig_count) = sr.u16() else { break 'parse };
            if sr.require_room(lig_count as usize, 2).is_err() {
                break 'parse;
            }
            let mut lig_offsets = Vec::with_capacity(lig_count as usize);
            for _ in 0..lig_count {
                lig_offsets.push(set_offset.wrapping_add(sr.u16().unwrap() as u32));
            }

            for &lig_offset in &lig_offsets {
                let Ok(mut lr) = FontReader::new(slice).at(lig_offset as usize) else { break 'parse };
                let Ok(lig_glyph) = lr.u16() else { break 'parse };
                let Ok(lig_components) = lr.u16() else { break 'parse };
                if lr.require_room((lig_components as usize).saturating_sub(1), 2).is_err() {
                    break 'parse;
                }
                let cov: *mut Coverage = otl_coverage_create();
                push_to_coverage(cov, handle_from_index((&(*start_coverage))[j].index) as GlyphHandle);
                for _ in 1..lig_components {
                    push_to_coverage(cov, handle_from_index(lr.u16().unwrap() as GlyphId) as GlyphHandle);
                }
                (*subtable).push(GsubLigatureEntry {
                    from: coverage_from_raw(cov),
                    to: handle_from_index(lig_glyph as GlyphId) as GlyphHandle,
                });
            }
        }
        otl_coverage_free(start_coverage);
        return subtable_from_raw(subtable, Subtable::GsubLigature);
    }
    subtable_gsub_ligature_free(subtable);
    if !start_coverage.is_null() {
        otl_coverage_free(start_coverage);
    }
    ::core::ptr::null_mut::<Subtable>()
}
pub unsafe extern "C" fn otl_gsub_dump_ligature(
    mut _subtable: *const Subtable,
) -> *mut BuiltValue {
    let Subtable::GsubLigature(mut_subtable) = &*_subtable else { unreachable!() };
    let subtable: *const GsubLigatureSubtable = mut_subtable;
    let mut st: *mut BuiltValue = json_array_new((*subtable).len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).len() {
        let mut entry: *mut BuiltValue = json_object_new(2 as usize);
        json_object_push(
            entry,
            b"from\0" as *const u8 as *const ::core::ffi::c_char,
            dump_coverage(
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
    let mut ret: *mut BuiltValue = json_object_new(1 as usize);
    json_object_push(
        ret,
        b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
        st,
    );
    return ret;
}
pub unsafe extern "C" fn otl_gsub_parse_ligature(
    mut _subtable: *const ParsedValue,
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
        let mut n: GlyphId = json_arr_len(_subtable) as GlyphId;
        let mut k: GlyphId = 0 as GlyphId;
        while (k as ::core::ffi::c_int) < n as ::core::ffi::c_int {
            let mut entry: *const ParsedValue = json_arr_at(_subtable, k as u32);
            let mut _from: *const ParsedValue = json_obj_get_type(
                entry,
                b"from\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            );
            let mut _to: *const ParsedValue = json_obj_get_type(
                entry,
                b"to\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            if !(_from.is_null() || _to.is_null()) {
                (*st).push(GsubLigatureEntry {
                    from: coverage_from_raw(
                        parse_coverage(_from),
                    ),
                    to: handle_from_name(Some(json_str_bytes(_to))) as GlyphHandle,
                });
            }
            k = k.wrapping_add(1);
        }
        return subtable_from_raw(st, Subtable::GsubLigature);
    } else {
        let st_0: *mut GsubLigatureSubtable = subtable_gsub_ligature_create();
        let mut n_0: GlyphId = json_obj_len(_subtable) as GlyphId;
        let mut k_0: GlyphId = 0 as GlyphId;
        while (k_0 as ::core::ffi::c_int) < n_0 as ::core::ffi::c_int {
            let mut _from_0: *const ParsedValue = json_obj_val_at(_subtable, k_0 as u32);
            if !(_from_0.is_null()
                || json_type_of(_from_0) != JsonType::Array)
            {
                (*st_0).push(GsubLigatureEntry {
                    from: coverage_from_raw(
                        parse_coverage(_from_0),
                    ),
                    to: handle_from_name(Some(json_obj_key_bytes_at(_subtable, k_0 as u32))) as GlyphHandle,
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
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(build_coverage(
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

#[cfg(test)]
mod otl_read_gsub_ligature_tests {
    use super::*;

    #[test]
    fn well_formed_table_reads_one_ligature() {
        let mut data = [0u8; 24];
        data[2..4].copy_from_slice(&8u16.to_be_bytes()); // coverageOffset -> 8
        data[4..6].copy_from_slice(&1u16.to_be_bytes()); // ligSetCount
        data[6..8].copy_from_slice(&14u16.to_be_bytes()); // ligSetOffsets[0] -> 14
        // Coverage format 1 at byte 8: one glyph, id 10.
        data[8..10].copy_from_slice(&1u16.to_be_bytes());
        data[10..12].copy_from_slice(&1u16.to_be_bytes());
        data[12..14].copy_from_slice(&10u16.to_be_bytes());
        // LigatureSet at byte 14: one ligature, offset 4 (-> byte 18).
        data[14..16].copy_from_slice(&1u16.to_be_bytes());
        data[16..18].copy_from_slice(&4u16.to_be_bytes());
        // Ligature at byte 18: ligGlyph=30, componentCount=2 (1 declared component after the implicit first).
        data[18..20].copy_from_slice(&30u16.to_be_bytes());
        data[20..22].copy_from_slice(&2u16.to_be_bytes());
        data[22..24].copy_from_slice(&20u16.to_be_bytes());
        unsafe {
            let raw = otl_read_gsub_ligature(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::GsubLigature(entries) = &*boxed else { unreachable!() };
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].from.iter().map(|h| h.index).collect::<Vec<_>>(), vec![10, 20]);
            assert_eq!(entries[0].to.index, 30);
        }
    }

    #[test]
    fn ligature_set_count_mismatch_with_coverage_is_rejected() {
        let mut data = [0u8; 20];
        data[2..4].copy_from_slice(&8u16.to_be_bytes());
        data[4..6].copy_from_slice(&2u16.to_be_bytes()); // ligSetCount claims 2
        data[6..8].copy_from_slice(&14u16.to_be_bytes());
        // Coverage format 1 at byte 8: only 1 glyph, not 2.
        data[8..10].copy_from_slice(&1u16.to_be_bytes());
        data[10..12].copy_from_slice(&1u16.to_be_bytes());
        data[12..14].copy_from_slice(&10u16.to_be_bytes());
        unsafe {
            let raw = otl_read_gsub_ligature(data.as_ptr() as FontFilePointer, data.len() as u32, 0, 0);
            assert!(raw.is_null());
        }
    }
}
