#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, strtol};

use crate::support::parsed_json::{ParsedValue, json_obj_get_type, json_obj_key_at, json_obj_key_len_at, json_obj_len, json_obj_val_at, json_str_len, json_str_ptr, json_type_of};
use crate::support::handle::{handle_from_index, handle_from_name, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_8u, read_16u, read_24u, read_32u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, TableId, Unicode};
use crate::vendor::sds::{Hex4Upper, SdsRaw};
use crate::vendor::json::{JsonType};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::{NULL};
use crate::bk::bkblock::{bk_new_block_from_buffer, bk_new_block_from_buffer_copy};
use crate::bk::bkgraph::{bk_build_block};
use crate::support::buffer::{buffree, buflen, bufnew, bufseek, bufwrite16b, bufwrite24b, bufwrite32b, bufwrite8, bufwrite_buf};
use crate::support::built_json::{BuiltValue, json_object_new, json_object_push, json_string_new_from_bytes};
use crate::vendor::sds::{sdsempty, sdsfree, sdsfromlonglong, sdslen, sdsnewlen};
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
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
#[repr(C)]
pub struct CmapTable {
    pub unicodes: std::collections::BTreeMap<::core::ffi::c_int, GlyphHandle>,
    pub uvs: std::collections::BTreeMap<CmapUvsKey, GlyphHandle>,
}
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub unsafe extern "C" fn otfcc_encode_cmap_by_index(
    mut cmap: *mut CmapTable,
    mut c: ::core::ffi::c_int,
    mut gid: u16,
) -> bool {
    match (*cmap).unicodes.entry(c) {
        std::collections::btree_map::Entry::Vacant(v) => {
            v.insert(handle_from_index(gid as GlyphId) as GlyphHandle);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}
pub unsafe extern "C" fn otfcc_encode_cmap_by_name(
    mut cmap: *mut CmapTable,
    mut c: ::core::ffi::c_int,
    mut name: SdsRaw,
) -> bool {
    match (*cmap).unicodes.entry(c) {
        std::collections::btree_map::Entry::Vacant(v) => {
            v.insert(handle_from_name(name) as GlyphHandle);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}
pub unsafe extern "C" fn otfcc_unmap_cmap(
    mut cmap: *mut CmapTable,
    mut c: ::core::ffi::c_int,
) -> bool {
    // Removing the entry drops its `GlyphHandle` (freeing the glyph
    // name), replacing the explicit `otfcc_handle_dispose` + manual
    // node walk this walk used to do.
    (*cmap).unicodes.remove(&c).is_some()
}
pub unsafe extern "C" fn otfcc_cmap_lookup(
    mut cmap: *const CmapTable,
    mut c: ::core::ffi::c_int,
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
pub unsafe extern "C" fn otfcc_encode_cmap_uvs_by_index(
    mut cmap: *mut CmapTable,
    mut c: CmapUvsKey,
    mut gid: u16,
) -> bool {
    match (*cmap).uvs.entry(c) {
        std::collections::btree_map::Entry::Vacant(v) => {
            v.insert(handle_from_index(gid as GlyphId) as GlyphHandle);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}
pub unsafe extern "C" fn otfcc_encode_cmap_uvs_by_name(
    mut cmap: *mut CmapTable,
    mut c: CmapUvsKey,
    mut name: SdsRaw,
) -> bool {
    match (*cmap).uvs.entry(c) {
        std::collections::btree_map::Entry::Vacant(v) => {
            v.insert(handle_from_name(name) as GlyphHandle);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}
pub unsafe extern "C" fn otfcc_unmap_cmap_uvs(
    mut cmap: *mut CmapTable,
    mut c: CmapUvsKey,
) -> bool {
    (*cmap).uvs.remove(&c).is_some()
}
pub unsafe extern "C" fn otfcc_cmap_lookup_uvs(
    mut cmap: *const CmapTable,
    mut c: CmapUvsKey,
) -> *mut GlyphHandle {
    match (*cmap).uvs.get(&c) {
        Some(glyph) => glyph as *const GlyphHandle as *mut GlyphHandle,
        None => ::core::ptr::null_mut::<GlyphHandle>(),
    }
}
unsafe extern "C" fn read_format12(
    mut start: FontFilePointer,
    mut length_limit: u32,
    mut cmap: *mut CmapTable,
) {
    if length_limit < 16 as u32 {
        return;
    }
    let mut n_groups: u32 =
        read_32u(start.offset(12 as ::core::ffi::c_int as isize) as *const u8);
    if length_limit < (16 as u32).wrapping_add((12 as u32).wrapping_mul(n_groups)) {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < n_groups {
        let mut start_code: u32 = read_32u(
            start
                .offset(16 as ::core::ffi::c_int as isize)
                .offset((12 as u32).wrapping_mul(j) as isize) as *const u8,
        );
        let mut end_code: u32 = read_32u(
            start
                .offset(16 as ::core::ffi::c_int as isize)
                .offset((12 as u32).wrapping_mul(j) as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        );
        let mut start_gid: u32 = read_32u(
            start
                .offset(16 as ::core::ffi::c_int as isize)
                .offset((12 as u32).wrapping_mul(j) as isize)
                .offset(8 as ::core::ffi::c_int as isize) as *const u8,
        );
        let mut c: u32 = start_code;
        while c <= end_code {
            otfcc_encode_cmap_by_index(
                cmap,
                c as ::core::ffi::c_int,
                c.wrapping_sub(start_code).wrapping_add(start_gid) as u16,
            );
            c = c.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn read_format4(
    mut start: FontFilePointer,
    mut length_limit: u32,
    mut cmap: *mut CmapTable,
) {
    if length_limit < 14 as u32 {
        return;
    }
    let mut segments_count: u16 =
        (read_16u(start.offset(6 as ::core::ffi::c_int as isize) as *const u8)
            as ::core::ffi::c_int
            / 2 as ::core::ffi::c_int) as u16;
    if length_limit
        < (16 as ::core::ffi::c_int + segments_count as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as u32
    {
        return;
    }
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < segments_count as ::core::ffi::c_int {
        let mut end_code: u16 = read_16u(
            start
                .offset(14 as ::core::ffi::c_int as isize)
                .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                as *const u8,
        );
        let mut start_code: u16 = read_16u(
            start
                .offset(14 as ::core::ffi::c_int as isize)
                .offset((segments_count as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                .offset(2 as ::core::ffi::c_int as isize)
                .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                as *const u8,
        );
        let mut id_delta: i16 = read_16u(
            start
                .offset(14 as ::core::ffi::c_int as isize)
                .offset((segments_count as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                .offset(2 as ::core::ffi::c_int as isize)
                .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                as *const u8,
        ) as i16;
        let mut id_range_offset_offset: u32 = (14 as ::core::ffi::c_int
            + segments_count as ::core::ffi::c_int * 6 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + j as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            as u32;
        let mut id_range_offset: u16 =
            read_16u(start.offset(id_range_offset_offset as isize) as *const u8);
        if id_range_offset as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut c: u32 = start_code as u32;
            while c < 0xffff as u32 && c <= end_code as u32 {
                let mut gid: u16 =
                    (c.wrapping_add(id_delta as u32) & 0xffff as u32) as u16;
                otfcc_encode_cmap_by_index(cmap, c as ::core::ffi::c_int, gid);
                c = c.wrapping_add(1);
            }
        } else {
            let mut c_0: u32 = start_code as u32;
            while c_0 < 0xffff as u32 && c_0 <= end_code as u32 {
                let mut glyph_offset: u32 = (id_range_offset as u32)
                    .wrapping_add(
                        c_0.wrapping_sub(start_code as u32)
                            .wrapping_mul(2 as u32),
                    )
                    .wrapping_add(id_range_offset_offset);
                if !(glyph_offset.wrapping_add(2 as u32) > length_limit) {
                    let mut gid_0: u16 =
                        (read_16u(start.offset(glyph_offset as isize) as *const u8)
                            as ::core::ffi::c_int
                            + id_delta as ::core::ffi::c_int
                            & 0xffff as ::core::ffi::c_int) as u16;
                    otfcc_encode_cmap_by_index(cmap, c_0 as ::core::ffi::c_int, gid_0);
                }
                c_0 = c_0.wrapping_add(1);
            }
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn read_uvs_default(
    mut start: FontFilePointer,
    mut length_limit: u32,
    mut selector: Unicode,
    mut cmap: *mut CmapTable,
) {
    if length_limit < 4 as u32 {
        return;
    }
    let mut num_unicode_value_ranges: u32 = read_32u(start as *const u8);
    if length_limit
        < (4 as u32).wrapping_add((4 as u32).wrapping_mul(num_unicode_value_ranges))
    {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < num_unicode_value_ranges {
        let mut vsr: FontFilePointer = start
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((4 as u32).wrapping_mul(j) as isize);
        let mut start_unicode_value: Unicode = read_24u(vsr as *const u8) as Unicode;
        let mut additional_count: u8 =
            read_8u(vsr.offset(3 as ::core::ffi::c_int as isize) as *const u8);
        let mut u: Unicode = start_unicode_value;
        while u <= start_unicode_value.wrapping_add(additional_count as Unicode) {
            let mut g: *mut GlyphHandle = otfcc_cmap_lookup(cmap, u as ::core::ffi::c_int);
            if !g.is_null() {
                otfcc_encode_cmap_uvs_by_index(
                    cmap,
                    CmapUvsKey {
                        unicode: u as u32,
                        selector: selector as u32,
                    },
                    (*g).index as u16,
                );
            }
            u = u.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn read_uvs_non_default(
    mut start: FontFilePointer,
    mut length_limit: u32,
    mut selector: Unicode,
    mut cmap: *mut CmapTable,
) {
    if length_limit < 4 as u32 {
        return;
    }
    let mut num_uvs_mappings: u32 = read_32u(start as *const u8);
    if length_limit < (4 as u32).wrapping_add((5 as u32).wrapping_mul(num_uvs_mappings)) {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < num_uvs_mappings {
        let mut vsr: FontFilePointer = start
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((5 as u32).wrapping_mul(j) as isize);
        let mut unicode_value: Unicode = read_24u(vsr as *const u8) as Unicode;
        let mut glyph_id: GlyphId =
            read_16u(vsr.offset(3 as ::core::ffi::c_int as isize) as *const u8) as GlyphId;
        otfcc_encode_cmap_uvs_by_index(
            cmap,
            CmapUvsKey {
                unicode: unicode_value as u32,
                selector: selector as u32,
            },
            glyph_id as u16,
        );
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn read_format14(
    mut start: FontFilePointer,
    mut length_limit: u32,
    mut cmap: *mut CmapTable,
) {
    if length_limit < 10 as u32 {
        return;
    }
    let mut n_groups: u32 =
        read_32u(start.offset(6 as ::core::ffi::c_int as isize) as *const u8);
    if length_limit < (11 as u32).wrapping_add((11 as u32).wrapping_mul(n_groups)) {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < n_groups {
        let mut vsr: FontFilePointer = start
            .offset(10 as ::core::ffi::c_int as isize)
            .offset((11 as u32).wrapping_mul(j) as isize);
        let mut selector: Unicode = read_24u(vsr as *const u8) as Unicode;
        let mut default_uvs_offset: u32 =
            read_32u(vsr.offset(3 as ::core::ffi::c_int as isize) as *const u8);
        let mut non_default_uvs_offset: u32 =
            read_32u(vsr.offset(7 as ::core::ffi::c_int as isize) as *const u8);
        if default_uvs_offset != 0 {
            read_uvs_default(
                start.offset(default_uvs_offset as isize),
                length_limit.wrapping_sub(default_uvs_offset),
                selector,
                cmap,
            );
        }
        if non_default_uvs_offset != 0 {
            read_uvs_non_default(
                start.offset(non_default_uvs_offset as isize),
                length_limit.wrapping_sub(non_default_uvs_offset),
                selector,
                cmap,
            );
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn read_cmap_mapping_table(
    mut start: FontFilePointer,
    mut length_limit: u32,
    mut cmap: *mut CmapTable,
    mut required_format: TableId,
) {
    let mut format: u16 = read_16u(start as *const u8);
    if format as ::core::ffi::c_int == required_format as ::core::ffi::c_int {
        if format as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            read_format4(start, length_limit, cmap);
        } else if format as ::core::ffi::c_int == 12 as ::core::ffi::c_int {
            read_format12(start, length_limit, cmap);
        }
    }
}
unsafe extern "C" fn read_cmap_mapping_table_uvs(
    mut start: FontFilePointer,
    mut length_limit: u32,
    mut cmap: *mut CmapTable,
) {
    let mut format: u16 = read_16u(start as *const u8);
    if format as ::core::ffi::c_int == 14 as ::core::ffi::c_int {
        read_format14(start, length_limit, cmap);
    }
}
#[inline]
unsafe extern "C" fn is_valid_cmap_encoding(mut platform: u16, mut encoding: u16) -> bool {
    return platform as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && encoding as ::core::ffi::c_int == 3 as ::core::ffi::c_int
        || platform as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && encoding as ::core::ffi::c_int == 4 as ::core::ffi::c_int
        || platform as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && encoding as ::core::ffi::c_int == 5 as ::core::ffi::c_int
        || platform as ::core::ffi::c_int == 3 as ::core::ffi::c_int
            && encoding as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        || platform as ::core::ffi::c_int == 3 as ::core::ffi::c_int
            && encoding as ::core::ffi::c_int == 10 as ::core::ffi::c_int;
}
pub static FORMAT_PRIORITIES: [TableId; 3] = [
    12 as ::core::ffi::c_int as TableId,
    4 as ::core::ffi::c_int as TableId,
    0 as ::core::ffi::c_int as TableId,
];
pub unsafe extern "C" fn otfcc_read_cmap(
    packet: Packet,
    mut options: *const Options,
) -> Option<Box<CmapTable>> {
    let mut num_tables: u16 = 0;
    let mut cmap_box: Option<Box<CmapTable>> = None;
    let mut cmap: *mut CmapTable = ::core::ptr::null_mut::<CmapTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_CMAP {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if !(length < 4 as u32) {
                        cmap_box = Some(Box::new(CmapTable {
                            unicodes: std::collections::BTreeMap::new(),
                            uvs: std::collections::BTreeMap::new(),
                        }));
                        cmap = cmap_box.as_deref_mut().unwrap() as *mut CmapTable;
                        num_tables = read_16u(
                            data.offset(2 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if !(length
                            < (4 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * num_tables as ::core::ffi::c_int)
                                as u32)
                        {
                            let mut k_subtable_type: usize = 0 as usize;
                            while FORMAT_PRIORITIES[k_subtable_type] != 0 {
                                let mut j: u16 = 0 as u16;
                                while (j as ::core::ffi::c_int) < num_tables as ::core::ffi::c_int {
                                    let mut platform: u16 = read_16u(
                                        data.offset(4 as ::core::ffi::c_int as isize).offset(
                                            (8 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                    );
                                    let mut encoding: u16 = read_16u(
                                        data.offset(4 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (8 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            as *const u8,
                                    );
                                    if is_valid_cmap_encoding(platform, encoding) {
                                        let mut table_offset: u32 = read_32u(
                                            data.offset(4 as ::core::ffi::c_int as isize)
                                                .offset(
                                                    (8 as ::core::ffi::c_int
                                                        * j as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                .offset(4 as ::core::ffi::c_int as isize)
                                                as *const u8,
                                        );
                                        read_cmap_mapping_table(
                                            data.offset(table_offset as isize),
                                            length.wrapping_sub(table_offset),
                                            cmap,
                                            FORMAT_PRIORITIES[k_subtable_type],
                                        );
                                    }
                                    j = j.wrapping_add(1);
                                }
                                k_subtable_type = k_subtable_type.wrapping_add(1);
                            }
                            let mut j_0: u16 = 0 as u16;
                            while (j_0 as ::core::ffi::c_int) < num_tables as ::core::ffi::c_int {
                                let mut platform_0: u16 = read_16u(
                                    data.offset(4 as ::core::ffi::c_int as isize).offset(
                                        (8 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                                );
                                let mut encoding_0: u16 = read_16u(
                                    data.offset(4 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (8 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                if is_valid_cmap_encoding(platform_0, encoding_0) {
                                    let mut table_offset_0: u32 = read_32u(
                                        data.offset(4 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (8 as ::core::ffi::c_int
                                                    * j_0 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            .offset(4 as ::core::ffi::c_int as isize)
                                            as *const u8,
                                    );
                                    read_cmap_mapping_table_uvs(
                                        data.offset(table_offset_0 as isize),
                                        length.wrapping_sub(table_offset_0),
                                        cmap,
                                    );
                                }
                                j_0 = j_0.wrapping_add(1);
                            }
                            return cmap_box;
                        }
                    }
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(b"table 'cmap' corrupted.\n"),
                    );
                    cmap_box = None;
                    cmap = ::core::ptr::null_mut::<CmapTable>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return None;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_dump_cmap(
    table: Option<&CmapTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
) {
    let table = match table {
        Some(t) => t as *const CmapTable,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::bytesbuild!(b"cmap"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        if !(*table).unicodes.is_empty() {
            let mut cmap: *mut BuiltValue = json_object_new((*table).unicodes.len());
            for (&unicode, glyph) in (*table).unicodes.iter() {
                if !glyph.name.is_empty() {
                    let mut key: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    if (*options).decimal_cmap {
                        key = sdsfromlonglong(unicode as ::core::ffi::c_longlong);
                    } else {
                        key = crate::sdsbuild!(sdsempty(), b"U+", Hex4Upper(unicode as u32));
                    }
                    json_object_push(
                        cmap,
                        key as *const ::core::ffi::c_char,
                        json_string_new_from_bytes(&glyph.name),
                    );
                    sdsfree(key);
                }
            }
            json_object_push(
                root,
                b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
                cmap,
            );
        }
        if !(*table).uvs.is_empty() {
            let mut uvs: *mut BuiltValue = json_object_new((*table).uvs.len());
            for (key, glyph) in (*table).uvs.iter() {
                if !glyph.name.is_empty() {
                    let mut key_0: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    if (*options).decimal_cmap {
                        key_0 = crate::sdsbuild!(
                            sdsempty(),
                            key.unicode,
                            b" ",
                            key.selector,
                        );
                    } else {
                        key_0 = crate::sdsbuild!(
                            sdsempty(),
                            b"U+",
                            Hex4Upper(key.unicode as u32),
                            b" U+",
                            Hex4Upper(key.selector as u32),
                        );
                    }
                    json_object_push(
                        uvs,
                        key_0 as *const ::core::ffi::c_char,
                        json_string_new_from_bytes(&glyph.name),
                    );
                    sdsfree(key_0);
                }
            }
            json_object_push(
                root,
                b"cmap_uvs\0" as *const u8 as *const ::core::ffi::c_char,
                uvs,
            );
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
#[inline]
unsafe extern "C" fn parse_unicode(unicode_str: SdsRaw) -> Unicode {
    if sdslen(unicode_str) > 2 as usize
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
unsafe extern "C" fn parse_cmap_unicodes(
    mut cmap: *mut CmapTable,
    mut table: *const ParsedValue,
    mut options: *const Options,
) {
    if table.is_null()
        || json_type_of(table) != JsonType::Object
    {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < json_obj_len(table) as u32 {
        let mut unicode_str: SdsRaw = sdsnewlen(
            json_obj_key_at(table, j as u32) as *const ::core::ffi::c_void,
            json_obj_key_len_at(table, j as u32) as usize,
        );
        let mut item: *const ParsedValue = json_obj_val_at(table, j as u32);
        let mut unicode: Unicode = parse_unicode(unicode_str);
        sdsfree(unicode_str);
        if json_type_of(item) == JsonType::String
            && unicode > 0 as Unicode
            && unicode <= 0x10ffff as Unicode
        {
            let mut gname: SdsRaw = sdsnewlen(
                json_str_ptr(item) as *const ::core::ffi::c_void,
                json_str_len(item) as usize,
            );
            if !otfcc_encode_cmap_by_name(cmap, unicode as ::core::ffi::c_int, gname) {
                let mut current_map: *mut GlyphHandle =
                    otfcc_cmap_lookup(cmap, unicode as ::core::ffi::c_int) as *mut GlyphHandle;
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(b"U+",
                        Hex4Upper(unicode as u32),
                        b" is already mapped to ",
                        &(*current_map).name,
                        b". Assignment to ",
                        gname,
                        b" is ignored.",
                    ),
                );
            }
        }
        j = j.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn parse_uvs_key(uvs_str: SdsRaw) -> CmapUvsKey {
    let mut len: usize = sdslen(uvs_str);
    let mut k: CmapUvsKey = CmapUvsKey {
        unicode: 0 as u32,
        selector: 0 as u32,
    };
    let mut scan: SdsRaw = uvs_str;
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
unsafe extern "C" fn parse_cmap_uvs(
    mut cmap: *mut CmapTable,
    mut table: *const ParsedValue,
    mut options: *const Options,
) {
    if table.is_null()
        || json_type_of(table) != JsonType::Object
    {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < json_obj_len(table) as u32 {
        let mut uvs_str: SdsRaw = sdsnewlen(
            json_obj_key_at(table, j as u32) as *const ::core::ffi::c_void,
            json_obj_key_len_at(table, j as u32) as usize,
        );
        let mut k: CmapUvsKey = parse_uvs_key(uvs_str);
        let mut item: *const ParsedValue = json_obj_val_at(table, j as u32);
        if json_type_of(item) == JsonType::String
            && k.unicode > 0 as u32
            && k.unicode <= 0x10ffff as u32
            && k.selector > 0 as u32
            && k.selector <= 0x10ffff as u32
        {
            let mut gname: SdsRaw = sdsnewlen(
                json_str_ptr(item) as *const ::core::ffi::c_void,
                json_str_len(item) as usize,
            );
            if !otfcc_encode_cmap_uvs_by_name(cmap, k, gname) {
                let mut current_map: *mut GlyphHandle =
                    otfcc_cmap_lookup_uvs(cmap, k) as *mut GlyphHandle;
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(b"UVS U+",
                        Hex4Upper((k.unicode) as u32),
                        b" U+",
                        Hex4Upper((k.selector) as u32),
                        b" is already mapped to ",
                        &(*current_map).name,
                        b". Assignment to ",
                        gname,
                        b" is ignored.",
                    ),
                );
            }
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe extern "C" fn otfcc_parse_cmap(
    mut root: *const ParsedValue,
    mut options: *const Options,
) -> Option<Box<CmapTable>> {
    if json_type_of(root) != JsonType::Object
    {
        return None;
    }
    let mut cmap_box: Box<CmapTable> = Box::new(CmapTable {
        unicodes: std::collections::BTreeMap::new(),
        uvs: std::collections::BTreeMap::new(),
    });
    let cmap: *mut CmapTable = cmap_box.as_mut() as *mut CmapTable;
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
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
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
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
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    return Some(cmap_box);
}
unsafe extern "C" fn otfcc_build_cmap_format4(mut cmap: *const CmapTable) -> *mut Buffer {
    let mut buf: *mut Buffer = bufnew();
    let mut end_count: *mut Buffer = bufnew();
    let mut start_count: *mut Buffer = bufnew();
    let mut id_delta: *mut Buffer = bufnew();
    let mut id_range_offset: *mut Buffer = bufnew();
    let mut glyph_id_array: *mut Buffer = bufnew();
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
                && !(glyph.index as ::core::ffi::c_int
                    != last_gid_end + 1 as ::core::ffi::c_int
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
                    && glyph.index as ::core::ffi::c_int
                        == last_gid_end + 1 as ::core::ffi::c_int;
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
    let mut i: u32 = 0;
    let mut j_1: u32 = 0;
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
        ((2 as ::core::ffi::c_int * segments_count as ::core::ffi::c_int) as u32)
            .wrapping_sub(i) as u16,
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
unsafe extern "C" fn otfcc_try_build_cmap_format4(mut cmap: *const CmapTable) -> *mut Buffer {
    let mut buf: *mut Buffer = otfcc_build_cmap_format4(cmap);
    if buflen(buf) > UINT16_MAX as usize {
        buffree(buf);
        return ::core::ptr::null_mut::<Buffer>();
    } else {
        return buf;
    };
}
unsafe extern "C" fn otfcc_build_cmap_format12(mut cmap: *const CmapTable) -> *mut Buffer {
    let mut buf: *mut Buffer = bufnew();
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
unsafe extern "C" fn write_default_range(
    mut dflt: *mut Buffer,
    mut n_ranges: *mut u32,
    mut start: Unicode,
    mut end: Unicode,
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
unsafe extern "C" fn build_format14_for_selector(
    mut cmap: *const CmapTable,
    mut selector: Unicode,
    mut dflt: *mut Buffer,
    mut nondflt: *mut Buffer,
) -> u8 {
    let mut defaults: *mut GlyphId = ::core::ptr::null_mut::<GlyphId>();
    let mut non_defaults: *mut GlyphId = ::core::ptr::null_mut::<GlyphId>();
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
        let mut u: Unicode = key.unicode as Unicode;
        if !(key.selector != selector || u >= MAX_UNICODE as Unicode) {
            if !glyph.name.is_empty() {
                let mut uvs_gid: GlyphId = glyph.index;
                let mut g: *mut GlyphHandle = otfcc_cmap_lookup(cmap, u as ::core::ffi::c_int);
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
        if *non_defaults.offset(u_0 as isize) as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int {
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
    defaults = ::core::ptr::null_mut::<GlyphId>();
    free(non_defaults as *mut ::core::ffi::c_void);
    non_defaults = ::core::ptr::null_mut::<GlyphId>();
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
unsafe extern "C" fn otfcc_build_cmap_format14(mut cmap: *const CmapTable) -> *mut Buffer {
    let mut valid_selectors: *mut bool = ::core::ptr::null_mut::<bool>();
    valid_selectors = __caryll_allocate_clean(
        (::core::mem::size_of::<bool>() as usize)
            .wrapping_mul(0x110001 as ::core::ffi::c_int as usize),
        681 as ::core::ffi::c_ulong,
    ) as *mut bool;
    for (key, _) in (*cmap).uvs.iter() {
        if key.selector < MAX_UNICODE as u32 {
            *valid_selectors.offset(key.selector as isize) = true;
        }
    }
    let mut n_selectors: u32 = 0 as u32;
    let mut selector: Unicode = 0 as Unicode;
    while selector < MAX_UNICODE as Unicode {
        if *valid_selectors.offset(selector as isize) {
            n_selectors = n_selectors.wrapping_add(1);
        }
        selector = selector.wrapping_add(1);
    }
    let mut st: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 14 as u32), bk_int(BkCellType::B32, 0 as u32), bk_int(BkCellType::B32, n_selectors as u32)]);
    let mut selector_0: Unicode = 0 as Unicode;
    while selector_0 < MAX_UNICODE as Unicode {
        if *valid_selectors.offset(selector_0 as isize) {
            let mut dflt: *mut Buffer = bufnew();
            let mut nondflt: *mut Buffer = bufnew();
            let mut results: u8 = build_format14_for_selector(cmap, selector_0, dflt, nondflt);
            if results as ::core::ffi::c_int & HAS_DEFAULT == 0 {
                buffree(dflt);
                dflt = ::core::ptr::null_mut::<Buffer>();
            }
            if results as ::core::ffi::c_int & HAS_NON_DEFAULT == 0 {
                buffree(nondflt);
                nondflt = ::core::ptr::null_mut::<Buffer>();
            }
            bk_push(st, &[bk_int(BkCellType::B8, (selector_0 >> 16 as ::core::ffi::c_int & 0xff as Unicode) as u32), bk_int(BkCellType::B8, (selector_0 >> 8 as ::core::ffi::c_int & 0xff as Unicode) as u32), bk_int(BkCellType::B8, (selector_0 >> 0 as ::core::ffi::c_int & 0xff as Unicode) as u32), bk_ptr(BkCellType::P32, bk_new_block_from_buffer(dflt)), bk_ptr(BkCellType::P32, bk_new_block_from_buffer(nondflt))]);
        }
        selector_0 = selector_0.wrapping_add(1);
    }
    let mut buf: *mut Buffer = bk_build_block(st);
    bufseek(buf, 2 as usize);
    bufwrite32b(buf, buflen(buf) as u32);
    return buf;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_cmap(
    cmap: Option<&CmapTable>,
    mut options: *const Options,
) -> *mut Buffer {
    let cmap = match cmap {
        Some(c) if !c.unicodes.is_empty() => c as *const CmapTable,
        _ => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut requires_format12: bool = false;
    let mut has_uvs: bool = !(*cmap).uvs.is_empty();
    for (&unicode, _) in (*cmap).unicodes.iter() {
        if unicode > 0xffff as ::core::ffi::c_int {
            requires_format12 = true;
        }
    }
    let mut format4: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
    if !requires_format12 || !(*options).stub_cmap4 {
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
    let mut format12: *mut Buffer = otfcc_build_cmap_format12(cmap);
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 0 as u32), bk_int(BkCellType::B16, (n_tables as ::core::ffi::c_int) as u32)]);
    bk_push(root, &[bk_int(BkCellType::B16, 0 as u32), bk_int(BkCellType::B16, 3 as u32), bk_ptr(BkCellType::P32, bk_new_block_from_buffer_copy(format4))]);
    if requires_format12 {
        bk_push(root, &[bk_int(BkCellType::B16, 0 as u32), bk_int(BkCellType::B16, 4 as u32), bk_ptr(BkCellType::P32, bk_new_block_from_buffer_copy(format12))]);
    }
    if has_uvs {
        let mut format14: *mut Buffer = otfcc_build_cmap_format14(cmap);
        bk_push(root, &[bk_int(BkCellType::B16, 0 as u32), bk_int(BkCellType::B16, 5 as u32), bk_ptr(BkCellType::P32, bk_new_block_from_buffer(format14))]);
    }
    bk_push(root, &[bk_int(BkCellType::B16, 3 as u32), bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P32, bk_new_block_from_buffer_copy(format4))]);
    if requires_format12 {
        bk_push(root, &[bk_int(BkCellType::B16, 3 as u32), bk_int(BkCellType::B16, 10 as u32), bk_ptr(BkCellType::P32, bk_new_block_from_buffer_copy(format12))]);
    }
    buffree(format4);
    buffree(format12);
    return bk_build_block(root);
}
