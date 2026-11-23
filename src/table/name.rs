use crate::font::sfnt::Packet;
use crate::support::base64::{base64_decode, base64_encode};
use crate::support::unicode::unicodeconv::{utf8toutf16be, utf16be_to_utf8};
use crate::version::{MAIN_VER, PATCH_VER, SECONDARY_VER};
use otfcc_binary::Buffer;
use otfcc_binary::{FontReader, ReadError};
use otfcc_json::BuiltValue;
use otfcc_json::JsonType;
use otfcc_json::ParsedValue;

// `Copy` dropped (`name_string` is now `Vec<u8>`, the `sds` sweep's last
// leaf field) -- `Clone` alone is enough, and nothing relied on
// `Vec<NameRecord>: Clone` doing a deep copy (the whole-table `.copy` slot
// was already dead before this conversion, per the earlier note this
// replaces).
/// One `name` record. How `name_string` is held depends on the record: UTF-8
/// text for the Unicode-encoded ones (platform 0, platform 2 encoding 1,
/// platform 3 encodings 0, 1 and 10), the raw bytes for platform 1 encoding 0
/// language 0, and base64 text for everything else.
#[derive(Clone, Debug)]
pub struct NameRecord {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub language_id: u16,
    pub name_id: u16,
    pub name_string: Vec<u8>,
}
pub type NameTable = Vec<NameRecord>;
pub const COPYRIGHT_LEN: usize = 32;
fn should_decode_as_utf16(record: &NameRecord) -> bool {
    record.platform_id == 0
        || record.platform_id == 2 && record.encoding_id == 1
        || record.platform_id == 3 && matches!(record.encoding_id, 0 | 1 | 10)
}
fn should_decode_as_bytes(record: &NameRecord) -> bool {
    record.platform_id == 1 && record.encoding_id == 0 && record.language_id == 0
}
// The record *array* (12 bytes/record starting at offset 6) was already
// guarded (`length < 6 + 12 * count`) -- but each record's *string*, read
// from `string_offset + offset` for `length_0` bytes, was not: nothing
// checked that span against the table's real length before handing it to
// `slice::from_raw_parts`/`utf16be_to_utf8`/`base64_encode`, any of which
// could read arbitrarily far past the table depending on the record's
// declared platform/encoding. Resolved through `FontReader::at` +
// `peek_bytes` before any of those three read the bytes; a record whose
// string span doesn't fit keeps its `platform_id`/`encoding_id`/
// `language_id`/`name_id` (still meaningful metadata) but gets an empty
// `name_string` instead of the out-of-bounds read -- the same "keep the
// record, drop only what doesn't fit" choice `table/post.rs::decode_post`
// made for an out-of-range `glyphNameIndex`.
fn decode_name(data: &[u8]) -> Result<NameTable, ReadError> {
    let mut header = FontReader::new(data);
    header.skip(2)?; // format, unused
    let count = u32::from(header.u16()?);
    let string_offset = u32::from(header.u16()?);
    // The record array itself must fit (`6 + 12 * count`, via
    // `checked_mul`/`checked_add`, so an overflowing `count` fails the
    // check instead of wrapping past it).
    FontReader::new(data)
        .at(6)?
        .require_room(count as usize, 12)?;

    let mut name: NameTable = Vec::with_capacity(count as usize);
    for j in 0..count {
        let mut r = FontReader::new(data).at(6usize + (j as usize) * 12)?;
        let mut record = NameRecord {
            platform_id: r.u16()?,
            encoding_id: r.u16()?,
            language_id: r.u16()?,
            name_id: r.u16()?,
            name_string: Vec::new(),
        };
        let length_0 = r.u16()?;
        let offset = r.u16()?;

        let string_bytes = (string_offset as usize)
            .checked_add(offset as usize)
            .and_then(|start| FontReader::new(data).at(start).ok())
            .and_then(|sr| sr.peek_bytes(length_0 as usize).ok());
        if let Some(bytes) = string_bytes {
            if should_decode_as_bytes(&record) {
                record.name_string = bytes.to_vec();
            } else if should_decode_as_utf16(&record) {
                record.name_string = utf16be_to_utf8(bytes);
            } else {
                record.name_string = base64_encode(bytes);
            }
        }
        name.push(record);
    }
    Ok(name)
}

