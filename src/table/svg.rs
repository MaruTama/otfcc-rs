use crate::font::sfnt::Packet;
use crate::support::base64::{base64_decode, base64_encode};
use crate::support::primitives::GlyphId;
use otfcc_binary::Buffer;
use otfcc_binary::bk::block::bk_new_block_from_buffer_copy;
use otfcc_binary::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use otfcc_binary::bk::graph::bk_build_block;
use otfcc_binary::{FontReader, ReadError};
use otfcc_json::BuiltValue;
use otfcc_json::JsonType;
use otfcc_json::ParsedValue;

/// One SVG document and the range of glyph ids (`start` to `end`, inclusive) it
/// draws.
#[derive(Debug)]
pub struct SvgAssignment {
    pub start: GlyphId,
    pub end: GlyphId,
    pub document: Vec<u8>,
}
/// The 'SVG ' table: the document assignments in the font.
pub type SvgTable = Vec<SvgAssignment>;
#[inline]
fn svg_assignment_empty() -> SvgAssignment {
    SvgAssignment {
        start: 0,
        end: 0,
        document: Vec::new(),
    }
}
/// A deep copy, document included.
fn svg_assignment_dup(src: &SvgAssignment) -> SvgAssignment {
    let mut dst: SvgAssignment = svg_assignment_empty();
    dst.start = src.start;
    dst.end = src.end;
    dst.document = src.document.clone();
    dst
}
/// `offset_to_svg_doc_index`, `docstart` and `doclen` are `u32`s from the
/// file, so each document's span is added up with checked arithmetic
/// (`FontReader::sub`): wrapping sums could otherwise pass the length check.
fn decode_svg(data: &[u8]) -> Result<SvgTable, ReadError> {
    if data.len() < 10 {
        return Err(ReadError {
            needed: 10,
            available: data.len(),
        });
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
        svg.push(SvgAssignment {
            start,
            end,
            document,
        });
    }
    Ok(svg)
}
/// Reads the 'SVG ' table from `packet`. Returns `None` when the table is absent
/// or corrupted.
pub fn read_svg(packet: &Packet) -> Option<SvgTable> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_SVG)?;
    decode_svg(&table.data).ok()
}
fn can_use_plain_format(doc: &[u8]) -> bool {
    doc.len() > 4
        && i32::from(doc[0]) == '<' as i32
        && i32::from(doc[1]) == 's' as i32
        && i32::from(doc[2]) == 'v' as i32
        && i32::from(doc[3]) == 'g' as i32
        || doc.len() > 5
            && i32::from(doc[0]) == '<' as i32
            && i32::from(doc[1]) == '?' as i32
            && i32::from(doc[2]) == 'x' as i32
            && i32::from(doc[3]) == 'm' as i32
            && i32::from(doc[4]) == 'l' as i32
}
/// Dumps the table as the `"SVG_"` array. A document that starts like an SVG or
/// XML text is written as `plain`; anything else (for example a compressed
/// document) is written as `base64`.
pub fn dump_svg(svg: Option<&SvgTable>, root: &mut BuiltValue) {
    let svg = match svg {
        Some(s) => s,
        None => return,
    };
    let stage = crate::logger::stage("SVG ");
    let entries: &Vec<SvgAssignment> = svg;
    {
        let mut _svg = BuiltValue::new_array(entries.len());
        for a in entries.iter() {
            let mut _a = BuiltValue::new_object(4);
            _a.push_field(b"start", BuiltValue::Int(i64::from(a.start)));
            _a.push_field(b"end", BuiltValue::Int(i64::from(a.end)));
            if can_use_plain_format(&a.document) {
                _a.push_field(b"format", BuiltValue::Str(b"plain".to_vec()));
                _a.push_field(b"document", BuiltValue::Str(a.document.clone()));
            } else {
                let encoded = base64_encode(&a.document);
                _a.push_field(b"format", BuiltValue::Str(b"base64".to_vec()));
                _a.push_field(b"document", BuiltValue::Str(encoded));
            }
            _svg.push_item(_a);
        }
        root.push_field(b"SVG_", _svg);
        stage.finish();
    }
}
/// Reads the `"SVG_"` array. `base64` documents are decoded (an invalid one is
/// skipped with a warning), `plain` documents are taken as they are. Entries that
/// are not objects or lack `format` or `document` are skipped.
pub fn parse_svg(root: &ParsedValue) -> Option<SvgTable> {
    let svg_val = root.get_typed(b"SVG_", JsonType::Array)?;
    let mut svg: SvgTable = Vec::new();
    let stage = crate::logger::stage("SVG ");
    {
        if let Some(items) = svg_val.as_array() {
            for a in items {
                if a.as_object().is_some() {
                    let format = a.get_bytes(b"format");
                    let doc = a.get_bytes_owned(b"document");
                    if let (Some(format), Some(doc)) = (format, doc) {
                        let mut asg: SvgAssignment = svg_assignment_empty();
                        asg.start = a.get_int(b"start") as GlyphId;
                        asg.end = a.get_int(b"end") as GlyphId;
                        if format == b"plain" {
                            asg.document = doc;
                        } else if let Some(decoded) = base64_decode(&doc) {
                            asg.document = decoded;
                        } else {
                            tracing::warn!(
                                "[OTFCC-svg] Ignoring an SVG document whose base64 is invalid."
                            );
                            continue;
                        }
                        svg.push(asg);
                    }
                }
            }
        }
        stage.finish();
    }
    Some(svg)
}
/// Serializes the table, with the documents sorted by their first glyph id.
/// Returns `None` when there are none.
pub fn build_svg(_svg: Option<&SvgTable>) -> Option<Buffer> {
    let _svg = match _svg {
        Some(s) if !s.is_empty() => s,
        _ => return None,
    };
    let mut svg: SvgTable = _svg.iter().map(svg_assignment_dup).collect();
    svg.sort_by_key(|a| a.start);
    let mut major: BkBlock = bk_new_block(vec![bk_int(BkCellType::B16, (svg.len()) as u32)]);
    for a in svg.iter() {
        // `bk_new_block_from_buffer_copy` takes a `Buffer`; this copies the
        // document once per SVG assignment.
        let doc_buf = Buffer::from_bytes(&a.document);
        bk_push(
            &mut major,
            vec![
                bk_int(BkCellType::B16, i32::from(a.start) as u32),
                bk_int(BkCellType::B16, i32::from(a.end) as u32),
                bk_ptr(
                    BkCellType::P32,
                    bk_new_block_from_buffer_copy(Some(&doc_buf)),
                ),
                bk_int(BkCellType::B32, (a.document.len()) as u32),
            ],
        );
    }
    let root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, 0),
        bk_ptr(BkCellType::P32, Some(major)),
        bk_int(BkCellType::B32, 0),
    ]);
    // `svg` drops naturally at the end of this scope -- `document` is a
    // plain `Vec<u8>` now, self-dropping along with the rest of
    // `SvgAssignment`, so no explicit disposal call is needed here.
    Some(bk_build_block(root))
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
        let svg = decode_svg(&data).unwrap();
        assert_eq!(svg.len(), 1);
        assert_eq!(svg[0].start, 5);
        assert_eq!(svg[0].end, 5);
        assert_eq!(svg[0].document, b"<svg/>");
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        assert!(decode_svg(&well_formed_svg_table()[..8]).is_err());
    }

    #[test]
    fn entry_count_larger_than_available_is_rejected_instead_of_reading_oob() {
        let mut data = well_formed_svg_table();
        data[10..12].copy_from_slice(&5u16.to_be_bytes()); // numEntries = 5, only 1 record present
        assert!(decode_svg(&data).is_err());
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
        let svg = decode_svg(&data).unwrap();
        assert_eq!(svg.len(), 1);
        assert!(svg[0].document.is_empty());
    }
}

