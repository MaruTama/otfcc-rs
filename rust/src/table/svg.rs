#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::bk::bkblock::bk_new_block_from_buffer_copy;
use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::bk::bkgraph::bk_build_block;
use crate::font::caryll_sfnt::Packet;
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::base64::base64_encode;
use crate::support::buffer::Buffer;
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new,
    json_object_push, json_string_new, json_string_new_length,
};
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_getint, json_obj_getsds,
    json_obj_getstr_share, json_type_of,
};
use crate::support::primitives::GlyphId;
use crate::vendor::json::JsonType;
use libc::{free, strcmp};

pub struct SvgAssignment {
    pub start: GlyphId,
    pub end: GlyphId,
    pub document: Vec<u8>,
}
// C由来の時点で素のベクタ形（ラッパー構造体なし）。要素の `document` はこの
// stage で `*mut Buffer`（`buffree` 所有）から `Vec<u8>` へ直接移行した
// （`Buffer` 自体はまだ libc アロケータのままだが、このフィールドに限り
// 経由せずに済ませる）。`Vec<u8>` は `Clone` を持つので `svg_assignment_dup`
// は素直な `.clone()` でディープコピーできる。
//
// Stage 6-4 "Box化": `Font.svg` becomes `Option<Vec<SvgAssignment>>` (not
// `Option<Box<Vec<...>>>` -- `Vec` already owns its own heap buffer, a
// second `Box` layer would be pure overhead). `document: Vec<u8>` now
// self-drops along with the rest of `SvgAssignment`, so no `Drop` impl is
// needed for this type any more.
pub type SvgTable = Vec<SvgAssignment>;
#[inline]
unsafe fn svg_assignment_empty() -> SvgAssignment {
    SvgAssignment {
        start: 0,
        end: 0,
        document: Vec::new(),
    }
}
/// 本物のディープコピー（`document` の `Vec<u8>` を複製する）。
unsafe fn svg_assignment_dup(src: &SvgAssignment) -> SvgAssignment {
    let mut dst: SvgAssignment = svg_assignment_empty();
    dst.start = src.start;
    dst.end = src.end;
    dst.document = src.document.clone();
    dst
}
/// `offset_to_svg_doc_index`, `docstart` and `doclen` are each a raw `u32`
/// read straight from the file (full attacker control, up to `u32::MAX`).
/// The original guarded the per-record document span with
/// `offset_to_svg_doc_index.wrapping_add(docstart).wrapping_add(doclen) <=
/// table.length` -- three chained 32-bit additions, any pair of which can
/// wrap the sum back down to something small enough to pass the check even
/// though the real (unwrapped) span reaches nowhere near this table. Same
/// shape as `table/cpal.rs`'s `offset_first_color_record` bug, just with
/// three operands chained instead of one. `FontReader::sub`'s
/// `checked_add` (used twice below, once per addition) closes it.
fn parse_svg(data: &[u8]) -> Result<SvgTable, ReadError> {
    if data.len() < 10 {
        return Err(ReadError { needed: 10, available: data.len() });
    }
    let offset_to_svg_doc_index = FontReader::new(data).at(2)?.u32()? as usize;
    let mut idx = FontReader::new(data).at(offset_to_svg_doc_index)?;
    let num_entries = idx.u16()?;
    idx.require_room(num_entries as usize, 12)?;

    let mut svg: SvgTable = Vec::new();
    for _ in 0..num_entries {
        let start = idx.u16()? as GlyphId;
        let end = idx.u16()? as GlyphId;
        let docstart = idx.u32()? as usize;
        let doclen = idx.u32()? as usize;
        let document = offset_to_svg_doc_index
            .checked_add(docstart)
            .and_then(|abs| FontReader::new(data).sub(abs, doclen).ok())
            .and_then(|mut r| r.bytes(doclen).ok())
            .map(|s| s.to_vec())
            .unwrap_or_default();
        svg.push(SvgAssignment { start, end, document });
    }
    Ok(svg)
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_read_svg(packet: &Packet) -> Option<SvgTable> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_SVG)?;
    parse_svg(&table.data).ok()
}
fn can_use_plain_format(doc: &[u8]) -> bool {
    return doc.len() > 4_usize
        && doc[0_usize] as ::core::ffi::c_int == '<' as i32
        && doc[1_usize] as ::core::ffi::c_int == 's' as i32
        && doc[2_usize] as ::core::ffi::c_int == 'v' as i32
        && doc[3_usize] as ::core::ffi::c_int == 'g' as i32
        || doc.len() > 5_usize
            && doc[0_usize] as ::core::ffi::c_int == '<' as i32
            && doc[1_usize] as ::core::ffi::c_int == '?' as i32
            && doc[2_usize] as ::core::ffi::c_int == 'x' as i32
            && doc[3_usize] as ::core::ffi::c_int == 'm' as i32
            && doc[4_usize] as ::core::ffi::c_int == 'l' as i32;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_svg(svg: Option<&SvgTable>, root: *mut BuiltValue, options: &Options) {
    let svg = match svg {
        Some(s) => s,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"SVG "),
    );
    let entries: &Vec<SvgAssignment> = svg;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _svg: *mut BuiltValue = json_array_new(entries.len());
        let mut __caryll_index: usize = 0_usize;
        let mut keep: usize = 1_usize;
        while keep != 0 && __caryll_index < entries.len() {
            let a: &SvgAssignment = &entries[__caryll_index];
            while keep != 0 {
                let mut _a: *mut BuiltValue = json_object_new(4_usize);
                json_object_push(
                    _a,
                    b"start\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*a).start as i64),
                );
                json_object_push(
                    _a,
                    b"end\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*a).end as i64),
                );
                if can_use_plain_format(&a.document) {
                    json_object_push(
                        _a,
                        b"format\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new(b"plain\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                    json_object_push(
                        _a,
                        b"document\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new_length(
                            a.document.len() as ::core::ffi::c_uint,
                            a.document.as_ptr() as *mut ::core::ffi::c_char,
                        ),
                    );
                } else {
                    let mut len: usize = 0_usize;
                    let mut buf: *mut u8 =
                        base64_encode(a.document.as_ptr(), a.document.len(), &raw mut len);
                    json_object_push(
                        _a,
                        b"format\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new(b"base64\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                    json_object_push(
                        _a,
                        b"document\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new_length(
                            len as ::core::ffi::c_uint,
                            buf as *mut ::core::ffi::c_char,
                        ),
                    );
                    free(buf as *mut ::core::ffi::c_void);
                    buf = ::core::ptr::null_mut::<u8>();
                }
                json_array_push(_svg, _a);
                keep = (keep == 0) as ::core::ffi::c_int as usize;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        json_object_push(
            root,
            b"SVG_\0" as *const u8 as *const ::core::ffi::c_char,
            _svg,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_parse_svg(root: *const ParsedValue, options: &Options) -> Option<SvgTable> {
    let mut _svg: *const ParsedValue = ::core::ptr::null();
    _svg = json_obj_get_type(
        root,
        b"SVG_\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _svg.is_null() {
        return None;
    }
    let mut svg: SvgTable = Vec::new();
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"SVG "),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: GlyphId = 0 as GlyphId;
        while (j as ::core::ffi::c_uint) < json_arr_len(_svg) {
            let mut _a: *const ParsedValue = json_arr_at(_svg, j as u32);
            if !(_a.is_null() || json_type_of(_a) != JsonType::Object) {
                let format: *const ::core::ffi::c_char = json_obj_getstr_share(
                    _a,
                    b"format\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let doc: Option<Vec<u8>> =
                    json_obj_getsds(_a, b"document\0" as *const u8 as *const ::core::ffi::c_char);
                if !format.is_null() {
                    if let Some(doc) = doc {
                        let mut asg: SvgAssignment = svg_assignment_empty();
                        asg.start = json_obj_getint(
                            _a,
                            b"start\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as GlyphId;
                        asg.end = json_obj_getint(
                            _a,
                            b"end\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as GlyphId;
                        if strcmp(
                            format,
                            b"plain\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            asg.document = doc;
                        } else {
                            let mut len: usize = 0_usize;
                            let mut buf: *mut u8 =
                                base64_encode(doc.as_ptr(), doc.len(), &raw mut len);
                            asg.document = ::core::slice::from_raw_parts(buf, len).to_vec();
                            free(buf as *mut ::core::ffi::c_void);
                            buf = ::core::ptr::null_mut::<u8>();
                        }
                        svg.push(asg);
                    }
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    return Some(svg);
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_svg(_svg: Option<&SvgTable>) -> *mut Buffer {
    let _svg = match _svg {
        Some(s) if !s.is_empty() => s,
        _ => return ::core::ptr::null_mut::<Buffer>(),
    };
    // `TABLE_I_SVG.copy` の代わりに各要素を `svg_assignment_dup` で明示的に
    // ディープコピー（`ColrTable`/`TsiTable` の前例どおり `.clone()` は不可）。
    let mut svg: SvgTable = _svg.iter().map(|a| svg_assignment_dup(a)).collect();
    svg.sort_by(|a, b| a.start.cmp(&b.start));
    let major: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (svg.len()) as u32)]);
    let mut __caryll_index: usize = 0_usize;
    let mut keep: usize = 1_usize;
    while keep != 0 && __caryll_index < svg.len() {
        let a: &SvgAssignment = &svg[__caryll_index];
        while keep != 0 {
            // `bk_new_block_from_buffer_copy` still takes `*const Buffer`
            // (it has other callers, e.g. `table/cmap.rs`, so its signature
            // stays as-is); build a stack-local `Buffer` view over
            // `a.document`'s bytes for this one call. Stage 7-2-e made
            // `Buffer.data` an owned `Vec<u8>`, so unlike before this is a
            // real clone, not a zero-copy borrow -- correctness-preserving
            // and cheap enough (once per SVG assignment during build, not a
            // hot per-byte path).
            let doc_buf = Buffer {
                cursor: a.document.len(),
                data: a.document.clone(),
            };
            bk_push(
                major,
                &[
                    bk_int(BkCellType::B16, ((*a).start as ::core::ffi::c_int) as u32),
                    bk_int(BkCellType::B16, ((*a).end as ::core::ffi::c_int) as u32),
                    bk_ptr(
                        BkCellType::P32,
                        bk_new_block_from_buffer_copy(&doc_buf as *const Buffer),
                    ),
                    bk_int(BkCellType::B32, (a.document.len()) as u32),
                ],
            );
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B16, 0_u32),
        bk_ptr(BkCellType::P32, major),
        bk_int(BkCellType::B32, 0_u32),
    ]);
    // `svg` drops naturally at the end of this scope -- `document` is a
    // plain `Vec<u8>` now, self-dropping along with the rest of
    // `SvgAssignment`, so no explicit disposal call is needed here.
    return bk_build_block(root);
}

#[cfg(test)]
mod parse_svg_tests {
    use super::*;

    // header(10) + SVG Document Index (2 + one 12-byte record) + one document
    fn well_formed_svg_table() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&0u16.to_be_bytes()); // version
        b.extend_from_slice(&10u32.to_be_bytes()); // offsetToSVGDocumentIndex
        b.extend_from_slice(&0u32.to_be_bytes()); // reserved
        // SVG Document Index @10
        b.extend_from_slice(&1u16.to_be_bytes()); // numEntries
        b.extend_from_slice(&5u16.to_be_bytes()); // startGlyphID
        b.extend_from_slice(&5u16.to_be_bytes()); // endGlyphID
        b.extend_from_slice(&14u32.to_be_bytes()); // svgDocOffset (rel. to offset 10)
        b.extend_from_slice(&6u32.to_be_bytes()); // svgDocLength
        // document @24 (10 + 14)
        b.extend_from_slice(b"<svg/>");
        b
    }

    #[test]
    fn well_formed_table_reads_the_document() {
        let data = well_formed_svg_table();
        let svg = parse_svg(&data).unwrap();
        assert_eq!(svg.len(), 1);
        assert_eq!(svg[0].start, 5);
        assert_eq!(svg[0].end, 5);
        assert_eq!(svg[0].document, b"<svg/>");
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        assert!(parse_svg(&well_formed_svg_table()[..8]).is_err());
    }

    #[test]
    fn entry_count_larger_than_available_is_rejected_instead_of_reading_oob() {
        let mut data = well_formed_svg_table();
        data[10..12].copy_from_slice(&5u16.to_be_bytes()); // numEntries = 5, only 1 record present
        assert!(parse_svg(&data).is_err());
    }

    #[test]
    fn doc_offset_near_u32_max_falls_back_to_empty_document_not_oob() {
        // The original guarded the document span with
        // `offset_to_svg_doc_index.wrapping_add(docstart).wrapping_add
        // (doclen) <= table.length` -- a `docstart` this close to
        // u32::MAX wraps that sum back into range even though the real
        // span points nowhere near this table.
        let mut data = well_formed_svg_table();
        data[16..20].copy_from_slice(&0xFFFF_FFF0u32.to_be_bytes()); // svgDocOffset
        let svg = parse_svg(&data).unwrap();
        assert_eq!(svg.len(), 1);
        assert!(svg[0].document.is_empty());
    }
}