/// Reads the `name` table from `packet`. Returns `None` if it is absent or its
/// header or record array does not fit (after logging "corrupted"). A record
/// whose string does not fit is kept with an empty `name_string`.
pub fn read_name(packet: &Packet) -> Option<NameTable> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_NAME)?;
    match decode_name(&table.data) {
        Ok(name) => Some(name),
        Err(_) => {
            tracing::warn!("table 'name' corrupted.\n");
            None
        }
    }
}
/// Adds a `name` array of records to `root`; does nothing if `name` is `None`.
pub fn dump_name(name: Option<&NameTable>, root: &mut BuiltValue) {
    let name = match name {
        Some(n) => n,
        None => return,
    };
    let stage = crate::logger::stage("name");
    let records: &Vec<NameRecord> = name;
    {
        let mut _name = BuiltValue::new_array(records.len());
        for r in records.iter() {
            let mut record = BuiltValue::new_object(5);
            record.push_field(b"platformID", BuiltValue::Int(i64::from(r.platform_id)));
            record.push_field(b"encodingID", BuiltValue::Int(i64::from(r.encoding_id)));
            record.push_field(b"languageID", BuiltValue::Int(i64::from(r.language_id)));
            record.push_field(b"nameID", BuiltValue::Int(i64::from(r.name_id)));
            record.push_field(b"nameString", BuiltValue::Str(r.name_string.clone()));
            _name.push_item(record);
        }
        root.push_field(b"name", _name);
        stage.finish();
    }
}
/// Reads the `name` array from JSON. Always returns `Some`, empty when the key
/// is missing. A record with a missing or mistyped field is skipped with a
/// warning, and the rest are sorted by platform, encoding, language and name
/// ID.
pub fn parse_name(root: &ParsedValue) -> Option<NameTable> {
    let mut name: NameTable = Vec::new();
    let Some(items) = root
        .get_typed(b"name", JsonType::Array)
        .and_then(ParsedValue::as_array)
    else {
        return Some(name);
    };
    let stage = crate::logger::stage("name");
    for (j, _record) in items.iter().enumerate() {
        let j = j as u32;
        if _record.as_object().is_none() {
            continue;
        }
        if _record
            .get_typed(b"platformID", JsonType::Integer)
            .is_none()
        {
            tracing::warn!("Missing or invalid platformID for name entry {}\n", j);
        } else if _record
            .get_typed(b"encodingID", JsonType::Integer)
            .is_none()
        {
            tracing::warn!("Missing or invalid encodingID for name entry {}\n", j);
        } else if _record
            .get_typed(b"languageID", JsonType::Integer)
            .is_none()
        {
            tracing::warn!("Missing or invalid languageID for name entry {}\n", j);
        } else if _record.get_typed(b"nameID", JsonType::Integer).is_none() {
            tracing::warn!("Missing or invalid nameID for name entry {}\n", j);
        } else if _record.get_typed(b"nameString", JsonType::String).is_none() {
            tracing::warn!("Missing or invalid name string for name entry {}\n", j);
        } else {
            let record: NameRecord = NameRecord {
                platform_id: _record.get_int(b"platformID") as u16,
                encoding_id: _record.get_int(b"encodingID") as u16,
                language_id: _record.get_int(b"languageID") as u16,
                name_id: _record.get_int(b"nameID") as u16,
                name_string: _record
                    .get_bytes(b"nameString")
                    .map(|b| b.to_vec())
                    .unwrap_or_default(),
            };
            name.push(record);
        }
    }
    name.sort_by(|a, b| {
        a.platform_id
            .cmp(&b.platform_id)
            .then(a.encoding_id.cmp(&b.encoding_id))
            .then(a.language_id.cmp(&b.language_id))
            .then(a.name_id.cmp(&b.name_id))
    });
    stage.finish();
    Some(name)
}
/// Serialises a format 0 `name` table, or `None` for no table. Strings are
/// encoded back by record kind (see [`NameRecord`]); a base64 string that does
/// not decode becomes empty. A 32-byte "-- By OTFCC x.y.z --" signature is
/// appended after the strings. The string offsets, lengths and the offset to
/// the strings are 16 bits, so a record that would not fit (past 65,535
/// bytes of strings, or past the 5,460 records whose header still fits) is
/// left out with a warning instead of being written with a wrapped offset.
pub fn build_name(name: Option<&NameTable>) -> Option<Buffer> {
    let records: &Vec<NameRecord> = name?;
    // 6 + 12 * count must itself be a 16-bit offset.
    const MAX_RECORDS: usize = (0xffff - 6) / 12;
    let mut strings = Buffer::new();
    let mut written: Vec<(&NameRecord, u16, u16)> = Vec::with_capacity(records.len());
    let mut left_out = 0usize;
    for record in records.iter() {
        let encoded: Vec<u8> = if should_decode_as_utf16(record) {
            utf8toutf16be(&record.name_string)
        } else if should_decode_as_bytes(record) {
            record.name_string.clone()
        } else {
            base64_decode(&record.name_string).unwrap_or_default()
        };
        let offset = strings.pos();
        if written.len() >= MAX_RECORDS || offset > 0xffff || encoded.len() > 0xffff {
            left_out += 1;
            continue;
        }
        strings.write_bytes(&encoded);
        written.push((record, encoded.len() as u16, offset as u16));
    }
    if left_out > 0 {
        tracing::warn!(
            "[name] {left_out} records do not fit in the 16-bit offsets of a 'name' table and are left out."
        );
    }
    let mut buf = Buffer::new();
    buf.write_u16be(0);
    buf.write_u16be(written.len() as u16);
    buf.write_u16be(0);
    for (record, length, offset) in written {
        buf.write_u16be(record.platform_id);
        buf.write_u16be(record.encoding_id);
        buf.write_u16be(record.language_id);
        buf.write_u16be(record.name_id);
        buf.write_u16be(length);
        buf.write_u16be(offset);
    }
    let mut copyright: Vec<u8> = crate::bytesbuild!(
        b"-- By OTFCC ",
        MAIN_VER,
        b".",
        SECONDARY_VER,
        b".",
        PATCH_VER,
        b" --",
    );
    // Truncated or NUL-padded to exactly COPYRIGHT_LEN bytes.
    copyright.resize(COPYRIGHT_LEN, 0);
    strings.write_bytes(&copyright);
    let strings_offset = buf.pos();
    buf.write_buffer_owned(strings);
    buf.seek(4);
    buf.write_u16be(strings_offset as u16);
    Some(buf)
}