#[cfg(test)]
mod parse_svg_json_tests {
    use super::*;
    use otfcc_json::parse_json;

    fn parse(json: &str) -> SvgTable {
        let root = parse_json(json.as_bytes()).expect("test JSON parses");
        parse_svg(&root).expect("SVG_ array is present")
    }

    #[test]
    fn base64_documents_are_decoded_not_encoded_again() {
        let svg = parse(
            r#"{"SVG_":[{"start":0,"end":1,"format":"base64","document":"PHN2Zz48L3N2Zz4="}]}"#,
        );
        assert_eq!(svg.len(), 1);
        assert_eq!(svg[0].document, b"<svg></svg>");
    }

    #[test]
    fn plain_documents_are_kept_as_is() {
        let svg = parse(r#"{"SVG_":[{"start":2,"end":3,"format":"plain","document":"<svg/>"}]}"#);
        assert_eq!(svg[0].document, b"<svg/>");
    }

    #[test]
    fn invalid_base64_drops_only_that_entry() {
        let svg = parse(
            r#"{"SVG_":[{"start":0,"end":0,"format":"base64","document":"abc"},
                        {"start":1,"end":1,"format":"plain","document":"<svg/>"}]}"#,
        );
        assert_eq!(svg.len(), 1);
        assert_eq!(svg[0].start, 1);
    }

    #[test]
    fn dump_then_parse_round_trips_a_binary_document() {
        let table: SvgTable = vec![SvgAssignment {
            start: 4,
            end: 6,
            document: vec![0x1f, 0x8b, 0x08, 0x00, 0xff],
        }];
        let mut root = BuiltValue::new_object(1);
        dump_svg(Some(&table), &mut root);
        let json = otfcc_json::json_serialize_ex(
            &root,
            otfcc_json::JsonSerializeOpts {
                mode: otfcc_json::JSON_SERIALIZE_MODE_SINGLE_LINE,
                opts: 0,
                indent_size: 0,
            },
        );
        let back = parse(&String::from_utf8(json).expect("JSON is UTF-8"));
        assert_eq!(back[0].document, table[0].document);
    }
}
