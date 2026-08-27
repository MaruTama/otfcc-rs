#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, strlen, strtol};

use crate::support::handle::{GlyphHandle, handle_from_index, handle_from_name};
use crate::support::parsed_json::{
    ParsedValue, json_obj_get_type, json_obj_key_at, json_obj_len, json_obj_val_at, json_str_bytes,
    json_type_of,
};

use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::bk::bkblock::{bk_new_block_from_buffer, bk_new_block_from_buffer_copy};
use crate::bk::bkgraph::bk_build_block;
use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::NULL;
use crate::support::alloc::__caryll_allocate_clean;
use crate::support::binio::read_16u;
use crate::support::buffer::Buffer;
use crate::support::buffer::{
    buffree, buflen, bufnew, bufseek, bufwrite_buf, bufwrite8, bufwrite16b, bufwrite24b,
    bufwrite32b,
};
use crate::support::built_json::{
    BuiltValue, json_object_new, json_object_push, json_object_push_bytes_key,
    json_string_new_from_bytes,
};
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, TableId, Unicode};
use crate::vendor::json::JsonType;
use crate::support::fmt::Hex4Upper;
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CmapUvsKey {
    pub unicode: u32,
    pub selector: u32,
}
/// `unicodes` replaces the uthash-based `CmapEntry` -- unlike every
/// uthash instance converted so far in this migration, this map is not
/// a transient, build-then-drain scratch structure: it's a *persistent*
/// field of `CmapTable` itself, read/written/iterated throughout the
/// table's whole lifetime (encode/unmap/lookup during parse and JSON
/// encode, sorted iteration during dump and binary build). `BTreeMap`'s
/// dedup key and `by_unicode`'s `HASH_SORT` key are the same field
/// (`unicode`), so it supports every operation this file needs natively
/// -- no separate sort step anywhere, matching `LanguageHash`.
///
/// `uvs` follows the same shape: `by_uvs_key` sorts by `(unicode,
/// selector)` in that order, which is exactly `CmapUvsKey`'s derived
/// `Ord` (fields compared in declaration order), and `HASH_FIND`'s key
/// equality is the same two-field comparison -- sort key, dedup key and
/// derived `Ord` all agree, so `BTreeMap<CmapUvsKey, GlyphHandle>` needs
/// no wrapper struct and no explicit sort at drain time either.
// Stage 6-4 "Box化": both fields are already `BTreeMap`s (owning
// `GlyphHandle` values, which themselves have real `Drop`/`Clone` from
// the `Handle` pilot earlier in this migration), so no manual `Drop`
// impl is needed -- `Box::new` construction plus the derived drop glue
// is sufficient. The entire vtable is deleted, but unlike every other
// table converted so far, four of its "method" slots (`.lookup`,
// `.encode_uvs_by_index`, used from `read_uvs_default`/
// `read_uvs_non_default`/`otfcc_build_cmap_format14`) genuinely were
// called *through the vtable*, not just assigned to it -- a first-pass
// grep for `TABLE_I_CMAP\.` on one line missed them because the call
// syntax wraps the method name onto its own line
// (`TABLE_I_CMAP\n    .lookup\n    .expect(...)`), a lesson for future
// vtable-deletion greps in this crate: search for the bare identifier,
// not an anchored one-line pattern. Fixed by calling the four live
// slots' backing functions directly (`otfcc_cmap_lookup`,
// `otfcc_encode_cmap_uvs_by_index`) instead of through the vtable --
// same functions, no behavior change. `.create`/`.free` were confirmed
// only ever called from `caryll_font.rs`'s table disposal (outside this
// file) and from this file's own former `table_cmap_create`/`_free`
// wrappers (now gone). `.unmap`/`.unmap_uvs`/`.encode_by_index`/
// `.encode_by_name`/`.encode_uvs_by_name` were dead in vtable form (kept
// as ordinary exported functions, since deleting live-looking public API
// during a type-only conversion would be scope creep).
pub struct CmapTable {
    pub unicodes: std::collections::BTreeMap<::core::ffi::c_int, GlyphHandle>,
    pub uvs: std::collections::BTreeMap<CmapUvsKey, GlyphHandle>,
}
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
#[inline]
unsafe fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub unsafe fn otfcc_encode_cmap_by_index(
    cmap: *mut CmapTable,
    c: ::core::ffi::c_int,
    gid: u16,
) -> bool {
    match (*cmap).unicodes.entry(c) {
        std::collections::btree_map::Entry::Vacant(v) => {
            v.insert(handle_from_index(gid as GlyphId) as GlyphHandle);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}
// `name` is a caller-owned `Vec<u8>` now (the two callers -- `parse_cmap_
// unicodes`/`parse_cmap_uvs` -- pass a clone, keeping their own copy for
// the log message that follows on the Occupied path). The Occupied
// ("already mapped") path used to leave the old `SdsRaw` `name` unfreed
// -- a pre-existing leak this migration didn't own until now -- but that
// hazard is gone by construction: an unused `Vec<u8>` just drops.
//
// Never a real FFI boundary -- internal call sites only, same rationale
// as every other instance of this allow in the crate.
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_encode_cmap_by_name(
    cmap: *mut CmapTable,
    c: ::core::ffi::c_int,
    name: Vec<u8>,
) -> bool {
    match (*cmap).unicodes.entry(c) {
        std::collections::btree_map::Entry::Vacant(v) => {
            v.insert(handle_from_name(Some(name)) as GlyphHandle);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}
pub unsafe fn otfcc_unmap_cmap(cmap: *mut CmapTable, c: ::core::ffi::c_int) -> bool {
    // Removing the entry drops its `GlyphHandle` (freeing the glyph
    // name), replacing the explicit `otfcc_handle_dispose` + manual
    // node walk this walk used to do.
    (*cmap).unicodes.remove(&c).is_some()
}
pub unsafe fn otfcc_cmap_lookup(
    cmap: *const CmapTable,
    c: ::core::ffi::c_int,
) -> *mut GlyphHandle {
    // `cmap` is `*const` but the original returned a mutable-looking
    // pointer into the same hash node regardless -- raw-pointer
    // constness was never enforced here, only advisory, matching the
    // rest of this crate's `*const`/`*mut` cmap plumbing.
    match (*cmap).unicodes.get(&c) {
        Some(glyph) => glyph as *const GlyphHandle as *mut GlyphHandle,
        None => ::core::ptr::null_mut::<GlyphHandle>(),
    }
}
pub unsafe fn otfcc_encode_cmap_uvs_by_index(
    cmap: *mut CmapTable,
    c: CmapUvsKey,
    gid: u16,
) -> bool {
    match (*cmap).uvs.entry(c) {
        std::collections::btree_map::Entry::Vacant(v) => {
            v.insert(handle_from_index(gid as GlyphId) as GlyphHandle);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}
// Same `Vec<u8>`-in shape as `otfcc_encode_cmap_by_name` above, same
// reason.
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_encode_cmap_uvs_by_name(
    cmap: *mut CmapTable,
    c: CmapUvsKey,
    name: Vec<u8>,
) -> bool {
    match (*cmap).uvs.entry(c) {
        std::collections::btree_map::Entry::Vacant(v) => {
            v.insert(handle_from_name(Some(name)) as GlyphHandle);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}
pub unsafe fn otfcc_unmap_cmap_uvs(cmap: *mut CmapTable, c: CmapUvsKey) -> bool {
    (*cmap).uvs.remove(&c).is_some()
}
pub unsafe fn otfcc_cmap_lookup_uvs(
    cmap: *const CmapTable,
    c: CmapUvsKey,
) -> *mut GlyphHandle {
    match (*cmap).uvs.get(&c) {
        Some(glyph) => glyph as *const GlyphHandle as *mut GlyphHandle,
        None => ::core::ptr::null_mut::<GlyphHandle>(),
    }
}
// Every reader below takes the *whole* cmap table's bytes (`data`) plus an
// absolute offset into it, instead of the original's `(start: pointer,
// length_limit: u32)` pair. That pairing is what let the plan's two
// headline bugs happen: `length_limit` was computed once, elsewhere, via
// `length.wrapping_sub(table_offset)` -- an offset read straight from the
// file and never checked against `length` first, so a `table_offset`
// larger than `length` wrapped the subtraction to a huge number and every
// downstream `length_limit < ...` guard passed vacuously. Dropping
// `length_limit` entirely and re-deriving "how much is left" as
// `data.len() - offset` fresh at each `FontReader::at(offset)` call closes
// this by construction: `at` itself rejects `offset > data.len()` before
// any arithmetic on it happens, so there is nothing left to underflow.
// This also applies recursively -- `read_format14`'s dispatch into
// `read_uvs_default`/`read_uvs_non_default` had the exact same
// `length_limit.wrapping_sub(offset)` shape one level down.
//
// The other bug class -- a `count`-driven guard computed with
// `wrapping_add`/`wrapping_mul` on a `count` read straight from the file
// (`n_groups`, `num_unicode_value_ranges`, `num_uvs_mappings`, all full
// 32-bit fields) -- is closed the same way it already was in `name.rs`/
// `meta.rs`: `FontReader::require_room`'s `checked_mul`/`checked_add`.
unsafe fn read_format12(data: &[u8], offset: usize, cmap: *mut CmapTable) {
    let mut r = match FontReader::new(data).at(offset) {
        Ok(r) => r,
        Err(_) => return,
    };
    if r.skip(12).is_err() {
        return; // format, reserved, length, language
    }
    let Ok(n_groups) = r.u32() else { return };
    if r.require_room(n_groups as usize, 12).is_err() {
        return;
    }
    for _ in 0..n_groups {
        let start_code = r.u32().unwrap();
        let end_code = r.u32().unwrap();
        let start_gid = r.u32().unwrap();
        // `startCharCode`/`endCharCode` are raw 32-bit fields with no
        // structural guard tying them to a sane range: a single 12-byte
        // group can claim `endCharCode = 0xFFFFFFFF`, which without a
        // clamp is not just slow but a genuine infinite loop --
        // `c.wrapping_add(1)` at `0xFFFFFFFF` wraps back to `0`, and
        // `0 <= 0xFFFFFFFF` is true forever. Even short of that, a group
        // like `0x0..0xffff0000` forces ~4 billion iterations from 12
        // bytes of input. No real cmap needs to enumerate past the
        // Unicode codepoint ceiling, so clamp the walked range to it --
        // this changes nothing for any well-formed group (real codepoints
        // are always <= 0x10FFFF) and turns the unbounded/infinite cases
        // into a bounded, still-correct partial read of the group.
        let clamped_end = end_code.min(0x10ffff);
        let mut c = start_code;
        while c <= clamped_end {
            otfcc_encode_cmap_by_index(
                cmap,
                c as ::core::ffi::c_int,
                c.wrapping_sub(start_code).wrapping_add(start_gid) as u16,
            );
            c = c.wrapping_add(1);
        }
    }
}
unsafe fn read_format4(data: &[u8], offset: usize, cmap: *mut CmapTable) {
    let available = data.len().saturating_sub(offset);
    if available < 14 {
        return;
    }
    let mut header = match FontReader::new(data).at(offset) {
        Ok(r) => r,
        Err(_) => return,
    };
    if header.skip(6).is_err() {
        return; // format, length, language
    }
    let Ok(seg_count_x2) = header.u16() else {
        return;
    };
    let segments_count = (seg_count_x2 / 2) as usize;
    let Some(needed) = segments_count
        .checked_mul(8)
        .and_then(|n| n.checked_add(16))
    else {
        return;
    };
    if available < needed {
        return;
    }

    // All four parallel arrays are subtable-relative, matching the
    // original's `start.offset(...)` layout: endCode[] at 14, a 2-byte
    // reserved pad, then startCode[]/idDelta[]/idRangeOffset[], each
    // `segments_count` u16s.
    let end_code_rel = 14usize;
    let start_code_rel = end_code_rel + segments_count * 2 + 2;
    let id_delta_rel = start_code_rel + segments_count * 2;
    let id_range_offset_rel = id_delta_rel + segments_count * 2;

    let read_u16 = |rel: usize| -> Option<u16> {
        FontReader::new(data)
            .at(offset + rel)
            .ok()
            .and_then(|mut r| r.u16().ok())
    };

    for j in 0..segments_count {
        let Some(end_code) = read_u16(end_code_rel + j * 2) else {
            return;
        };
        let Some(start_code) = read_u16(start_code_rel + j * 2) else {
            return;
        };
        let Some(id_delta_raw) = read_u16(id_delta_rel + j * 2) else {
            return;
        };
        let id_delta = id_delta_raw as i16;
        let id_range_offset_entry_rel = id_range_offset_rel + j * 2;
        let Some(id_range_offset) = read_u16(id_range_offset_entry_rel) else {
            return;
        };
        if id_range_offset == 0 {
            let mut c = start_code as u32;
            while c < 0xffff && c <= end_code as u32 {
                let gid = (c.wrapping_add(id_delta as u32) & 0xffff) as u16;
                otfcc_encode_cmap_by_index(cmap, c as ::core::ffi::c_int, gid);
                c = c.wrapping_add(1);
            }
        } else {
            let mut c = start_code as u32;
            while c < 0xffff && c <= end_code as u32 {
                // idRangeOffset's value is a byte distance measured from the
                // idRangeOffset array *entry itself* -- matches the
                // original's `.wrapping_add(id_range_offset_offset)`.
                let glyph_offset_rel = (id_range_offset as u32)
                    .wrapping_add(c.wrapping_sub(start_code as u32).wrapping_mul(2))
                    .wrapping_add(id_range_offset_entry_rel as u32);
                if let Some(raw) = read_u16(glyph_offset_rel as usize) {
                    let gid = ((raw as i32 + id_delta as i32) & 0xffff) as u16;
                    otfcc_encode_cmap_by_index(cmap, c as ::core::ffi::c_int, gid);
                }
                c = c.wrapping_add(1);
            }
        }
    }
}
unsafe fn read_uvs_default(data: &[u8], offset: usize, selector: Unicode, cmap: *mut CmapTable) {
    let mut r = match FontReader::new(data).at(offset) {
        Ok(r) => r,
        Err(_) => return,
    };
    let Ok(num_ranges) = r.u32() else { return };
    if r.require_room(num_ranges as usize, 4).is_err() {
        return;
    }
    for _ in 0..num_ranges {
        let start_unicode_value = r.u24().unwrap();
        let additional_count = r.u8().unwrap();
        let mut u = start_unicode_value;
        let end = start_unicode_value.wrapping_add(additional_count as u32);
        while u <= end {
            let g = otfcc_cmap_lookup(cmap, u as ::core::ffi::c_int);
            if !g.is_null() {
                otfcc_encode_cmap_uvs_by_index(
                    cmap,
                    CmapUvsKey {
                        unicode: u,
                        selector,
                    },
                    (*g).index as u16,
                );
            }
            u = u.wrapping_add(1);
        }
    }
}
unsafe fn read_uvs_non_default(
    data: &[u8],
    offset: usize,
    selector: Unicode,
    cmap: *mut CmapTable,
) {
    let mut r = match FontReader::new(data).at(offset) {
        Ok(r) => r,
        Err(_) => return,
    };
    let Ok(num_mappings) = r.u32() else { return };
    if r.require_room(num_mappings as usize, 5).is_err() {
        return;
    }
    for _ in 0..num_mappings {
        let unicode_value = r.u24().unwrap();
        let glyph_id = r.u16().unwrap();
        otfcc_encode_cmap_uvs_by_index(
            cmap,
            CmapUvsKey {
                unicode: unicode_value,
                selector,
            },
            glyph_id,
        );
    }
}
unsafe fn read_format14(data: &[u8], offset: usize, cmap: *mut CmapTable) {
    let mut r = match FontReader::new(data).at(offset) {
        Ok(r) => r,
        Err(_) => return,
    };
    if r.skip(6).is_err() {
        return; // format, length
    }
    let Ok(n_groups) = r.u32() else { return }; // numVarSelectorRecords, at offset+6
    // The original's guard is `length_limit >= 11 + 11*n_groups` -- one
    // byte more than the VarSelectorRecord array's actual size
    // (10 + 11*n_groups) needs. Preserved exactly: it's stricter, not
    // weaker, so keeping it doesn't reopen any bound.
    let Some(needed) = (n_groups as usize)
        .checked_mul(11)
        .and_then(|n| n.checked_add(11))
    else {
        return;
    };
    if data.len().saturating_sub(offset) < needed {
        return;
    }
    for j in 0..n_groups as usize {
        let record_rel = 10 + 11 * j;
        let Ok(selector) = FontReader::new(data)
            .at(offset + record_rel)
            .and_then(|mut r| r.u24())
        else {
            return;
        };
        let Ok(default_uvs_offset) = FontReader::new(data)
            .at(offset + record_rel + 3)
            .and_then(|mut r| r.u32())
        else {
            return;
        };
        let Ok(non_default_uvs_offset) = FontReader::new(data)
            .at(offset + record_rel + 7)
            .and_then(|mut r| r.u32())
        else {
            return;
        };
        if default_uvs_offset != 0 {
            if let Some(sub_offset) = offset.checked_add(default_uvs_offset as usize) {
                read_uvs_default(data, sub_offset, selector, cmap);
            }
        }
        if non_default_uvs_offset != 0 {
            if let Some(sub_offset) = offset.checked_add(non_default_uvs_offset as usize) {
                read_uvs_non_default(data, sub_offset, selector, cmap);
            }
        }
    }
}
unsafe fn read_cmap_mapping_table(
    data: &[u8],
    offset: usize,
    cmap: *mut CmapTable,
    required_format: TableId,
) {
    let Some(format) = FontReader::new(data)
        .at(offset)
        .ok()
        .and_then(|mut r| r.u16().ok())
    else {
        return;
    };
    if format == required_format {
        if format == 4 {
            read_format4(data, offset, cmap);
        } else if format == 12 {
            read_format12(data, offset, cmap);
        }
    }
}
unsafe fn read_cmap_mapping_table_uvs(data: &[u8], offset: usize, cmap: *mut CmapTable) {
    let Some(format) = FontReader::new(data)
        .at(offset)
        .ok()
        .and_then(|mut r| r.u16().ok())
    else {
        return;
    };
    if format == 14 {
        read_format14(data, offset, cmap);
    }
}
#[inline]
fn is_valid_cmap_encoding(platform: u16, encoding: u16) -> bool {
    matches!(
        (platform, encoding),
        (0, 3) | (0, 4) | (0, 5) | (3, 1) | (3, 10)
    )
}
pub static FORMAT_PRIORITIES: [TableId; 3] = [12, 4, 0];
// `FORMAT_PRIORITIES` ends with a `0` sentinel that is also, confusingly,
// a real cmap subtable format number (Apple standard byte encoding) --
// `take_while(|&f| f != 0)` stops before reaching it, matching the
// original's `while FORMAT_PRIORITIES[k] != 0` loop exactly: only formats
// 12 and 4 are ever dispatched as a `required_format`, never 0.
unsafe fn parse_cmap(data: &[u8]) -> Result<Box<CmapTable>, ReadError> {
    let mut header = FontReader::new(data);
    header.skip(2)?; // version
    let num_tables = header.u16()? as usize;
    header.require_room(num_tables, 8)?;

    let mut cmap_box = Box::new(CmapTable {
        unicodes: std::collections::BTreeMap::new(),
        uvs: std::collections::BTreeMap::new(),
    });
    let cmap: *mut CmapTable = cmap_box.as_mut() as *mut CmapTable;

    // Nothing in the cmap directory requires each entry's subtable offset
    // to be distinct -- the spec explicitly allows encoding records to
    // share a subtable (e.g. the Windows Unicode BMP and full-repertoire
    // records legitimately pointing at the same data). `num_tables` valid
    // entries can all alias one large format 4/12 subtable, and without
    // dedup every alias re-parses (and re-walks every codepoint of) that
    // same subtable from scratch -- an offset-aliasing amplification of
    // the same shape as the OTL script/language one (see
    // `MAX_TOTAL_LANGUAGES` in `table/otl/read.rs`), except here the
    // multiplier is `num_tables` itself (bounded only by table size / 8
    // bytes per entry) rather than a fixed cap. A fuzz session found a
    // small font with just two aliasing pairs already pushing a single
    // parse past 47 seconds under the fuzzer's instrumentation.
    //
    // Deduping by raw subtable offset is safe for well-formed fonts:
    // re-parsing the same bytes at the same offset is idempotent (every
    // insert into `cmap.unicodes`/`cmap.uvs` produces the same mapping
    // each time), so skipping the repeat parses never changes the final
    // table -- only the wasted work is removed.
    let mut processed_offsets: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
    for &required_format in FORMAT_PRIORITIES.iter().take_while(|&&f| f != 0) {
        for j in 0..num_tables {
            let entry_rel = 4 + 8 * j;
            let mut entry = FontReader::new(data).at(entry_rel)?;
            let platform = entry.u16()?;
            let encoding = entry.u16()?;
            if is_valid_cmap_encoding(platform, encoding) {
                let table_offset = FontReader::new(data).at(entry_rel + 4)?.u32()? as usize;
                let Some(format) = FontReader::new(data)
                    .at(table_offset)
                    .ok()
                    .and_then(|mut r| r.u16().ok())
                else {
                    continue;
                };
                if format == required_format && processed_offsets.insert(table_offset) {
                    read_cmap_mapping_table(data, table_offset, cmap, required_format);
                }
            }
        }
    }
    let mut processed_uvs_offsets: std::collections::BTreeSet<usize> =
        std::collections::BTreeSet::new();
    for j in 0..num_tables {
        let entry_rel = 4 + 8 * j;
        let mut entry = FontReader::new(data).at(entry_rel)?;
        let platform = entry.u16()?;
        let encoding = entry.u16()?;
        if is_valid_cmap_encoding(platform, encoding) {
            let table_offset = FontReader::new(data).at(entry_rel + 4)?.u32()? as usize;
            if processed_uvs_offsets.insert(table_offset) {
                read_cmap_mapping_table_uvs(data, table_offset, cmap);
            }
        }
    }
    Ok(cmap_box)
}
pub unsafe fn otfcc_read_cmap(packet: &Packet, options: &Options) -> Option<Box<CmapTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_CMAP)?;
    match parse_cmap(&table.data) {
        Ok(cmap) => Some(cmap),
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"table 'cmap' corrupted.\n"),
            );
            None
        }
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_cmap(
    table: Option<&CmapTable>,
    root: *mut BuiltValue,
    options: &Options,
) {
    let table = match table {
        Some(t) => t as *const CmapTable,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"cmap"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        if !(*table).unicodes.is_empty() {
            let cmap: *mut BuiltValue = json_object_new((*table).unicodes.len());
            for (&unicode, glyph) in (*table).unicodes.iter() {
                if !glyph.name.is_empty() {
                    let key: Vec<u8> = if options.decimal_cmap {
                        crate::bytesbuild!(unicode)
                    } else {
                        crate::bytesbuild!(b"U+", Hex4Upper(unicode as u32))
                    };
                    json_object_push_bytes_key(cmap, &key, json_string_new_from_bytes(&glyph.name));
                }
            }
            json_object_push(
                root,
                b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
                cmap,
            );
        }
        if !(*table).uvs.is_empty() {
            let uvs: *mut BuiltValue = json_object_new((*table).uvs.len());
            for (key, glyph) in (*table).uvs.iter() {
                if !glyph.name.is_empty() {
                    let key_0: Vec<u8> = if options.decimal_cmap {
                        crate::bytesbuild!(key.unicode, b" ", key.selector,)
                    } else {
                        crate::bytesbuild!(
                            b"U+",
                            Hex4Upper(key.unicode as u32),
                            b" U+",
                            Hex4Upper(key.selector as u32),
                        )
                    };
                    json_object_push_bytes_key(
                        uvs,
                        &key_0,
                        json_string_new_from_bytes(&glyph.name),
                    );
                }
            }
            json_object_push(
                root,
                b"cmap_uvs\0" as *const u8 as *const ::core::ffi::c_char,
                uvs,
            );
        }
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
// `unicode_str` borrows `json_obj_key_at`'s pointer directly rather than
// going through an owned `sds` copy: every JSON object key is already
// NUL-terminated in `ParsedValue`'s own storage (see `json_obj_key_at`'s
// doc comment), so `strlen` here sees exactly the same length `sdslen`
// used to on the `sdsnewlen`-copied version -- no allocation or free
// needed at either call site any more.
#[inline]
unsafe fn parse_unicode(unicode_str: *const ::core::ffi::c_char) -> Unicode {
    if strlen(unicode_str) > 2 as usize
        && *unicode_str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'U' as i32
        && *unicode_str.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '+' as i32
    {
        return strtol(
            unicode_str.offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            16 as ::core::ffi::c_int,
        ) as Unicode;
    } else {
        return atoi(unicode_str as *const ::core::ffi::c_char) as Unicode;
    };
}
unsafe fn parse_cmap_unicodes(
    cmap: *mut CmapTable,
    table: *const ParsedValue,
    options: &Options,
) {
    if table.is_null() || json_type_of(table) != JsonType::Object {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < json_obj_len(table) as u32 {
        let item: *const ParsedValue = json_obj_val_at(table, j as u32);
        let unicode: Unicode = parse_unicode(json_obj_key_at(table, j as u32));
        if json_type_of(item) == JsonType::String
            && unicode > 0 as Unicode
            && unicode <= 0x10ffff as Unicode
        {
            let gname: Vec<u8> = json_str_bytes(item);
            if !otfcc_encode_cmap_by_name(cmap, unicode as ::core::ffi::c_int, gname.clone()) {
                let current_map: *mut GlyphHandle =
                    otfcc_cmap_lookup(cmap, unicode as ::core::ffi::c_int) as *mut GlyphHandle;
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"U+",
                        Hex4Upper(unicode as u32),
                        b" is already mapped to ",
                        &(*current_map).name,
                        b". Assignment to ",
                        &gname,
                        b" is ignored.",
                    ),
                );
            }
        }
        j = j.wrapping_add(1);
    }
}
// Same borrow-`json_obj_key_at`-directly reasoning as `parse_unicode`.
#[inline]
unsafe fn parse_uvs_key(uvs_str: *const ::core::ffi::c_char) -> CmapUvsKey {
    let len: usize = strlen(uvs_str);
    let mut k: CmapUvsKey = CmapUvsKey {
        unicode: 0 as u32,
        selector: 0 as u32,
    };
    let mut scan: *const ::core::ffi::c_char = uvs_str;
    while scan < uvs_str.offset(len as isize) {
        if *scan as ::core::ffi::c_int == ' ' as i32 {
            k.unicode = parse_unicode(uvs_str) as u32;
            k.selector = parse_unicode(scan.offset(1 as ::core::ffi::c_int as isize)) as u32;
            return k;
        }
        scan = scan.offset(1);
    }
    return k;
}
unsafe fn parse_cmap_uvs(
    cmap: *mut CmapTable,
    table: *const ParsedValue,
    options: &Options,
) {
    if table.is_null() || json_type_of(table) != JsonType::Object {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < json_obj_len(table) as u32 {
        let k: CmapUvsKey = parse_uvs_key(json_obj_key_at(table, j as u32));
        let item: *const ParsedValue = json_obj_val_at(table, j as u32);
        if json_type_of(item) == JsonType::String
            && k.unicode > 0 as u32
            && k.unicode <= 0x10ffff as u32
            && k.selector > 0 as u32
            && k.selector <= 0x10ffff as u32
        {
            let gname: Vec<u8> = json_str_bytes(item);
            if !otfcc_encode_cmap_uvs_by_name(cmap, k, gname.clone()) {
                let current_map: *mut GlyphHandle =
                    otfcc_cmap_lookup_uvs(cmap, k) as *mut GlyphHandle;
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"UVS U+",
                        Hex4Upper((k.unicode) as u32),
                        b" U+",
                        Hex4Upper((k.selector) as u32),
                        b" is already mapped to ",
                        &(*current_map).name,
                        b". Assignment to ",
                        &gname,
                        b" is ignored.",
                    ),
                );
            }
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe fn otfcc_parse_cmap(
    root: *const ParsedValue,
    options: &Options,
) -> Option<Box<CmapTable>> {
    if json_type_of(root) != JsonType::Object {
        return None;
    }
    let mut cmap_box: Box<CmapTable> = Box::new(CmapTable {
        unicodes: std::collections::BTreeMap::new(),
        uvs: std::collections::BTreeMap::new(),
    });
    let cmap: *mut CmapTable = cmap_box.as_mut() as *mut CmapTable;
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"cmap"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        parse_cmap_unicodes(
            cmap,
            json_obj_get_type(
                root,
                b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Object,
            ),
            options,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"cmap_uvs"),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        parse_cmap_uvs(
            cmap,
            json_obj_get_type(
                root,
                b"cmap_uvs\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Object,
            ),
            options,
        );
        ___loggedstep_v_0 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    return Some(cmap_box);
}
unsafe fn otfcc_build_cmap_format4(cmap: *const CmapTable) -> *mut Buffer {
    let buf: *mut Buffer = bufnew();
    let end_count: *mut Buffer = bufnew();
    let start_count: *mut Buffer = bufnew();
    let id_delta: *mut Buffer = bufnew();
    let id_range_offset: *mut Buffer = bufnew();
    let glyph_id_array: *mut Buffer = bufnew();
    let mut started: bool = false;
    let mut last_unicode_start: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut last_unicode_end: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut last_gid_start: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut last_gid_end: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut last_glyph_id_array_offset: usize = 0 as usize;
    let mut is_sequencial: bool = true;
    let mut segments_count: u16 = 0 as u16;
    for (&unicode, glyph) in (*cmap).unicodes.iter() {
        if unicode <= 0xffff as ::core::ffi::c_int {
            if !started {
                started = true;
                last_unicode_end = unicode;
                last_unicode_start = last_unicode_end;
                last_gid_end = glyph.index as ::core::ffi::c_int;
                last_gid_start = last_gid_end;
                is_sequencial = true;
            } else if unicode == last_unicode_end + 1 as ::core::ffi::c_int
                && !(glyph.index as ::core::ffi::c_int != last_gid_end + 1 as ::core::ffi::c_int
                    && is_sequencial as ::core::ffi::c_int != 0
                    && last_gid_end - last_gid_start >= 4 as ::core::ffi::c_int)
            {
                if is_sequencial as ::core::ffi::c_int != 0
                    && !(glyph.index as ::core::ffi::c_int
                        == last_gid_end + 1 as ::core::ffi::c_int)
                {
                    last_glyph_id_array_offset = (*glyph_id_array).cursor;
                    let mut j: ::core::ffi::c_int = last_gid_start;
                    while j <= last_gid_end {
                        bufwrite16b(glyph_id_array, j as u16);
                        j += 1;
                    }
                }
                last_unicode_end = unicode;
                is_sequencial = is_sequencial as ::core::ffi::c_int != 0
                    && glyph.index as ::core::ffi::c_int == last_gid_end + 1 as ::core::ffi::c_int;
                last_gid_end = glyph.index as ::core::ffi::c_int;
                if !is_sequencial {
                    bufwrite16b(glyph_id_array, last_gid_end as u16);
                }
            } else {
                bufwrite16b(end_count, last_unicode_end as u16);
                bufwrite16b(start_count, last_unicode_start as u16);
                if is_sequencial {
                    bufwrite16b(id_delta, (last_gid_start - last_unicode_start) as u16);
                    bufwrite16b(id_range_offset, 0 as u16);
                } else {
                    bufwrite16b(id_delta, 0 as u16);
                    bufwrite16b(
                        id_range_offset,
                        last_glyph_id_array_offset.wrapping_add(1 as usize) as u16,
                    );
                }
                segments_count =
                    (segments_count as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
                last_unicode_end = unicode;
                last_unicode_start = last_unicode_end;
                last_gid_end = glyph.index as ::core::ffi::c_int;
                last_gid_start = last_gid_end;
                is_sequencial = true;
            }
        }
    }
    bufwrite16b(end_count, last_unicode_end as u16);
    bufwrite16b(start_count, last_unicode_start as u16);
    if is_sequencial {
        bufwrite16b(id_delta, (last_gid_start - last_unicode_start) as u16);
        bufwrite16b(id_range_offset, 0 as u16);
    } else {
        bufwrite16b(id_delta, 0 as u16);
        bufwrite16b(
            id_range_offset,
            last_glyph_id_array_offset.wrapping_add(1 as usize) as u16,
        );
    }
    segments_count = (segments_count as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
    if last_gid_end < 0xffff as ::core::ffi::c_int {
        bufwrite16b(end_count, 0xffff as u16);
        bufwrite16b(start_count, 0xffff as u16);
        bufwrite16b(id_delta, 1 as u16);
        bufwrite16b(id_range_offset, 0 as u16);
        segments_count = (segments_count as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
    }
    let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j_0 < segments_count as ::core::ffi::c_int {
        let mut ro: u16 = read_16u(
            (*id_range_offset)
                .data
                .as_ptr()
                .offset((j_0 * 2 as ::core::ffi::c_int) as isize),
        );
        if ro != 0 {
            ro = (ro as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u16;
            ro = (ro as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * (segments_count as ::core::ffi::c_int - j_0))
                as u16;
            bufseek(id_range_offset, (2 as ::core::ffi::c_int * j_0) as usize);
            bufwrite16b(id_range_offset, ro);
        }
        j_0 += 1;
    }
    bufwrite16b(buf, 4 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(
        buf,
        ((segments_count as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as u16,
    );
    let mut i: u32;
    let mut j_1: u32;
    j_1 = 0 as u32;
    i = 1 as u32;
    while i <= segments_count as u32 {
        i <<= 1 as ::core::ffi::c_int;
        j_1 = j_1.wrapping_add(1);
    }
    bufwrite16b(buf, i as u16);
    bufwrite16b(buf, j_1.wrapping_sub(1 as u32) as u16);
    bufwrite16b(
        buf,
        ((2 as ::core::ffi::c_int * segments_count as ::core::ffi::c_int) as u32).wrapping_sub(i)
            as u16,
    );
    bufwrite_buf(buf, end_count);
    bufwrite16b(buf, 0 as u16);
    bufwrite_buf(buf, start_count);
    bufwrite_buf(buf, id_delta);
    bufwrite_buf(buf, id_range_offset);
    bufwrite_buf(buf, glyph_id_array);
    bufseek(buf, 2 as usize);
    bufwrite16b(buf, buflen(buf) as u16);
    buffree(end_count);
    buffree(start_count);
    buffree(id_delta);
    buffree(id_range_offset);
    buffree(glyph_id_array);
    return buf;
}
unsafe fn otfcc_try_build_cmap_format4(cmap: *const CmapTable) -> *mut Buffer {
    let buf: *mut Buffer = otfcc_build_cmap_format4(cmap);
    if buflen(buf) > UINT16_MAX as usize {
        buffree(buf);
        return ::core::ptr::null_mut::<Buffer>();
    } else {
        return buf;
    };
}
unsafe fn otfcc_build_cmap_format12(cmap: *const CmapTable) -> *mut Buffer {
    let buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 12 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite32b(buf, 0 as u32);
    bufwrite32b(buf, 0 as u32);
    bufwrite32b(buf, 0 as u32);
    let mut n_groups: u32 = 0 as u32;
    let mut started: bool = false;
    let mut last_unicode_start: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut last_unicode_end: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut last_gid_start: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut last_gid_end: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    for (&unicode, glyph) in (*cmap).unicodes.iter() {
        if !started {
            started = true;
            last_unicode_end = unicode;
            last_unicode_start = last_unicode_end;
            last_gid_end = glyph.index as ::core::ffi::c_int;
            last_gid_start = last_gid_end;
        } else if unicode == last_unicode_end + 1 as ::core::ffi::c_int
            && glyph.index as ::core::ffi::c_int == last_gid_end + 1 as ::core::ffi::c_int
        {
            last_unicode_end = unicode;
            last_gid_end = glyph.index as ::core::ffi::c_int;
        } else {
            bufwrite32b(buf, last_unicode_start as u32);
            bufwrite32b(buf, last_unicode_end as u32);
            bufwrite32b(buf, last_gid_start as u32);
            n_groups = n_groups.wrapping_add(1 as u32);
            last_unicode_end = unicode;
            last_unicode_start = last_unicode_end;
            last_gid_end = glyph.index as ::core::ffi::c_int;
            last_gid_start = last_gid_end;
        }
    }
    bufwrite32b(buf, last_unicode_start as u32);
    bufwrite32b(buf, last_unicode_end as u32);
    bufwrite32b(buf, last_gid_start as u32);
    n_groups = n_groups.wrapping_add(1 as u32);
    bufseek(buf, 4 as usize);
    bufwrite32b(buf, buflen(buf) as u32);
    bufseek(buf, 12 as usize);
    bufwrite32b(buf, n_groups);
    return buf;
}
pub const MAX_UNICODE: ::core::ffi::c_int = 0x110001 as ::core::ffi::c_int;
pub const HAS_DEFAULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HAS_NON_DEFAULT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe fn write_default_range(
    dflt: *mut Buffer,
    n_ranges: *mut u32,
    mut start: Unicode,
    end: Unicode,
) {
    while end.wrapping_sub(start) > 0xff as Unicode {
        bufwrite24b(dflt, start as u32);
        bufwrite8(dflt, 0xff as u8);
        start = start.wrapping_add(0x100 as Unicode);
        *n_ranges = (*n_ranges).wrapping_add(1 as u32);
    }
    bufwrite24b(dflt, start as u32);
    bufwrite8(dflt, end.wrapping_sub(start) as u8);
    *n_ranges = (*n_ranges).wrapping_add(1 as u32);
}
unsafe fn build_format14_for_selector(
    cmap: *const CmapTable,
    selector: Unicode,
    dflt: *mut Buffer,
    nondflt: *mut Buffer,
) -> u8 {
    let defaults: *mut GlyphId;
    let non_defaults: *mut GlyphId;
    defaults = __caryll_allocate_clean(
        (::core::mem::size_of::<GlyphId>() as usize)
            .wrapping_mul(0x110001 as ::core::ffi::c_int as usize),
        626 as ::core::ffi::c_ulong,
    ) as *mut GlyphId;
    non_defaults = __caryll_allocate_clean(
        (::core::mem::size_of::<GlyphId>() as usize)
            .wrapping_mul(0x110001 as ::core::ffi::c_int as usize),
        627 as ::core::ffi::c_ulong,
    ) as *mut GlyphId;
    let mut s: Unicode = 0 as Unicode;
    while s < MAX_UNICODE as Unicode {
        *defaults.offset(s as isize) = 0xffff as GlyphId;
        *non_defaults.offset(s as isize) = 0xffff as GlyphId;
        s = s.wrapping_add(1);
    }
    for (key, glyph) in (*cmap).uvs.iter() {
        let u: Unicode = key.unicode as Unicode;
        if !(key.selector != selector || u >= MAX_UNICODE as Unicode) {
            if !glyph.name.is_empty() {
                let uvs_gid: GlyphId = glyph.index;
                let g: *mut GlyphHandle = otfcc_cmap_lookup(cmap, u as ::core::ffi::c_int);
                if g.is_null() {
                    *non_defaults.offset(u as isize) = uvs_gid;
                } else if uvs_gid as ::core::ffi::c_int == (*g).index as ::core::ffi::c_int {
                    *defaults.offset(u as isize) = uvs_gid;
                } else {
                    *non_defaults.offset(u as isize) = uvs_gid;
                }
            }
        }
    }
    let ref mut fresh8 = *non_defaults.offset(0 as ::core::ffi::c_int as isize);
    *fresh8 = 0xffff as GlyphId;
    *defaults.offset(0 as ::core::ffi::c_int as isize) = *fresh8;
    let ref mut fresh9 = *non_defaults.offset((MAX_UNICODE - 1 as ::core::ffi::c_int) as isize);
    *fresh9 = 0xffff as GlyphId;
    *defaults.offset((MAX_UNICODE - 1 as ::core::ffi::c_int) as isize) = *fresh9;
    let mut num_unicode_value_ranges: u32 = 0 as u32;
    let mut start_unicode_value: Unicode = 0 as Unicode;
    let mut num_uvs_mappings: u32 = 0 as u32;
    bufwrite32b(dflt, 0 as u32);
    bufwrite32b(nondflt, 0 as u32);
    let mut u_0: Unicode = 1 as Unicode;
    while u_0 < MAX_UNICODE as Unicode {
        if *defaults.offset(u_0 as isize) as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int
            && *defaults.offset(u_0.wrapping_sub(1 as Unicode) as isize) as ::core::ffi::c_int
                == 0xffff as ::core::ffi::c_int
        {
            start_unicode_value = u_0;
        }
        if *defaults.offset(u_0 as isize) as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
            && *defaults.offset(u_0.wrapping_sub(1 as Unicode) as isize) as ::core::ffi::c_int
                != 0xffff as ::core::ffi::c_int
        {
            write_default_range(
                dflt,
                &raw mut num_unicode_value_ranges,
                start_unicode_value,
                u_0.wrapping_sub(1 as Unicode),
            );
        }
        if *non_defaults.offset(u_0 as isize) as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int
        {
            bufwrite24b(nondflt, u_0 as u32);
            bufwrite16b(nondflt, *non_defaults.offset(u_0 as isize) as u16);
            num_uvs_mappings = num_uvs_mappings.wrapping_add(1);
        }
        u_0 = u_0.wrapping_add(1);
    }
    bufseek(dflt, 0 as usize);
    bufwrite32b(dflt, num_unicode_value_ranges);
    bufseek(nondflt, 0 as usize);
    bufwrite32b(nondflt, num_uvs_mappings);
    free(defaults as *mut ::core::ffi::c_void);
    free(non_defaults as *mut ::core::ffi::c_void);
    return ((if num_unicode_value_ranges != 0 {
        HAS_DEFAULT
    } else {
        0 as ::core::ffi::c_int
    }) | (if num_uvs_mappings != 0 {
        HAS_NON_DEFAULT
    } else {
        0 as ::core::ffi::c_int
    })) as u8;
}
unsafe fn otfcc_build_cmap_format14(cmap: *const CmapTable) -> *mut Buffer {
    let mut valid_selectors: Vec<bool> = vec![false; MAX_UNICODE as usize];
    for (key, _) in (*cmap).uvs.iter() {
        if key.selector < MAX_UNICODE as u32 {
            valid_selectors[key.selector as usize] = true;
        }
    }
    let mut n_selectors: u32 = 0 as u32;
    let mut selector: Unicode = 0 as Unicode;
    while selector < MAX_UNICODE as Unicode {
        if valid_selectors[selector as usize] {
            n_selectors = n_selectors.wrapping_add(1);
        }
        selector = selector.wrapping_add(1);
    }
    let st: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B16, 14 as u32),
        bk_int(BkCellType::B32, 0 as u32),
        bk_int(BkCellType::B32, n_selectors as u32),
    ]);
    let mut selector_0: Unicode = 0 as Unicode;
    while selector_0 < MAX_UNICODE as Unicode {
        if valid_selectors[selector_0 as usize] {
            let mut dflt: *mut Buffer = bufnew();
            let mut nondflt: *mut Buffer = bufnew();
            let results: u8 = build_format14_for_selector(cmap, selector_0, dflt, nondflt);
            if results as ::core::ffi::c_int & HAS_DEFAULT == 0 {
                buffree(dflt);
                dflt = ::core::ptr::null_mut::<Buffer>();
            }
            if results as ::core::ffi::c_int & HAS_NON_DEFAULT == 0 {
                buffree(nondflt);
                nondflt = ::core::ptr::null_mut::<Buffer>();
            }
            bk_push(
                st,
                &[
                    bk_int(
                        BkCellType::B8,
                        (selector_0 >> 16 as ::core::ffi::c_int & 0xff as Unicode) as u32,
                    ),
                    bk_int(
                        BkCellType::B8,
                        (selector_0 >> 8 as ::core::ffi::c_int & 0xff as Unicode) as u32,
                    ),
                    bk_int(
                        BkCellType::B8,
                        (selector_0 >> 0 as ::core::ffi::c_int & 0xff as Unicode) as u32,
                    ),
                    bk_ptr(BkCellType::P32, bk_new_block_from_buffer(dflt)),
                    bk_ptr(BkCellType::P32, bk_new_block_from_buffer(nondflt)),
                ],
            );
        }
        selector_0 = selector_0.wrapping_add(1);
    }
    let buf: *mut Buffer = bk_build_block(st);
    bufseek(buf, 2 as usize);
    bufwrite32b(buf, buflen(buf) as u32);
    return buf;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_cmap(cmap: Option<&CmapTable>, options: &Options) -> *mut Buffer {
    let cmap = match cmap {
        Some(c) if !c.unicodes.is_empty() => c as *const CmapTable,
        _ => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut requires_format12: bool = false;
    let has_uvs: bool = !(*cmap).uvs.is_empty();
    for (&unicode, _) in (*cmap).unicodes.iter() {
        if unicode > 0xffff as ::core::ffi::c_int {
            requires_format12 = true;
        }
    }
    let mut format4: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
    if !requires_format12 || !options.stub_cmap4 {
        format4 = otfcc_try_build_cmap_format4(cmap);
        if format4.is_null() {
            requires_format12 = true;
        }
    }
    let mut n_tables: u8 = (if requires_format12 as ::core::ffi::c_int != 0 {
        4 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as u8;
    if has_uvs {
        n_tables = (n_tables as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u8;
    }
    if format4.is_null() {
        format4 = bufnew();
        bufwrite16b(format4, 4 as u16);
        bufwrite16b(format4, 32 as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 4 as u16);
        bufwrite16b(format4, 4 as u16);
        bufwrite16b(format4, 1 as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 0xffff as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 0xffff as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 1 as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 0 as u16);
    }
    let format12: *mut Buffer = otfcc_build_cmap_format12(cmap);
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B16, 0 as u32),
        bk_int(BkCellType::B16, (n_tables as ::core::ffi::c_int) as u32),
    ]);
    bk_push(
        root,
        &[
            bk_int(BkCellType::B16, 0 as u32),
            bk_int(BkCellType::B16, 3 as u32),
            bk_ptr(BkCellType::P32, bk_new_block_from_buffer_copy(format4)),
        ],
    );
    if requires_format12 {
        bk_push(
            root,
            &[
                bk_int(BkCellType::B16, 0 as u32),
                bk_int(BkCellType::B16, 4 as u32),
                bk_ptr(BkCellType::P32, bk_new_block_from_buffer_copy(format12)),
            ],
        );
    }
    if has_uvs {
        let format14: *mut Buffer = otfcc_build_cmap_format14(cmap);
        bk_push(
            root,
            &[
                bk_int(BkCellType::B16, 0 as u32),
                bk_int(BkCellType::B16, 5 as u32),
                bk_ptr(BkCellType::P32, bk_new_block_from_buffer(format14)),
            ],
        );
    }
    bk_push(
        root,
        &[
            bk_int(BkCellType::B16, 3 as u32),
            bk_int(BkCellType::B16, 1 as u32),
            bk_ptr(BkCellType::P32, bk_new_block_from_buffer_copy(format4)),
        ],
    );
    if requires_format12 {
        bk_push(
            root,
            &[
                bk_int(BkCellType::B16, 3 as u32),
                bk_int(BkCellType::B16, 10 as u32),
                bk_ptr(BkCellType::P32, bk_new_block_from_buffer_copy(format12)),
            ],
        );
    }
    buffree(format4);
    buffree(format12);
    return bk_build_block(root);
}

#[cfg(test)]
mod cmap_read_tests {
    use super::*;

    fn empty_cmap() -> Box<CmapTable> {
        Box::new(CmapTable {
            unicodes: std::collections::BTreeMap::new(),
            uvs: std::collections::BTreeMap::new(),
        })
    }

    #[test]
    fn format4_direct_delta_segment_maps_one_codepoint() {
        let mut data = Vec::new();
        data.extend_from_slice(&4u16.to_be_bytes()); // format
        data.extend_from_slice(&32u16.to_be_bytes()); // length (informational)
        data.extend_from_slice(&0u16.to_be_bytes()); // language
        data.extend_from_slice(&4u16.to_be_bytes()); // segCountX2 (2 segments)
        data.extend_from_slice(&[0u8; 6]); // searchRange, entrySelector, rangeShift
        data.extend_from_slice(&0x0041u16.to_be_bytes()); // endCode[0]
        data.extend_from_slice(&0xFFFFu16.to_be_bytes()); // endCode[1]
        data.extend_from_slice(&0u16.to_be_bytes()); // reservedPad
        data.extend_from_slice(&0x0041u16.to_be_bytes()); // startCode[0]
        data.extend_from_slice(&0xFFFFu16.to_be_bytes()); // startCode[1]
        data.extend_from_slice(&((5i32 - 0x41i32) as i16).to_be_bytes()); // idDelta[0]
        data.extend_from_slice(&1i16.to_be_bytes()); // idDelta[1]
        data.extend_from_slice(&0u16.to_be_bytes()); // idRangeOffset[0]
        data.extend_from_slice(&0u16.to_be_bytes()); // idRangeOffset[1]
        assert_eq!(data.len(), 32);

        let mut cmap = empty_cmap();
        unsafe { read_format4(&data, 0, cmap.as_mut() as *mut CmapTable) };
        assert_eq!(cmap.unicodes.get(&0x41).unwrap().index, 5);
        assert!(!cmap.unicodes.contains_key(&0xFFFF));
    }

    #[test]
    fn format4_indirect_segment_follows_id_range_offset_into_the_glyph_array() {
        let mut data = Vec::new();
        data.extend_from_slice(&4u16.to_be_bytes());
        data.extend_from_slice(&36u16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&4u16.to_be_bytes()); // 2 segments
        data.extend_from_slice(&[0u8; 6]);
        data.extend_from_slice(&0x0042u16.to_be_bytes()); // endCode[0]
        data.extend_from_slice(&0xFFFFu16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes()); // reservedPad
        data.extend_from_slice(&0x0041u16.to_be_bytes()); // startCode[0]
        data.extend_from_slice(&0xFFFFu16.to_be_bytes());
        data.extend_from_slice(&0i16.to_be_bytes()); // idDelta[0]
        data.extend_from_slice(&1i16.to_be_bytes());
        data.extend_from_slice(&4u16.to_be_bytes()); // idRangeOffset[0]: 4 bytes ahead of its own entry
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&7u16.to_be_bytes()); // glyphIdArray[0] (for 0x41)
        data.extend_from_slice(&8u16.to_be_bytes()); // glyphIdArray[1] (for 0x42)
        assert_eq!(data.len(), 36);

        let mut cmap = empty_cmap();
        unsafe { read_format4(&data, 0, cmap.as_mut() as *mut CmapTable) };
        assert_eq!(cmap.unicodes.get(&0x41).unwrap().index, 7);
        assert_eq!(cmap.unicodes.get(&0x42).unwrap().index, 8);
    }

    #[test]
    fn format12_direct_group_maps_a_range() {
        let mut data = Vec::new();
        data.extend_from_slice(&12u16.to_be_bytes()); // format
        data.extend_from_slice(&0u16.to_be_bytes()); // reserved
        data.extend_from_slice(&28u32.to_be_bytes()); // length (informational)
        data.extend_from_slice(&0u32.to_be_bytes()); // language
        data.extend_from_slice(&1u32.to_be_bytes()); // nGroups
        data.extend_from_slice(&0x1F600u32.to_be_bytes()); // startCharCode
        data.extend_from_slice(&0x1F601u32.to_be_bytes()); // endCharCode
        data.extend_from_slice(&10u32.to_be_bytes()); // startGlyphID
        assert_eq!(data.len(), 28);

        let mut cmap = empty_cmap();
        unsafe { read_format12(&data, 0, cmap.as_mut() as *mut CmapTable) };
        assert_eq!(cmap.unicodes.get(&0x1F600).unwrap().index, 10);
        assert_eq!(cmap.unicodes.get(&0x1F601).unwrap().index, 11);
    }

    #[test]
    fn format12_n_groups_large_enough_to_overflow_the_multiplication_is_a_noop() {
        // Was `length_limit < 16.wrapping_add(12.wrapping_mul(n_groups))`:
        // an n_groups this large wraps `12 * n_groups` back down to a small
        // number, so the guard passed even though the real group array is
        // nowhere near that short, and the loop then read groups straight
        // past this 16-byte buffer's end. `require_room`'s `checked_mul`
        // must reject it instead.
        let mut data = Vec::new();
        data.extend_from_slice(&12u16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&16u32.to_be_bytes());
        data.extend_from_slice(&0u32.to_be_bytes());
        data.extend_from_slice(&0x1555_5556u32.to_be_bytes()); // nGroups
        assert_eq!(data.len(), 16);

        let mut cmap = empty_cmap();
        unsafe { read_format12(&data, 0, cmap.as_mut() as *mut CmapTable) };
        assert!(cmap.unicodes.is_empty());
    }

    #[test]
    fn format12_group_range_past_the_unicode_ceiling_is_clamped_not_walked_in_full() {
        // A single 12-byte group can claim an endCharCode anywhere in
        // the full u32 range: `0xFFFFFFFF` makes the walk a genuine
        // infinite loop (`c.wrapping_add(1)` wraps `0xFFFFFFFF` back to
        // `0`, so `c <= end_code` is true forever), and anything merely
        // huge (a fuzz-found font used `0xffff0000`) still forces
        // billions of iterations from those same 12 bytes. Real
        // codepoints never exceed the Unicode ceiling, so clamping the
        // walked range to it must both terminate promptly and still map
        // every codepoint up to (and including) the ceiling correctly.
        let mut data = Vec::new();
        data.extend_from_slice(&12u16.to_be_bytes()); // format
        data.extend_from_slice(&0u16.to_be_bytes()); // reserved
        data.extend_from_slice(&28u32.to_be_bytes()); // length (informational)
        data.extend_from_slice(&0u32.to_be_bytes()); // language
        data.extend_from_slice(&1u32.to_be_bytes()); // nGroups
        data.extend_from_slice(&0x10fffeu32.to_be_bytes()); // startCharCode: just below the ceiling
        data.extend_from_slice(&0xffffffffu32.to_be_bytes()); // endCharCode: the u32 max
        data.extend_from_slice(&10u32.to_be_bytes()); // startGlyphID
        assert_eq!(data.len(), 28);

        let mut cmap = empty_cmap();
        let start = std::time::Instant::now();
        unsafe { read_format12(&data, 0, cmap.as_mut() as *mut CmapTable) };
        assert!(start.elapsed() < std::time::Duration::from_secs(5));

        assert_eq!(cmap.unicodes.get(&0x10fffe).unwrap().index, 10);
        assert_eq!(cmap.unicodes.get(&0x10ffff).unwrap().index, 11);
        assert!(!cmap.unicodes.contains_key(&0x110000));
    }

    #[test]
    fn uvs_default_num_ranges_overflow_is_a_noop_not_oob() {
        let mut data = Vec::new();
        data.extend_from_slice(&0x4000_0001u32.to_be_bytes()); // numUnicodeValueRanges
        assert_eq!(data.len(), 4);

        let mut cmap = empty_cmap();
        unsafe { read_uvs_default(&data, 0, 0xFE00, cmap.as_mut() as *mut CmapTable) };
        assert!(cmap.uvs.is_empty());
    }

    #[test]
    fn uvs_non_default_num_mappings_overflow_is_a_noop_not_oob() {
        let mut data = Vec::new();
        data.extend_from_slice(&0x3333_3334u32.to_be_bytes()); // numUVSMappings
        assert_eq!(data.len(), 4);

        let mut cmap = empty_cmap();
        unsafe { read_uvs_non_default(&data, 0, 0xFE00, cmap.as_mut() as *mut CmapTable) };
        assert!(cmap.uvs.is_empty());
    }

    #[test]
    fn uvs_non_default_well_formed_registers_a_mapping() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u32.to_be_bytes()); // numUVSMappings
        data.extend_from_slice(&[0x00, 0x00, 0x41]); // unicodeValue (24-bit): 0x41
        data.extend_from_slice(&9u16.to_be_bytes()); // glyphID
        assert_eq!(data.len(), 9);

        let mut cmap = empty_cmap();
        unsafe { read_uvs_non_default(&data, 0, 0xFE00, cmap.as_mut() as *mut CmapTable) };
        let g = cmap
            .uvs
            .get(&CmapUvsKey {
                unicode: 0x41,
                selector: 0xFE00,
            })
            .unwrap();
        assert_eq!(g.index, 9);
    }

    #[test]
    fn format14_n_groups_overflow_is_a_noop_not_oob() {
        let mut data = Vec::new();
        data.extend_from_slice(&14u16.to_be_bytes()); // format
        data.extend_from_slice(&0u32.to_be_bytes()); // length
        data.extend_from_slice(&0x1999_999Au32.to_be_bytes()); // numVarSelectorRecords
        assert_eq!(data.len(), 10);

        let mut cmap = empty_cmap();
        unsafe { read_format14(&data, 0, cmap.as_mut() as *mut CmapTable) };
        assert!(cmap.uvs.is_empty());
    }

    #[test]
    fn format14_default_uvs_offset_past_the_table_end_is_a_noop_not_oob() {
        // The original computed the recursive call's remaining-length
        // budget as `length_limit.wrapping_sub(default_uvs_offset)` --
        // an `default_uvs_offset` larger than the whole table's own
        // length wrapped that subtraction into a huge number, so
        // `read_uvs_default`'s own guards, now looking at a bogus giant
        // budget, no longer protected anything. This reader has no
        // separate length_limit to underflow at all: the recursive call's
        // own `FontReader::at` rejects the out-of-range offset directly.
        let mut data = Vec::new();
        data.extend_from_slice(&14u16.to_be_bytes());
        data.extend_from_slice(&0u32.to_be_bytes());
        data.extend_from_slice(&1u32.to_be_bytes()); // numVarSelectorRecords
        data.extend_from_slice(&[0x00, 0xFE, 0x00]); // varSelector (24-bit): 0xFE00
        data.extend_from_slice(&0xFFFF_FFF0u32.to_be_bytes()); // defaultUVSOffset: far past the table
        data.extend_from_slice(&0u32.to_be_bytes()); // nonDefaultUVSOffset: absent
        assert_eq!(data.len(), 21);

        let mut cmap = empty_cmap();
        unsafe { read_format14(&data, 0, cmap.as_mut() as *mut CmapTable) };
        assert!(cmap.uvs.is_empty());
    }

    #[test]
    fn parse_cmap_directory_entry_pointing_past_the_table_end_skips_just_that_subtable() {
        // The headline bug this file's migration exists to fix: the
        // original derived each subtable's remaining-length budget as
        // `length.wrapping_sub(table_offset)`, so a `table_offset` larger
        // than the table's own length wrapped that subtraction into a
        // huge number, defeating every downstream guard in whichever
        // format reader ran next. `parse_cmap` never computes that
        // subtraction at all -- it just hands the format readers the same
        // `data` slice and the (unvalidated) absolute `table_offset`, and
        // each one's own `FontReader::at` rejects an out-of-range offset
        // on contact.
        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_be_bytes()); // version
        data.extend_from_slice(&1u16.to_be_bytes()); // numTables
        data.extend_from_slice(&3u16.to_be_bytes()); // platformID (Windows)
        data.extend_from_slice(&1u16.to_be_bytes()); // encodingID (Unicode BMP)
        data.extend_from_slice(&0xFFFF_FFF0u32.to_be_bytes()); // offset: far past the table
        assert_eq!(data.len(), 12);

        let cmap = unsafe { parse_cmap(&data).unwrap() };
        assert!(cmap.unicodes.is_empty());
    }

    #[test]
    fn parse_cmap_directory_shorter_than_declared_num_tables_errs() {
        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&5u16.to_be_bytes()); // numTables, but no entries follow
        assert!(unsafe { parse_cmap(&data) }.is_err());
    }

    #[test]
    fn parse_cmap_end_to_end_finds_the_format4_subtable() {
        let mut subtable = Vec::new();
        subtable.extend_from_slice(&4u16.to_be_bytes());
        subtable.extend_from_slice(&32u16.to_be_bytes());
        subtable.extend_from_slice(&0u16.to_be_bytes());
        subtable.extend_from_slice(&4u16.to_be_bytes());
        subtable.extend_from_slice(&[0u8; 6]);
        subtable.extend_from_slice(&0x0041u16.to_be_bytes());
        subtable.extend_from_slice(&0xFFFFu16.to_be_bytes());
        subtable.extend_from_slice(&0u16.to_be_bytes());
        subtable.extend_from_slice(&0x0041u16.to_be_bytes());
        subtable.extend_from_slice(&0xFFFFu16.to_be_bytes());
        subtable.extend_from_slice(&((5i32 - 0x41i32) as i16).to_be_bytes());
        subtable.extend_from_slice(&1i16.to_be_bytes());
        subtable.extend_from_slice(&0u16.to_be_bytes());
        subtable.extend_from_slice(&0u16.to_be_bytes());
        assert_eq!(subtable.len(), 32);

        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_be_bytes()); // version
        data.extend_from_slice(&1u16.to_be_bytes()); // numTables
        data.extend_from_slice(&3u16.to_be_bytes()); // platformID
        data.extend_from_slice(&1u16.to_be_bytes()); // encodingID
        data.extend_from_slice(&12u32.to_be_bytes()); // offset: right after the 12-byte directory
        data.extend_from_slice(&subtable);

        let cmap = unsafe { parse_cmap(&data).unwrap() };
        assert_eq!(cmap.unicodes.get(&0x41).unwrap().index, 5);
    }

    #[test]
    #[cfg_attr(
        miri,
        ignore = "timing-based; 400,000 codepoint inserts is far too slow to run meaningfully under Miri's interpreter"
    )]
    fn parse_cmap_duplicate_directory_entries_aliasing_the_same_subtable_are_only_parsed_once() {
        // The cmap spec puts no requirement on encoding-record subtable
        // offsets being distinct -- real fonts legitimately have several
        // records point at the very same subtable. Without a dedup,
        // `parse_cmap` re-parses (and re-walks every codepoint of) the
        // aliased subtable once per record pointing at it, an
        // amplification whose multiplier is `numTables` itself, bounded
        // only by the cmap table's own size divided by 8 bytes per
        // record -- not by any fixed cap. This builds 200 duplicate,
        // individually-valid directory entries all pointing at one
        // format 12 subtable large enough that re-walking it 200 times
        // (undeduped) takes seconds, while parsing it once (deduped)
        // is near-instant -- proving the redundant records are actually
        // skipped, not just that the end result happens to be correct
        // (parsing the same bytes twice is idempotent either way).
        const NUM_ENTRIES: usize = 200;
        const NUM_GROUPS: u32 = 200;
        const GROUP_SPAN: u32 = 2000; // codepoints per group

        let mut subtable = Vec::new();
        subtable.extend_from_slice(&12u16.to_be_bytes()); // format
        subtable.extend_from_slice(&0u16.to_be_bytes()); // reserved
        subtable.extend_from_slice(&0u32.to_be_bytes()); // length (informational)
        subtable.extend_from_slice(&0u32.to_be_bytes()); // language
        subtable.extend_from_slice(&NUM_GROUPS.to_be_bytes());
        for g in 0..NUM_GROUPS {
            let start = g * GROUP_SPAN;
            let end = start + GROUP_SPAN - 1;
            subtable.extend_from_slice(&start.to_be_bytes());
            subtable.extend_from_slice(&end.to_be_bytes());
            subtable.extend_from_slice(&1u32.to_be_bytes()); // startGlyphID
        }

        let directory_len = 4 + NUM_ENTRIES * 8;
        let subtable_offset = directory_len as u32;

        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_be_bytes()); // version
        data.extend_from_slice(&(NUM_ENTRIES as u16).to_be_bytes()); // numTables
        for _ in 0..NUM_ENTRIES {
            data.extend_from_slice(&3u16.to_be_bytes()); // platformID (Windows)
            data.extend_from_slice(&10u16.to_be_bytes()); // encodingID (Unicode full repertoire)
            data.extend_from_slice(&subtable_offset.to_be_bytes());
        }
        data.extend_from_slice(&subtable);
        assert_eq!(data.len(), directory_len + subtable.len());

        let start = std::time::Instant::now();
        let cmap = unsafe { parse_cmap(&data).unwrap() };
        let elapsed = start.elapsed();

        assert_eq!(cmap.unicodes.len() as u32, NUM_GROUPS * GROUP_SPAN);
        assert_eq!(cmap.unicodes.get(&0).unwrap().index, 1);
        // The last codepoint of the last group: startGlyphID is 1 in
        // every group, so a codepoint's gid is `1 + its offset within
        // its own group` -- the last codepoint of any GROUP_SPAN-wide
        // group is GROUP_SPAN itself, not 1.
        assert_eq!(
            cmap.unicodes
                .get(&((NUM_GROUPS * GROUP_SPAN - 1) as ::core::ffi::c_int))
                .unwrap()
                .index,
            GROUP_SPAN as u16
        );
        assert!(
            elapsed < std::time::Duration::from_secs(2),
            "parsing {NUM_ENTRIES} aliasing directory entries took {elapsed:?} -- \
             looks like the same subtable is being re-parsed per entry again"
        );
    }
}