#[cfg(test)]
mod parse_name_tests {
    use super::*;

    // header: format(2, unused) + count(u16) + string_offset(u16)
    fn header(count: u16, string_offset: u16) -> Vec<u8> {
        let mut b = vec![0u8, 0u8];
        b.extend_from_slice(&count.to_be_bytes());
        b.extend_from_slice(&string_offset.to_be_bytes());
        b
    }

    // record: platform(2) + encoding(2) + language(2) + name_id(2) + length(2) + offset(2)
    fn record(
        platform_id: u16,
        encoding_id: u16,
        language_id: u16,
        name_id: u16,
        length: u16,
        offset: u16,
    ) -> Vec<u8> {
        let mut b = Vec::new();
        for v in [
            platform_id,
            encoding_id,
            language_id,
            name_id,
            length,
            offset,
        ] {
            b.extend_from_slice(&v.to_be_bytes());
        }
        b
    }

    #[test]
    fn mac_roman_bytes_decode_directly() {
        // platform 1, encoding 0, language 0 -> should_decode_as_bytes.
        let mut data = header(1, 6 + 12);
        data.extend(record(1, 0, 0, 0, 5, 0));
        data.extend_from_slice(b"Hello");
        let name = decode_name(&data).unwrap();
        assert_eq!(name.len(), 1);
        assert_eq!(name[0].name_string, b"Hello");
    }

    #[test]
    fn windows_unicode_decodes_utf16be() {
        // platform 3, encoding 1 -> should_decode_as_utf16. "Hi" in UTF-16BE.
        let mut data = header(1, 6 + 12);
        data.extend(record(3, 1, 0x0409, 0, 4, 0));
        data.extend_from_slice(&[0x00, b'H', 0x00, b'i']);
        let name = decode_name(&data).unwrap();
        assert_eq!(name[0].name_string, b"Hi");
    }

    #[test]
    fn truncated_header_errs() {
        assert!(decode_name(&[0, 0, 0, 1]).is_err());
    }

    #[test]
    fn record_array_shorter_than_declared_count_errs() {
        // count says 2 records (24 bytes) but only one (12 bytes) is present.
        let mut data = header(2, 6 + 24);
        data.extend(record(1, 0, 0, 0, 0, 0));
        assert!(decode_name(&data).is_err());
    }

    #[test]
    fn count_large_enough_to_overflow_the_multiplication_errs() {
        let data = header(0xFFFF, 0);
        assert!(decode_name(&data).is_err());
    }

    #[test]
    fn string_span_past_the_table_end_keeps_the_record_with_an_empty_name() {
        // The record array bound is fine, but a record's *string* span
        // (string_offset + offset, for `length` bytes) must also fit.
        // Declares a 100-byte Mac-Roman string where only 5 bytes of table
        // remain.
        let mut data = header(1, 6 + 12);
        data.extend(record(1, 0, 0, 7, 100, 0));
        data.extend_from_slice(b"Hello"); // only 5 bytes actually present
        let name = decode_name(&data).unwrap();
        assert_eq!(name.len(), 1); // record kept
        assert_eq!(name[0].name_id, 7); // metadata preserved
        assert!(name[0].name_string.is_empty()); // string dropped, not read OOB
    }

    #[test]
    fn string_offset_itself_past_the_table_end_keeps_the_record_with_an_empty_name() {
        let mut data = header(1, 0xFFFF); // string_offset far past the table
        data.extend(record(1, 0, 0, 0, 1, 0));
        let name = decode_name(&data).unwrap();
        assert_eq!(name.len(), 1);
        assert!(name[0].name_string.is_empty());
    }

    #[test]
    fn zero_length_string_is_empty_not_an_error() {
        let mut data = header(1, 6 + 12);
        data.extend(record(1, 0, 0, 0, 0, 0));
        let name = decode_name(&data).unwrap();
        assert_eq!(name[0].name_string, Vec::<u8>::new());
    }

    // String offsets are 16 bits. Records whose strings start past 65,535
    // bytes used to be written with a wrapped offset, pointing at another
    // record's string; they are left out now.
    #[test]
    fn records_past_the_16_bit_string_offsets_are_left_out() {
        let record = |name_id: u16| NameRecord {
            platform_id: 3,
            encoding_id: 1,
            language_id: 0x409,
            name_id,
            name_string: vec![b'a'; 20_000],
        };
        // Each string is 40,000 bytes in UTF-16: the third starts at 80,000.
        let table: NameTable = vec![record(1), record(2), record(3)];
        let data = build_name(Some(&table)).unwrap().data;
        assert_eq!(u16::from_be_bytes([data[2], data[3]]), 2);
        let offsets: Vec<u16> = (0..2)
            .map(|i| u16::from_be_bytes([data[6 + 12 * i + 10], data[6 + 12 * i + 11]]))
            .collect();
        assert_eq!(offsets, [0, 40_000]);
    }
}
